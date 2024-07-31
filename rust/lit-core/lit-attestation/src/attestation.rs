use std::collections::BTreeMap;
use std::io::Cursor;
use std::path::Path;
use std::str::FromStr;
#[cfg(feature = "generate")]
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fmt, fs};

use aleo_std_cpu::{get_cpu, Cpu};
use async_trait::async_trait;
use libsecp256k1::{recover, sign, verify, Message, PublicKey, RecoveryId, SecretKey, Signature};
use serde::{Deserialize, Serialize};
use serde_bytes_base64::Bytes;
use serde_json::json;
#[cfg(feature = "generate")]
use sev_snp_utilities::Requester;
use sev_snp_utilities::{AttestationReport, Policy, Verification};
use sha2::digest::Output;
use sha2::{Digest, Sha512};
use tracing::trace;

use lit_blockchain::resolver::contract::ContractResolver;
use lit_blockchain::util::release::{
    build_id_from_release_id, release_id_bin_from_string, subnet_id_from_release_id,
};
use lit_core::config::LitConfig;
#[allow(unused_imports)]
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;

use crate::error::parser_err;
#[allow(unused_imports)]
use crate::error::{
    attestation_err, attestation_err_code, conversion_err, sev_snp_err_code, validation_err,
    validation_err_code, Result, EC,
};
#[cfg(feature = "generate")]
use crate::error::{config_err, generic_err};
#[cfg(feature = "generate-via-service")]
use crate::service::client::{AttestationServiceClient, ATTESTATION_SERVICE_CLIENT};
#[cfg(feature = "generate-via-service")]
use crate::service::types::{AttestationIntentReq, AttestationReq};
use crate::verification::VerificationPolicy;

pub static FACILITY_GUEST_INIT: &str = "GI";
pub static FACILITY_GUEST_SERVICE: &str = "GS";

pub static DATA_KEY_REQ_BODY_HASH: &str = "REQ_BODY_HASH";
pub static DATA_KEY_MANIFEST_HASH: &str = "MANIFEST_HASH";
pub static DATA_KEY_FACILITY: &str = "FACILITY";
pub static DATA_KEY_INSTANCE_ID: &str = "INSTANCE_ID";
pub static DATA_KEY_RELEASE_ID: &str = "RELEASE_ID";
pub static DATA_KEY_BUILD_ID: &str = "BUILD_ID";
pub static DATA_KEY_SUBNET_ID: &str = "SUBNET_ID";
pub static DATA_KEY_UNIX_TIME: &str = "UNIX_TIME";
pub static DATA_KEY_EXTERNAL_ADDR: &str = "EXTERNAL_ADDR";

pub const ENV_ATTESTATION_TYPE_OVERRIDE: &str = "LIT_ATTESTATION_TYPE_OVERRIDE";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(unused)]
pub enum AdminSignedType {
    Admin,
    Operator,
}

impl fmt::Display for AdminSignedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AdminSignedType::Admin => write!(f, "ADMIN"),
            AdminSignedType::Operator => write!(f, "OPERATOR"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(unused)]
pub enum AttestationType {
    AmdSevSnp,
    AdminSigned,
}

impl AttestationType {
    pub fn from_system() -> Option<AttestationType> {
        if let Ok(typ) = env::var(ENV_ATTESTATION_TYPE_OVERRIDE) {
            return match typ.as_str() {
                "AMD_SEV_SNP" => Some(Self::AmdSevSnp),
                _ => None,
            };
        }

        match get_cpu() {
            Cpu::AMD => Some(Self::AmdSevSnp),
            _ => None,
        }
    }
}

impl fmt::Display for AttestationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttestationType::AmdSevSnp => write!(f, "AMD_SEV_SNP"),
            AttestationType::AdminSigned => write!(f, "ADMIN_SIGNED"),
        }
    }
}

impl FromStr for AttestationType {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "AMD_SEV_SNP" => Ok(AttestationType::AmdSevSnp),
            "ADMIN_SIGNED" => Ok(AttestationType::AdminSigned),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct Attestation {
    #[serde(alias = "type", rename(serialize = "type"))]
    typ: AttestationType,
    noonce: Bytes,
    data: BTreeMap<String, Bytes>,
    signatures: Vec<Bytes>,
    report: Option<Bytes>,
    #[serde(skip)]
    #[cfg(feature = "generate-via-service")]
    session_id: Option<String>,
}

impl Attestation {
    #[cfg(not(feature = "generate-via-service"))]
    pub async fn new(typ: AttestationType, noonce: Vec<u8>) -> Result<Self> {
        Ok(Self {
            typ,
            noonce: Bytes::from(noonce),
            data: BTreeMap::new(),
            signatures: Vec::new(),
            report: None,
        })
    }

