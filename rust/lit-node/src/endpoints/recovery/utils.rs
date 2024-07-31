use ethers::types::H160;
use lit_blockchain::contracts::backup_recovery::NextStateDownloadable;
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use rocket::State;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

use crate::auth::auth_material::JsonAuthSig;
use crate::error::{conversion_err, unexpected_err, validation_err, Result};
use crate::tss::common::curve_type::CurveType;
use crate::tss::common::key_share::KeyShare;
use crate::tss::common::restore::RestoreState;

const SHARE_DOWNLOAD_SIG_EXP: u64 = 10;
const BACKUP_KEYSHARE_EPOCH: u64 = 0;

pub async fn check_auth_sig_for_dec_share_upload(
    cfg: &LitConfig,
    restore_state: &State<Arc<RwLock<RestoreState>>>,
    auth_sig: &JsonAuthSig,
) -> Result<()> {
    let restore_state = restore_state.read().await;
    let party_members = restore_state.get_recovery_party_members()?;
    check_auth_sig_for_recovery_party(cfg, auth_sig, party_members)
}

pub fn check_auth_sig_for_share_download(
    cfg: &LitConfig,
    auth_sig: &JsonAuthSig,
    party_member: &H160,
) -> Result<()> {
    check_auth_sig_for_recovery_party(cfg, auth_sig, &[*party_member])
}

pub fn check_auth_sig_for_recovery_party(
    cfg: &LitConfig,
    auth_sig: &JsonAuthSig,
    party_members: &[H160],
) -> Result<()> {
    match ethers::types::Signature::from_str(&auth_sig.sig) {
        Ok(sig) => {
            let presented_address = ethers::types::Address::from_str(&auth_sig.address)
                .expect_or_err("Failed to parse auth sig address")?;
            if !party_members.contains(&presented_address) {
                return Err(validation_err(
                    "Error in signatures addresses",
                    Some("Address mismatch, aborting".into()),
                ));
            }

            sig.verify(auth_sig.signed_message.clone(), presented_address)
                .map_err(|e| {
                    validation_err(e, Some("Invalid signature verification".into()))
                        .add_msg_to_details()
                })?;

            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|e| {
                    unexpected_err(e, Some("Could not get current time from system".into()))
                })?;
            let sm = auth_sig.signed_message.parse::<u128>().map_err(|e| {
                conversion_err(
                    e,
                    Some("could not convert message timestamp to integer".into()),
                )
            })?;
            let sig_ts = Duration::from_millis(sm as u64);
            let delta = now - sig_ts;
            if delta.as_secs() > SHARE_DOWNLOAD_SIG_EXP {
                return Err(validation_err(
                    "Expiration error",
                    Some("expiration on signature reached aborting".into()),
                ));
            }
        }
        Err(e) => {
            // Here we're not providing the full error as a detail to the user for security.
            return Err(validation_err(
                e,
                Some("Error parsing the signature".into()),
            ));
        }
    };

    Ok(())
}

pub async fn resolve_key_shares_from_disk(
    next_backup_state: &NextStateDownloadable,
    share_index: u16,
    staker_address: &str,
) -> Result<(KeyShare, KeyShare)> {
    let mut bls_share: Option<KeyShare> = None;
    let mut ecdsa_share: Option<KeyShare> = None;

    for key in next_backup_state.registered_recovery_keys.clone() {
        if key.key_type == ethers::types::U256::from(1) {
            trace!("Key type found to be bls: {:?}", &key.pubkey);
            trace!("Attempting to read key share of type: {:?}", key.key_type);
            let share: KeyShare = match crate::tss::common::storage::read_key_share_from_disk(
                bytes_to_hex(&key.pubkey).as_str(),
                share_index,
                BACKUP_KEYSHARE_EPOCH,
                CurveType::BLS,
                staker_address,
            )
            .await
            {
                Ok(s) => s,
                Err(e) => {
                    return Err(unexpected_err(
                        e,
                        Some(
                            "Error while getting next backup state from contract. aborting".into(),
                        ),
                    ));
                }
            };
            bls_share = Some(share);
        } else if key.key_type == ethers::types::U256::from(2) {
            trace!("Key type found to be ecdsa: {:?}", &key.pubkey);
            trace!("Attempting to read key share of type: {:?}", key.key_type);
            let share: KeyShare = match crate::tss::common::storage::read_key_share_from_disk(
                bytes_to_hex(&key.pubkey).as_str(),
                share_index,
                BACKUP_KEYSHARE_EPOCH,
                CurveType::K256,
                staker_address,
            )
            .await
            {
                Ok(s) => s,
                Err(e) => {
                    return Err(unexpected_err(
                        e,
                        Some(
                            "Error while getting next backup state from contract. aborting".into(),
                        ),
                    ));
                }
            };
            ecdsa_share = Some(share);
        }
    }

    if bls_share.is_none() || ecdsa_share.is_none() {
        return Err(unexpected_err(
            "Could not resolve key shares",
            Some("Error while getting next backup state from contract. aborting".into()),
        ));
    }
    trace!("Shares found on disk, returning data");
    let bs = match bls_share {
        Some(b) => b,
        None => {
            return Err(unexpected_err(
                "Could not resolve key shares",
                Some("Error while getting next backup state from contract. aborting".into()),
            ));
        }
    };

    let es = match ecdsa_share {
        Some(e) => e,
        None => {
            return Err(unexpected_err(
                "Could not resolve key shares",
                Some("Error while getting next backup state from contract. aborting".into()),
            ));
        }
    };

    Ok((bs, es))
}

// will be used for deletion once share verification is implemented
#[allow(dead_code)]
pub async fn delete_key_shares_from_disk(
    next_backup_state: &NextStateDownloadable,
    share_index: u16,
    staker_address: &str,
) -> Result<bool> {
    for key in next_backup_state.registered_recovery_keys.clone() {
        if key.key_type == ethers::types::U256::from(1) {
            trace!("Key type found to be bls: {:?}", &key.pubkey);
            trace!("Attempting to read key share of type: {:?}", key.key_type);
            let res = match crate::tss::common::storage::delete_keyshare(
                CurveType::BLS,
                bytes_to_hex(&key.pubkey).as_str(),
                share_index,
                BACKUP_KEYSHARE_EPOCH,
                staker_address,
            )
            .await
            {
                Ok(s) => true,
                Err(e) => false,
            };
            return Ok(res);
        } else if key.key_type == ethers::types::U256::from(2) {
            trace!("Key type found to be ecdsa: {:?}", &key.pubkey);
            trace!("Attempting to read key share of type: {:?}", key.key_type);
            let res: bool = match crate::tss::common::storage::delete_keyshare(
                CurveType::K256,
                bytes_to_hex(&key.pubkey).as_str(),
                share_index,
                BACKUP_KEYSHARE_EPOCH,
                staker_address,
            )
            .await
            {
                Ok(s) => true,
                Err(e) => false,
            };
            return Ok(res);
        }
    }

    Ok(false)
}
