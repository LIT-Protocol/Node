use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use log::{error, info};

use lit_os_core::config::LitOsGuestConfig;
use lit_os_core::error::{io_err, Result};
use lit_os_core::guest::cloud_init::network_config::NET_IF_INTERNAL;
use lit_os_core::utils::mount::busybox_umount;

use crate::init::context::InitContext;
use crate::init::stage::Outcome;

pub(crate) async fn run(ctx: &mut InitContext) -> Result<Outcome> {
    // Unmount everything
    unmount_all(ctx)?;

    if !ctx.has_oneshot_actions() {
        // Everything else (not really needed for oneshot as we'll poweroff).
        cleanup_script(ctx)?;
    }

    Ok(Outcome::Continue)
}

pub(crate) fn unmount_all(ctx: &mut InitContext) -> Result<()> {
    unmount(ctx.cfg().litos_cloud_init_mnt(), false, true);
    unmount(ctx.cfg().litos_oneshot_mnt(), false, true);
    unmount(ctx.cfg().litos_initrd_var_mnt(), true, false); // No unlink as it lives under /mnt/root
    unmount(ctx.cfg().litos_initrd_root_mnt(), true, true);

    Ok(())
}

pub(crate) fn unmount(mnt: PathBuf, force: bool, unlink: bool) {
    if mnt.exists() {
        info!("Unmounting {:?}", &mnt);
        if let Err(e) = busybox_umount(&mnt, force) {
            error!("error unmounting: {}", e);
        }

        if unlink {
            if let Err(e) = fs::remove_dir_all(mnt.as_path()) {
                error!("error removing dir ({:?}): {}", mnt, e);
            }
        }
    }
}

pub(crate) fn cleanup_script(_ctx: &mut InitContext) -> Result<()> {
    let out = Command::new("/opt/lit/os/init/lit-os-init-cleanup.sh")
        .arg("--net-internal-iface")
        .arg(NET_IF_INTERNAL)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| io_err(e, Some("failed to run: lit-os-init-cleanup.sh".into())))?;

    if !out.status.success() {
        return Err(io_err("failed to run: lit-os-init-cleanup.sh", None));
    }

    Ok(())
}