    #[cfg(feature = "generate-via-service")]
    pub async fn new(typ: AttestationType, noonce: Vec<u8>) -> Result<Self> {
        match typ {
            AttestationType::AdminSigned => Ok(Self {
                typ,
                noonce: Bytes::from(noonce),
                data: BTreeMap::new(),
                signatures: Vec::new(),
                report: None,
                session_id: None,
            }),
            AttestationType::AmdSevSnp => Self::new_via_service(typ, noonce).await,
        }
    }

    #[cfg(feature = "generate-via-service")]
    async fn new_via_service(_typ: AttestationType, noonce: Vec<u8>) -> Result<Self> {
        match ATTESTATION_SERVICE_CLIENT
            .load()
            .prepare_attestation(&AttestationIntentReq::new(Some(noonce)))
            .await
        {
            Ok(resp) => {
                let mut us = resp.attestation;
                us.session_id = Some(resp.session_id);
                Ok(us)
            }
            Err(e) => Err(attestation_err(
                e,
                Some("failed to prepare attestation via attestation service".into()),
            )),
        }
    }

    pub async fn new_with_report(
        typ: AttestationType, noonce: Vec<u8>, report: Vec<u8>,
    ) -> Result<Self> {
        Ok(Self { report: Some(Bytes::from(report)), ..Self::new(typ, noonce).await? })
    }

    pub fn to_file(&self, path: &Path) -> Result<()> {
        let value = json!(self);

        fs::write(path, value.to_string()).map_err(|e| crate::error::io_err(e, None))?;

        Ok(())
    }

    pub fn typ(&self) -> &AttestationType {
        &self.typ
    }

    pub fn noonce(&self) -> &Vec<u8> {
        &self.noonce
    }

    pub fn data(&self) -> &BTreeMap<String, Bytes> {
        &self.data
    }

    pub fn signatures(&self) -> &Vec<Bytes> {
        &self.signatures
    }

    pub fn insert_report(&mut self, report: Vec<u8>) -> &mut Self {
        self.report = Some(Bytes::from(report));
        self
    }

    pub fn insert_data<K>(&mut self, key: K, val: Vec<u8>) -> &mut Self
    where
        K: Into<String>,
    {
        self.data.insert(key.into(), Bytes::from(val));
        self
    }

    pub fn get_data<K>(&self, key: K) -> Option<&Bytes>
    where
        K: AsRef<str>,
    {
        self.data.get(key.as_ref())
    }

    pub fn get_data_string<K>(&self, key: K) -> Option<String>
    where
        K: AsRef<str>,
    {
        if let Some(val) = self.get_data(key.as_ref()) {
            String::from_utf8(val.to_vec()).ok()
        } else {
            None
        }
    }

    pub fn is_facility(&self, facility: &str) -> bool {
        if let Some(val) = self.get_data_string(DATA_KEY_FACILITY) {
            return val.eq(facility);
        }

        false
    }

    pub fn is_facility_guest_service(&self) -> bool {
        self.is_facility(FACILITY_GUEST_SERVICE)
    }

    pub fn report_raw(&self) -> Result<&Bytes> {
        self.report
            .as_ref()
            .ok_or(crate::error::sev_snp_err("missing report data in attestation body", None))
    }

    pub async fn verify(&self) -> Result<()> {
        match self.typ {
            AttestationType::AmdSevSnp => AmdSevSnpAttestation::verify(self).await,
            AttestationType::AdminSigned => AdminSignedAttestation::verify(self),
        }
    }

    pub async fn verify_full(
        &self, cfg: &LitConfig, resolver: Option<&ContractResolver>,
        policy: Option<impl VerificationPolicy>,
    ) -> Result<Option<PublicKey>> {
        crate::verification::verify_full(
            cfg,
            resolver,
            self,
            policy.map(|p| Box::new(p) as Box<dyn VerificationPolicy>),
        )
        .await
    }

