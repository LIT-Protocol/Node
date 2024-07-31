use std::any::Any;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use blake3::derive_key;
use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

use lit_attestation::attestation::{
    DATA_KEY_BUILD_ID, DATA_KEY_FACILITY, DATA_KEY_INSTANCE_ID, DATA_KEY_RELEASE_ID,
    DATA_KEY_REQ_BODY_HASH, DATA_KEY_SUBNET_ID, DATA_KEY_UNIX_TIME, FACILITY_GUEST_INIT,
};
use lit_attestation::kdf::{Kdf, KdfType};
use lit_attestation::{Attestation, AttestationType};
use lit_core::config::{LitConfig, LitConfigBuilder};
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use lit_core::utils::hash::sha256;
use lit_os_core::config::LitOsGuestConfig;
use lit_os_core::error::{config_err, generic_err, validation_err, Result};
use lit_os_core::guest::cloud_init::context::CloudInitContext;
use lit_os_core::guest::env::build::{read_lit_os_build_env, GuestBuildEnv, LIT_OS_BUILD_FILE};
use lit_os_core::guest::env::cmdline::{read_guest_cmdline, GuestCmdLine, PROC_CMDLINE};
use lit_os_core::guest::env::release::GuestReleaseEnv;
use lit_os_core::guest::oneshot::config::ACTION_TYPE_BOOTSTRAP;
use lit_os_core::guest::oneshot::context::OneShotContext;
use lit_os_core::guest::types::GuestType;

use crate::init::stage::sync::{SyncItem, SyncItemAcl};

pub const CTX_KEY_FIRST_BOOT: &str = "FIRST_BOOT";
pub const CTX_KEY_ACTIVATED: &str = "ACTIVATED";
pub const CTX_KEY_SAFE_BOOT: &str = "SAFE_BOOT";
pub const CTX_KEY_NO_RESIZE: &str = "NO_RESIZE";
pub const CTX_KEY_PASSPHRASE_INIT: &str = "PASSPHRASE_INIT";
pub const CTX_KEY_ROOT_PASSPHRASE_BOOT: &str = "ROOT_PASSPHRASE_BOOT";
pub const CTX_KEY_VAR_PASSPHRASE_BOOT: &str = "VAR_PASSPHRASE_BOOT";
pub const CTX_KEY_CONTEXT_HASH: &str = "CONTEXT_HASH";
pub const CTX_KEY_CLOUD_INIT_CTX: &str = "CLOUD_INIT_CTX";
pub const CTX_KEY_ONESHOT_CTX: &str = "ONESHOT_CTX";

pub struct InitContext {
    cfg: LitConfig,
    build_env: GuestBuildEnv,
    cmdline_env: GuestCmdLine,
    release_env: Option<GuestReleaseEnv>,
    context: HashMap<String, Box<dyn Any>>,
    /// Collects files to write to the /var mount
    synced: Vec<SyncItem>,
    /// Collects file paths to delete from the /var mount
    purged: Vec<PathBuf>,
    kdf: Kdf,
}

impl InitContext {
    pub(crate) fn new(expect_instance_id: bool) -> Result<Self> {
        // Load config
        let cfg = load_cfg()?;

        // Load build env
        let build_env = read_lit_os_build_env()
            .map_err(|e| {
                generic_err(
                    e,
                    Some(format!("failed to load lit os build env from: {LIT_OS_BUILD_FILE}")),
                )
            })?
            .ok_or(generic_err(format!("lit os build env missing: {LIT_OS_BUILD_FILE}"), None))?;

        // Load cmdline env
        let cmdline_env = read_guest_cmdline()?
            .ok_or(generic_err(format!("lit os cmdline missing: {PROC_CMDLINE}"), None))?;

        // Set context
        let mut context: HashMap<String, Box<dyn Any>> = HashMap::new();
        if expect_instance_id {
            context.insert(
                CTX_KEY_CONTEXT_HASH.into(),
                Box::new(build_context_hash(&cfg, &cmdline_env)?),
            );
        }

        // Load KDF
        let kdf_type =
            KdfType::from_system().expect_or_err("failed to determine KDF type for this guest")?;
        let kdf = Kdf::try_new(&cfg, kdf_type, Some("lit-initrd".into()))
            .expect_or_err("failed to init KDF struct")?;

        Ok(Self {
            cfg,
            build_env,
            cmdline_env,
            release_env: None,
            context,
            synced: Vec::new(),
            purged: Vec::new(),
            kdf,
        })
    }

