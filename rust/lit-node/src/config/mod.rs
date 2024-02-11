use async_std::path::PathBuf;
use std::collections::HashMap;
use std::path::Path;
use url::Url;

use ethers::{prelude::k256::ecdsa::SigningKey, types::H160, utils::secret_key_to_address};
use generic_array::GenericArray;

use lit_api_core::config::{
    CFG_KEY_DOMAIN, CFG_KEY_ENABLED, CFG_KEY_PORT, CFG_KEY_REDIRECT_TO_HTTPS,
    CFG_KEY_TLS_AUTO_ENABLED,
};
use lit_api_core::{
    config::{http_section_key, https_section_key, LitApiConfig, CFG_KEY_IDENT},
    error::Unexpected,
};
use lit_blockchain::config::{
    LitBlockchainConfig, CFG_KEY_BLOCKCHAIN_CHAIN_ID, CFG_KEY_BLOCKCHAIN_CHAIN_NAME,
    CFG_KEY_BLOCKCHAIN_WALLET_DEFAULT_PRIVATE_KEY,
};
use lit_blockchain::resolver::rpc::rpc_url;
use lit_core::config::{LitConfig, LitConfigBuilder, ReloadableLitConfig};
use lit_logging::config::LitLoggingConfig;

use crate::{
    error::{parser_err, validation_err, Result},
    utils::encoding,
};

pub mod chain;

pub const CFG_SECTION_KEY: &str = "node";

// NB: Before adding keys here ensure they don't conflict with LitApiConfig
// - port, address, ident e.t.c. are all reserved.
pub static CFG_KEY_RPC_URL: &str = "rpc_url";
pub static CFG_KEY_STAKER_ADDRESS: &str = "staker_address";
pub static CFG_KEY_COMS_KEYS_SENDER_PRIVKEY: &str = "coms_keys_sender_privkey";
pub static CFG_KEY_COMS_KEYS_RECEIVER_PRIVKEY: &str = "coms_keys_receiver_privkey";
pub static CFG_KEY_ADMIN_ADDRESS: &str = "admin_address";
pub static CFG_KEY_ENABLE_PROXIED_HTTP_CLIENT: &str = "enable_proxied_http_client";
pub static CFG_KEY_ENABLE_RATE_LIMITING: &str = "enable_rate_limiting";
pub static CFG_KEY_ENABLE_ACTIONS_ALLOWLIST: &str = "enable_actions_allowlist";
pub static CFG_KEY_ENABLE_EPOCH_TRANSITIONS: &str = "enable_epoch_transitions";
pub static CFG_KEY_ENABLE_ECDSA_DKG: &str = "enable_ecdsa_dkg";
pub static CFG_KEY_ENABLE_ECDSA_BATCH_SENDING: &str = "enable_ecdsa_batch_sending";
pub static CFG_KEY_ENABLE_ECDSA_DKG_BATCH_SENDING: &str = "enable_ecdsa_dkg_batch_sending";
pub static CFG_KEY_ECDSA_ROUND_TIMEOUT: &str = "ecdsa_round_timeout";
pub static CFG_KEY_ECDSA_BATCH_SEND_INTERVAL: &str = "ecdsa_batch_send_interval";
pub static CFG_KEY_WEBAUTHN_ALLOWED_ORIGINS: &str = "webauthn_allowed_origins";
pub static CFG_KEY_MESSAGE_QUEUE_PROCESS_LENGTH: &str = "message_queue_process_length";
pub static CFG_KEY_CHAIN_POLLING_INTERVAL_MS: &str = "chain_polling_interval";
pub static CFG_KEY_PEER_REVIEWER_LIMIT: &str = "peer_reviewer_limit";
pub static CFG_KEY_PEER_REVIEWER_INTERVAL: &str = "peer_reviewer_interval";
pub static CFG_KEY_HTTP_CLIENT_TIMEOUT: &str = "http_client_timeout";
pub static CFG_KEY_HTTP_CLIENT_PATIENCE: &str = "http_client_patience";
pub static CFG_KEY_ENABLE_HTTP_HEADER_DESCRIPTORS: &str = "enable_http_header_descriptors";
pub static CFG_KEY_ECDSA_ROOT_PUBKEY_COUNT: &str = "ecdsa_root_pubkey_count";
pub static CFG_KEY_ENTER_RESTORE_STATE: &str = "enter_restore_state";
pub static CFG_KEY_BLS_KEY_BLINDER: &str = "bls_key_blinder";
pub static CFG_KEY_ECDSA_KEY_BLINDER: &str = "ecdsa_key_blinder";