    pub fn verify_signatures(&self) -> Result<()> {
        if self.signatures.is_empty() {
            // Nothing to prepare
            return Ok(());
        }

        for idx in 0..self.signatures.len() {
            let sha = self.sha512_truncated_32(last_sig_idx_before(idx));

            let ctx_message =
                Message::parse_slice(sha.as_slice()).map_err(|e| conversion_err(e, None))?;

            let (sig_bytes, recovery_id, public_key) = self.signature(idx)?;

            let ctx_sig =
                Signature::parse_der(sig_bytes.as_slice()).map_err(|e| conversion_err(e, None))?;

            let ctx_pub_key = parse_and_verify_public_key(
                public_key.as_slice(),
                recovery_id,
                &ctx_message,
                &ctx_sig,
                format!("signature at idx '{idx}' is invalid").as_str(),
            )
            .map_err(|e| {
                e.add_detail(format!(
                    "Verification of public key in signature at idx '{idx}' failed"
                ))
            })?;

            if !verify(&ctx_message, &ctx_sig, &ctx_pub_key) {
                return Err(attestation_err_code(
                    format!("attestation verification of signature at idx '{idx}' failed"),
                    EC::AttestationSignatureVerifyFailed,
                    None,
                )
                .add_detail(format!("Verification of signature at idx '{idx}' failed")));
            }
        }

        Ok(())
    }

    pub fn sha512(&self, last_sig_idx: Option<isize>) -> Output<Sha512> {
        let mut hasher = Sha512::new();

        self.update_hash(&mut hasher, last_sig_idx);

        hasher.finalize()
    }

    pub fn sha512_truncated(&self, last_sig_idx: Option<isize>, len: usize) -> Vec<u8> {
        self.sha512(last_sig_idx).as_slice()[..len].to_vec()
    }

    pub fn sha512_truncated_32(&self, last_sig_idx: Option<isize>) -> Vec<u8> {
        self.sha512_truncated(last_sig_idx, 32)
    }

    fn update_hash(&self, hasher: &mut impl Digest, last_sig_idx: Option<isize>) {
        hasher.update("noonce");
        hasher.update(self.noonce.as_slice());

        hasher.update("data");
        for (key, val) in self.data.iter() {
            hasher.update(key);
            hasher.update(val.as_slice());
        }

        let sig_len = self.signatures.len();
        if sig_len > 0 {
            let max_last_sig_idx = (sig_len - 1) as isize;
            let mut last_sig_idx = match last_sig_idx {
                Some(last_sig_idx) => last_sig_idx,
                None => max_last_sig_idx,
            };
            if last_sig_idx > max_last_sig_idx {
                last_sig_idx = max_last_sig_idx;
            }

            if last_sig_idx >= 0 {
                hasher.update("signatures");

                for idx in 0..(last_sig_idx as usize) {
                    if let Some(sig) = self.signatures.get(idx) {
                        hasher.update(sig.as_slice());
                    }
                }
            }
        }
    }

    pub fn sign(&mut self, private_key: &str) -> Result<()> {
        let priv_key_bytes = hex::decode(private_key).map_err(|e| conversion_err(e, None))?;
        let ctx_priv_key = SecretKey::parse_slice(priv_key_bytes.as_slice())
            .map_err(|e| conversion_err(e, None))?;

        let public_key = PublicKey::from_secret_key(&ctx_priv_key);

        let sha = self.sha512_truncated_32(None);

        let ctx_message =
            Message::parse_slice(sha.as_slice()).map_err(|e| conversion_err(e, None))?;

        let (sig, recovery_id) = sign(&ctx_message, &ctx_priv_key);
        let sig_der = sig.serialize_der();

        let mut sig_data = Vec::from(sig_der.as_ref());
        sig_data.push(recovery_id.serialize());
        sig_data.extend_from_slice(&public_key.serialize());

        self.signatures.push(Bytes::from(sig_data));

        Ok(())
    }

    pub fn signature(&self, idx: usize) -> Result<(Vec<u8>, u8, Vec<u8>)> {
        let mut sig = self
            .signatures
            .get(idx)
            .ok_or(validation_err(format!("attestation signature index not found: {idx}"), None))?
            .clone();

        let sig_len = sig.len();

        let public_key: Vec<u8> = sig.drain(sig_len - 65..).collect();
        if public_key.len() != 65 {
            return Err(validation_err(
                format!(
                    "attestation signature missing valid public key (len: {})",
                    public_key.len()
                ),
                None,
            ));
        }

        let recovery_id =
            sig.pop().ok_or(validation_err("attestation signature missing recovery id", None))?;

        Ok((sig.to_vec(), recovery_id, public_key))
    }

    pub fn last_signature(&self) -> Result<(Vec<u8>, u8, Vec<u8>)> {
        self.signature(self.signatures.len() - 1)
    }

