use lit_core::utils::asserts::string_option_is_defined;
use std::path::{Path, PathBuf};

use crate::error::{config_err, Result};
use lit_core::utils::env::parse_env_file_to_map;
use lit_core::utils::env::Error as EnvError;

pub const GUEST_INSTANCE_ENV_FILE: &str = "instance.env";

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
#[allow(unused)]
pub struct GuestInstanceEnv {
    pub build_id: Option<String>,
    pub subnet_id: Option<String>,
    pub instance_id: Option<String>,
    pub instance_name: Option<String>,
    pub instance_name_suffix: Option<String>,
    pub instance_labels: Option<String>,
    pub instance_unix: Option<String>,
    pub instance_date: Option<String>,
    pub instance_service: Option<String>,
    pub instance_vcpus: Option<String>,
    pub instance_mem: Option<String>,
    pub instance_img_size: Option<String>,
    pub instance_net_int_mac: Option<String>,
    pub instance_net_ext_mac: Option<String>,
}

impl GuestInstanceEnv {
    pub fn labels(&self) -> Vec<String> {
        if let Some(labels) = self.instance_labels.as_ref() {
            return labels.split(' ').map(|v| v.to_string()).collect();
        }

        Vec::new()
    }

    pub fn verify(&self) -> Result<()> {
        if !string_option_is_defined(self.build_id.as_ref()) {
            return Err(config_err("GuestInstanceEnv build_id is not defined", None));
        }
        if self.build_id.as_ref().unwrap().len() < 8 {
            return Err(config_err("GuestBuildEnv build_id is too short", None));
        }
        if !string_option_is_defined(self.subnet_id.as_ref()) {
            return Err(config_err("GuestInstanceEnv subnet_id is not defined", None));
        }
        if !string_option_is_defined(self.instance_id.as_ref()) {
            return Err(config_err("GuestInstanceEnv instance_id is not defined", None));
        }
        if !string_option_is_defined(self.instance_name.as_ref()) {
            return Err(config_err("GuestInstanceEnv instance_name is not defined", None));
        }
        if !string_option_is_defined(self.instance_service.as_ref()) {
            return Err(config_err("GuestInstanceEnv instance_service is not defined", None));
        }
        if !string_option_is_defined(self.instance_vcpus.as_ref()) {
            return Err(config_err("GuestInstanceEnv instance_vcpus is not defined", None));
        }
        if !string_option_is_defined(self.instance_mem.as_ref()) {
            return Err(config_err("GuestInstanceEnv instance_mem is not defined", None));
        }

        Ok(())
    }

    pub fn instance_vcpus_i64(&self) -> Option<i64> {
        self.instance_vcpus.as_ref()?.parse().ok()
    }
}

pub fn read_guest_instance_env<P: AsRef<Path>>(
    instance_dir: P,
) -> std::result::Result<Option<GuestInstanceEnv>, EnvError> {
    let mut env_file = PathBuf::from(instance_dir.as_ref());
    env_file.push(GUEST_INSTANCE_ENV_FILE);

    if !env_file.exists() {
        return Ok(None);
    }

    let mut env = GuestInstanceEnv::default();

    let env_data = parse_env_file_to_map(env_file.to_str().unwrap(), true)?;

    for (key, value) in env_data {
        match key.as_str() {
            "BUILD_ID" => env.build_id = Some(value),
            "SUBNET_ID" => env.subnet_id = Some(value),
            "INSTANCE_ID" => env.instance_id = Some(value),
            "INSTANCE_NAME" => env.instance_name = Some(value),
            "INSTANCE_NAME_SUFFIX" => env.instance_name_suffix = Some(value),
            "INSTANCE_LABELS" => env.instance_labels = Some(value),
            "INSTANCE_UNIX" => env.instance_unix = Some(value),
            "INSTANCE_DATE" => env.instance_date = Some(value),
            "INSTANCE_SERVICE" => env.instance_service = Some(value),
            "INSTANCE_VCPUS" => env.instance_vcpus = Some(value),
            "INSTANCE_MEM" => env.instance_mem = Some(value),
            "INSTANCE_IMG_SIZE" => env.instance_img_size = Some(value),
            "INSTANCE_NET_INT_MAC" => env.instance_net_int_mac = Some(value),
            "INSTANCE_NET_EXT_MAC" => env.instance_net_ext_mac = Some(value),
            _ => {}
        }
    }

    Ok(Some(env))
}
