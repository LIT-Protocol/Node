use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{fs, thread};

use log::{as_error, error, info};

use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use lit_core::utils::hash::sha512_file;
use lit_core::utils::option::bool_option_to_bool;
use lit_os_core::config::LitOsGuestConfig;
use lit_os_core::error::{config_err, generic_err, io_err, unexpected_err, validation_err, Result};
use lit_os_core::guest::types::GuestType;
use lit_os_core::utils::mount::mount;

use crate::init::context::{
    InitContext, CTX_KEY_ROOT_PASSPHRASE_BOOT, CTX_KEY_VAR_PASSPHRASE_BOOT,
};
use crate::init::stage::Outcome;
#[cfg(not(target_os = "macos"))]
use crate::utils::libcryptsetup::{cryptsetup_activate_dev, cryptsetup_deactivate_dev};
use crate::utils::volume::{blockdev_rereadpt, e2fsck, resize2fs, resize_partition_to_max};

const LUKS_NAME_ROOT: &str = "luks-rootfs";
const LUKS_NAME_VAR: &str = "luks-varfs";

pub(crate) async fn run(ctx: &mut InitContext) -> Result<Outcome> {
    // Activate volumes
    activate_luks_volumes(ctx)?;

    // Verify hashes
    verify_hashes(ctx)?;

    if let Err(e) = maybe_resize_volumes(ctx) {
        error!(error = as_error!(e); "unable to proceed: volume resize failed");

        return Ok(Outcome::Diagnose);
    }

    // Mount volumes
    mount_volumes(ctx)?;

    Ok(Outcome::Continue)
}

pub(crate) fn maybe_resize_volumes(ctx: &mut InitContext) -> Result<()> {
    if ctx.is_no_resize() {
        return Ok(());
    }

    let disk_dev = ctx.cfg().litos_disk_dev();
    let (_luks_var_dev, label) = var_volume_dev_and_label(ctx)?;
    let var_dev = ctx.build_env().build_var_dev().ok_or(config_err(
        "expected build_var_dev (based on build_var_uuid) to be defined",
        None,
    ))?;

    if resize_partition_to_max(disk_dev.as_path(), LUKS_NAME_VAR, false)? {
        info!("Resized volume '{label}' detected");

        // Deactivate to perform resize.
        deactivate_luks_volumes(ctx)?;

        // Now we can resize the partition
        info!("Resizing partition for volume '{label}'");

        if !resize_partition_to_max(disk_dev.as_path(), LUKS_NAME_VAR, true)? {
            return Err(unexpected_err(
                "expected a var volume resize was required but got miss-match",
                None,
            ));
        }

        // Re-read the partition table
        blockdev_rereadpt(disk_dev.as_path())?;
        thread::sleep(Duration::from_millis(1000));

        // Now we resize the file system
        info!("Resizing file system for var device: {var_dev:?}");

        activate_luks_var_volume(ctx)?;
        e2fsck(&var_dev)?;
        resize2fs(&var_dev)?;

        info!("Resized volume '{label}': Ok");

        // Activate root
        activate_luks_root_volume(ctx)?;

        return Ok(());
    }

    Ok(())
}

fn activate_luks_volumes(ctx: &mut InitContext) -> Result<()> {
    activate_luks_root_volume(ctx)?;
    activate_luks_var_volume(ctx)?;
    Ok(())
}

pub(crate) fn activate_luks_root_volume(ctx: &mut InitContext) -> Result<()> {
    let (root_dev, root_dev_label) = root_volume_dev_and_label(ctx)?;

    let root_passphrase = ctx
        .get_bin(CTX_KEY_ROOT_PASSPHRASE_BOOT)
        .ok_or_else(|| {
            generic_err(format!("missing init context key: {CTX_KEY_ROOT_PASSPHRASE_BOOT}"), None)
        })?
        .clone();

    let read_only =
        !ctx.cfg().is_dev() || bool_option_to_bool(ctx.cmdline_env().build_opt_ro.as_ref());

    activate_luks_volume(
        ctx,
        root_dev.as_path(),
        &root_dev_label,
        LUKS_NAME_ROOT,
        &root_passphrase[..],
        read_only,
    )
}

pub(crate) fn activate_luks_var_volume(ctx: &mut InitContext) -> Result<()> {
    let (var_dev, var_dev_label) = var_volume_dev_and_label(ctx)?;

    let var_passphrase = ctx
        .get_bin(CTX_KEY_VAR_PASSPHRASE_BOOT)
        .ok_or_else(|| {
            generic_err(format!("missing init context key: {CTX_KEY_VAR_PASSPHRASE_BOOT}"), None)
        })?
        .clone();

    activate_luks_volume(
        ctx,
        var_dev.as_path(),
        &var_dev_label,
        LUKS_NAME_VAR,
        &var_passphrase[..],
        false,
    )
}

#[allow(unused_variables)]
fn activate_luks_volume(
    ctx: &mut InitContext, dev_path: &Path, label: &String, name: &str, boot_passphrase: &[u8],
    read_only: bool,
) -> Result<()> {
    info!("Activating volume '{}' (read-only: {})", label, read_only);

    #[cfg(not(target_os = "macos"))]
    cryptsetup_activate_dev(dev_path, label, name, boot_passphrase, read_only)?;

    ctx.set_activated(true);

    Ok(())
}

pub(crate) fn deactivate_luks_volumes(ctx: &mut InitContext) -> Result<()> {
    deactivate_luks_root_volume(ctx)?;
    deactivate_luks_var_volume(ctx)?;
    Ok(())
}