// Defaults
pub static CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT: i64 = 30000;
pub static CFG_KEY_ECDSA_BATCH_SEND_INTERVAL_MS_DEFAULT: i64 = 250;
pub static CFG_KEY_ECDSA_ROUND_TIMEOUT_MS_DEFAULT: i64 = 30000;

static REQUIRED_CFG_KEYS: [&str; 9] = [
    CFG_KEY_STAKER_ADDRESS,
    CFG_KEY_COMS_KEYS_SENDER_PRIVKEY,
    CFG_KEY_COMS_KEYS_RECEIVER_PRIVKEY,
    CFG_KEY_ADMIN_ADDRESS,
    CFG_KEY_ENABLE_RATE_LIMITING,
    CFG_KEY_ENABLE_ACTIONS_ALLOWLIST,
    CFG_KEY_ENABLE_EPOCH_TRANSITIONS,
    CFG_KEY_ENABLE_ECDSA_DKG,
    CFG_KEY_DOMAIN,
];

static USER_EDITABLE_KEYS: [&str; 19] = [
    CFG_KEY_RPC_URL,
    CFG_KEY_ADMIN_ADDRESS,
    CFG_KEY_STAKER_ADDRESS,
    CFG_KEY_ENABLE_PROXIED_HTTP_CLIENT,
    CFG_KEY_ENABLE_RATE_LIMITING,
    CFG_KEY_ENABLE_ACTIONS_ALLOWLIST,
    CFG_KEY_ENABLE_EPOCH_TRANSITIONS,
    CFG_KEY_ENABLE_ECDSA_DKG,
    CFG_KEY_ENABLE_ECDSA_BATCH_SENDING,
    CFG_KEY_ENABLE_ECDSA_DKG_BATCH_SENDING,
    CFG_KEY_MESSAGE_QUEUE_PROCESS_LENGTH,
    CFG_KEY_ECDSA_ROUND_TIMEOUT,
    CFG_KEY_ECDSA_BATCH_SEND_INTERVAL,
    CFG_KEY_WEBAUTHN_ALLOWED_ORIGINS,
    CFG_KEY_CHAIN_POLLING_INTERVAL_MS,
    CFG_KEY_ECDSA_ROOT_PUBKEY_COUNT,
    CFG_KEY_ENTER_RESTORE_STATE,
    CFG_KEY_BLS_KEY_BLINDER,
    CFG_KEY_ECDSA_KEY_BLINDER,
];

static USER_EDITABLE_KEYS_IN_SECTIONS: [&str; 3] = [
    CFG_KEY_BLOCKCHAIN_WALLET_DEFAULT_PRIVATE_KEY,
    CFG_KEY_BLOCKCHAIN_CHAIN_ID,
    CFG_KEY_BLOCKCHAIN_CHAIN_NAME,
];

pub trait LitNodeConfig {
    fn try_new() -> Result<LitConfig>;
    fn from_builder(builder: LitConfigBuilder) -> Result<LitConfig>;
    fn verify(&self) -> Result<()>;
    fn export_user_editable(&self) -> Result<HashMap<String, String>>;
    fn verify_user_editable(&self, data: &HashMap<String, String>) -> Result<()>;

