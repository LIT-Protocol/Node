use std::fs;
use std::path::Path;
use std::result::Result as StdResult;

use serde::{Deserialize, Serialize};

use lit_core::config::envs::LitEnv;

use crate::error::{io_err, serializer_err, validation_err, Result};
use crate::guest::types::GuestType;
use crate::utils::validate::validate_host_name;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct CloudInitUserData {
    fqdn: String,
}

impl CloudInitUserData {
    // Accessors
    pub fn fqdn(&self) -> &String {
        &self.fqdn
    }

    // Verify
    pub fn verify(
        &self, guest_type: Option<&GuestType>, env: Option<&LitEnv>, instance_id: Option<&str>,
        guest_kind: Option<&str>,
    ) -> Result<()> {
        validate_host_name(self.fqdn.as_str(), guest_type, env, instance_id, guest_kind).map_err(
            |e| validation_err(e, Some("cloud-init user-data validation failed for fqdn".into())),
        )?;

        Ok(())
    }
}

impl TryFrom<&Path> for CloudInitUserData {
    type Error = crate::error::Error;

    fn try_from(value: &Path) -> StdResult<Self, Self::Error> {
        Self::try_from(fs::read(value).map_err(|e| io_err(e, None))?)
    }
}

impl TryFrom<Vec<u8>> for CloudInitUserData {
    type Error = crate::error::Error;

    fn try_from(bytes: Vec<u8>) -> StdResult<Self, Self::Error> {
        Self::try_from(&bytes[..])
    }
}

impl TryFrom<&[u8]> for CloudInitUserData {
    type Error = crate::error::Error;

    fn try_from(bytes: &[u8]) -> StdResult<Self, Self::Error> {
        serde_yaml::from_slice(bytes).map_err(|e| serializer_err(e, None))
    }
}

#[cfg(test)]
mod tests {
    use lit_core::config::envs::LitEnv;
    use std::path::PathBuf;

    use crate::guest::cloud_init::user_data::CloudInitUserData;
    use crate::guest::types::GuestType;

    const RESOURCES_TEST_DIR: &str = "resources/test/guest/cloud_init";

    #[test]
    fn load_ok_test() {
        let file_path = get_test_path("user-data-ok");

        let _ = CloudInitUserData::try_from(file_path.as_path()).expect("failed to load user-data");
    }

    #[test]
    fn load_invalid_test() {
        {
            let (test, exp_err) = ("user-data-invalid1", "unknown field `other`");
            println!("test: {test}");

            let file_path = get_test_path(test);

            let res = CloudInitUserData::try_from(file_path.as_path());

            assert!(res.is_err());

            let err = res.err().unwrap();
            if !err.to_string().contains(exp_err) {
                panic!("err does not contain '{exp_err}': {err:?}");
            }
        }
    }

    #[test]
    fn verify_ok_test() {
        let file_path = get_test_path("user-data-ok");

        let meta_data =
            CloudInitUserData::try_from(file_path.as_path()).expect("failed to load user-data");

        let res =
            meta_data.verify(Some(&GuestType::Prov), Some(&LitEnv::Dev), Some("0563e3e8"), None);
        if res.is_err() {
            panic!("failed to verify user-data: {:?}", res.err().unwrap());
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
