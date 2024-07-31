use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::result::Result as StdResult;

use serde::{Deserialize, Serialize};
use serde_bytes_base64::Bytes;

use crate::error::{io_err, serializer_err, validation_err, Result};

pub const ONESHOT_VERSION_V1: u32 = 1;

pub const ACTION_TYPE_BOOTSTRAP: &str = "bootstrap";

pub const ACTION_SETTINGS_KEY_REQUEST: &str = "request";
pub const ACTION_SETTINGS_KEY_GUEST_VCPUS: &str = "guest_vcpus";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OneShotConfig {
    version: u32,
    actions: HashMap<String, ActionEntry>,
}

impl OneShotConfig {
    pub fn new() -> Self {
        Self { version: ONESHOT_VERSION_V1, actions: HashMap::new() }
    }

    // Accessors
    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn actions(&self) -> &HashMap<String, ActionEntry> {
        &self.actions
    }

    pub fn insert_action(&mut self, action: String, entry: ActionEntry) -> &mut Self {
        self.actions.insert(action, entry);
        self
    }

    pub fn len(&self) -> usize {
        self.actions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    // Verify
    pub fn verify(&self) -> Result<()> {
        if self.version != ONESHOT_VERSION_V1 {
            return Err(validation_err(
                format!("oneshot config invalid: version '{}' unsupported", self.version),
                None,
            ));
        }
        if self.actions.is_empty() {
            return Err(validation_err(
                "oneshot config invalid: no actions defined".to_string(),
                None,
            ));
        }
        for (action, entry) in self.actions.iter() {
            if action.is_empty() {
                return Err(validation_err(
                    "oneshot config invalid: empty action key defined".to_string(),
                    None,
                ));
            }
            entry.verify().map_err(|e| {
                validation_err(e, Some(format!("validation of action failed: {action}")))
            })?;
        }

        Ok(())
    }

    // Write
    pub fn write_file(&self, file: &Path) -> Result<()> {
        let toml_str = serde_yaml::to_string(&self).map_err(|e| serializer_err(e, None))?;
        fs::write(file, toml_str).map_err(|e| io_err(e, None))?;

        Ok(())
    }
}

impl Default for OneShotConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&Path> for OneShotConfig {
    type Error = crate::error::Error;

    fn try_from(value: &Path) -> StdResult<Self, Self::Error> {
        Self::try_from(fs::read(value).map_err(|e| io_err(e, None))?)
    }
}

impl TryFrom<Vec<u8>> for OneShotConfig {
    type Error = crate::error::Error;

    fn try_from(bytes: Vec<u8>) -> StdResult<Self, Self::Error> {
        Self::try_from(&bytes[..])
    }
}

impl TryFrom<&[u8]> for OneShotConfig {
    type Error = crate::error::Error;

    fn try_from(bytes: &[u8]) -> StdResult<Self, Self::Error> {
        serde_yaml::from_slice(bytes).map_err(|e| serializer_err(e, None))
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionEntry {
    settings: HashMap<String, Bytes>,
}

impl ActionEntry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn settings(&self) -> &HashMap<String, Bytes> {
        &self.settings
    }

    pub fn insert_setting(&mut self, key: String, val: Vec<u8>) -> &mut Self {
        self.settings.insert(key, Bytes::from(val));
        self
    }

    // Verify
    pub fn verify(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::guest::oneshot::config::OneShotConfig;

    const RESOURCES_TEST_DIR: &str = "resources/test/guest/oneshot";

    #[test]
    fn load_ok_test() {
        let file_path = get_test_path("config-ok.yaml");

        let _ =
            OneShotConfig::try_from(file_path.as_path()).expect("failed to load oneshot config");
    }

    #[test]
    fn load_invalid_test() {
        {
            let (test, exp_err) = ("config-invalid1.yaml", "unknown field `some`");
            println!("test: {test}");

            let file_path = get_test_path(test);

            let res = OneShotConfig::try_from(file_path.as_path());

            assert!(res.is_err());

            let err = res.err().unwrap();
            if !err.to_string().contains(exp_err) {
                panic!("err does not contain '{exp_err}': {err:?}");
            }
        }
    }

    #[test]
    fn verify_ok_test() {
        let file_path = get_test_path("config-ok.yaml");

        let config =
            OneShotConfig::try_from(file_path.as_path()).expect("failed to load oneshot config");

        let res = config.verify();
        if res.is_err() {
            panic!("failed to verify oneshot config: {:?}", res.err().unwrap());
        }
    }

    #[test]
    fn verify_invalid_test() {
        for (test, exp_err) in [
            ("config-invalid2.yaml", "oneshot config invalid: version '22' unsupported"),
            ("config-invalid3.yaml", "oneshot config invalid: no actions defined"),
        ] {
            println!("test: {test}");

            let file_path = get_test_path(test);

            let config = OneShotConfig::try_from(file_path.as_path()).expect("failed to load");

            let res = config.verify();

            assert!(res.is_err());

            let err = res.err().unwrap();
            if !err.to_string().contains(exp_err) {
                panic!("err does not contain '{exp_err}': {err:?}");
            }
        }
    }

    // Util
    fn get_test_path(path: &str) -> PathBuf {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push(RESOURCES_TEST_DIR);
        test_path.push(path);
        test_path
    }
}
