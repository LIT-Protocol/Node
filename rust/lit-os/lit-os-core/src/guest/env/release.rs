use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;

use lit_core::utils::asserts::string_option_is_defined;
use lit_core::utils::env::{parse_env_to_map, Error as EnvError};

use crate::error::{generic_err, Result};
use crate::guest::env::build::GuestBuildEnv;
use crate::guest::env::instance::GuestInstanceEnv;

pub const GUEST_RELEASE_ENV_FILE: &str = "release.env";

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
#[allow(unused)]
pub struct GuestReleaseEnv {
    pub release_id: Option<String>,
    pub release_unix: Option<String>,
    pub release_subnet_id: Option<String>,
}

impl GuestReleaseEnv {
    pub fn verify(&self) -> Result<()> {
        if !string_option_is_defined(self.release_id.as_ref()) {
            return Err(generic_err("GuestReleaseEnv release_id is not defined", None));
        }
        if !string_option_is_defined(self.release_unix.as_ref()) {
            return Err(generic_err("GuestReleaseEnv release_unix is not defined", None));
        }
        if !string_option_is_defined(self.release_subnet_id.as_ref()) {
            return Err(generic_err("GuestReleaseEnv release_subnet_id is not defined", None));
        }

        // Verify release_id
        let release_id = self.release_id.as_ref().unwrap();
        let release_subnet_id = self.release_subnet_id.as_ref().unwrap();

        if !release_id.ends_with(release_subnet_id) {
            return Err(generic_err(
                format!(
                    "GuestReleaseEnv release_id ({release_id}) is invalid, does not end in subnet id ({release_subnet_id})"
                ),
                None,
            ));
        }

        Ok(())
    }

    pub fn matches_build_env(&self, build_env: &GuestBuildEnv) -> Result<()> {
        // Assumes both sides have had prepare() called first.
        let release_id = self.release_id.as_ref().unwrap();
        let build_id = build_env.build_id.as_ref().unwrap();

        if !release_id.starts_with(build_id) {
            return Err(generic_err(format!("GuestReleaseEnv release_id does not match build.env build_id prefix: '{release_id}' vs '{build_id:?}'"), None));
        }

        Ok(())
    }
}

pub fn find_instance_release_path(
    instance_path: &Path, instance_env: &GuestInstanceEnv,
) -> Option<PathBuf> {
    let mut instance_release_pb = instance_path.to_path_buf();
    instance_release_pb.push("release");

    if instance_release_pb.exists() {
        return Some(instance_release_pb);
    }

    if let Some(subnet_id) = instance_env.subnet_id.as_ref() {
        let mut instance_release_pb = instance_path.to_path_buf();
        instance_release_pb.push("releases");
        instance_release_pb.push(subnet_id);
        if instance_release_pb.exists() {
            return Some(instance_release_pb);
        }
    }

    None
}

pub fn read_guest_release_env<P: AsRef<Path>>(
    path: P,
) -> StdResult<Option<GuestReleaseEnv>, EnvError> {
    let mut filename = PathBuf::from(path.as_ref());
    filename.push(GUEST_RELEASE_ENV_FILE);

    if !filename.exists() {
        return Ok(None);
    }

    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    load_guest_release_env(&mut reader)
}

pub fn load_guest_release_env<R: Read>(
    reader: &mut BufReader<R>,
) -> StdResult<Option<GuestReleaseEnv>, EnvError> {
    let mut env = GuestReleaseEnv::default();

    let env_data = parse_env_to_map(reader, true)?;

    for (key, value) in env_data {
        match key.as_str() {
            "RELEASE_ID" => env.release_id = Some(value),
            "RELEASE_UNIX" => env.release_unix = Some(value),
            "RELEASE_SUBNET_ID" => env.release_subnet_id = Some(value),
            _ => {}
        }
    }

    Ok(Some(env))
}
