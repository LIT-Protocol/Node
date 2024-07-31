use lit_blockchain::contracts::release::{
    RELEASE_OPTION_RO, RELEASE_OPTION_SSH, RELEASE_OPTION_USERS,
};
use lit_core::config::envs::LitEnv;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;
use std::str::FromStr;

use lit_core::utils::asserts::string_option_is_defined;
use lit_core::utils::env::{parse_env_to_map, Error as EnvError};
use lit_core::utils::option::bool_option_to_bool;

use crate::error::{config_err, conversion_err, generic_err, Result};
use crate::guest::types::{GuestCpuType, GuestOsType, GuestType};

pub const GUEST_ENV_BUILD_FILE: &str = "build.env";

pub const LIT_OS_BUILD_FILE: &str = "/etc/lit-os-build";
pub const LIT_OS_ENV_PREFIX: &str = "LIT_OS";

pub const DEV_DISK_BY_UUID: &str = "/dev/disk/by-uuid";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
#[allow(unused)]
pub struct GuestBuildEnv {
    pub build_id: Option<String>,
    pub build_unix: Option<String>,
    pub build_date: Option<String>,
    pub build_uname: Option<String>,
    pub build_type: Option<String>,
    pub build_kind: Option<String>,
    pub build_release: Option<String>,
    pub build_cpu_type: Option<String>,
    pub build_os_type: Option<String>,
    pub build_os_version: Option<String>,
    pub build_ref_img: Option<String>,
    pub build_img_name: Option<String>,
    pub build_img_size: Option<String>,
    pub build_root_size: Option<String>,
    pub build_hostname: Option<String>,
    pub build_opt_ro: Option<bool>,
    pub build_opt_users: Option<bool>,
    pub build_opt_ssh: Option<bool>,
    pub build_luks_root_uuid: Option<String>,
    pub build_luks_var_uuid: Option<String>,
    pub build_root_uuid: Option<String>,
    pub build_root_hash: Option<String>,
    pub build_var_uuid: Option<String>,
    pub build_var_hash: Option<String>,
    pub build_custom_source: Option<String>,
}

impl GuestBuildEnv {
    pub fn build_options(&self) -> Vec<String> {
        let mut options: Vec<String> = Vec::new();
        if self.build_opt_ro.unwrap_or(true) {
            options.push("ro".to_string());
        } else {
            options.push("rw".to_string());
        }
        if self.build_opt_users.unwrap_or(false) {
            options.push("users".to_string());
        }
        if self.build_opt_ssh.unwrap_or(false) {
            options.push("ssh".to_string());
        }

        options
    }

    pub fn build_options_bits(&self) -> u8 {
        let mut opts: u8 = 0;

        if bool_option_to_bool(self.build_opt_ro.as_ref()) {
            opts |= RELEASE_OPTION_RO;
        }
        if bool_option_to_bool(self.build_opt_users.as_ref()) {
            opts |= RELEASE_OPTION_USERS;
        }
        if bool_option_to_bool(self.build_opt_ssh.as_ref()) {
            opts |= RELEASE_OPTION_SSH;
        }

        opts
    }

    pub fn verify(&self, should_root_hash: bool, should_var_hash: bool) -> Result<()> {
        if !string_option_is_defined(self.build_id.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_id is not defined", None));
        }
        if self.build_id.as_ref().unwrap().len() < 8 {
            return Err(generic_err("GuestBuildEnv build_id is too short", None));
        }
        if !string_option_is_defined(self.build_unix.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_unix is not defined", None));
        }
        if !string_option_is_defined(self.build_date.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_date is not defined", None));
        }
        if !string_option_is_defined(self.build_uname.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_uname is not defined", None));
        }
        if !string_option_is_defined(self.build_type.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_type is not defined", None));
        }
        if !string_option_is_defined(self.build_release.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_release is not defined", None));
        }
        if !string_option_is_defined(self.build_cpu_type.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_cpu_type is not defined", None));
        }
        if !string_option_is_defined(self.build_os_type.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_os_type is not defined", None));
        }
        if !string_option_is_defined(self.build_os_version.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_os_version is not defined", None));
        }
        if !string_option_is_defined(self.build_ref_img.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_ref_img is not defined", None));
        }
        if !string_option_is_defined(self.build_img_name.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_img_name is not defined", None));
        }
        if !string_option_is_defined(self.build_img_size.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_img_size is not defined", None));
        }
        if !string_option_is_defined(self.build_root_size.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_root_size is not defined", None));
        }
        if !string_option_is_defined(self.build_hostname.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_hostname is not defined", None));
        }
        if !string_option_is_defined(self.build_luks_root_uuid.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_luks_root_uuid is not defined", None));
        }
        if !string_option_is_defined(self.build_luks_var_uuid.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_luks_var_uuid is not defined", None));
        }
        if !string_option_is_defined(self.build_root_uuid.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_root_uuid is not defined", None));
        }
        if should_root_hash && !string_option_is_defined(self.build_root_hash.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_root_hash is not defined", None));
        }
        if !string_option_is_defined(self.build_var_uuid.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_var_uuid is not defined", None));
        }
        if should_var_hash && !string_option_is_defined(self.build_var_hash.as_ref()) {
            return Err(generic_err("GuestBuildEnv build_var_hash is not defined", None));
        }

        // Custom
        let guest_type = self.guest_type()?;
        if GuestType::Custom.eq(&guest_type) && !string_option_is_defined(self.build_kind.as_ref())
        {
            return Err(generic_err(
                "GuestBuildEnv build_kind is not defined (required for custom)",
                None,
            ));
        }

        Ok(())
    }

