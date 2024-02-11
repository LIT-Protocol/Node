pub const CLOUD_INIT_FILE_META_DATA: &str = "meta-data";
pub const CLOUD_INIT_FILE_USER_DATA: &str = "user-data";
pub const CLOUD_INIT_FILE_NETWORK_CONFIG: &str = "network-config";
pub const CLOUD_INIT_FILE_LIT_CONFIG: &str = "config.toml";
pub const CLOUD_INIT_FILE_INIT_PW: &str = ".init.pw";
pub const CLOUD_INIT_FILE_SAFE_BOOT: &str = ".safe-boot";
pub const CLOUD_INIT_FILE_NO_RESIZE: &str = ".no-resize";

pub const ALLOWED_CLOUD_INIT_FILES: [&str; 7] = [
    CLOUD_INIT_FILE_LIT_CONFIG, CLOUD_INIT_FILE_META_DATA, CLOUD_INIT_FILE_USER_DATA,
    CLOUD_INIT_FILE_NETWORK_CONFIG, CLOUD_INIT_FILE_INIT_PW, CLOUD_INIT_FILE_SAFE_BOOT,
    CLOUD_INIT_FILE_NO_RESIZE,
];

pub mod context;
pub mod meta_data;
pub mod network_config;
pub mod user_data;
