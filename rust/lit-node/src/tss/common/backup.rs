use blsful::inner_types::{Bls12381G1, G1Projective};
use bulletproofs::BulletproofCurveArithmetic as BCA;
use ethers::types::H160;
use rocket::serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};
use verifiable_share_encryption::{Ciphertext, Proof};

use crate::error::{parser_err, unexpected_err, Result};
use crate::utils::contract::get_backup_recovery_contract;
use crate::utils::encoding::CompressedPointBytes;
use lit_core::config::LitConfig;

#[derive(Serialize, Deserialize)]
pub struct VerifiableBackup<C: BCA> {
    // Identifies the subnet this backup belongs to.
    pub subnet_id: String,
    // Identifies the node this backup comes from.
    pub staker_address: String,
    // Identifies the DKG round
    pub txn_prefix: String,
    // Ciphertext and proof corresponding to the secret key share
    #[serde(bound(serialize = "Ciphertext<C>: Serialize"))]
    #[serde(bound(deserialize = "Ciphertext<C>: Deserialize<'de>"))]
    pub ciphertext: Ciphertext<C>,
    #[serde(bound(serialize = "Proof<C>: Serialize"))]
    #[serde(bound(deserialize = "Proof<C>: Deserialize<'de>"))]
    pub proof: Proof<C>,
    // Remaining metadata to recover the 'KeyShare<Secp256k1>' type.
    pub public_key: String,
    pub share_index: u16,
    pub threshold: u16,
    pub total_shares: u16,
}

/// Internally kept version
pub struct RecoveryParty {
    pub party_members: Vec<H160>,
    pub session_id: String,
    pub bls_encryption_key: <Bls12381G1 as BCA>::Point,
    pub ecdsa_encryption_key: k256::ProjectivePoint,
    pub threshold: usize,
}

pub async fn get_recovery_party(cfg: &LitConfig) -> Result<RecoveryParty> {
    let recovery_contract = get_backup_recovery_contract(cfg).await?;
    let state = recovery_contract
        .get_backup_party_state()
        .await
        .map_err(|e| {
            unexpected_err(
                e,
                Some(
                    "Cannot retrieve the recovery party state from the smart contract".to_string(),
                ),
            )
        })?;

    let bls_encryption_key = read_bls_pub_key(&state.bls_12381g1_enc_key)?;
    let ecdsa_encryption_key = read_ecdsa_pub_key(&state.secp_256k1_ecdsa_pub_key)?;

    Ok(RecoveryParty {
        party_members: state.party_members,
        session_id: state.session_id.to_string(),
        bls_encryption_key,
        ecdsa_encryption_key,
        threshold: state.party_threshold.as_usize(),
    })
}

pub fn read_bls_pub_key(bytes: &[u8]) -> Result<<Bls12381G1 as BCA>::Point> {
    if bytes.len() != 48 {
        return Err(parser_err(
            Error::new(
                ErrorKind::Other,
                format!(
                    "Expected 48 bytes for a BLS public key, got: {:?}",
                    bytes.len()
                ),
            ),
            None,
        ));
    }
    let bytes_array: &[u8; 48] = bytes.try_into().map_err(|e| {
        parser_err(
            e,
            Some("Could not convert BLS pubkey bytes to an array of 96 bytes".into()),
        )
    })?;
    let bls_encryption_key = G1Projective::from_compressed(bytes_array);
    if bool::from(bls_encryption_key.is_some()) {
        #[allow(clippy::unwrap_used)]
        // `unwrap` seems to be the most suitable option for `CtOption`.
        Ok(bls_encryption_key.unwrap())
    } else {
        Err(parser_err(
            Error::new(
                ErrorKind::Other,
                format!("Cannot convert bytes to a BLS public key: {:?}", bytes),
            ),
            None,
        ))
    }
}

fn read_ecdsa_pub_key(bytes: &[u8]) -> Result<k256::ProjectivePoint> {
    k256::ProjectivePoint::from_compressed(bytes).ok_or_else(|| {
        parser_err(
            Error::new(
                ErrorKind::Other,
                format!("Cannot convert bytes to an ECDSA public key: {:?}", bytes),
            ),
            None,
        )
    })
}
