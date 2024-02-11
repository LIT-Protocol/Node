pub const ONESHOT_FILE_CONFIG: &str = "config.yaml";

pub const ALLOWED_ONESHOT_FILES: [&str; 2] = [ONESHOT_FILE_CONFIG, "lost+found"];

pub mod config;
pub mod context;