    // Accessors
    fn external_port(&self) -> Result<u16>;
    fn external_addr(&self) -> Result<String>;
    fn http_prefix_when_talking_to_other_nodes(&self) -> String;
    fn internal_port(&self) -> Result<u16>;
    fn node_address(&self) -> Result<H160>;
    fn rpc_url(&self) -> Result<String>;
    fn staker_address(&self) -> Result<String>;
    fn coms_keys_sender_privkey(&self) -> Result<String>;
    fn coms_keys_receiver_privkey(&self) -> Result<String>;
    fn admin_address(&self) -> Result<H160>;
    fn key_path(&self) -> PathBuf;
    fn ecdsa_key_path(&self) -> PathBuf;
    fn bls_key_path(&self) -> PathBuf;
    fn webauthn_allowed_origins(&self) -> Result<Vec<Url>>;
    fn peer_reviewer_limit(&self) -> Result<u8>;
    fn peer_reviewer_interval(&self) -> Result<u64>;
    fn http_client_timeout(&self) -> Result<u64>;
    fn http_client_patience(&self) -> Result<u64>;

    // Feature flag bool accessors
    fn enable_proxied_http_client(&self) -> Result<bool>;
    fn enable_rate_limiting(&self) -> Result<bool>;
    fn enable_actions_allowlist(&self) -> Result<bool>;
    fn enable_epoch_transitions(&self) -> Result<bool>;
    fn enable_ecdsa_dkg(&self) -> Result<bool>;
    fn enable_ecdsa_batch_sending(&self) -> Result<bool>;
    fn enable_ecdsa_dkg_batch_sending(&self) -> Result<bool>;
    fn enable_http_header_descriptors(&self) -> Result<bool>;

    // communications parameters for ECDSA rounds
    fn ecdsa_round_timeout(&self) -> Result<i64>;
    fn ecdsa_batch_send_interval(&self) -> Result<i64>;
    fn message_queue_process_length(&self) -> Result<i64>;
    fn chain_polling_interval_ms(&self) -> Result<i64>;
    fn ecdsa_root_pubkey_count(&self) -> Result<i64>;
    fn enter_restore_state(&self) -> Result<bool>;
    fn bls_key_blinder(&self) -> Result<String>;
    fn ecdsa_key_blinder(&self) -> Result<String>;
}

impl LitNodeConfig for LitConfig {
    fn try_new() -> Result<Self> {
        <LitConfig as LitNodeConfig>::from_builder(LitConfigBuilder::default())
    }

    fn from_builder(mut builder: LitConfigBuilder) -> Result<LitConfig> {
        // Set defaults
        builder = builder
            .set_key(Some(CFG_SECTION_KEY.into()))
            .set_section_default(CFG_KEY_IDENT, "Lit Node")
            // See LitApiConfig for other API options.
            .set_section_default(http_section_key(CFG_KEY_REDIRECT_TO_HTTPS), "true")
            .set_section_default(http_section_key(CFG_KEY_PORT), "8080")
            .set_section_default(http_section_key(CFG_KEY_ENABLED), "true")
            .set_section_default(https_section_key(CFG_KEY_PORT), "8443")
            .set_section_default(https_section_key(CFG_KEY_ENABLED), "false")
            .set_section_default(CFG_KEY_TLS_AUTO_ENABLED, "false")
            .set_section_default(CFG_KEY_ENABLE_ECDSA_BATCH_SENDING, "false")
            .set_section_default(CFG_KEY_ENABLE_ECDSA_DKG_BATCH_SENDING, "false")
            .set_section_default(
                CFG_KEY_ECDSA_ROUND_TIMEOUT,
                CFG_KEY_ECDSA_ROUND_TIMEOUT_MS_DEFAULT.to_string(),
            )
            .set_section_default(
                CFG_KEY_ECDSA_BATCH_SEND_INTERVAL,
                CFG_KEY_ECDSA_BATCH_SEND_INTERVAL_MS_DEFAULT.to_string(),
            )
            .set_section_default(CFG_KEY_MESSAGE_QUEUE_PROCESS_LENGTH, "1000")
            .set_section_default(CFG_KEY_PEER_REVIEWER_LIMIT, "10")
            .set_section_default(CFG_KEY_PEER_REVIEWER_INTERVAL, "900")
            .set_section_default(CFG_KEY_HTTP_CLIENT_TIMEOUT, "30")
            .set_section_default(CFG_KEY_HTTP_CLIENT_PATIENCE, "30")
            .set_section_default(CFG_KEY_ENABLE_PROXIED_HTTP_CLIENT, "false")
            .set_section_default(CFG_KEY_WEBAUTHN_ALLOWED_ORIGINS, "http://localhost:3000")
            .set_section_default(
                CFG_KEY_CHAIN_POLLING_INTERVAL_MS,
                CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT.to_string(),
            )
            .set_section_default(CFG_KEY_ECDSA_ROOT_PUBKEY_COUNT, "10")
            .set_section_default(CFG_KEY_ENABLE_HTTP_HEADER_DESCRIPTORS, "false")
            .set_section_default(CFG_KEY_ENTER_RESTORE_STATE, "false");

        // Apply others
        builder = <LitConfig as LitBlockchainConfig>::apply_defaults(builder)?;
        builder = <LitConfig as LitLoggingConfig>::apply_defaults(builder)?;

        <LitConfig as LitApiConfig>::from_builder(builder)
    }

