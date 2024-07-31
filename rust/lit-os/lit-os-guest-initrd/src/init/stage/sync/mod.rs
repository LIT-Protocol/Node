use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use file_mode::ModePath;
use log::info;
use posix_acl::{PosixACL, Qualifier};
use users::{get_group_by_name, get_user_by_name};
use walkdir::WalkDir;

use lit_os_core::config::LitOsGuestConfig;
use lit_os_core::error::{config_err, generic_err, io_err, unexpected_err, Result};

use crate::init::context::InitContext;
use crate::init::stage::Outcome;

const LITOS_INIT_PREPARE_STATE_FILE: &str = "/var/local/litos-init-prepare.install";

pub struct SyncItem {
    pub src: PathBuf,
    pub dst: Option<PathBuf>,
    pub acl: Vec<SyncItemAcl>,
}

impl SyncItem {
    pub fn new(src: PathBuf, dst: Option<PathBuf>, acl: Vec<SyncItemAcl>) -> Self {
        Self { src, dst, acl }
    }
}

// So dumb, but posix-acl Qualifier doesn't impl clone()??
pub enum AclType {
    User,
    Group,
}

pub struct SyncItemAcl {
    pub acl_type: AclType,
    pub acl_name: String,
    pub perms: u32,
}

impl SyncItemAcl {
    pub fn new<N>(acl_type: AclType, acl_name: N, perms: u32) -> Self
    where
        N: Into<String>,
    {
        Self { acl_type, acl_name: acl_name.into(), perms }
    }
}

pub(crate) async fn run(ctx: &mut InitContext) -> Result<Outcome> {
    // Load config
    let root_mnt = ctx.cfg().litos_initrd_root_mnt();
    if !root_mnt.exists() {
        return Err(io_err(format!("root not mounted (expected {root_mnt:?} to exist)"), None));
    }

    let var_mnt = ctx.cfg().litos_initrd_var_mnt();
    if !var_mnt.exists() {
        return Err(io_err(format!("var not mounted (expected {var_mnt:?} to exist)"), None));
    }

    // Purge paths
    purge_paths(ctx, &var_mnt)?;

    // Sync paths
    sync_paths(ctx, &var_mnt)?;

    if !ctx.has_oneshot_actions() {
        // Run prepare (on first boot)
        run_prepare(ctx, &root_mnt, &var_mnt)?;
    }

    Ok(Outcome::Continue)
}

fn purge_paths(ctx: &mut InitContext, var_mnt: &Path) -> Result<()> {
    for path in ctx.purged() {
        if !path.starts_with("/var") {
            return Err(generic_err(
                format!("unable to purge path: {path:?} (does not fall under /var)"),
                None,
            ));
        }

        let mut full_path = var_mnt.to_path_buf();
        full_path.push(path.strip_prefix("/var").map_err(|e| {
            io_err(e, Some(format!("unable to remove /var prefix from: {path:?}")))
        })?);

        if full_path.exists() {
            if full_path.is_file() {
                fs::remove_file(full_path.as_path()).map_err(|e| {
                    io_err(e, Some(format!("failed to remove file: {:?}", &full_path)))
                })?;

                info!("Purged file {:?}", path);
            } else if full_path.is_dir() {
                fs::remove_dir_all(full_path.as_path()).map_err(|e| {
                    io_err(e, Some(format!("failed to remove dir: {:?}", &full_path)))
                })?;

                info!("Purged directory {:?}", path);
            }
        }
    }

    Ok(())
}

