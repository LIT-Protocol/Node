use crate::error::{generic_err, Result};
use nix::unistd::Uid;
use std::process::{Command, Stdio};

pub fn dmesg_contains(msg: &str) -> Result<bool> {
    if !Uid::effective().is_root() {
        return Err(generic_err("must be root to run dmesg_contains", None));
    }

    let dmesg_cmd = Command::new("dmesg")
        .stderr(Stdio::inherit())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| generic_err(e, None))?;

    let mut grep_cmd = Command::new("grep")
        .arg(msg)
        .stderr(Stdio::inherit())
        .stdin(Stdio::from(dmesg_cmd.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| generic_err(e, None))?;

    let status = grep_cmd.wait().map_err(|e| generic_err(e, None))?;

    Ok(status.success())
}
