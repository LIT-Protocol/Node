use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubmitReq {
    pub entries: Vec<Value>,
}

impl SubmitReq {
    pub fn new(entries: Vec<Value>) -> Self {
        Self { entries }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitResp {
    pub submitted: usize,
}