    pub fn public_key(&self, idx: usize) -> Result<PublicKey> {
        let (sig_bytes, recovery_id, orig_public_key) = self.signature(idx)?;

        let sha = self.sha512_truncated_32(last_sig_idx_before(idx));

        let ctx_message =
            Message::parse_slice(sha.as_slice()).map_err(|e| conversion_err(e, None))?;

        let ctx_sig =
            Signature::parse_der(sig_bytes.as_slice()).map_err(|e| conversion_err(e, None))?;

        parse_and_verify_public_key(
            orig_public_key.as_slice(),
            recovery_id,
            &ctx_message,
            &ctx_sig,
            format!("public key in signature at idx '{idx}' is invalid").as_str(),
        )
        .map_err(|e| {
            e.add_detail(format!("Verification of public key in signature at idx '{idx}' failed"))
        })
    }

    pub fn last_public_key(&self) -> Result<PublicKey> {
        self.public_key(self.signatures.len() - 1)
    }

    pub fn public_keys(&self) -> Result<Vec<PublicKey>> {
        let mut res = Vec::new();

        if !self.signatures.is_empty() {
            for idx in 0..self.signatures.len() {
                res.push(self.public_key(idx)?);
            }
        }

        Ok(res)
    }

    #[must_use = "generate() returns the newly constructed attestation struct"]
    #[allow(unreachable_code)]
    #[cfg(feature = "generate")]
    pub async fn generate(self) -> Result<Self> {
        if matches!(self.typ, AttestationType::AdminSigned) {
            return Ok(self);
        }

        #[cfg(feature = "generate-via-service")]
        return self.generate_via_service().await;

        #[cfg(feature = "generate-via-system")]
        return self.generate_via_system().await;

        unimplemented!(
            "Unexpected: generate called for type {} and neither generate-via-service nor generate-via-system feature enabled",
            self.typ
        )
    }

    #[cfg(feature = "generate-via-system")]
    async fn generate_via_system(self) -> Result<Self> {
        match self.typ {
            AttestationType::AmdSevSnp => AmdSevSnpAttestation::generate(self),
            AttestationType::AdminSigned => {
                // NO-OP
                Ok(self)
            }
        }
    }

    #[cfg(feature = "generate-via-service")]
    async fn generate_via_service(self) -> Result<Self> {
        match self.typ {
            AttestationType::AmdSevSnp => {
                let session_id = self
                    .session_id
                    .as_ref()
                    .ok_or_else(|| {
                        attestation_err("Expected session_id to be present during generate()", None)
                    })?
                    .clone();

                match ATTESTATION_SERVICE_CLIENT
                    .load()
                    .confirm_attestation(&AttestationReq::new(self, session_id))
                    .await
                {
                    Ok(resp) => Ok(resp.attestation),
                    Err(e) => Err(attestation_err(
                        e,
                        Some("failed to confirm attestation via attestation service".into()),
                    )),
                }
            }
            AttestationType::AdminSigned => {
                // NO-OP
                Ok(self)
            }
        }
    }

    pub fn clear(&mut self) {
        self.signatures.clear();
        self.report = None;
    }

    // Accessors for data types
    pub fn release_id(&self) -> Option<String> {
        self.get_data_string(DATA_KEY_RELEASE_ID)
    }

    pub fn release_id_as_b32_slice(&self) -> Option<[u8; 32]> {
        if let Some(release_id) = self.release_id() {
            release_id_bin_from_string(release_id).ok()
        } else {
            None
        }
    }

    pub fn build_id(&self) -> Option<String> {
        if let Some(release_id) = self.release_id() {
            // Always read from RELEASE_ID if present.
            build_id_from_release_id(release_id).ok()
        } else {
            self.get_data_string(DATA_KEY_BUILD_ID)
        }
    }

    pub fn subnet_id(&self) -> Option<String> {
        if let Some(release_id) = self.release_id() {
            // Always read from RELEASE_ID if present.
            subnet_id_from_release_id(release_id).ok()
        } else {
            self.get_data_string(DATA_KEY_SUBNET_ID)
        }
    }

    pub fn instance_id(&self) -> Option<String> {
        self.get_data_string(DATA_KEY_INSTANCE_ID)
    }

    pub fn unix_time(&self) -> Option<String> {
        self.get_data_string(DATA_KEY_UNIX_TIME)
    }

