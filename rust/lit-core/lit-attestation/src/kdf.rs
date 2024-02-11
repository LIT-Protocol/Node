use aleo_std_cpu::{get_cpu, Cpu};
use blake3::derive_key;
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{env, fmt};

#[allow(unused_imports)]
use crate::error::{
    attestation_err, attestation_err_code, conversion_err, sev_snp_err, sev_snp_err_code,
    validation_err, validation_err_code, Result, EC,
};
#[cfg(feature = "generate-via-service")]
use crate::service::client::{AttestationServiceClient, ATTESTATION_SERVICE_CLIENT};
#[cfg(feature = "generate-via-service")]
use crate::service::types::KdfReq;
#[cfg(feature = "generate-via-system")]
use crate::utils::sev_snp::sev_snp_derive_key;

pub const ENV_KDF_TYPE_OVERRIDE: &str = "LIT_KDF_TYPE_OVERRIDE";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(unused)]
pub enum KdfType {
    AmdSevSnp,
    Local,
}

impl KdfType {
    pub fn from_system() -> Option<KdfType> {
        if let Ok(typ) = env::var(ENV_KDF_TYPE_OVERRIDE) {
            return match typ.as_str() {
                "AMD_SEV_SNP" => Some(Self::AmdSevSnp),
                "LOCAL" => Some(Self::Local),
                _ => None,
            };
        }

        match get_cpu() {
            Cpu::AMD => Some(Self::AmdSevSnp),
            _ => None,
        }
    }
}

impl fmt::Display for KdfType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KdfType::AmdSevSnp => write!(f, "AMD_SEV_SNP"),
            KdfType::Local => write!(f, "LOCAL"),
        }
    }
}

impl FromStr for KdfType {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "AMD_SEV_SNP" => Ok(KdfType::AmdSevSnp),
            "LOCAL" => Ok(KdfType::Local),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(unused)]
pub struct Kdf {
    typ: KdfType,
    namespace: String,
}

impl Kdf {
    pub fn try_new(cfg: &LitConfig, typ: KdfType, namespace: Option<String>) -> Result<Self> {
        if matches!(typ, KdfType::Local) && !cfg.is_dev() {
            return Err(validation_err("KDF type of LOCAL is only allowed for env dev", None));
        }

        Ok(Self { typ, namespace: namespace.unwrap_or_else(|| cfg.key().to_string()) })
    }

    /// Wrapper for new then derive.
    pub async fn try_derive<S: AsRef<str>>(cfg: &LitConfig, context: S) -> Result<[u8; 32]> {
        let kdf_type = match cfg.litos_guest() {
            Ok(true) => KdfType::from_system()
                .expect_or_err("failed to determine KDF type for this guest")?,
            _ => KdfType::Local,
        };

        Self::try_new(cfg, kdf_type, None)?.derive(context.as_ref()).await
    }

    #[allow(unreachable_code, unused_variables)]
    pub async fn derive<S: AsRef<str>>(&self, context: S) -> Result<[u8; 32]> {
        let context = format!("{}::{}", self.namespace, context.as_ref());

        #[cfg(feature = "generate-via-service")]
        return self.derive_via_service(context).await;

        #[cfg(feature = "generate-via-system")]
        return self.derive_via_system(context).await;

        unimplemented!(
            "Unexpected: Kdf::derive() called for type {} and neither generate-via-service nor generate-via-system feature enabled",
            self.typ
        )
    }

    #[cfg(feature = "generate-via-system")]
    async fn derive_via_system(&self, context: String) -> Result<[u8; 32]> {
        match self.typ {
            KdfType::AmdSevSnp => sev_snp_derive_key(context.as_str())
                .map_err(|e| sev_snp_err(e, Some("failed to derive_via_system".into()))),
            KdfType::Local => self.derive_via_local(context).await,
        }
    }

    #[cfg(feature = "generate-via-service")]
    async fn derive_via_service(&self, context: String) -> Result<[u8; 32]> {
        match self.typ {
            KdfType::AmdSevSnp => {
                match ATTESTATION_SERVICE_CLIENT.load().kdf(&KdfReq::new(context)).await {
                    Ok(resp) => {
                        let key: [u8; 32] = resp.key.val.try_into().map_err(|_e| {
                            conversion_err("failed to convert kdf key to [u8; 32]", None)
                        })?;
                        Ok(key)
                    }
                    Err(e) => Err(attestation_err(
                        e,
                        Some("failed to request kdf via attestation service".into()),
                    )),
                }
            }
            KdfType::Local => self.derive_via_local(context).await,
        }
    }

    /// Only used for local testing.
    #[allow(dead_code)]
    async fn derive_via_local(&self, context: String) -> Result<[u8; 32]> {
        Ok(derive_key(context.as_str(), b"local-key-material"))
    }
}