#[allow(unused_variables)]
pub(crate) fn deactivate_luks_root_volume(ctx: &mut InitContext) -> Result<()> {
    let (root_dev, root_dev_label) = root_volume_dev_and_label(ctx)?;

    info!("Deactivating volume '{}'", root_dev_label);
    #[cfg(not(target_os = "macos"))]
    cryptsetup_deactivate_dev(root_dev.as_path(), root_dev_label.as_str(), LUKS_NAME_ROOT)?;

    Ok(())
}

#[allow(unused_variables)]
pub(crate) fn deactivate_luks_var_volume(ctx: &mut InitContext) -> Result<()> {
    let (var_dev, var_dev_label) = var_volume_dev_and_label(ctx)?;

    info!("Deactivating volume '{}'", var_dev_label);
    #[cfg(not(target_os = "macos"))]
    cryptsetup_deactivate_dev(var_dev.as_path(), var_dev_label.as_str(), LUKS_NAME_VAR)?;

    Ok(())
}

fn verify_hashes(ctx: &mut InitContext) -> Result<()> {
    verify_root_hash(ctx)?;
    verify_var_hash(ctx)?;

    Ok(())
}

fn verify_root_hash(ctx: &mut InitContext) -> Result<()> {
    let root_dev = ctx.build_env().build_root_dev().ok_or_else(|| {
        config_err("expected build_root_dev (based on build_root_uuid) to be defined", None)
    })?;
    let root_dev_label = format!("{}:{:?}", "root", root_dev.as_path());

    let root_hash = ctx
        .cmdline_env()
        .build_roothash
        .as_ref()
        .ok_or_else(|| config_err("expected build_roothash to be defined", None))?;
    let root_ro = bool_option_to_bool(ctx.build_env().build_opt_ro.as_ref());

    if root_ro || ctx.is_first_boot() {
        verify_hash(ctx, &root_dev, root_hash, &root_dev_label)?;
    }

    Ok(())
}

fn verify_var_hash(ctx: &mut InitContext) -> Result<()> {
    let guest_type = ctx.build_env().guest_type()?;

    let var_dev = ctx.build_env().build_var_dev().ok_or_else(|| {
        config_err("expected build_var_dev (based on build_var_uuid) to be defined", None)
    })?;
    let var_dev_label = format!("{}:{:?}", "var", var_dev.as_path());

    if let Some(var_hash) = ctx.cmdline_env().build_varhhash.as_ref() {
        if ctx.is_first_boot() && guest_type != GuestType::Prov {
            verify_hash(ctx, &var_dev, var_hash, &var_dev_label)?;
        }
    }

    Ok(())
}

fn verify_hash(_ctx: &InitContext, dev: &Path, hash: &String, label: &String) -> Result<()> {
    info!("Verifying hash for '{}'", label);

    let out = sha512_file(dev)?;
    let out = bytes_to_hex(out);
    if !out.eq(hash) {
        return Err(validation_err(
            format!("hash validation failed for '{label}' ('{hash}' vs '{out}')"),
            None,
        ));
    }

    Ok(())
}

fn mount_volumes(ctx: &mut InitContext) -> Result<()> {
    mount_root_volume(ctx)?;
    mount_var_volume(ctx)?;

    Ok(())
}

pub(crate) fn mount_root_volume(ctx: &mut InitContext) -> Result<()> {
    let root_dev = ctx.build_env().build_root_dev().ok_or(config_err(
        "expected build_root_dev (based on build_root_uuid) to be defined",
        None,
    ))?;

    let root_mnt = ctx.cfg().litos_initrd_root_mnt();

    mount_volume(&root_dev, &root_mnt, true)
}

pub(crate) fn mount_var_volume(ctx: &mut InitContext) -> Result<()> {
    let var_dev = ctx.build_env().build_var_dev().ok_or(config_err(
        "expected build_var_dev (based on build_var_uuid) to be defined",
        None,
    ))?;

    let var_mnt = ctx.cfg().litos_initrd_var_mnt();

    mount_volume(&var_dev, &var_mnt, false)
}

fn mount_volume(dev_path: &PathBuf, mnt_path: &Path, read_only: bool) -> Result<()> {
    let fs_type = "ext4";

    if !mnt_path.exists() {
        fs::create_dir_all(mnt_path)
            .map_err(|e| io_err(e, Some(format!("failed to make dir: {:?}", &mnt_path))))?;
    }

    let options = if read_only {
        // Add 'noload' to ENSURE no modifications.
        Some("ro,noload")
    } else {
        None
    };

    info!(
        "Mounting {:?} on {:?} (type: {}, read-only: {})",
        &dev_path, &mnt_path, &fs_type, read_only
    );
    mount(fs_type, dev_path, mnt_path, options)?;

    Ok(())
}

pub(crate) fn root_volume_dev_and_label(ctx: &InitContext) -> Result<(PathBuf, String)> {
    let root_dev_path = ctx
        .build_env()
        .build_luks_root_dev()
        .expect_or_err("expected build_luks_root_dev to have already been verified")?;
    let root_dev_label = format!("{}:{:?}", "root", root_dev_path.as_path());

    Ok((root_dev_path, root_dev_label))
}

pub(crate) fn var_volume_dev_and_label(ctx: &InitContext) -> Result<(PathBuf, String)> {
    let var_dev_path = ctx
        .build_env()
        .build_luks_var_dev()
        .expect_or_err("expected build_luks_var_dev to have already been verified")?;
    let var_dev_label = format!("{}:{:?}", "var", var_dev_path.as_path());

    Ok((var_dev_path, var_dev_label))
}
