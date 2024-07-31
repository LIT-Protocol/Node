use lit_os_core::error::{generic_err, Result};
use nix::unistd::Uid;
use std::process::{Command, Stdio};

pub fn busybox_poweroff() -> Result<()> {
    if !Uid::effective().is_root() {
        return Err(generic_err("must be root to run busybox_poweroff", None));
    }

    let mut cmd = Command::new("poweroff")
        .arg("-f")
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .map_err(|e| generic_err(e, None))?;

    let status = cmd.wait().map_err(|e| generic_err(e, None))?;

    if !status.success() {
        return Err(generic_err("poweroff failed to run", None));
    }

    Ok(())
}

/// ipconfig -t ${ROUNDTTT} -c "${IP}" -d "${DEVICE}"
pub fn busybox_ipconfig(method: &str, dev: &str, timeout: usize) -> Result<()> {
    if !Uid::effective().is_root() {
        return Err(generic_err("must be root to run busybox_ipconfig", None));
    }

    let mut cmd = Command::new("ipconfig")
        .arg("-t")
        .arg(format!("{timeout}"))
        .arg("-c")
        .arg(method)
        .arg("-d")
        .arg(dev)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .map_err(|e| generic_err(e, None))?;

    let status = cmd.wait().map_err(|e| generic_err(e, None))?;

    if !status.success() {
        return Err(generic_err("ipconfig failed to run", None));
    }

    Ok(())
}