    pub fn cfg(&self) -> &LitConfig {
        &self.cfg
    }

    pub fn reload_cfg(&mut self, expect_instance_id: bool) -> Result<()> {
        self.cfg = load_cfg()?;
        if expect_instance_id {
            self.insert(
                CTX_KEY_CONTEXT_HASH,
                Box::new(build_context_hash(&self.cfg, &self.cmdline_env)?),
            );
        }

        Ok(())
    }

    pub fn build_env(&self) -> &GuestBuildEnv {
        &self.build_env
    }

    pub fn release_env(&self) -> Option<&GuestReleaseEnv> {
        self.release_env.as_ref()
    }

    pub fn set_release_env(&mut self, release_env: GuestReleaseEnv) {
        self.release_env = Some(release_env);
    }

    pub fn cmdline_env(&self) -> &GuestCmdLine {
        &self.cmdline_env
    }

    pub fn insert<K>(&mut self, key: K, value: Box<dyn Any>) -> Option<Box<dyn Any>>
    where
        K: Into<String>,
    {
        self.context.insert(key.into(), value)
    }

    pub fn get<K>(&self, key: K) -> Option<&Box<dyn Any>>
    where
        K: AsRef<str>,
    {
        self.context.get(key.as_ref())
    }

    pub fn get_bin<K>(&self, key: K) -> Option<&Vec<u8>>
    where
        K: AsRef<str>,
    {
        match self.context.get(key.as_ref()) {
            Some(val) => val.downcast_ref(),
            _ => None,
        }
    }

    pub fn get_bool<K>(&self, key: K) -> Option<&bool>
    where
        K: AsRef<str>,
    {
        match self.context.get(key.as_ref()) {
            Some(val) => val.downcast_ref(),
            _ => None,
        }
    }

    pub fn get_string<K>(&self, key: K) -> Option<&String>
    where
        K: AsRef<str>,
    {
        match self.context.get(key.as_ref()) {
            Some(val) => val.downcast_ref(),
            _ => None,
        }
    }

    pub fn get_cloud_init_ctx(&self) -> Option<&CloudInitContext> {
        match self.context.get(CTX_KEY_CLOUD_INIT_CTX) {
            Some(val) => val.downcast_ref(),
            _ => None,
        }
    }

    pub fn get_oneshot_ctx(&self) -> Option<&OneShotContext> {
        match self.context.get(CTX_KEY_ONESHOT_CTX) {
            Some(val) => val.downcast_ref(),
            _ => None,
        }
    }

    pub fn synced(&self) -> &Vec<SyncItem> {
        &self.synced
    }

    pub fn push_synced(&mut self, path: PathBuf, maybe_dest: Option<PathBuf>) {
        self.synced.push(SyncItem::new(path, maybe_dest, Vec::new()));
    }

    pub fn push_synced_acl(
        &mut self, path: PathBuf, maybe_dest: Option<PathBuf>, acls: Vec<SyncItemAcl>,
    ) {
        self.synced.push(SyncItem::new(path, maybe_dest, acls));
    }

    pub fn purged(&self) -> &Vec<PathBuf> {
        &self.purged
    }

    pub fn push_purged(&mut self, path: PathBuf) {
        self.purged.push(path);
    }

    // Accessors

    pub fn set_is_first_boot(&mut self, val: bool) {
        self.insert(CTX_KEY_FIRST_BOOT, Box::new(val));
    }

