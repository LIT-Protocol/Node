use hex::FromHex;
use lit_attestation::{
    attestation::DATA_KEY_EXTERNAL_ADDR, Attestation, AttestationType, TryGenerate,
};
use lit_blockchain::config::LitBlockchainConfig;
use lit_core::{config::LitConfig, error::Result};
use std::{collections::BTreeMap, sync::Arc};

use crate::{config::LitNodeConfig, error::unexpected_err};

pub async fn create_attestation(cfg: Arc<LitConfig>, noonce: &str) -> Result<Attestation> {
    let typ = if let Some(typ) = AttestationType::from_system() {
        typ
    } else {
        #[cfg(not(feature = "testing"))]
        warn!("attestation type could not be determined from system");
        return Err(unexpected_err(
            "attestation type could not be determined from system",
            None,
        ));
    };

    let noonce = <[u8; 32]>::from_hex(noonce)
        .map_err(|e| unexpected_err(e, Some("cannot parse noonce".into())))?;
    let mut data = BTreeMap::new();
    data.insert(
        DATA_KEY_EXTERNAL_ADDR.to_string(),
        cfg.external_addr()?.as_bytes().to_vec(),
    );
    let attest = Attestation::try_generate(
        cfg.as_ref(),
        (
            Some(noonce.to_vec()),
            Some(data),
            cfg.blockchain_wallet_private_key(None)
                .ok()
                .and_then(|p| p.strip_prefix("0x").map(String::from)),
        ),
    )
    .await
    .map_err(|e| unexpected_err(e, Some("cannot generate attestation".into())))?;
    Ok(attest)
}
