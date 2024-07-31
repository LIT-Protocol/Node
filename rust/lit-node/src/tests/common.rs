use std::env;
use std::path::PathBuf;

use crate::config::LitNodeConfig;
use ethers::types::H160;
use lit_core::{
    config::{LitConfig, LitConfigBuilder},
    utils::binary::bytes_to_hex,
};

pub(crate) fn get_test_config_with_key(key: Option<String>) -> LitConfig {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("config/test/lit_sig_cfg.toml");

    env::set_var("LIT_CONFIG_FILE", test_path);

    LitConfigBuilder::new_with_paths(
        key,
        Some("/tmp/fake/nope".to_string()),
        "whatever",
        "/tmp/fake/nope",
    )
    .build()
    .expect("failed to load config")
}

pub(crate) fn get_test_config() -> LitConfig {
    get_test_config_with_key(None)
}

pub(crate) fn get_backup_config() -> LitConfig {
    let config_builder = LitConfigBuilder::default()
        .set_default("subnet.id", "test subnet id")
        .set_default(
            "node.staker_address",
            bytes_to_hex(H160::from([1; 20]).as_bytes()),
        )
        .set_default("lit.env", "dev");
    LitConfig::from_builder(config_builder).unwrap()
}
