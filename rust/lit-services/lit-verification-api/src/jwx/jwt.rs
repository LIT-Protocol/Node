use super::models::Jwt;
use data_encoding::BASE64URL_NOPAD;
use lit_core::error::Result;
use lit_os_core::error::conversion_err;

impl Jwt {
    pub fn as_base64(&self) -> Result<String> {
        let header_as_json = serde_json::to_string(&self.header)
            .map_err(|e| conversion_err(e, Some("Error while encoding jwt headers".to_string())))?;
        let header_as_base64 = BASE64URL_NOPAD.encode(header_as_json.as_bytes());

        let payload_as_json = serde_json::to_string(&self.payload)
            .map_err(|e| conversion_err(e, Some("Error while encoding jwt payload".to_string())))?;
        let payload_as_base64 = BASE64URL_NOPAD.encode(payload_as_json.as_bytes());

        if let Some(s) = &self.signature {
            let encoded_s = BASE64URL_NOPAD.encode(s.as_bytes());
            let to_sign = format!("{}.{}.{}", header_as_base64, payload_as_base64, encoded_s);
            Ok(to_sign)
        } else {
            let to_sign = format!("{}.{}", header_as_base64, payload_as_base64);
            Ok(to_sign)
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{offset, Duration};

    use crate::jwx::models::{Jwt, JwtHeader, JwtPayload};

    #[test]
    fn should_serilze_base64() {
        let time_stamp = offset::Utc::now();
        let data: String = format!("{}|{}", "test", time_stamp.to_rfc3339());
        let pl = JwtPayload {
            iss: "LIT-Protocol".to_string(),
            sub: "LIT-OTP".to_string(),
            iat: time_stamp.timestamp_millis(),
            exp: (time_stamp + Duration::minutes(30)).timestamp_millis(),
            org_id: "LIT".to_string(),
            role: "user".to_string(),
            extra_data: data.to_string(),
        };

        let jwt: Jwt = Jwt {
            header: JwtHeader { alg: "secp256k1".to_string(), typ: "JWT".to_string() },
            payload: pl,
            signature: None,
        };

        jwt.as_base64().expect("Error while serialzing jwt");
    }
}
