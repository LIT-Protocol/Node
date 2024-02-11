use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;

use serde::{Deserialize, Serialize};

use crate::error::{io_err, serializer_err, validation_err, Result};

pub const CONTRACT_RESOLVER_SUBNET_CFG_LOCAL_PATH: &str = "./<SUBNET_ID>-subnet-config.yaml";
pub const CONTRACT_RESOLVER_SUBNET_CFG_PATHS: [&str; 2] = [
    CONTRACT_RESOLVER_SUBNET_CFG_LOCAL_PATH,
    "/var/lit/os/blockchain/resolver/<SUBNET_ID>-subnet-config.yaml",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubnetConfig {
    subnet_id: String,
    contracts: HashMap<String, SubnetContractEntry>,
}

impl SubnetConfig {
    pub fn new<S>(subnet_id: S) -> Self
    where
        S: Into<String>,
    {
        Self { subnet_id: subnet_id.into(), contracts: HashMap::new() }
    }

    pub fn load<S>(subnet_id: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        for path in CONTRACT_RESOLVER_SUBNET_CFG_PATHS {
            if let Some(us) = Self::try_load(subnet_id.as_ref(), path) {
                return Some(us);
            }
        }

        None
    }

    pub fn load_local<S>(subnet_id: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Self::try_load(subnet_id, CONTRACT_RESOLVER_SUBNET_CFG_LOCAL_PATH)
    }

    fn try_load<S, P>(subnet_id: S, path: P) -> Option<Self>
    where
        S: AsRef<str>,
        P: AsRef<str>,
    {
        let path = path.as_ref().replace("<SUBNET_ID>", subnet_id.as_ref());
        let path = PathBuf::from(path);
        if path.exists() {
            return Self::try_from(path.as_path()).ok();
        }

        None
    }

    // Accessors
    pub fn contracts(&self) -> &HashMap<String, SubnetContractEntry> {
        &self.contracts
    }

    // Validator
    pub fn verify(&self) -> Result<()> {
        for (contract_key, contract) in self.contracts.iter() {
            contract.verify().map_err(|e| {
                validation_err(
                    e,
                    Some(format!(
                        "invalid config: subnet ('{}') contract '{}' invalid",
                        &self.subnet_id, contract_key
                    )),
                )
            })?;
        }

        Ok(())
    }

    // Insert
    pub fn insert<K, A>(&mut self, contract_key: K, address: A) -> &mut Self
    where
        K: Into<String>,
        A: Into<String>,
    {
        self.contracts.insert(contract_key.into(), SubnetContractEntry::new(address));
        self
    }

    // Save
    pub fn save_local(&self) -> Result<()> {
        let path = PathBuf::from(CONTRACT_RESOLVER_SUBNET_CFG_LOCAL_PATH);
        let content = serde_yaml::to_string(&self).map_err(|e| serializer_err(e, None))?;
        fs::write(path.as_path(), content).map_err(|e| io_err(e, None))?;

        Ok(())
    }
}

impl TryFrom<&Path> for SubnetConfig {
    type Error = crate::error::Error;

    fn try_from(value: &Path) -> StdResult<Self, Self::Error> {
        Self::try_from(fs::read(value).map_err(|e| io_err(e, None))?)
    }
}

impl TryFrom<Vec<u8>> for SubnetConfig {
    type Error = crate::error::Error;

    fn try_from(bytes: Vec<u8>) -> StdResult<Self, Self::Error> {
        Self::try_from(&bytes[..])
    }
}

impl TryFrom<&[u8]> for SubnetConfig {
    type Error = crate::error::Error;

    fn try_from(bytes: &[u8]) -> StdResult<Self, Self::Error> {
        serde_yaml::from_slice(bytes).map_err(|e| serializer_err(e, None))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubnetContractEntry {
    address: String,
}

impl SubnetContractEntry {
    pub fn new<A>(address: A) -> Self
    where
        A: Into<String>,
    {
        Self { address: address.into() }
    }

    // Accessors
    pub fn address(&self) -> &String {
        &self.address
    }

    // Validator
    pub fn verify(&self) -> Result<()> {
        if self.address.is_empty() {
            return Err(validation_err("no address defined", None));
        }
        if !self.address.starts_with("0x") {
            return Err(validation_err("address does not start with 0x", None));
        }

        Ok(())
    }
}
