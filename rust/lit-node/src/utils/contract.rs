use crate::error::{unexpected_err, Result};
use ethers::abi::ErrorExt;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::Wallet;
use k256::ecdsa::SigningKey;
use lit_blockchain::contracts::backup_recovery::BackupRecovery;
use lit_blockchain::contracts::pkp_permissions::PKPPermissions;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::LitConfig;
use std::sync::Arc;

pub async fn get_pkp_permissions_contract(
    config: Arc<LitConfig>,
) -> Result<PKPPermissions<Provider<Http>>> {
    // Get contract resolver.
    let resolver = ContractResolver::try_from(config.as_ref())
        .map_err(|e| unexpected_err(e, Some("failed to load ContractResolver".into())))?;

    // Get PKP permissions contract.
    let pkp_permissions_contract = resolver
        .pkp_permissions_contract(config.as_ref())
        .await
        .map_err(|e| unexpected_err(e, Some("failed to load PKP permissions contract".into())))?;
    debug!(
        "pkp_permissions_contract address: {}",
        pkp_permissions_contract.address()
    );

    Ok(pkp_permissions_contract)
}

pub async fn get_backup_recovery_contract(
    config: &LitConfig,
) -> Result<BackupRecovery<Provider<Http>>> {
    let resolver = ContractResolver::try_from(config)
        .map_err(|e| unexpected_err(e, Some("failed to load ContractResolver".into())))?;

    let backup_recovery_contract = resolver
        .backup_recovery_contract(config)
        .await
        .map_err(|e| unexpected_err(e, Some("failed to load BackupRecovery contract".into())))?;

    Ok(backup_recovery_contract)
}

pub async fn get_backup_recovery_contract_with_signer(
    config: &LitConfig,
) -> Result<BackupRecovery<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
    let resolver = ContractResolver::try_from(config)
        .map_err(|e| unexpected_err(e, Some("failed to load ContractResolver".into())))?;

    let backup_recovery_contract = resolver
        .backup_recovery_contract_with_signer(config)
        .await
        .map_err(|e| unexpected_err(e, Some("failed to load BackupRecovery contract".into())))?;

    Ok(backup_recovery_contract)
}

pub fn decode_revert<M>(
    e: &ethers::prelude::ContractError<M>,
    abi: &ethers::abi::Contract,
) -> String
where
    M: ethers::providers::Middleware,
{
    let ethers_bytes = match e.as_revert() {
        Some(bytes) => bytes,
        None => return "Contract Error is not a revert error.".to_string(),
    };
    let bytes = ethers_bytes.as_ref();
    if bytes.len() < 4 {
        return "Not enough bytes for a contract signature in this revert".to_string();
    }

    let errors = abi.errors().collect::<Vec<_>>();

    let mut signature = [0; 4];
    signature.copy_from_slice(&bytes[..4]);
    let data_bytes = &bytes[4..];

    let error = match errors.into_iter().find(|x| x.selector() == signature) {
        Some(error) => error,
        None => {
            let revert_reason_text_sig: [u8; 4] = [0x08, 0xc3, 0x79, 0xa0]; // the standard code for the revert reason just being a text string
            if signature == revert_reason_text_sig {
                let revert_reason_text = match String::from_utf8(data_bytes.to_vec()) {
                    Ok(text) => text,
                    Err(_) => {
                        return "Contract Error is a revert error with invalid UTF-8".to_string()
                    }
                };
                return format!(
                    "Contract Error is a revert error with reason: {}",
                    revert_reason_text
                );
            }
            return "Contract Error could not be decoded.".to_string();
        }
    };

    let input_data = match error.decode(data_bytes) {
        Ok(data) => data,
        Err(_) => return "Contract Error could not be decoded.".to_string(),
    };

    format!(
        "Contract error details; Error name: {:?} / Error data: {:?}",
        error.abi_signature(),
        input_data
    )
}
