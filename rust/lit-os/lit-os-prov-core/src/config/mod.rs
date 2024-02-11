use std::path::PathBuf;

use lit_core::config::{LitConfig, LitConfigBuilder};
use lit_os_core::config::LitOsGuestConfig;

use crate::error::Result;
use crate::release::common::keys::AUTHOR_KEY_FILE_NAME;

pub const PROV_SHARED_PATH: &str = "/var/lit/os/prov/shared";

pub trait LitOsProvConfig {
    fn apply_defaults(builder: LitConfigBuilder) -> Result<LitConfigBuilder>;
    fn litos_prov_shared_release_path(&self) -> PathBuf;
    fn litos_prov_shared_keys_path(&self) -> PathBuf;
    fn litos_prov_shared_author_key_path(&self) -> PathBuf;
}

impl LitOsProvConfig for LitConfig {
    fn apply_defaults(mut builder: LitConfigBuilder) -> Result<LitConfigBuilder> {
        // Set defaults
        builder = <LitConfig as LitOsGuestConfig>::apply_defaults(builder)?;

        Ok(builder)
    }

    fn litos_prov_shared_release_path(&self) -> PathBuf {
        PathBuf::from(format!("{}/{}", PROV_SHARED_PATH, "releases"))
    }

    fn litos_prov_shared_keys_path(&self) -> PathBuf {
        PathBuf::from(format!("{}/{}", PROV_SHARED_PATH, "keys"))
    }

    fn litos_prov_shared_author_key_path(&self) -> PathBuf {
        let mut path = self.litos_prov_shared_keys_path();
        path.push(AUTHOR_KEY_FILE_NAME);
        path
    }
}