    fn verify(&self) -> Result<()> {
        for req in REQUIRED_CFG_KEYS {
            self.get_section_checked_string(req)?;
        }

        self.rpc_url()?;
        self.api_port(self.https_enabled())?;
        self.api_port_external(self.https_enabled())?;
        self.blockchain_wallet_private_key(None)?;
        self.blockchain_chain_id()?;
        self.blockchain_chain_name()?;

        Ok(())
    }

    /// Export user editable config
    fn export_user_editable(&self) -> Result<HashMap<String, String>> {
        let mut map: HashMap<String, String> = HashMap::new();

        for key in USER_EDITABLE_KEYS {
            map.insert(
                format!("{}.{}", CFG_SECTION_KEY, key),
                self.get_section_string(key).unwrap_or("".into()),
            );
        }

        for key in USER_EDITABLE_KEYS_IN_SECTIONS {
            let val = if CFG_KEY_BLOCKCHAIN_WALLET_DEFAULT_PRIVATE_KEY.eq(key) {
                self.blockchain_wallet_private_key(None)
                    .unwrap_or("".into())
            } else if CFG_KEY_RPC_URL.eq(key) {
                self.rpc_url().unwrap_or("".into())
            } else {
                self.get_string(key).unwrap_or("".into())
            };

            map.insert(key.to_owned(), val);
        }

        Ok(map)
    }

    /// Verify the user editable config map (ensure keys are valid)
    fn verify_user_editable(&self, data: &HashMap<String, String>) -> Result<()> {
        for (full_key, _) in data.iter() {
            if let Some((section_key, key)) = full_key.split_once('.') {
                if (section_key != CFG_SECTION_KEY || !USER_EDITABLE_KEYS.contains(&key))
                    && !USER_EDITABLE_KEYS_IN_SECTIONS.contains(&full_key.as_str())
                {
                    return Err(validation_err(
                        format!("user editing of config key '{}' not allowed", full_key),
                        None,
                    ));
                }
            } else {
                return Err(validation_err(
                    format!("user editing of config key '{}' not allowed", full_key),
                    None,
                ));
            }
        }

        Ok(())
    }

    // Accessors

    /// The external port used to access the node (rocket may bind to a different port but iptables will
    /// forward to it).
    fn external_port(&self) -> Result<u16> {
        Ok(self.api_port_external(self.https_enabled())? as u16)
    }

    fn external_addr(&self) -> Result<String> {
        Ok(format!("{}:{}", self.api_domain()?, self.external_port()?))
    }

