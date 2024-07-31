use crate::endpoints::recovery::utils::delete_key_shares_from_disk;
use crate::tss::common::curve_type::CurveType;
use crate::tss::dkg::curves::common::{BeBytes, KeyHelper, KeyPersistence};
use crate::utils::encoding::CompressedPointBytes;
use crate::{
    config::LitNodeConfig,
    error::{config_err, conversion_err, unexpected_err},
    models::DownloadedShareData,
};
use blsful::inner_types::G1Projective;
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::Wallet,
    types::H160,
};
use k256::ecdsa::SigningKey;
use lit_blockchain::contracts::backup_recovery::{BackupRecovery, NextStateDownloadable};
use lit_core::{config::LitConfig, utils::binary::bytes_to_hex};
use sha2::{Digest, Sha256};

use self::utils::resolve_key_shares_from_disk;
pub mod endpoints;
mod utils;

pub const DECRYPTION_SHARE_CURVE_BLS: &str = "BLS12381G1";
pub const DECRYPTION_SHARE_CURVE_ECDSA: &str = "Secp256k1";

pub async fn do_share_download_from_rec_dkg(
    cfg: &LitConfig,
    party_members: &H160,
    recovery_contract: &BackupRecovery<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) -> lit_core::error::Result<Vec<DownloadedShareData>> {
    let subnet_id = match cfg.subnet_id() {
        Ok(id) => id,
        Err(e) => {
            return Err(unexpected_err(
                e,
                Some("Error while getting subnet id from config".into()),
            ));
        }
    };

    trace!("Pulling octals from chain state");
    let recovery_peer_addresses = match recovery_contract
        .get_staker_addresses_for_dkg()
        .call()
        .await
    {
        Ok(staker_addrs) => staker_addrs,
        Err(e) => {
            return Err(unexpected_err(e, None));
        }
    };

    trace!("reading staker address from config");
    let staking_addr = match cfg.staker_address() {
        Ok(addr) => addr,
        Err(e) => {
            return Err(config_err(
                e,
                Some("Error while loading staker address".into()),
            ));
        }
    };

    let staking_addr: H160 = match staking_addr.parse() {
        Ok(addr) => addr,
        Err(e) => {
            return Err(conversion_err(
                e,
                Some("Could not convert staking address to H160 type".into()),
            ));
        }
    };

    let mut index: i32 = -1;
    for (i, addr) in recovery_peer_addresses.iter().enumerate() {
        if *addr == staking_addr {
            index = i as i32;
            break;
        }
    }

    if index as usize > recovery_peer_addresses.len() {
        return Err(unexpected_err(
            "Could not find wallet address in peer set",
            None,
        ));
    }

    let next_backup_state: NextStateDownloadable =
        match recovery_contract.get_next_backup_state().await {
            Ok(nbs) => nbs,
            Err(e) => {
                return Err(unexpected_err(
                    e,
                    Some("Error while getting next backup state from contract. aborting".into()),
                ));
            }
        };

    trace!("Found next state in contract, pulling shares from disk");
    let staker_address = &bytes_to_hex(staking_addr.as_bytes());
    let (bls_share, ecdsa_share) = match resolve_key_shares_from_disk(
        &next_backup_state,
        index as u16,
        staker_address,
    )
    .await
    {
        Ok(shares) => shares,
        Err(e) => {
            return Err(unexpected_err(
                e,
                Some("Error while getting shares from disk".into()),
            ));
        }
    };

    // ecdsa and bls public points (public keys)
    let bls_pub_key = bls_share
        .public_key::<blsful::inner_types::G1Projective>(CurveType::BLS)?
        .to_compressed();
    let ecdsa_pub_key = ecdsa_share
        .public_key::<k256::ProjectivePoint>(CurveType::K256)?
        .to_compressed();

    // ecdsa and bls private shares (scalars as hex)
    let key_helper = KeyHelper::<G1Projective>::default();
    let bls_priv_share = &key_helper
        .secret_from_hex(&bls_share.hex_private_share)?
        .to_be_bytes();

    let key_helper = KeyHelper::<k256::ProjectivePoint>::default();
    let ecdsa_priv_share = &key_helper
        .secret_from_hex(&ecdsa_share.hex_private_share)?
        .to_be_bytes();

    let mut digest = Sha256::new();
    digest.update(bls_pub_key);
    digest.update(&ecdsa_pub_key);

    let session_id = bytes_to_hex(digest.finalize());

    Ok(vec![
        DownloadedShareData {
            participant_id: index as usize,
            session_id: bytes_to_hex(session_id.as_bytes()),
            decryption_key_share: bytes_to_hex(bls_priv_share),
            encryption_key: bytes_to_hex(bls_pub_key),
            curve: DECRYPTION_SHARE_CURVE_BLS.to_string(),
            subnet_id: subnet_id.clone(),
        },
        DownloadedShareData {
            participant_id: index as usize,
            session_id: bytes_to_hex(session_id.as_bytes()),
            encryption_key: bytes_to_hex(ecdsa_pub_key),
            decryption_key_share: bytes_to_hex(ecdsa_priv_share),
            curve: DECRYPTION_SHARE_CURVE_ECDSA.to_string(),
            subnet_id: subnet_id.clone(),
        },
    ])
}

