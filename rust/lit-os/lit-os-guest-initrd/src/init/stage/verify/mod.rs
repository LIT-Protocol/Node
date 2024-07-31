use std::path::{Path, PathBuf};

use lit_blockchain::resolver::rpc::config::RPC_RESOLVER_CFG_SYSTEM;
use log::info;

use lit_core::utils::binary::bytes_to_hex;
use lit_core::utils::hash::sha512_file;
use lit_core::utils::option::bool_option_to_bool;
use lit_os_core::config::LitOsGuestConfig;
use lit_os_core::error::{io_err, validation_err, Result};

use crate::init::context::InitContext;
use crate::init::stage::Outcome;

const VERIFY_PATHS_ROOT: [&str; 4] = [
    "/etc/lit/config.toml", "/etc/lit-os-build", "/etc/ssl/certs/build.pem",
    "/usr/share/ca-certificates/mozilla",
];
const VERIFY_PATHS_VAR: [&str; 2] =
    ["/var/cache/sev-snp/certs/Milan/cert_chain.pem", RPC_RESOLVER_CFG_SYSTEM];

pub(crate) async fn run(ctx: &mut InitContext) -> Result<Outcome> {
    // Verify the static files written to both initrd as well as the disks.
    verify_files(ctx)?;

    // Ok
    info!("Final verification: OK");

    Ok(Outcome::Continue)
}

fn verify_files(ctx: &InitContext) -> Result<()> {
    let root_ro = bool_option_to_bool(ctx.build_env().build_opt_ro.as_ref());
    if root_ro || ctx.is_first_boot() {
        verify_mnt_files(
            ctx,
            &ctx.cfg().litos_initrd_root_mnt(),
            &VERIFY_PATHS_ROOT,
            &"root".to_string(),
        )?;
    }
    if ctx.is_first_boot() {
        verify_mnt_files(
            ctx,
            &ctx.cfg().litos_initrd_root_mnt(),
            &VERIFY_PATHS_VAR,
            &"var".to_string(),
        )?;
    }

    Ok(())
}

fn verify_mnt_files(
    _ctx: &InitContext, mnt: &Path, paths: &[&'static str], label: &String,
) -> Result<()> {
    for path in paths {
        let sys_path = PathBuf::from(path);

        let path = if let Some(p) = path.strip_prefix('/') { p } else { *path };
        let mut mnt_path = mnt.to_path_buf();
        mnt_path.push(path);

        if sys_path.exists() {
            if sys_path.is_file() {
                verify_mnt_file(&mnt_path, &sys_path, label)?;
            } else if sys_path.is_dir() {
                let files = sys_path.read_dir().map_err(|e| {
                    io_err(e, Some(format!("failed to read sys path: {:?}", sys_path.as_path())))
                })?;

                for file in files {
                    let file = file.map_err(|e| {
                        io_err(
                            e,
                            Some(format!("failed to read sys path: {:?}", sys_path.as_path())),
                        )
                    })?;
                    let sys_file = file.path();
                    let mut mnt_file = mnt_path.clone();
                    mnt_file.push(file.path().file_name().unwrap());

                    verify_mnt_file(&mnt_file, &sys_file, label)?;
                }
            }
        } else {
            return Err(validation_err(format!("missing required system path: {path}"), None));
        }
    }

    Ok(())
}

fn verify_mnt_file(mnt_path: &Path, sys_path: &Path, label: &String) -> Result<()> {
    if mnt_path.eq(sys_path) {
        return Err(validation_err(
            format!(
                "passing identical paths forbidden '{}:' (mnt '{}' vs sys '{}')",
                label,
                mnt_path.to_str().unwrap(),
                sys_path.to_str().unwrap()
            ),
            None,
        ));
    }
    let mnt_hash = sha512_file(mnt_path)
        .map_err(|e| io_err(e, Some(format!("failed to hash file: {mnt_path:?}"))))?
        .to_vec();
    let sys_hash = sha512_file(sys_path)
        .map_err(|e| io_err(e, Some(format!("failed to hash file: {sys_path:?}"))))?
        .to_vec();

    if !mnt_hash.eq(&sys_hash) {
        return Err(validation_err(
            format!(
                "file validation failed for '{}:{}' ('{}' vs '{}')",
                label,
                sys_path.to_str().unwrap(),
                bytes_to_hex(&sys_hash),
                bytes_to_hex(&mnt_hash)
            ),
            None,
        ));
    }

    Ok(())
}
