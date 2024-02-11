use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CapabilityObject {
    pub def: Option<Vec<String>>,
    pub tar: Option<HashMap<String, Value>>,
    pub ext: Option<HashMap<String, Value>>,
}
