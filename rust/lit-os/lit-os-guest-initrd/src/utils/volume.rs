use lit_os_core::error::{generic_err, io_err, unexpected_err, Result};
use nix::unistd::Uid;
use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, Stdio};

// The minimum size (in sectors) to consider for resize.
const MIN_SIZE_TO_RESIZE: u64 = 1024000; // 500Mb

/// Warning: This ONLY works with the LAST partition.
/// Assumptions:
///  - The backup LBA has been moved to the end of the disk prior to running this (i.e. sgdisk -e /dev/nbd1).
pub fn resize_partition_to_max<D, P>(dev: D, part_name: P, resize: bool) -> Result<bool>
where
    D: AsRef<Path>,
    P: AsRef<str>,
{
    let cfg = gpt::GptConfig::new().writable(true).initialized(true);
    let mut disk = cfg.open(dev.as_ref()).map_err(|e| {
        io_err(e, Some(format!("failed to open partition table for {:?}", dev.as_ref())))
    })?;

    let header = disk.primary_header().ok_or_else(|| {
        unexpected_err(
            format!("failed to load partition table for device {:?}", dev.as_ref()),
            None,
        )
    })?;

    let (part_id, part) = disk
        .partitions()
        .iter()
        .find(|p| p.1.name.eq(part_name.as_ref()))
        .map(|p| (*p.0, p.1.clone()))
        .ok_or_else(|| {
            unexpected_err(
                format!(
                    "failed to find partition '{}' on device {:?}",
                    part_name.as_ref(),
                    dev.as_ref()
                ),
                None,
            )
        })?;

    let cur_size = part.size().map_err(|e| unexpected_err(e, None))?;
    let max_size = header.last_usable - part.first_lba;

    if cur_size == max_size || ((max_size - cur_size) < MIN_SIZE_TO_RESIZE) {
        return Ok(false);
    }

    if resize {
        disk.remove_partition(Some(part_id), None).map_err(|e| {
            io_err(
                e,
                Some(format!("failed to remove partition {} on {:?}", &part_id, dev.as_ref())),
            )
        })?;

        disk.add_partition(
            &part.name,
            max_size * u64::from(*disk.logical_block_size()),
            part.part_type_guid.clone(),
            Some(part.part_guid),
            part.flags,
            None,
        )
        .map_err(|e| {
            io_err(e, Some(format!("failed to add partition {} on {:?}", &part_id, dev.as_ref())))
        })?;

        disk.write().map_err(|e| {
            io_err(e, Some(format!("failed to write partition table to {:?}", dev.as_ref())))
        })?;
    }

    Ok(true)
}

/// resize2fs /dev/sda1
pub fn resize2fs<D>(dev: D) -> Result<()>
where
    D: AsRef<OsStr>,
{
    if !Uid::effective().is_root() {
        return Err(generic_err("must be root to run resize2fs", None));
    }

    let mut cmd = Command::new("resize2fs")
        .arg(dev.as_ref())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .map_err(|e| io_err(e, None))?;

    let status = cmd.wait().map_err(|e| io_err(e, Some("failed to execute resize2fs".into())))?;

    if !status.success() {
        return Err(io_err("resize2fs failed to run", None));
    }

    Ok(())
}

/// e2fsck -p -f /dev/sda1
pub fn e2fsck<D>(dev: D) -> Result<()>
where
    D: AsRef<OsStr>,
{
    if !Uid::effective().is_root() {
        return Err(generic_err("must be root to run e2fsck", None));
    }

    let mut cmd = Command::new("e2fsck")
        .arg("-p")
        .arg("-f")
        .arg(dev.as_ref())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .map_err(|e| io_err(e, None))?;

    let status = cmd.wait().map_err(|e| io_err(e, Some("failed to execute e2fsck".into())))?;

    if !status.success() {
        return Err(io_err("e2fsck failed to run", None));
    }

    Ok(())
}

/// blockdev --rereadpt /dev/sda1
pub fn blockdev_rereadpt<D>(dev: D) -> Result<()>
where
    D: AsRef<OsStr>,
{
    if !Uid::effective().is_root() {
        return Err(generic_err("must be root to run blockdev", None));
    }

    let mut cmd = Command::new("blockdev")
        .arg("--rereadpt")
        .arg(dev.as_ref())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .map_err(|e| io_err(e, None))?;

    let status = cmd.wait().map_err(|e| io_err(e, Some("failed to execute blockdev".into())))?;

    if !status.success() {
        return Err(io_err("blockdev failed to run", None));
    }

    Ok(())
}