    pub fn is_first_boot(&self) -> bool {
        match self.get_bool(CTX_KEY_FIRST_BOOT) {
            Some(val) => *val,
            _ => false,
        }
    }

    pub fn set_activated(&mut self, val: bool) {
        self.insert(CTX_KEY_ACTIVATED, Box::new(val));
    }

    pub fn is_activated(&self) -> bool {
        match self.get_bool(CTX_KEY_ACTIVATED) {
            Some(val) => *val,
            _ => false,
        }
    }

    pub fn set_safe_boot(&mut self, val: bool) {
        self.insert(CTX_KEY_SAFE_BOOT, Box::new(val));
    }

    pub fn is_safe_boot(&self) -> bool {
        match self.get_bool(CTX_KEY_SAFE_BOOT) {
            Some(val) => *val,
            _ => false,
        }
    }

    pub fn set_no_resize(&mut self, val: bool) {
        self.insert(CTX_KEY_NO_RESIZE, Box::new(val));
    }

    pub fn is_no_resize(&self) -> bool {
        match self.get_bool(CTX_KEY_NO_RESIZE) {
            Some(val) => *val,
            _ => false,
        }
    }

    // Utility

    pub fn is_guest_type(&self, typ: GuestType) -> bool {
        if let Ok(guest_type) = self.build_env().guest_type() {
            return guest_type.eq(&typ);
        }

        false
    }

    pub fn has_oneshot_action(&self, action: &str) -> bool {
        if let Some(oneshot_ctx) = self.get_oneshot_ctx() {
            return oneshot_ctx.config().actions().get(action).is_some();
        }

        false
    }

    pub fn has_oneshot_actions(&self) -> bool {
        if let Some(oneshot_ctx) = self.get_oneshot_ctx() {
            return !oneshot_ctx.config().actions().is_empty();
        }

        false
    }

    pub fn is_release(&self) -> bool {
        if self.is_guest_type(GuestType::Prov) && self.has_oneshot_action(ACTION_TYPE_BOOTSTRAP) {
            // Do not treat prov bootstrap as a release (we aren't one yet).
            return false;
        }

        self.cmdline_env.release_id.is_some()
    }

    pub fn context_key(&self) -> Result<String> {
        let context_key = self
            .get_bin(CTX_KEY_CONTEXT_HASH)
            .ok_or_else(|| config_err(format!("missing context: {CTX_KEY_CONTEXT_HASH}"), None))?;

        Ok(bytes_to_hex(context_key))
    }

    pub async fn derive_key(&self, context: &str) -> Result<[u8; 32]> {
        self.kdf.derive(format!("{}:{}", self.context_key()?, context).as_str()).await
    }

    // Not the best, but I wanted a longer key for a passphrase.
    pub async fn passphrase_of_length(&self, context: &str, len: usize) -> Result<Vec<u8>> {
        let mut key: Vec<u8> = Vec::new();
        let mut seq = 0;
        while key.len() < len {
            let part = self.derive_key(format!("{context}-{seq}").as_str()).await?;

            if (part.len() + key.len()) <= len {
                key.extend_from_slice(&part);
            } else {
                for item in &part {
                    key.push(*item);

                    if key.len() >= len {
                        break;
                    }
                }
            }

            seq += 1;
        }

        Ok(key)
    }

    // Intended to mix "unsafe" (/dev/random isn't trusted) with "safe".
    pub async fn random_key(&self, salt: &str) -> Result<[u8; 32]> {
        // Derive a base key
        let base_key = self.derive_key(salt).await?;
        let mut base_key = base_key.to_vec();

        // Generate random bytes
        let mut rng = ChaChaRng::from_entropy();
        let mut random_bytes = [0u8; 32];
        rng.fill_bytes(&mut random_bytes);

        // Derive result from both
        base_key.extend_from_slice(&random_bytes);

        Ok(derive_key("final", &base_key[..]))
    }

