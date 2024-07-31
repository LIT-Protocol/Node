use std::{collections::BTreeMap, fs, path::Path};

use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::result::Result as StdResult;

use crate::error::{io_err, serializer_err};

pub const PROXY_MAPPING_PATH: &str = "./config/test/proxy_mappings.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HttpClientProxyMapping {
    proxy_mappings: BTreeMap<Url, BTreeMap<Url, Url>>,
}

#[allow(dead_code)]
impl Default for HttpClientProxyMapping {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClientProxyMapping {
    pub fn new() -> Self {
        Self {
            proxy_mappings: BTreeMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn new_with_mappings(proxy_mappings: &BTreeMap<Url, BTreeMap<Url, Url>>) -> Self {
        Self {
            proxy_mappings: proxy_mappings.clone(),
        }
    }

    pub fn proxy_mappings(&self) -> &BTreeMap<Url, BTreeMap<Url, Url>> {
        &self.proxy_mappings
    }

    #[allow(dead_code)]
    pub fn write_file_local(&self) -> StdResult<(), crate::error::Error> {
        let json = serde_json::to_string_pretty(self).map_err(|e| serializer_err(e, None))?;
        fs::write(PROXY_MAPPING_PATH, json).map_err(|e| io_err(e, None))
    }
}

impl TryFrom<&Path> for HttpClientProxyMapping {
    type Error = crate::error::Error;

    fn try_from(value: &Path) -> StdResult<Self, Self::Error> {
        Self::try_from(fs::read(value).map_err(|e| io_err(e, None))?)
    }
}

impl TryFrom<Vec<u8>> for HttpClientProxyMapping {
    type Error = crate::error::Error;

    fn try_from(bytes: Vec<u8>) -> StdResult<Self, Self::Error> {
        Self::try_from(&bytes[..])
    }
}

impl TryFrom<&[u8]> for HttpClientProxyMapping {
    type Error = crate::error::Error;

    fn try_from(bytes: &[u8]) -> StdResult<Self, Self::Error> {
        serde_json::from_slice(bytes).map_err(|e| serializer_err(e, None))
    }
}