    // Accessors
    pub fn env(&self) -> Result<LitEnv> {
        let env_str = self
            .build_release
            .as_ref()
            .ok_or_else(|| config_err("unexpected: missing build_release in build env", None))?
            .as_str();

        LitEnv::from_str(env_str).map_err(|e| conversion_err(e, None))
    }

    pub fn guest_type(&self) -> Result<GuestType> {
        let build_type_str = self
            .build_type
            .as_ref()
            .ok_or_else(|| config_err("unexpected: missing build_type in build env", None))?
            .as_str();

        GuestType::from_str(build_type_str).map_err(|e| conversion_err(e, None))
    }

    pub fn guest_kind(&self) -> Result<String> {
        let build_kind_str = self
            .build_kind
            .as_ref()
            .ok_or_else(|| config_err("unexpected: missing build_kind in build env", None))?
            .as_str();

        Ok(build_kind_str.to_string())
    }

    /// Will return a binary of the kind string (or default to an empty string)
    pub fn guest_kind_to_vec(&self) -> Vec<u8> {
        self.build_kind.clone().unwrap_or_default().as_bytes().to_vec()
    }

    pub fn guest_cpu_type(&self) -> Result<GuestCpuType> {
        let build_cpu_type_str = self
            .build_cpu_type
            .as_ref()
            .ok_or_else(|| config_err("unexpected: missing build_cpu_type in build env", None))?
            .as_str();

        GuestCpuType::from_str(build_cpu_type_str).map_err(|e| conversion_err(e, None))
    }

    pub fn guest_os_type(&self) -> Result<GuestOsType> {
        let build_os_type_str = self
            .build_os_type
            .as_ref()
            .ok_or_else(|| config_err("unexpected: missing build_os_type in build env", None))?
            .as_str();

        GuestOsType::from_str(build_os_type_str).map_err(|e| conversion_err(e, None))
    }

    pub fn guest_os_version(&self) -> Result<u8> {
        let build_os_version_str = self
            .build_os_version
            .as_ref()
            .ok_or_else(|| config_err("unexpected: missing build_os_version in build env", None))?
            .as_str();

        u8::from_str(build_os_version_str).map_err(|e| conversion_err(e, None))
    }

    pub fn build_luks_root_dev(&self) -> Option<PathBuf> {
        Some(PathBuf::from(format!("{}/{}", DEV_DISK_BY_UUID, self.build_luks_root_uuid.as_ref()?)))
    }

    pub fn build_luks_var_dev(&self) -> Option<PathBuf> {
        Some(PathBuf::from(format!("{}/{}", DEV_DISK_BY_UUID, self.build_luks_var_uuid.as_ref()?)))
    }

    pub fn build_root_dev(&self) -> Option<PathBuf> {
        Some(PathBuf::from(format!("{}/{}", DEV_DISK_BY_UUID, self.build_root_uuid.as_ref()?)))
    }

    pub fn build_var_dev(&self) -> Option<PathBuf> {
        Some(PathBuf::from(format!("{}/{}", DEV_DISK_BY_UUID, self.build_var_uuid.as_ref()?)))
    }

    pub fn guest_custom_source(&self) -> Result<String> {
        let build_custom_source_str = self
            .build_custom_source
            .as_ref()
            .ok_or_else(|| {
                config_err("unexpected: missing build_custom_source in build env", None)
            })?
            .as_str();

        Ok(build_custom_source_str.to_string())
    }
}

impl Default for GuestBuildEnv {
    fn default() -> Self {
        Self {
            build_id: None,
            build_unix: None,
            build_date: None,
            build_uname: None,
            build_type: None,
            build_release: None,
            build_cpu_type: None,
            build_os_type: None,
            build_os_version: None,
            build_ref_img: None,
            build_img_name: None,
            build_img_size: None,
            build_root_size: None,
            build_hostname: None,
            build_opt_ro: Some(true),
            build_opt_users: Some(false),
            build_opt_ssh: Some(false),
            build_luks_root_uuid: None,
            build_luks_var_uuid: None,
            build_root_uuid: None,
            build_root_hash: None,
            build_var_uuid: None,
            build_var_hash: None,
            build_kind: None,
            build_custom_source: None,
        }
    }
}

