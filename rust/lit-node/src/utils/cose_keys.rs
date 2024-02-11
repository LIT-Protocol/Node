use ciborium::{
    de::from_reader,
    value::{Integer, Value},
};
use ethers::types::Bytes;
use lit_core::error::Unexpected;
use openssl::{
    bn::{BigNum, BigNumContext},
    ec::PointConversionForm,
};
use webauthn_rs_core::proto::{
    Base64UrlSafeData, COSEAlgorithm, COSEEC2Key, COSEKey, COSEKeyType, COSEKeyTypeId, ECDSACurve,
};

use crate::error::{conversion_err, parser_err};
use crate::error::{validation_err, Result};

pub fn decode_cbor_cose_key(key: Bytes) -> Result<(COSEKey, String)> {
    let value: Value = from_reader(key.as_ref())
        .map_err(|e| parser_err(e, Some("Failed to decode CBOR encoded COSE key".into())))?;

    let decoded_map = value
        .as_map()
        .expect_or_err("Could not convert value to map")
        .map_err(|e| {
            validation_err(
                e,
                Some(format!(
                    "Expected CBOR encoded COSE key to be a map, got {:?}",
                    value
                )),
            )
        })?;

    // Get key type (key: 1)
    let key_type = get_cbor_map_value_by_key(decoded_map, 1)?;
    let key_type = i128::from(
        key_type
            .as_integer()
            .expect_or_err("Could not convert key type to integer")?,
    ) as usize;

    match map_usize_to_cose_key_type_id(key_type)
        .map_err(|e| conversion_err(e, Some("Failed to convert key type".into())))?
    {
        COSEKeyTypeId::EC_EC2 => {
            // Get curve id (key: -1)
            let curve_id = get_cbor_map_value_by_key(decoded_map, -1)?;
            let curve_id = i128::from(
                curve_id
                    .as_integer()
                    .expect_or_err("Could not convert curve id to integer")?,
            ) as usize;

            // Get x (key: -2)
            let x = get_cbor_map_value_by_key(decoded_map, -2)?;
            let x = x.as_bytes().expect_or_err("Could not convert x to bytes")?;

            // Get y (key: -3)
            let y = get_cbor_map_value_by_key(decoded_map, -3)?;
            let y = y.as_bytes().expect_or_err("Could not convert y to bytes")?;

            // Get n (key: -1)
            let n = get_cbor_map_value_by_key(decoded_map, -1)?;
            let n = i128::from(
                n.as_integer()
                    .expect_or_err("Could not convert n to integer")?,
            ) as isize;

            // Get e (key: -2)
            let e = get_cbor_map_value_by_key(decoded_map, -2)?;
            let e = e.as_bytes().expect_or_err("Could not convert e to bytes")?;

            // Get COSEAlgorithm (key: 3)
            let alg_id = get_cbor_map_value_by_key(decoded_map, 3)?;
            let alg_id = i128::from(
                alg_id
                    .as_integer()
                    .expect_or_err("Could not convert alg id to integer")?,
            ) as isize;
            let cose_alg = map_isize_to_cose_alg(alg_id)
                .map_err(|e| conversion_err(e, Some("Failed to convert COSE algorithm".into())))?;

            // Create COSEKey struct
            let cose_ec2_key = COSEEC2Key {
                curve: map_usize_to_ecdsa_curve(curve_id)
                    .map_err(|e| conversion_err(e, Some("Failed to convert ECDSA curve".into())))?,
                x: Base64UrlSafeData(x.to_vec()),
                y: Base64UrlSafeData(y.to_vec()),
            };

            let cose_key = COSEKey {
                type_: cose_alg,
                key: COSEKeyType::EC_EC2(cose_ec2_key.clone()),
            };

            // Get public key from COSEEC2Key
            let group = openssl::ec::EcGroup::from_curve_name((&cose_ec2_key.curve).into())
                .map_err(|e| conversion_err(e, Some("Failed to create EC group".into())))?;
            let ec_public_key = openssl::ec::EcKey::from_public_key_affine_coordinates(
                &group,
                BigNum::from_slice(x)
                    .map_err(|e| conversion_err(e, Some("Failed to convert x to BigNum".into())))?
                    .as_ref(),
                BigNum::from_slice(y)
                    .map_err(|e| conversion_err(e, Some("Failed to convert y to BigNum".into())))?
                    .as_ref(),
            )
            .map_err(|e| {
                conversion_err(e, Some("Failed to convert x and y to EC public key".into()))
            })?;
            let mut big_num_ctx = BigNumContext::new()
                .map_err(|e| conversion_err(e, Some("Failed to create BigNum context".into())))?;
            let ec_public_key_bytes = ec_public_key
                .public_key()
                .to_bytes(&group, PointConversionForm::COMPRESSED, &mut big_num_ctx)
                .map_err(|e| {
                    conversion_err(e, Some("Failed to convert EC public key to bytes".into()))
                })?;
            let ec_public_key_hex = format!("{:x}", Bytes::from(ec_public_key_bytes.clone()));

            debug!(
                "ec_public_key_bytes: {:?} - {:?}",
                ec_public_key_hex, ec_public_key_bytes
            );

            Ok((cose_key, ec_public_key_hex))
        }
        _ => Err(validation_err(
            format!("Currently not supporting {:?} key types", key_type),
            None,
        )),
    }
}

