use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebAuthnSignatureVerificationMaterial {
    pub signature: String,
    pub signature_base: String,
    pub credential_public_key: String,
}