pub fn read_guest_build_env<P: AsRef<Path>>(
    build_dir: P,
) -> StdResult<Option<GuestBuildEnv>, EnvError> {
    let mut filename = PathBuf::from(build_dir.as_ref());
    filename.push(GUEST_ENV_BUILD_FILE);

    if !filename.exists() {
        return Ok(None);
    }

    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    load_guest_build_env(&mut reader, None)
}

pub fn read_lit_os_build_env() -> StdResult<Option<GuestBuildEnv>, EnvError> {
    let filename = PathBuf::from(LIT_OS_BUILD_FILE);

    if !filename.exists() {
        return Ok(None);
    }

    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    load_guest_build_env(&mut reader, Some(LIT_OS_ENV_PREFIX))
}

pub fn load_guest_build_env<R: Read>(
    reader: &mut BufReader<R>, prefix: Option<&str>,
) -> StdResult<Option<GuestBuildEnv>, EnvError> {
    let mut env = GuestBuildEnv::default();

    let env_data = parse_env_to_map(reader, true)?;

    fn apply_prefix(prefix: Option<&str>, key: &str) -> String {
        match prefix {
            Some(prefix) => format!("{prefix}_{key}"),
            _ => key.to_string(),
        }
    }

    for (key, value) in env_data {
        match key {
            k if k == apply_prefix(prefix, "BUILD_ID") => env.build_id = Some(value),
            k if k == apply_prefix(prefix, "BUILD_UNIX") => env.build_unix = Some(value),
            k if k == apply_prefix(prefix, "BUILD_DATE") => env.build_date = Some(value),
            k if k == apply_prefix(prefix, "BUILD_UNAME") => env.build_uname = Some(value),
            k if k == apply_prefix(prefix, "BUILD_TYPE") => env.build_type = Some(value),
            k if k == apply_prefix(prefix, "BUILD_KIND") => env.build_kind = Some(value),
            k if k == apply_prefix(prefix, "BUILD_RELEASE") => env.build_release = Some(value),
            k if k == apply_prefix(prefix, "BUILD_CPU_TYPE") => env.build_cpu_type = Some(value),
            k if k == apply_prefix(prefix, "BUILD_OS_TYPE") => env.build_os_type = Some(value),
            k if k == apply_prefix(prefix, "BUILD_OS_VERSION") => {
                env.build_os_version = Some(value)
            }
            k if k == apply_prefix(prefix, "BUILD_REF_IMG") => env.build_ref_img = Some(value),
            k if k == apply_prefix(prefix, "BUILD_IMG_NAME") => env.build_img_name = Some(value),
            k if k == apply_prefix(prefix, "BUILD_IMG_SIZE") => env.build_img_size = Some(value),
            k if k == apply_prefix(prefix, "BUILD_ROOT_SIZE") => env.build_root_size = Some(value),
            k if k == apply_prefix(prefix, "BUILD_HOSTNAME") => env.build_hostname = Some(value),
            k if k == apply_prefix(prefix, "BUILD_OPT_RO") => env.build_opt_ro = Some(value != "0"),
            k if k == apply_prefix(prefix, "BUILD_OPT_USERS") => {
                env.build_opt_users = Some(value == "1")
            }
            k if k == apply_prefix(prefix, "BUILD_OPT_SSH") => {
                env.build_opt_ssh = Some(value == "1")
            }
            k if k == apply_prefix(prefix, "BUILD_LUKS_ROOT_UUID") => {
                env.build_luks_root_uuid = Some(value)
            }
            k if k == apply_prefix(prefix, "BUILD_LUKS_VAR_UUID") => {
                env.build_luks_var_uuid = Some(value)
            }
            k if k == apply_prefix(prefix, "BUILD_ROOT_UUID") => env.build_root_uuid = Some(value),
            k if k == apply_prefix(prefix, "BUILD_ROOT_HASH") => env.build_root_hash = Some(value),
            k if k == apply_prefix(prefix, "BUILD_VAR_UUID") => env.build_var_uuid = Some(value),
            k if k == apply_prefix(prefix, "BUILD_VAR_HASH") => env.build_var_hash = Some(value),
            k if k == apply_prefix(prefix, "BUILD_CUSTOM_SOURCE") => {
                env.build_custom_source = Some(value)
            }
            _ => {}
        }
    }

    Ok(Some(env))
}
