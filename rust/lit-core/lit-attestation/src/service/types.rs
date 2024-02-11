use crate::Attestation;
use serde::{Deserialize, Serialize};
use serde_bytes_base64::Bytes;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AttestationIntentReq {
    pub noonce: Option<Bytes>,
}

impl AttestationIntentReq {
    pub fn new(noonce: Option<Vec<u8>>) -> Self {
        Self { noonce: noonce.map(|v| v.into()) }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttestationIntentResp {
    pub attestation: Attestation,
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AttestationReq {
    pub attestation: Attestation,
    pub session_id: String,
}

impl AttestationReq {
    pub fn new(attestation: Attestation, session_id: String) -> Self {
        Self { attestation, session_id }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttestationResp {
    pub attestation: Attestation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KdfReq {
    pub context: String,
}

impl KdfReq {
    pub fn new<S: Into<String>>(context: S) -> Self {
        Self { context: context.into() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KdfResp {
    pub key: Bytes,
}