fn get_cbor_map_value_by_key(cbor_map: &Vec<(Value, Value)>, key: i32) -> Result<Value> {
    Ok(cbor_map
        .iter()
        .find(|(k, _)| {
            let k = match k
                .as_integer()
                .expect_or_err("Could not convert key to integer")
            {
                Ok(k) => k,
                Err(e) => return false,
            };
            k == Integer::from(key)
        })
        .expect_or_err("Could not find key type")
        .map_err(|e| {
            validation_err(
                e,
                Some(format!(
                    "Map does not include key {:?}, got {:?}",
                    key, cbor_map
                )),
            )
        })?
        .1
        .to_owned())
}

fn map_usize_to_ecdsa_curve(ecdsa_curve_id: usize) -> Result<ECDSACurve> {
    Ok(match ecdsa_curve_id {
        1 => ECDSACurve::SECP256R1,
        2 => ECDSACurve::SECP384R1,
        3 => ECDSACurve::SECP521R1,
        _ => {
            return Err(validation_err(
                format!("Unknown ECDSA curve id: {}", ecdsa_curve_id),
                None,
            ))
        }
    })
}

fn map_usize_to_cose_key_type_id(key_type_id: usize) -> Result<COSEKeyTypeId> {
    Ok(match key_type_id {
        0 => COSEKeyTypeId::EC_Reserved,
        1 => COSEKeyTypeId::EC_OKP,
        2 => COSEKeyTypeId::EC_EC2,
        3 => COSEKeyTypeId::EC_RSA,
        4 => COSEKeyTypeId::EC_Symmetric,
        _ => {
            return Err(validation_err(
                format!("Unknown key type id: {}", key_type_id),
                None,
            ))
        }
    })
}

fn map_isize_to_cose_alg(alg_id: isize) -> Result<COSEAlgorithm> {
    Ok(match alg_id {
        -7 => COSEAlgorithm::ES256,
        -35 => COSEAlgorithm::ES384,
        -36 => COSEAlgorithm::ES512,

        -257 => COSEAlgorithm::RS256,
        -258 => COSEAlgorithm::RS384,
        -259 => COSEAlgorithm::RS512,

        -37 => COSEAlgorithm::PS256,
        -38 => COSEAlgorithm::PS384,
        -39 => COSEAlgorithm::PS512,

        -8 => COSEAlgorithm::EDDSA,

        -65535 => COSEAlgorithm::INSECURE_RS1,

        _ => return Err(validation_err(format!("Unknown alg id: {}", alg_id), None)),
    })
}
