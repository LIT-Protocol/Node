use crate::error::{io_err, serializer_err, validation_err, Result};
use crate::guest::types::GuestType;
use crate::utils::validate::validate_host_name;
use lit_core::config::envs::LitEnv;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::result::Result as StdResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct CloudInitMetaData {
    instance_id: String,
}

impl CloudInitMetaData {
    // Accessors
    pub fn instance_id(&self) -> &String {
        &self.instance_id
    }

    // Verify
    pub fn verify(
        &self, guest_type: Option<&GuestType>, env: Option<&LitEnv>, instance_id: Option<&str>,
        guest_kind: Option<&str>,
    ) -> Result<()> {
        validate_host_name(self.instance_id.as_str(), guest_type, env, instance_id, guest_kind)
            .map_err(|e| {
                validation_err(
                    e,
                    Some("cloud-init meta-data validation failed for instance_id".into()),
                )
            })?;

        Ok(())
    }
}

impl TryFrom<&Path> for CloudInitMetaData {
    type Error = crate::error::Error;

    fn try_from(value: &Path) -> StdResult<Self, Self::Error> {
        Self::try_from(fs::read(value).map_err(|e| io_err(e, None))?)
    }
}

impl TryFrom<Vec<u8>> for CloudInitMetaData {
    type Error = crate::error::Error;

    fn try_from(bytes: Vec<u8>) -> StdResult<Self, Self::Error> {
        Self::try_from(&bytes[..])
    }
}

impl TryFrom<&[u8]> for CloudInitMetaData {
    type Error = crate::error::Error;

    fn try_from(bytes: &[u8]) -> StdResult<Self, Self::Error> {
        serde_json::from_slice(bytes).map_err(|e| serializer_err(e, None))
    }
}

#[cfg(test)]
mod tests {
    use crate::guest::cloud_init::meta_data::CloudInitMetaData;
    use crate::guest::types::GuestType;
    use lit_core::config::envs::LitEnv;
    use std::path::PathBuf;

    const RESOURCES_TEST_DIR: &str = "resources/test/guest/cloud_init";

    #[test]
    fn load_ok_test() {
        let file_path = get_test_path("meta-data-ok");

        let _ = CloudInitMetaData::try_from(file_path.as_path()).expect("failed to load meta-data");
    }

    #[test]
    fn load_invalid_test() {
        {
            let (test, exp_err) = ("meta-data-invalid1", "unknown field `other-field`");
            println!("test: {test}");

            let file_path = get_test_path(test);

            let res = CloudInitMetaData::try_from(file_path.as_path());

            assert!(res.is_err());

            let err = res.err().unwrap();
            if !err.to_string().contains(exp_err) {
                panic!("err does not contain '{exp_err}': {err:?}");
            }
        }
    }

    #[test]
    fn verify_ok_test() {
        for file in ["meta-data-ok", "meta-data-ok2"] {
            let file_path = get_test_path(file);

            let meta_data =
                CloudInitMetaData::try_from(file_path.as_path()).expect("failed to load meta-data");

            let res = meta_data.verify(
                Some(&GuestType::Prov),
                Some(&LitEnv::Dev),
                Some("0563e3e8"),
                None,
            );
            if res.is_err() {
                panic!("failed to verify meta-data ({}): {:?}", file, res.err().unwrap());
            }
        }
    }

    #[test]
    fn verify_custom_ok_test() {
        for file in ["meta-data-ok3"] {
            let file_path = get_test_path(file);

            let meta_data =
                CloudInitMetaData::try_from(file_path.as_path()).expect("failed to load meta-data");

            let res = meta_data.verify(
                Some(&GuestType::Custom),
                Some(&LitEnv::Dev),
                Some("0563e3e8"),
                Some("salt-master"),
            );
            if res.is_err() {
                panic!("failed to verify meta-data ({}): {:?}", file, res.err().unwrap());
            }
        }
    }

    #[test]
    fn verify_invalid_test() {
        for (test, exp_err, guest_type, env,  instance_id, guest_kind) in [
            ("meta-data-invalid2", "hostname is not a valid for lit-os guests",
             None, None, None, None),
            ("meta-data-invalid3", "hostname is not a valid for lit-os guests",
             None, None, None, None),
            ("meta-data-ok", "hostname 'prov-dev-0563e3e8.litos-guest.litnet.io' does not contain provided instance id 'abcd'",
             Some(&GuestType::Prov), Some(&LitEnv::Dev), Some("abcd"), None),
            ("meta-data-ok", "hostname 'prov-dev-0563e3e8.litos-guest.litnet.io' does not contain provided env 'prod'",
             Some(&GuestType::Prov), Some(&LitEnv::Prod), Some("0563e3e8"), None),
            ("meta-data-ok", "hostname 'prov-dev-0563e3e8.litos-guest.litnet.io' does not contain provided guest_type 'node'",
             Some(&GuestType::Node), Some(&LitEnv::Dev), Some("0563e3e8"), None),
            ("meta-data-ok3", "hostname is not a valid for lit-os guests (does not match)",
             Some(&GuestType::Node), Some(&LitEnv::Dev), Some("0563e3e8"), None),
        ] {
            println!("test: {test}");

            let file_path = get_test_path(test);

            let cloud_init = CloudInitMetaData::try_from(
                file_path.as_path())
                .expect("failed to load");

            let res = cloud_init.verify(guest_type, env, instance_id, guest_kind);

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
