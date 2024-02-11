use crate::error::{
    blockchain_err_code, parser_err, parser_err_code, validation_err, validation_err_code, Result,
    EC,
};
use encoding::hex_to_bytes;
use ethers::abi::Token::Bytes;
use ethers::types::{
    transaction::{eip2718::TypedTransaction, eip2930::AccessList},
    Eip1559TransactionRequest, NameOrAddress,
};
use ethers::{abi::parse_abi, prelude::BaseContract, providers::Middleware};
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use std::str::FromStr;

use crate::{auth::auth_material::JsonAuthSig, utils::encoding};
use ethers::{types::Signature, utils::keccak256};
use lit_blockchain::resolver::rpc::get_provider;

const VALID_SIGNATURE_WORD: &str =
    "1626ba7e00000000000000000000000000000000000000000000000000000000";

/// Validate a signature that is meant to be validated by a smart contract per EIP-1271.
pub async fn validate_eip1271_signature(
    auth_sig: &JsonAuthSig,
    chain: &Option<String>,
) -> Result<()> {
    let sig = Signature::from_str(&auth_sig.sig)
        .map_err(|e| parser_err(e, Some("Error parsing the signature".into())))?;

    let presented_address = ethers::types::Address::from_str(&auth_sig.address)
        .map_err(|e| validation_err_code(e, EC::NodeAuthSigAddressConversionError, None))?;
    debug!("presented_address: {:?}", presented_address);

    let recovered_address = sig
        .recover(auth_sig.signed_message.clone())
        .map_err(|e| validation_err(e, Some("Invalid signature recovery".into())))?;

    let c = chain
        .clone()
        .expect_or_err_code(EC::NodeBlockchainChainUnknown, "Empty chain value")?;

    let abi = BaseContract::from(parse_abi(&[
        "function isValidSignature(bytes32 _hash, bytes calldata _signature) external override view returns (bytes4)"
    ]).map_err(|e| parser_err(e, Some("Unable to parse ABI".into())))?);

    let hex_signed_message = data_encoding::HEXLOWER.encode(auth_sig.signed_message.as_bytes());
    let hash_bytes = keccak256(hex_signed_message);
    let sig = Bytes(hex_to_bytes(&auth_sig.sig)?);
    let encoded = abi
        .encode("isValidSignature", (hash_bytes, sig))
        .map_err(|e| {
            parser_err_code(
                e,
                EC::NodeContractFunctionParamsEncodingError,
                Some("Unable to encode _hash or _signature into function isValidSignature".into()),
            )
            .add_msg_to_details()
        })?;

    let provider = get_provider(&c, 0)?;
    let verify_signature_transaction = Eip1559TransactionRequest {
        to: Some(NameOrAddress::Address(presented_address)),
        access_list: AccessList(vec![]),
        data: Some(encoded),
        chain_id: None,
        from: None,
        gas: None,
        value: None,
        nonce: None,
        max_priority_fee_per_gas: None,
        max_fee_per_gas: None,
    };
    let is_valid_signature = provider
        .call(
            &TypedTransaction::Eip1559(verify_signature_transaction),
            None,
        )
        .await
        .map_err(|e| {
            blockchain_err_code(
                e,
                EC::NodeBlockchainError,
                Some(format!("Execution Reverted- Call to contract's {} isValidSignature function failed on chain {}", &auth_sig.address, c))
            )
            .add_detail(format!("Execution Reverted- Call to contract's {} isValidSignature function failed on chain {}", &auth_sig.address, c))
        })?;
    if bytes_to_hex(is_valid_signature.as_ref()) != VALID_SIGNATURE_WORD {
        return Err(validation_err_code(
            "EIP1271 Authsig failed",
            EC::NodeContractAuthsigUnauthorized,
            Some(format!("Authsig failed for contract {}", &auth_sig.address)),
        ));
    }

    Ok(())
}
