use crate::auth::auth_material::JsonAuthSig;
use crate::constants::{CHAIN_CHEQD, CHAIN_COSMOS, CHAIN_EVMOS, CHAIN_JUNO, CHAIN_KYVE};
use crate::error::{conversion_err, parser_err, parser_err_code, validation_err_code, Result, EC};
use crate::utils::encoding;
use bech32::{FromBase32, ToBase32};
use ethers::types::Address;
use ripemd::{Digest, Ripemd160};
use sha2::Sha256;

pub fn validate_cosmos_auth_sig(auth_sig: &JsonAuthSig, chain: &String) -> Result<bool> {
    let sig_bytes = data_encoding::BASE64
        .decode(auth_sig.sig.as_bytes())
        .map_err(|e| parser_err(e, Some("Unable to decode base64".into())))?;
    let signature = libsecp256k1::Signature::parse_standard_slice(&sig_bytes)
        .map_err(|e| validation_err_code(e, EC::NodeAuthSigSignatureConversionError, None))?;
    let message_bytes = encoding::hex_to_bytes(&auth_sig.signed_message)
        .map_err(|e| validation_err_code(e, EC::NodeAuthSigSignedMessageConversionError, None))?;
    let message = libsecp256k1::Message::parse_slice(&message_bytes)
        .map_err(|e| parser_err_code(e, EC::NodeParserError, None))?;
    // need to check all 4 possible recovery ids because cosmos chops off the last byte
    // which is the V param which is the recovery sig.  whyyyy cosmos, why
    try_all_recovery_ids(signature, message, auth_sig.address.clone(), chain)
}

fn try_all_recovery_ids(
    signature: libsecp256k1::Signature,
    message: libsecp256k1::Message,
    submitted_address: String,
    chain: &String,
) -> Result<bool> {
    let chain_derivation_prefix = get_chain_derivation_prefix_for_chain_name(chain)?;
    for i in 0..4 {
        debug!("trying recovery id {}", i);
        let recovery_id = libsecp256k1::RecoveryId::parse(i)
            .map_err(|e| parser_err_code(e, EC::NodeRecoveryIdError, None))?;
        match libsecp256k1::recover(&message, &signature, &recovery_id) {
            Ok(pubkey) => {
                // The Cosmos address is derived by the follows: address = ripemd160(sha256(pubKey)), encoded by bech32 with cosmos as prefix.
                let mut hasher = Sha256::new();
                hasher.update(pubkey.serialize_compressed());
                let shaed = hasher.finalize();

                let mut ripe_hasher = Ripemd160::new();
                ripe_hasher.update(shaed);
                let address_before_bech32 = ripe_hasher.finalize();

                let address = bech32::encode(
                    chain_derivation_prefix.as_str(),
                    address_before_bech32.to_vec().to_base32(),
                    bech32::Variant::Bech32,
                )
                .map_err(|e| parser_err_code(e, EC::NodeParserError, None))?;

                debug!("address: {}", address);
                debug!("submitted address: {}", submitted_address);

                if address == submitted_address {
                    return Ok(true);
                }
            }
            Err(err) => {
                warn!("Error checking cosmos auth sig, swallowing: {:?}", err);
            }
        }
    }
    Ok(false)
}

fn get_chain_derivation_prefix_for_chain_name(chain_name: &String) -> Result<String> {
    match chain_name.as_str() {
        CHAIN_COSMOS => Ok(CHAIN_COSMOS.to_string()),
        CHAIN_KYVE => Ok(CHAIN_KYVE.to_string()),
        CHAIN_CHEQD | "cheqdMainnet" | "cheqdTestnet" => Ok(CHAIN_CHEQD.to_string()),
        "evmosCosmos" | "evmosCosmosTestnet" => Ok(CHAIN_EVMOS.to_string()),
        CHAIN_JUNO => Ok(CHAIN_JUNO.to_string()),
        _ => Err(validation_err_code(
            format!("invalid chain for cosmos: {}", chain_name),
            EC::NodeBlockchainChainUnknown,
            None,
        )),
    }
}

/// Parses a bech32 string into an EVM-compatible address.
pub fn get_evm_address(address: &str) -> Result<Address> {
    let (_hrp, data, _) = bech32::decode(address)
        .map_err(|e| conversion_err(e, Some("Error decoding bech32 string".into())))?;
    let bytes = Vec::<u8>::from_base32(&data).map_err(|e| {
        parser_err(
            e,
            Some("Error parsing byte array from bech32 string".into()),
        )
    })?;
    Ok(Address::from_slice(&bytes))
}