    // Possibly not the best, I would prefer blake3 to give me a variable length key.
    pub async fn random_key_of_length(&self, salt: &str, len: usize) -> Result<Vec<u8>> {
        let mut key: Vec<u8> = Vec::new();
        let mut seq = 0;
        while key.len() < len {
            let part = self.random_key(format!("{salt}-{seq}").as_str()).await?;

            if (part.len() + key.len()) <= len {
                key.extend_from_slice(&part);
            } else {
                for item in &part {
                    key.push(*item);

                    if key.len() >= len {
                        break;
                    }
                }
            }

            seq += 1;
        }

        Ok(key)
    }

    // Attestation
    pub async fn request_attestation(
        &self, private_key: Option<&str>, mut req_hash: Option<Vec<u8>>,
    ) -> Result<Attestation> {
        // Collect data
        let unix_time =
            SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| generic_err(e, None))?;
        let instance_id = self.cfg().litos_guest_instance_id().map_err(|e| config_err(e, None))?;

        // Attest
        let noonce = self.random_key("attestation-noonce").await?.to_vec();
        if let Some(attestation_type) = AttestationType::from_system() {
            let mut attestation = Attestation::new(attestation_type, noonce).await?;
            attestation
                .insert_data(DATA_KEY_FACILITY, FACILITY_GUEST_INIT.as_bytes().to_vec())
                .insert_data(DATA_KEY_UNIX_TIME, unix_time.as_secs().to_le_bytes().to_vec())
                .insert_data(DATA_KEY_INSTANCE_ID, instance_id.as_bytes().to_vec());

            if let Ok(release_id) = self.cfg.litos_guest_release_id() {
                attestation.insert_data(DATA_KEY_RELEASE_ID, release_id.as_bytes().to_vec());
            } else {
                let build_id = self.cfg().litos_build_id().map_err(|e| config_err(e, None))?;

                attestation.insert_data(DATA_KEY_BUILD_ID, build_id.as_bytes().to_vec());

                if let Ok(subnet_id) = self.cfg.subnet_id() {
                    attestation.insert_data(DATA_KEY_SUBNET_ID, subnet_id.as_bytes().to_vec());
                }
            }

            if let Some(hash) = req_hash.take() {
                attestation.insert_data(DATA_KEY_REQ_BODY_HASH, hash);
            }

            if let Some(private_key) = private_key {
                // Sign it
                attestation
                    .sign(private_key)
                    .map_err(|e| generic_err(e, Some("failed to sign attestation".into())))?;
            }

            // Generate it
            attestation
                .generate()
                .await
                .map_err(|e| generic_err(e, Some("failed to generate attestation".into())))
        } else {
            Err(validation_err("unable to detect system attestation type", None))
        }
    }
}

// Util

fn load_cfg() -> Result<LitConfig> {
    let mut builder = LitConfigBuilder::default();
    builder = <LitConfig as LitOsGuestConfig>::apply_defaults(builder).map_err(|e| {
        generic_err(e, Some("failed to apply LitOsGuestConfig defaults".to_string()))
    })?;

    let cfg = builder
        .build()
        .map_err(|e| generic_err(e, Some("failed to build lit config".to_string())))?;

    Ok(cfg)
}

/// A sha256 hex string of the guest context elements.
fn build_context_hash(cfg: &LitConfig, cmdline_env: &GuestCmdLine) -> Result<Vec<u8>> {
    let build_id =
        cfg.litos_build_id().map_err(|e| config_err(e, Some("missing build id config".into())))?;
    let instance_id = cfg
        .litos_guest_instance_id()
        .map_err(|e| config_err(e, Some("missing instance id config".into())))?;

    let context_key = match cmdline_env.release_id.is_some() {
        true => format!("{}-{}", cmdline_env.release_id.as_ref().unwrap(), &instance_id),
        false => format!("{}-{}", &build_id, &instance_id),
    };
    let context_key = sha256(context_key.as_bytes());

    Ok(context_key.to_vec())
}
