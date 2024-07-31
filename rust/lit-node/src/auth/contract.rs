use crate::error::{blockchain_err_code, validation_err_code, Result, EC};
use encoding::hex_to_bytes;
use ethers::middleware::SignerMiddleware;
use ethers::signers::LocalWallet;
use ethers::{contract::abigen, types::Bytes};

use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use rand_core::OsRng;
use std::str::FromStr;
use std::sync::Arc;

use crate::{auth::auth_material::JsonAuthSig, utils::encoding};
use lit_blockchain::resolver::rpc::{RpcHealthcheckPoller, ENDPOINT_MANAGER};

const VALID_SIGNATURE_WORD: &str = "1626ba7e";

/// Validate a signature that is meant to be validated by a smart contract per EIP-1271.
pub async fn validate_eip1271_signature(
    auth_sig: &JsonAuthSig,
    chain: &Option<String>,
) -> Result<()> {
    let presented_address = ethers::types::Address::from_str(&auth_sig.address)
        .map_err(|e| validation_err_code(e, EC::NodeAuthSigAddressConversionError, None))?;
    // debug!(
    //     "presented_address where eip1271 contract lives: {:?} on chain {:?}",
    //     presented_address, chain
    // );

    let c = chain
        .clone()
        .expect_or_err_code(EC::NodeBlockchainChainUnknown, "Empty chain value")?;

    abigen!(
        EIP1271,
        r#"[
                function isValidSignature(bytes32 _hash, bytes calldata _signature) external override view returns (bytes4)
            ]"#,
    );
    let wallet = LocalWallet::new(&mut OsRng);
    let provider = ENDPOINT_MANAGER.get_provider(&c)?;
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let contract = EIP1271::new(presented_address, Arc::new(client.clone()));

    let signed_message_vec = hex_to_bytes(&auth_sig.signed_message)?;
    let signed_message_bytes: [u8; 32] = signed_message_vec.try_into().map_err(|e| {
        validation_err_code(
            "Invalid length for EIP1271 signed message - must be 32 bytes",
            EC::NodeSerializationError,
            None,
        )
    })?;
    let sig = hex_to_bytes(&auth_sig.sig)?;
    // debug!(
    //     "signed_message_bytes {} with length {:?}",
    //     bytes_to_hex(&signed_message_bytes),
    //     signed_message_bytes.len()
    // );
    // debug!("sig {} with length {:?}", bytes_to_hex(&sig), sig.len());
    let is_valid_signature = contract
        .is_valid_signature(signed_message_bytes, sig.into())
        .call()
        .await.map_err(|e| {
            blockchain_err_code(
                e,
                EC::NodeBlockchainError,
                Some(format!("Execution Reverted- Call to contract's {} isValidSignature function failed on chain {}", &auth_sig.address, c))
            )
        })?;
    let valid_result_bytes: Bytes = hex_to_bytes(VALID_SIGNATURE_WORD)?.into();
    let is_valid_bytes: Bytes = is_valid_signature.into();
    if is_valid_bytes != valid_result_bytes {
        return Err(validation_err_code(
            "EIP1271 Authsig failed",
            EC::NodeContractAuthsigUnauthorized,
            Some(format!(
                "Authsig failed for contract {}.  Return value was {}.  We sent params isValidSignature({}, {})",
                &auth_sig.address,
                &bytes_to_hex(is_valid_signature.as_ref()),
                &auth_sig.signed_message,
                &auth_sig.sig
            )),
        ));
    }

    Ok(())
}
