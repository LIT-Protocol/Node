use elliptic_curve::SecretKey;
use k256::ecdsa::SigningKey;
use k256::Scalar;
use lit_attestation::AttestationType;
use lit_core::{config::ReloadableLitConfig, error::Result};

use crate::errors::{unexpected_err_code, EC};

use self::utils::{pubkey_to_checksum, scalar_hash};
use lit_attestation::kdf::Kdf;

pub mod endpoints;
pub mod models;
mod utils;

#[derive(Debug, Clone)]
pub struct SigningPair {
    pub private_key: Box<SecretKey<k256::Secp256k1>>,
    pub public_key_checksum: String,
    pub signing_key: Box<SigningKey>,
}

// Some arbitrarily picked words with little associations between them.
const KEY_SEED: &str = "duck_camera_beautiful";

impl SigningPair {
    #[doc = "generates private key and singing key, should only be single instance in the lifetime of the server"]
    pub async fn from_seed(cfg: ReloadableLitConfig) -> Result<Self> {
        let cfg = cfg.load_full();
        let scalar: Scalar =
            if matches!(AttestationType::from_system(), Some(AttestationType::AmdSevSnp)) {
                let key = Kdf::try_derive(cfg.as_ref(), KEY_SEED).await?;
                scalar_hash(key.as_ref())
            } else {
                scalar_hash(KEY_SEED.as_bytes())
            };

        let sk = SecretKey::<k256::Secp256k1>::from_bytes(&scalar.to_bytes())
            .map_err(|e| unexpected_err_code(e, EC::VerificationEcdsaKeyGenerationError, None))?;
        let checksum = pubkey_to_checksum(sk.public_key().to_sec1_bytes());
        let singing_key = SigningKey::try_from(sk.clone())
            .map_err(|e| unexpected_err_code(e, EC::VerificationEcdsaKeyGenerationError, None))?;

        Ok(SigningPair {
            private_key: Box::new(sk),
            signing_key: Box::new(singing_key),
            public_key_checksum: checksum,
        })
    }
}
