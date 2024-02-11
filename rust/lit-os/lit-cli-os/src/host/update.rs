use lit_core::config::LitConfig;
use std::process::{Command, Stdio};

use crate::config::LitCliOsConfig;

pub(crate) fn host_update(cfg: LitConfig) {
    let update_cmd =
        cfg.litos_host_update_cmd().expect("failed to determine the lit os install location");

    let mut cmd = Command::new(update_cmd);
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

    cmd.status().expect("failed to run host update command");
}