    pub fn external_addr(&self) -> Option<String> {
        self.get_data_string(DATA_KEY_EXTERNAL_ADDR)
    }
}

fn bytes_to_public_key(public_key: &[u8]) -> Result<PublicKey> {
    PublicKey::parse(public_key.try_into().map_err(|e| {
        conversion_err(
            e,
            Some(format!("failed to map public key: {} to &[u8;65]", bytes_to_hex(&public_key))),
        )
    })?)
    .map_err(|e| {
        parser_err(e, Some(format!("failed to parse public key: {}", bytes_to_hex(&public_key))))
    })
}

fn parse_and_verify_public_key(
    public_key: &[u8], recovery_id: u8, ctx_message: &Message, ctx_sig: &Signature, label: &str,
) -> Result<PublicKey> {
    let ctx_pub_key = bytes_to_public_key(public_key)?;

    let ctx_recovery_id = RecoveryId::parse(recovery_id).map_err(|e| conversion_err(e, None))?;

    let recovery_pub_key =
        recover(ctx_message, ctx_sig, &ctx_recovery_id).map_err(|e| conversion_err(e, None))?;

    if !ctx_pub_key.eq(&recovery_pub_key) {
        return Err(attestation_err_code(
            format!("{label}: recovery public key miss-match (message SHA may be invalid)"),
            EC::AttestationSignatureVerifyFailed,
            None,
        ));
    }

    Ok(ctx_pub_key)
}

fn last_sig_idx_before(idx: usize) -> Option<isize> {
    if idx > 0 {
        Some(idx as isize - 1)
    } else {
        Some(-1)
    }
}

impl TryFrom<&Path> for Attestation {
    type Error = crate::error::Error;

    fn try_from(path: &Path) -> std::result::Result<Self, Self::Error> {
        let res = fs::read(path).map_err(|e| crate::error::io_err(e, None))?;

        Self::try_from(res.as_slice())
    }
}

impl TryFrom<&[u8]> for Attestation {
    type Error = crate::error::Error;

    fn try_from(slice: &[u8]) -> std::result::Result<Self, Self::Error> {
        let res: Self = serde_json::from_slice(slice).map_err(|e| crate::error::io_err(e, None))?;

        Ok(res)
    }
}

#[async_trait]
#[cfg(feature = "generate")]
pub trait TryGenerate<T>: Sized {
    /// Performs the generation.
    async fn try_generate(cfg: &LitConfig, values: T) -> Result<Self>;
}

/// Try to generate an attestation with: body_hash, noonce
#[async_trait]
#[cfg(feature = "generate")]
impl TryGenerate<(Vec<u8>, Option<Vec<u8>>)> for Attestation {
    async fn try_generate(cfg: &LitConfig, value: (Vec<u8>, Option<Vec<u8>>)) -> Result<Self> {
        let (body_hash, noonce) = value;
        Attestation::try_generate(cfg, (body_hash, noonce, None)).await
    }
}

/// Try to generate an attestation with: body_hash, noonce, key
#[async_trait]
#[cfg(feature = "generate")]
impl TryGenerate<(Vec<u8>, Option<Vec<u8>>, Option<String>)> for Attestation {
    async fn try_generate(
        cfg: &LitConfig, values: (Vec<u8>, Option<Vec<u8>>, Option<String>),
    ) -> Result<Self> {
        let (body_hash, noonce, key) = values;

        let mut data: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        data.insert(DATA_KEY_REQ_BODY_HASH.into(), body_hash);

        Attestation::try_generate(cfg, (noonce, Some(data), key)).await
    }
}

/// Try to generate an attestation with: noonce, data, key
#[async_trait]
#[cfg(feature = "generate")]
impl TryGenerate<(Option<Vec<u8>>, Option<BTreeMap<String, Vec<u8>>>, Option<String>)>
    for Attestation
{
    async fn try_generate(
        cfg: &LitConfig,
        values: (Option<Vec<u8>>, Option<BTreeMap<String, Vec<u8>>>, Option<String>),
    ) -> Result<Self> {
        let (noonce, mut data, key) = values;

        let now = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| generic_err(e, None))?;

        let attestation_type = match cfg.litos_guest() {
            Ok(true) => AttestationType::from_system()
                .expect_or_err("failed to determine attestation type for this guest")?,
            _ => AttestationType::AdminSigned,
        };

        #[cfg(feature = "generate-via-system")]
        let via_system = true;

        #[cfg(not(feature = "generate-via-system"))]
        let via_system = false;

        let noonce = noonce.unwrap_or_else(|| now.as_millis().to_le_bytes().to_vec());
        let mut res = Attestation::new(attestation_type, noonce).await?;

        if let Some(data) = data.take() {
            for (key, value) in data {
                res.insert_data(key, value);
            }
        }

        if AttestationType::AdminSigned.eq(&attestation_type) || via_system {
            // Only add for system generated, the service adds this.
            res.insert_data(DATA_KEY_UNIX_TIME, now.as_millis().to_le_bytes().to_vec());
        }

        match attestation_type {
            AttestationType::AdminSigned => {
                let key = if let Some(key) = key {
                    key
                } else {
                    cfg.admin_key().map_err(|e| {
                        config_err(e, Some("config missing required for AdminSigned".into()))
                            .add_msg_to_details()
                    })?
                };

                res.sign(key.as_str())?;
            }
            AttestationType::AmdSevSnp => {
                if let Some(key) = key.as_ref() {
                    res.sign(key.as_str())?;
                }
            }
        }

        res.generate().await
    }
}