fn sync_paths(ctx: &mut InitContext, var_mnt: &Path) -> Result<()> {
    let mut synced: HashMap<String, bool> = HashMap::new();

    for item in ctx.synced() {
        let src_path = &item.src;
        let maybe_dest = &item.dst;

        let full_dest = match maybe_dest {
            Some(dest) => dest,
            None => src_path,
        };

        if !full_dest.starts_with("/var") {
            return Err(generic_err(
                format!("unable to sync path: {full_dest:?} (does not fall under /var)"),
                None,
            ));
        }

        // Construct dest path (relative to var mount)
        let mut dest_path = var_mnt.to_path_buf();
        dest_path.push(full_dest.strip_prefix("/var").map_err(|e| {
            io_err(e, Some(format!("unable to remove /var prefix from: {full_dest:?}")))
        })?);

        // Make parent dir
        let dest_parent = dest_path.parent().ok_or_else(|| {
            io_err(format!("failed to get parent dir of: {:?}", &dest_path), None)
        })?;

        if !dest_parent.exists() {
            fs::create_dir_all(dest_parent)
                .map_err(|e| io_err(e, Some(format!("failed to create: {dest_parent:?}"))))?;

            apply_acl(dest_parent, &item.acl)?;
        }

        if src_path.exists() {
            if src_path.is_file() {
                let dest_path_str = dest_path.to_str().unwrap().to_string();
                if let std::collections::hash_map::Entry::Vacant(e) = synced.entry(dest_path_str) {
                    // Copy file
                    fs::copy(src_path, &dest_path).map_err(|e| {
                        io_err(
                            e,
                            Some(format!("failed to copy: {:?} to {:?}", src_path, &dest_path)),
                        )
                    })?;

                    apply_acl(&dest_path, &item.acl)?;

                    info!("Synced file {:?}", full_dest);
                    e.insert(true);
                }
            } else if src_path.is_dir() {
                for entry in WalkDir::new(src_path.as_path()) {
                    let entry = entry.map_err(|e| io_err(e, None))?;
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        let relative_path =
                            entry_path.strip_prefix(src_path).map_err(|e| io_err(e, None))?;
                        let mut cur_dest = dest_path.to_path_buf();
                        cur_dest.push(relative_path);

                        let cur_dest_str = cur_dest.to_str().unwrap().to_string();
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            synced.entry(cur_dest_str)
                        {
                            // Make parent dir
                            let cur_dest_parent = cur_dest.parent().ok_or_else(|| {
                                io_err(
                                    format!("failed to get parent dir of: {:?}", &cur_dest),
                                    None,
                                )
                            })?;

                            fs::create_dir_all(cur_dest_parent).map_err(|e| {
                                io_err(e, Some(format!("failed to create: {cur_dest_parent:?}")))
                            })?;

                            apply_acl(cur_dest_parent, &item.acl)?;

                            // Copy file
                            fs::copy(entry_path, &cur_dest).map_err(|e| {
                                io_err(
                                    e,
                                    Some(format!(
                                        "failed to copy: {:?} to {:?}",
                                        entry_path, &cur_dest
                                    )),
                                )
                            })?;

                            apply_acl(&cur_dest, &item.acl)?;

                            info!("Synced file {:?}", entry_path);
                            e.insert(true);
                        }
                    }
                }
            }
        } else {
            return Err(generic_err(
                format!("unable to sync path: {full_dest:?} (src {src_path:?} does not exist)"),
                None,
            ));
        }
    }

    Ok(())
}

fn apply_acl(path: &Path, acls: &Vec<SyncItemAcl>) -> Result<()> {
    if acls.is_empty() {
        return Ok(());
    }

    // Assume protected files if we're applying further ACLs.
    if path.is_file() {
        path.set_mode(600)
            .map_err(|e| io_err(e, Some(format!("failed to chmod file: {path:?}"))))?;
    } else {
        path.set_mode(700)
            .map_err(|e| io_err(e, Some(format!("failed to chmod dir: {path:?}"))))?;
    }

    let mut acl = PosixACL::read_acl(path).map_err(|e| io_err(e, None))?;

    for cur in acls {
        let qualifier = match cur.acl_type {
            AclType::User => Qualifier::User(
                // This won't work at the moment as /etc/passwd isn't copied to initrd.
                // - Use groups.
                get_user_by_name(&cur.acl_name)
                    .ok_or_else(|| {
                        unexpected_err(
                            format!("could not get user id for '{}'", &cur.acl_name),
                            None,
                        )
                    })?
                    .uid(),
            ),
            AclType::Group => Qualifier::Group(
                get_group_by_name(&cur.acl_name)
                    .ok_or_else(|| {
                        unexpected_err(
                            format!("could not get group id for '{}'", &cur.acl_name),
                            None,
                        )
                    })?
                    .gid(),
            ),
        };

        acl.set(qualifier, cur.perms);
    }

    acl.write_acl(path).map_err(|e| io_err(e, None))?;

    Ok(())
}

fn run_prepare(ctx: &mut InitContext, root_mnt: &Path, var_mnt: &Path) -> Result<()> {
    let mut state_path = var_mnt.to_path_buf();
    state_path.push(LITOS_INIT_PREPARE_STATE_FILE.strip_prefix("/var").ok_or_else(|| {
        config_err(
            format!("failed to strip /var prefix from {LITOS_INIT_PREPARE_STATE_FILE}"),
            None,
        )
    })?);

    if state_path.exists() {
        // No-op
        return Ok(());
    }

    info!("First boot, preparing guest");

    let cloud_init_ctx = ctx
        .get_cloud_init_ctx()
        .ok_or_else(|| config_err("expected to have cloud-init ctx", None))?;

    // Extract FQDN
    let fqdn = cloud_init_ctx.user_data().fqdn();
    let hostname = fqdn.split('.').next().ok_or_else(|| {
        config_err(
            format!("unable to split fqdn ({fqdn}) in cloud-init user-data to get hostname"),
            None,
        )
    })?;

    let mut cmd = Command::new("/opt/lit/os/init/lit-os-init-prepare.sh");
    cmd.arg("--mnt-path")
        .arg(root_mnt.to_str().unwrap())
        .arg("--hostname")
        .arg(hostname)
        .arg("--fqdn")
        .arg(fqdn);

    if let Some(allow_ssh) = ctx.build_env().build_opt_ssh.as_ref() {
        if *allow_ssh {
            cmd.arg("--init-ssh");
        }
    }

    let out = cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| io_err(e, Some("failed to run: lit-os-init-prepare.sh".into())))?;

    if !out.status.success() {
        return Err(io_err("failed to run: lit-os-init-prepare.sh", None));
    }

    Ok(())
}
