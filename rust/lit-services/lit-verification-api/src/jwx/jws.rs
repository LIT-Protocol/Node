use k256::ecdsa::Signature;
use lit_core::{
    error::{self, Unexpected},
    utils::binary::bytes_to_hex,
};
use lit_os_core::error::{conversion_err, unexpected_err};
use serde_json::Value;

use super::utils::hex_to_scalar;

const R: &str = "r";
const S: &str = "s";

pub trait Json {
    fn to_json(&self) -> error::Result<serde_json::Map<String, Value>>;
    fn from_value(value: Value) -> error::Result<Signature>;
}

impl Json for Signature {
    fn to_json(&self) -> error::Result<serde_json::Map<String, Value>> {
        let hex_r = bytes_to_hex(self.r().to_bytes());
        let hex_s = bytes_to_hex(self.s().to_bytes());

        let mut signature = serde_json::Map::new();
        signature.insert(S.to_string(), serde_json::Value::String(hex_s));
        signature.insert(R.to_string(), serde_json::Value::String(hex_r));

        Ok(signature)
    }

    fn from_value(value: Value) -> error::Result<Signature> {
        let signature =
            value.as_object().expect_or_err("Error while converting signature to JSON")?;

        let (r, s) = match (signature.get(R), signature.get(S)) {
            (Some(r), Some(s)) => {
                let hex_r = r.as_str().expect_or_err("could not convert r value to string")?;
                let hex_s = s.as_str().expect_or_err("could not convert s value to string")?;
                (
                    hex_to_scalar(hex_r).map_err(|e| {
                        conversion_err(e, Some("Error while converting scalar r".into()))
                    })?,
                    hex_to_scalar(hex_s).map_err(|e| {
                        conversion_err(e, Some("Error while converting scalar s".into()))
                    })?,
                )
            }
            _ => {
                return Err(unexpected_err("Signature values not present, aborting", None));
            }
        };

        let signature = Signature::from_scalars(r, s).map_err(|e| unexpected_err(e, None))?;

        Ok(signature)
    }
}

#[cfg(test)]
mod tests {
    use digest::Digest;
    use elliptic_curve::rand_core;
    use k256::ecdsa::signature::Signer;
    use k256::ecdsa::signature::Verifier;
    use k256::ecdsa::{Signature, SigningKey};
    use rand_core::OsRng;
    use serde_json::Value;
    use sha3::Sha3_256;

    use super::Json;

    #[test]
    fn should_convert_to_json() {
        let signing_key = SigningKey::random(&mut OsRng);
        let message = b"testing-message";

        let mut hasher = sha3::Sha3_256::new();
        hasher.update(message);
        let digest = hasher.finalize();

        let signature: Signature = signing_key.sign(&digest);

        let sig_json = signature.to_json().expect("Error while converting signature to json");

        // Will panic with a "Key not found" if not present
        let _r = &sig_json["r"];
        let _s = &sig_json["s"];
    }

    #[test]
    fn should_convert_from_value() {
        let signing_key = SigningKey::random(&mut OsRng);
        let message = b"testing-message";

        let mut hasher = Sha3_256::new();
        hasher.update(message);
        let digest = hasher.finalize();

        let signature: Signature = signing_key.sign(&digest);

        let sig_json = signature.to_json().expect("Error while converting signature to json");

        println!("sig value: {:?}", sig_json);
        let value: Value = sig_json.into();
        println!("sig converted value: {:?}", value);

        let signature = Signature::from_value(value).expect("error while decoding signature");

        let _ = signing_key
            .verifying_key()
            .verify(&digest, &signature)
            .expect("Error while verifying signature");
    }
}
