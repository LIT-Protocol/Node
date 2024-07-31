use crate::error::{generic_err, Result};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn mount(typ: &str, dev: &Path, mnt: &Path, options: Option<&str>) -> Result<()> {
    let mut cmd = Command::new("mount");
    cmd.arg("-t").arg(typ);

    if let Some(options) = options {
        cmd.arg("-o").arg(options);
    }

    let mut cmd = cmd
        .arg(dev)
        .arg(mnt)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .map_err(|e| generic_err(e, None))?;

    let status = cmd.wait().map_err(|e| generic_err(e, None))?;

    if !status.success() {
        return Err(generic_err("mount command failed", None));
    }

    Ok(())
}

pub fn umount(mnt: &Path, recursive: bool) -> Result<()> {
    let mut cmd = Command::new("umount");

    if recursive {
        cmd.arg("-R");
    }

    let mut cmd = cmd
        .arg(mnt)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .map_err(|e| generic_err(e, None))?;

    let status = cmd.wait().map_err(|e| generic_err(e, None))?;

    if !status.success() {
        return Err(generic_err("umount command failed", None));
    }

    Ok(())
}

pub fn busybox_umount(mnt: &Path, force: bool) -> Result<()> {
    let mut cmd = Command::new("umount");

    if force {
        cmd.arg("-f");
    }

    let mut cmd = cmd
        .arg(mnt)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .map_err(|e| generic_err(e, None))?;

    let status = cmd.wait().map_err(|e| generic_err(e, None))?;

    if !status.success() {
        return Err(generic_err("umount command failed", None));
    }

    Ok(())
}
