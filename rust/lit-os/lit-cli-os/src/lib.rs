extern crate core;

pub mod cmd;
pub mod config;
#[cfg(any(feature = "guest-instance", feature = "guest-build"))]
pub mod guest;
#[cfg(feature = "host-core")]
pub mod host;

pub use cmd::os::{handle_cmd_os, OsArgs};