#[async_trait]
pub trait AmdSevSnpAttestation {
    fn report(&self) -> Result<AttestationReport>;
    async fn verify(&self) -> Result<()>;
    #[cfg(feature = "generate")]
    fn generate(self) -> Result<Attestation>;
}

#[async_trait]
impl AmdSevSnpAttestation for Attestation {
    fn report(&self) -> Result<AttestationReport> {
        let report = self.report_raw()?;

        let mut cur = Cursor::new(&report[..]);

        AttestationReport::from_reader(&mut cur)
            .map_err(|e| sev_snp_err_code(e, EC::AttestationReportParseFailed, None))
    }

    async fn verify(&self) -> Result<()> {
        // Verify report
        trace!("Verifying AMD SEV-SNP attestation report");
        let report = self.report()?;
        let res = report
            .verify(Some(Policy::strict()))
            .await
            .map_err(|e| sev_snp_err_code(e, EC::AttestationReportVerifyFailed, None))?;
        if !res {
            return Err(sev_snp_err_code(
                "AMD SEV-SNP report failed to verify policy",
                EC::AttestationReportVerifyFailed,
                None,
            ));
        }

        // Verify signatures
        self.verify_signatures()?;

        // Compare hash
        let sha = self.sha512(None);
        if !report.report_data.eq(&sha.to_vec()) {
            return Err(sev_snp_err_code(
                format!(
                    "attestation report hash ({}) does not match report data ({})",
                    bytes_to_hex(sha),
                    bytes_to_hex(&report.report_data)
                ),
                EC::AttestationReportVerifyHashFailed,
                None,
            ));
        }

        Ok(())
    }

    #[cfg(feature = "generate")]
    fn generate(mut self) -> Result<Self> {
        // Construct the report
        let report_data = self.sha512(None);
        let report = AttestationReport::request_raw(report_data.as_slice()).map_err(|e| {
            sev_snp_err_code(
                e,
                EC::AttestationReportGenerateFailed,
                Some("failed to generate AMD SEV-SNP attestation report".into()),
            )
        })?;

        self.report = Some(Bytes::from(report));

        Ok(self)
    }
}

pub trait AdminSignedAttestation {
    fn verify(&self) -> Result<()>;
}