    fn http_prefix_when_talking_to_other_nodes(&self) -> String {
        if self.https_enabled() {
            "https://".to_string()
        } else {
            "http://".to_string()
        }
    }

    fn internal_port(&self) -> Result<u16> {
        Ok(self.api_port(self.https_enabled())? as u16)
    }

    fn node_address(&self) -> Result<H160> {
        let secret_key = SigningKey::from_bytes(GenericArray::from_slice(
            &encoding::hex_to_bytes(self.blockchain_wallet_private_key(None)?)
                .expect_or_err("Failed to hex encode node.private_key config var")?,
        ))
        .expect_or_err("Could not convert node.private_key config var to SigningKey")?;
        Ok(secret_key_to_address(&secret_key))
    }

    fn rpc_url(&self) -> Result<String> {
        if let Ok(url) = self.get_section_string(CFG_KEY_RPC_URL) {
            return Ok(url);
        }

        rpc_url(self.blockchain_chain_name()?, 0)
    }

    fn staker_address(&self) -> Result<String> {
        self.get_section_string(CFG_KEY_STAKER_ADDRESS)
    }

    fn coms_keys_sender_privkey(&self) -> Result<String> {
        self.get_section_string(CFG_KEY_COMS_KEYS_SENDER_PRIVKEY)
    }

    fn coms_keys_receiver_privkey(&self) -> Result<String> {
        self.get_section_string(CFG_KEY_COMS_KEYS_RECEIVER_PRIVKEY)
    }

    fn admin_address(&self) -> Result<H160> {
        self.get_section_string(CFG_KEY_ADMIN_ADDRESS)?
            .parse::<H160>()
            .map_err(|e| {
                parser_err(
                    e,
                    Some("Could not convert admin_address to H160".to_string()),
                )
            })
    }

    fn key_path(&self) -> PathBuf {
        key_path()
    }

    fn ecdsa_key_path(&self) -> PathBuf {
        ecdsa_key_path()
    }

    fn bls_key_path(&self) -> PathBuf {
        bls_key_path()
    }

    fn webauthn_allowed_origins(&self) -> Result<Vec<Url>> {
        let origins: Vec<String> = self
            .get_section_string(CFG_KEY_WEBAUTHN_ALLOWED_ORIGINS)
            .map(|s| s.split(',').map(|s| s.to_string()).collect())?;

        let origins: Vec<Url> = origins
            .iter()
            .map(|s| {
                Url::parse(s).map_err(|e| {
                    parser_err(
                        e,
                        Some(format!(
                            "Could not parse webauthn_allowed_origins url: {}",
                            s
                        )),
                    )
                })
            })
            .collect::<Result<Vec<Url>>>()?;

        Ok(origins)
    }

    fn peer_reviewer_limit(&self) -> Result<u8> {
        self.get_section_int(CFG_KEY_PEER_REVIEWER_LIMIT)
            .map(|i| i as u8)
    }

    fn peer_reviewer_interval(&self) -> Result<u64> {
        self.get_section_int(CFG_KEY_PEER_REVIEWER_INTERVAL)
            .map(|i| i as u64)
    }

    fn http_client_timeout(&self) -> Result<u64> {
        self.get_section_int(CFG_KEY_HTTP_CLIENT_TIMEOUT)
            .map(|i| i as u64)
    }

    fn http_client_patience(&self) -> Result<u64> {
        self.get_section_int(CFG_KEY_HTTP_CLIENT_PATIENCE)
            .map(|i| i as u64)
    }