pub async fn do_delete_share_from_disk(
    cfg: &LitConfig,
    party_members: &H160,
    recovery_contract: &BackupRecovery<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) -> lit_core::error::Result<bool> {
    let subnet_id = match cfg.subnet_id() {
        Ok(id) => id,
        Err(e) => {
            return Err(unexpected_err(
                e,
                Some("Error while getting subnet id from config".into()),
            ));
        }
    };

    trace!("Pulling octals from chain state");
    let recovery_peer_addresses = match recovery_contract
        .get_staker_addresses_for_dkg()
        .call()
        .await
    {
        Ok(staker_addrs) => staker_addrs,
        Err(e) => {
            return Err(unexpected_err(e, None));
        }
    };

    trace!("reading staker address from config");
    let staking_addr = match cfg.staker_address() {
        Ok(addr) => addr,
        Err(e) => {
            return Err(config_err(
                e,
                Some("Error while loading staker address".into()),
            ));
        }
    };

    let staking_addr: H160 = match staking_addr.parse() {
        Ok(addr) => addr,
        Err(e) => {
            return Err(conversion_err(
                e,
                Some("Could not convert staking address to H160 type".into()),
            ));
        }
    };

    let mut index: i32 = -1;
    for (i, addr) in recovery_peer_addresses.iter().enumerate() {
        if *addr == staking_addr {
            index = i as i32;
            break;
        }
    }

    if index as usize > recovery_peer_addresses.len() {
        return Err(unexpected_err(
            "Could not find wallet address in peer set",
            None,
        ));
    }

    let next_backup_state: NextStateDownloadable =
        match recovery_contract.get_next_backup_state().await {
            Ok(nbs) => nbs,
            Err(e) => {
                return Err(unexpected_err(
                    e,
                    Some("Error while getting next backup state from contract. aborting".into()),
                ));
            }
        };

    let staker_address = &bytes_to_hex(staking_addr.as_bytes());
    let is_deleted =
        delete_key_shares_from_disk(&next_backup_state, index as u16, staker_address).await?;

    Ok(is_deleted)
}

pub fn get_staker_address(cfg: &LitConfig) -> crate::error::Result<String> {
    let staker_address = match cfg.staker_address() {
        Ok(addr) => addr,
        Err(e) => return Err(unexpected_err(e, None)),
    };

    let staker_address: ethers::types::H160 = match staker_address.parse() {
        Ok(addr) => addr,
        Err(e) => {
            return Err(conversion_err(
                e,
                Some(format!(
                    "Could not convert staking address to H160 type from {}",
                    staker_address
                )),
            ));
        }
    };

    Ok(bytes_to_hex(staker_address))
}