impl AdminSignedAttestation for Attestation {
    fn verify(&self) -> Result<()> {
        if self.signatures.is_empty() {
            return Err(validation_err_code(
                "attestation has no signatures",
                EC::AttestationSignatureVerifyFailed,
                None,
            ));
        }

        self.verify_signatures()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use lit_core::utils::binary::bytes_to_hex;

    use crate::attestation::{Attestation, AttestationType};

    static ADMIN_SIGNER_PRIVATE_KEY: &str =
        "5bbd72a9de74d335f1a05ee2fc6f59ff9ab899c497541a015d556990513a169f";
    static ADMIN_SIGNER_PUBLIC_KEY: &str = "041e8c01d7ea24b6a91a4baa0abcac369cec8258881975417922f524b38fe6513f39069cb76254846902a5ccec999ca7f92feb00cd57ae4b87bf60bd1412006d5d";

    #[tokio::test]
    async fn sha512_test() {
        let noonce: Vec<u8> = "1234567890".into();
        let mut attestation = Attestation::new(AttestationType::AdminSigned, noonce.clone())
            .await
            .expect("failed to create Attestation");

        attestation.insert_data("one", "1".into());
        attestation.insert_data("two", "2".into());
        attestation.insert_data("three", "3".into());

        assert_eq!(bytes_to_hex(&attestation.sha512(None).to_vec()),
                   "8a90c09f393c8dd6628910c2546becd58079ac788a7b6012d38e97f203c4e495c05b0563ccf949363ee288e700c61faaa507842312d87c9308c8a4f1baee9aac");

        // Create again in a different order.
        let mut attestation = Attestation::new(AttestationType::AdminSigned, noonce.clone())
            .await
            .expect("failed to create Attestation");

        attestation.insert_data("three", "3".into());
        attestation.insert_data("two", "2".into());
        attestation.insert_data("one", "1".into());

        assert_eq!(bytes_to_hex(&attestation.sha512(None).to_vec()),
                   "8a90c09f393c8dd6628910c2546becd58079ac788a7b6012d38e97f203c4e495c05b0563ccf949363ee288e700c61faaa507842312d87c9308c8a4f1baee9aac");

        // Change data
        let mut attestation = Attestation::new(AttestationType::AdminSigned, noonce.clone())
            .await
            .expect("failed to create Attestation");

        attestation.insert_data("three", "3".into());
        attestation.insert_data("two", "3".into());

        assert_eq!(bytes_to_hex(&attestation.sha512(None).to_vec()),
                   "ef47201ea215846ea5b2a56967ff30256e856084fb099c355c1fb97ea1bb1255f0d3580bbebb71aa20f1b185120c4adf265e76f6df41fd1a660b4e67a6ed33e2");

        // Change noonce.
        let noonce: Vec<u8> = "223456789011".into();
        let mut attestation = Attestation::new(AttestationType::AdminSigned, noonce.clone())
            .await
            .expect("failed to create Attestation");

        attestation.insert_data("three", "3".into());
        attestation.insert_data("two", "2".into());
        attestation.insert_data("one", "1".into());

        assert_eq!(bytes_to_hex(&attestation.sha512(None).to_vec()),
                   "058f76a894e97558f6715ad852bfa58a8fa4e01ba30167d2b87921c7bdf191a6e2785e029eee71d6103047e7f498f5b7b24e44479790ccccc5299339e5fe28a3");
    }

    #[tokio::test]
    async fn sha512_truncated_32_test() {
        let noonce: Vec<u8> = "1234567890".into();
        let mut attestation = Attestation::new(AttestationType::AdminSigned, noonce.clone())
            .await
            .expect("failed to create Attestation");

        attestation.insert_data("one", "1".into());
        attestation.insert_data("two", "2".into());
        attestation.insert_data("three", "3".into());

        assert_eq!(bytes_to_hex(&attestation.sha512(None).to_vec()),
                   "8a90c09f393c8dd6628910c2546becd58079ac788a7b6012d38e97f203c4e495c05b0563ccf949363ee288e700c61faaa507842312d87c9308c8a4f1baee9aac");

        let trunc = attestation.sha512_truncated_32(None);

        assert_eq!(trunc.len(), 32);
        assert_eq!(
            bytes_to_hex(&trunc),
            "8a90c09f393c8dd6628910c2546becd58079ac788a7b6012d38e97f203c4e495"
        );
    }

    #[tokio::test]
    async fn admin_signer_sign_test() {
        let noonce: Vec<u8> = "fdsdfdsf23123123323".into();
        let mut attestation = Attestation::new(AttestationType::AdminSigned, noonce.clone())
            .await
            .expect("failed to create Attestation");

        attestation.insert_data("one", "1".into());
        attestation.insert_data("two", "2".into());
        attestation.insert_data("three", "3".into());

        attestation.sign(ADMIN_SIGNER_PRIVATE_KEY).expect("failed to sign attestation");

        assert_eq!(attestation.signatures.len(), 1);

        let (signature, recovery_id, public_key) =
            attestation.last_signature().expect("failed to get last signature");

        assert_eq!(bytes_to_hex(&signature),
                   "3044022066cb6dbe77293b0ee678557383fdd7c063290ca05fa717fee770de2a5402c0bf022013cdeaeba626708f27041b2cbb923f04fde5e0a8b8aca012070c39f6c073c603");
        assert_eq!(recovery_id, 1);
        assert_eq!(bytes_to_hex(public_key), ADMIN_SIGNER_PUBLIC_KEY);
    }

    #[tokio::test]
    async fn admin_signer_public_key_test() {
        let noonce: Vec<u8> = "fdsdfdsf23123123323".into();
        let mut attestation = Attestation::new(AttestationType::AdminSigned, noonce.clone())
            .await
            .expect("failed to create Attestation");

        attestation.insert_data("one", "1".into());
        attestation.insert_data("two", "2".into());
        attestation.insert_data("three", "3".into());

        attestation.sign(ADMIN_SIGNER_PRIVATE_KEY).expect("failed to sign attestation");

        let res = attestation.last_public_key();
        if res.is_err() {
            panic!("unexpected error: {:?}", res.err().unwrap())
        }

        assert_eq!(bytes_to_hex(res.unwrap().serialize().to_vec()), ADMIN_SIGNER_PUBLIC_KEY);
    }

    #[tokio::test]
    async fn admin_signer_verify_test() {
        let noonce: Vec<u8> = "fdsdfdsf23123123323".into();
        let mut attestation = Attestation::new(AttestationType::AdminSigned, noonce.clone())
            .await
            .expect("failed to create Attestation");

        attestation.insert_data("one", "1".into());
        attestation.insert_data("two", "2".into());
        attestation.insert_data("three", "3".into());

        attestation.sign(ADMIN_SIGNER_PRIVATE_KEY).expect("failed to sign attestation");

        attestation.verify().await.expect("failed to verify attestation");
    }

    #[tokio::test]
    async fn admin_signer_verify_failed_test() {
        let noonce: Vec<u8> = "fdsdfdsf23123123323".into();
        let mut attestation = Attestation::new(AttestationType::AdminSigned, noonce.clone())
            .await
            .expect("failed to create Attestation");

        attestation.insert_data("one", "1".into());
        attestation.insert_data("two", "2".into());
        attestation.insert_data("three", "3".into());

        attestation.sign(ADMIN_SIGNER_PRIVATE_KEY).expect("failed to sign attestation");

        // Should pass
        attestation.verify().await.expect("failed to verify attestation");

        // Should invalidate.
        attestation.insert_data("four", "4".into());

        // Should fail
        assert!(attestation.verify().await.is_err());
    }

    #[tokio::test]
    async fn save_and_load_file_test() {
        let noonce: Vec<u8> = "fdsdfdsf23123123323".into();
        let mut attestation = Attestation::new(AttestationType::AdminSigned, noonce.clone())
            .await
            .expect("failed to create Attestation");

        attestation.insert_data("one", "1".into());
        attestation.insert_data("two", "2".into());
        attestation.insert_data("three", "3".into());

        attestation.sign(ADMIN_SIGNER_PRIVATE_KEY).expect("failed to sign attestation");

        let orig_sig = attestation.signatures.get(0).unwrap().clone();

        let file_path = Path::new("/tmp/attestation.test");

        // Save
        attestation.to_file(file_path).expect("failed to save attestation file");

        // Load
        let attestation =
            Attestation::try_from(file_path).expect("failed to load attestation file");

        assert_eq!(String::from_utf8(attestation.noonce().clone()).unwrap(), "fdsdfdsf23123123323");
        assert_eq!(attestation.data.len(), 3);
        assert_eq!(attestation.signatures.get(0).unwrap(), &orig_sig);
    }

    #[tokio::test]
    async fn attestation_to_json_test() {
        let noonce: Vec<u8> = "fdsdfdsf23123123323".into();
        let mut attestation = Attestation::new(AttestationType::AdminSigned, noonce.clone())
            .await
            .expect("failed to create Attestation");

        attestation.insert_data("one", "1".into());
        attestation.insert_data("two", "2".into());
        attestation.insert_data("three", "3".into());

        attestation.sign(ADMIN_SIGNER_PRIVATE_KEY).expect("failed to sign attestation");

        let orig_sig = attestation.signatures.get(0).unwrap().clone();

        let json =
            serde_json::to_string(&attestation).expect("failed to serialize Attestation to json");

        assert_eq!(json, "{\"type\":\"ADMIN_SIGNED\",\"noonce\":\"ZmRzZGZkc2YyMzEyMzEyMzMyMw==\",\"data\":{\"one\":\"MQ==\",\"three\":\"Mw==\",\"two\":\"Mg==\"},\"signatures\":[\"MEQCIGbLbb53KTsO5nhVc4P918BjKQygX6cX/udw3ipUAsC/AiATzerrpiZwjycEGyy7kj8E/eXgqLisoBIHDDn2wHPGAwEEHowB1+oktqkaS6oKvKw2nOyCWIgZdUF5IvUks4/mUT85Bpy3YlSEaQKlzOyZnKf5L+sAzVeuS4e/YL0UEgBtXQ==\"],\"report\":null}");

        let attestation: Attestation =
            serde_json::from_str(json.as_str()).expect("failed to deserialize json to Attestation");

        assert_eq!(String::from_utf8(attestation.noonce().clone()).unwrap(), "fdsdfdsf23123123323");
        assert_eq!(attestation.data.len(), 3);
        assert_eq!(attestation.signatures.get(0).unwrap(), &orig_sig);
    }
}