    // Feature flag bool accessors
    fn enable_proxied_http_client(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_PROXIED_HTTP_CLIENT)
    }

    fn enable_rate_limiting(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_RATE_LIMITING)
    }

    fn enable_actions_allowlist(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_ACTIONS_ALLOWLIST)
    }

    fn enable_epoch_transitions(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_EPOCH_TRANSITIONS)
    }

    fn enable_ecdsa_dkg(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_ECDSA_DKG)
    }

    fn enable_ecdsa_batch_sending(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_ECDSA_BATCH_SENDING)
    }

    fn enable_ecdsa_dkg_batch_sending(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_ECDSA_DKG_BATCH_SENDING)
    }

    fn enable_http_header_descriptors(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_HTTP_HEADER_DESCRIPTORS)
    }

    fn ecdsa_round_timeout(&self) -> Result<i64> {
        self.get_section_int(CFG_KEY_ECDSA_ROUND_TIMEOUT)
    }

    fn ecdsa_batch_send_interval(&self) -> Result<i64> {
        self.get_section_int(CFG_KEY_ECDSA_BATCH_SEND_INTERVAL)
    }

    fn message_queue_process_length(&self) -> Result<i64> {
        self.get_section_int(CFG_KEY_MESSAGE_QUEUE_PROCESS_LENGTH)
    }

    fn chain_polling_interval_ms(&self) -> Result<i64> {
        self.get_section_int(CFG_KEY_CHAIN_POLLING_INTERVAL_MS)
    }

    fn ecdsa_root_pubkey_count(&self) -> Result<i64> {
        self.get_section_int(CFG_KEY_ECDSA_ROOT_PUBKEY_COUNT)
    }

    fn enter_restore_state(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENTER_RESTORE_STATE)
    }

    fn bls_key_blinder(&self) -> Result<String> {
        self.get_section_string(CFG_KEY_BLS_KEY_BLINDER)
    }

    fn ecdsa_key_blinder(&self) -> Result<String> {
        self.get_section_string(CFG_KEY_ECDSA_KEY_BLINDER)
    }
}

pub(crate) fn key_path() -> PathBuf {
    // this feels pretty ugly... but it proves a point.
    let staker_address = match self::load_cfg() {
        Ok(c) => c.load().staker_address(),
        Err(e) => {
            println!("load_cfg error: {}", e);
            return PathBuf::from("./node_keys/no_config");
        }
    };
    let staker_address = match staker_address {
        Ok(a) => a.to_lowercase(),
        Err(e) => {
            println!("staker_address error: {}", e);
            return PathBuf::from("./node_keys/bad_staker_address");
        }
    };
    let path_root = format!("./node_keys/{}", staker_address);
    PathBuf::from(&path_root)
}

pub(crate) fn backup_key_path() -> PathBuf {
    let mut path = key_path();
    path.push("backup");
    path
}

pub(crate) fn encrypted_key_path() -> PathBuf {
    let mut path = key_path();
    path.push("encrypted");
    path
}
pub(crate) fn beaver_triple_path() -> PathBuf {
    let mut path = key_path();
    path.push("beaver_triples");
    path
}

pub(crate) fn ecdsa_key_path() -> PathBuf {
    let mut path = key_path();
    path.push("ecdsa");
    path
}

pub(crate) fn bls_key_path() -> PathBuf {
    let mut path = key_path();
    path.push("bls");
    path
}

pub fn segmented_paths(
    path: impl AsRef<Path>,
    key: &str,
    levels: usize,
    from_end: bool,
) -> Result<PathBuf> {
    let mut keys: Vec<&str> = key.split("").collect();
    if keys.len() < levels {
        return Err(validation_err(
            "segmented_paths: provided key is not long enough for the levels required",
            None,
        ));
    }
    if from_end {
        keys.reverse();
    }

    let mut new = PathBuf::from(path.as_ref());
    for i in 0..levels {
        if let Some(k) = keys.get(i) {
            new.push(k);
        }
    }

    Ok(new)
}

pub fn load_cfg() -> Result<ReloadableLitConfig> {
    ReloadableLitConfig::new(|| {
        let cfg = <LitConfig as LitNodeConfig>::try_new()?;

        // Verify every load (will not replace running config unless it works)
        cfg.verify()?;

        Ok(cfg)
    })
}
