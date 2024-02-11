// this function doesn't appear to be used, but saving it just in case.  Oct 18,2023

// use crate::error::{unexpected_err, Result};
// use ethers::abi::AbiEncode;
// use lit_attestation::kdf::Kdf;
// use lit_core::config::LitConfig;
// use openssl::sha::sha256;
// use rand::rngs::StdRng;
// use rand_core::SeedableRng;
// use std::time::{SystemTime, UNIX_EPOCH};

// pub async fn rng<C: AsRef<LitConfig>>(cfg: C) -> Result<StdRng> {
//     let time = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .map_err(|e| unexpected_err(e, Some("cannot produce secure RNG".into())))?;
//     let hex_nanos = time.as_nanos().encode_hex();
//     let secure_entropy = Kdf::try_derive(cfg.as_ref(), hex_nanos).await?;
//     let seed = sha256(&secure_entropy);
//     let rng = StdRng::from_seed(seed);
//     Ok(rng)
// }
