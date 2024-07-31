use crate::utils::siwe::validate_siwe;
use crate::{auth::auth_material::JsonAuthSig, config::LitNodeConfig as _};
use ethers::{
    types::{Signature, H160},
    utils::keccak256,
};
use libsecp256k1::PublicKey;
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use siwe::Message;
use std::str::FromStr;

use crate::utils::encoding;

use crate::error::{
    conversion_err, conversion_err_code, parser_err_code, validation_err, validation_err_code,
    Result, EC,
};

pub const LITNODE_ADMIN_RES: &str = "litNodeAdmin://*";

pub(crate) fn check_auth_sig(
    config: &LitConfig,
    auth_sig: &JsonAuthSig,
    resource: &str,
    valid_addresses: &Vec<H160>,
) -> Result<()> {
    match Signature::from_str(&auth_sig.sig) {
        Ok(sig) => {
            let presented_address = ethers::types::Address::from_str(&auth_sig.address)
                .expect_or_err("Failed to parse auth sig address")?;

            sig.verify(auth_sig.signed_message.clone(), presented_address)
                .map_err(|e| {
                    validation_err_code(
                        e,
                        EC::NodeAdminUnauthorized,
                        Some("Invalid signature verification".into()),
                    )
                    .add_msg_to_details()
                })?;

            match sig.recover(auth_sig.signed_message.clone()) {
                Ok(recovered_address) => {
                    trace!("presented_address: {:?}", presented_address);
                    trace!("recovered_address: {:?}", recovered_address);

                    if recovered_address != presented_address {
                        return Err(validation_err_code(
                            "Recovered Address not equal to Presented Address",
                            EC::NodeAdminUnauthorized,
                            None,
                        )
                        .add_source_to_details());
                    }

                    if !valid_addresses.contains(&recovered_address) {
                        return Err(validation_err_code(
                            "Recovered Address is not a Valid Address",
                            EC::NodeAdminUnauthorized,
                            None,
                        )
                        .add_source_to_details());
                    }

                    // check the signed message.  it must be a SIWE
                    let message: Message = auth_sig.signed_message.parse().map_err(|e| {
                        parser_err_code(
                            e,
                            EC::NodeAdminUnauthorized,
                            Some("Parse error on SIWE".into()),
                        )
                    })?;

                    // make sure litNodeAdmin resource is present
                    let mut resource_present = false;
                    for r in &message.resources {
                        if r.as_str() == resource {
                            resource_present = true;
                            break;
                        }
                    }

                    if !resource_present {
                        return Err(validation_err_code(
                            "Required resource of litNodeAdmin://* not found",
                            EC::NodeAdminUnauthorized,
                            None,
                        )
                        .add_source_to_details());
                    }

                    if let Ok(true) = config.enable_siwe_validation() {
                        validate_siwe(&message).map_err(|e| {
                            validation_err(e, Some("SIWE validation failed".into()))
                        })?;
                    }

                    let sig_as_array = encoding::hex_to_bytes(&auth_sig.sig)
                        .map_err(|e| parser_err_code(e, EC::NodeSIWESigConversionError, None))?
                        .try_into()
                        .map_err(|_| {
                            conversion_err("Could not convert into fixed size slice", None)
                        })?;

                    let verification_result =
                        message.verify_eip191(&sig_as_array).map_err(|err| {
                            validation_err(err, Some("Error verifying SIWE signature".into()))
                        })?;

                    trace!(
                        "SIWE verification result: {:?}",
                        encoding::bytes_to_hex(&verification_result)
                    );

                    // convert compressed pubkey to uncompressed
                    let mut pubkey_fixed_length: [u8; 33] = [0; 33];
                    pubkey_fixed_length.copy_from_slice(&verification_result);
                    let pubkey =
                        PublicKey::parse_compressed(&pubkey_fixed_length).map_err(|e| {
                            conversion_err_code(
                                e,
                                EC::NodeAdminUnauthorized,
                                Some("Error parsing pubkey".into()),
                            )
                            .add_msg_to_details()
                        })?;

                    // convert verification_result public key to eth address
                    let pubkey_hash = keccak256(&pubkey.serialize()[1..]);
                    let mut pubkey_hash_fixed_length: [u8; 20] = [0; 20];
                    pubkey_hash_fixed_length.copy_from_slice(&pubkey_hash[12..]);
                    let siwe_recovered_address =
                        ethers::types::Address::from_slice(&pubkey_hash_fixed_length);
                    if siwe_recovered_address != presented_address {
                        // Here we're not providing the full error as a detail to the user for security.
                        return Err(validation_err_code(
                            "Authentication error: recovered address does not match presented address",
                            EC::NodeAdminUnauthorized,
                            None,
                        ));
                    }

                    Ok(())
                }
                Err(e) => {
                    // Here we're not providing the full error as a detail to the user for security.
                    Err(validation_err_code(
                        e,
                        EC::NodeAdminUnauthorized,
                        Some("Invalid signature recovery".into()),
                    ))
                }
            }
        }
        Err(e) => {
            // Here we're not providing the full error as a detail to the user for security.
            Err(validation_err_code(
                e,
                EC::NodeAdminUnauthorized,
                Some("Error parsing the signature".into()),
            ))
        }
    }
}
