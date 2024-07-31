use std::fs;
use std::io::{BufReader, Cursor};
use std::path::PathBuf;

use lit_core::utils::asserts::string_options_match;
use log::info;

use lit_core::utils::ipfs::ipfs_cat_cached_content;
use lit_os_core::error::{config_err, io_err, ipfs_err, parser_err, validation_err, Result};
use lit_os_core::guest::env::release::load_guest_release_env;

use crate::init::context::InitContext;

// We will store it locally in /etc but need to install under /var/local (a symlink already exists)
pub(crate) const RELEASE_ENV_FILE_SRC_PATH: &str = "/etc/lit-os-release";
pub(crate) const RELEASE_ENV_FILE_DST_PATH: &str = "/var/local/etc/lit-os-release";

pub(crate) async fn install_release_env(ctx: &mut InitContext) -> Result<()> {
    // Download
    let release_env_cid = ctx.cmdline_env().release_env.clone() // Clone to avoid borrow error.
        .ok_or_else(|| config_err("expected litos.release_env to be defined in the cmdline".to_string(), None))?;

    let (_, release_env_content) = ipfs_cat_cached_content(ctx.cfg(), &release_env_cid)
        .await
        .map_err(|e| ipfs_err(e, None))?;

    // Load
    let mut cur = Cursor::new(release_env_content.to_vec());
    let mut reader = BufReader::new(&mut cur);

    let release_env = load_guest_release_env(&mut reader)
        .map_err(|e| {
            parser_err(
                e,
                Some(format!(
                    "failed to parse 'litos.release_env' (CID: {release_env_cid}) loaded from IPFS"
                )),
            )
        })?
        .ok_or_else(|| {
            parser_err(
                "unexpected missing GuestReleaseEnv parsing 'litos.release_env'".to_string(),
                None,
            )
        })?;

    // Validate
    if !string_options_match(release_env.release_id.as_ref(), ctx.cmdline_env().release_id.as_ref())
    {
        return Err(validation_err(
            format!(
                "release_env has invalid release_id ({:?} vs {:?}))",
                release_env.release_id.as_ref(),
                ctx.cmdline_env().release_id.as_ref()
            ),
            None,
        ));
    }
    if !string_options_match(
        release_env.release_subnet_id.as_ref(),
        ctx.cmdline_env().subnet_id.as_ref(),
    ) {
        return Err(validation_err(
            format!(
                "release_env has invalid subnet_id ({:?} vs {:?}))",
                release_env.release_subnet_id.as_ref(),
                ctx.cmdline_env().subnet_id.as_ref()
            ),
            None,
        ));
    }

    // Write
    let src = PathBuf::from(RELEASE_ENV_FILE_SRC_PATH);
    let dst = PathBuf::from(RELEASE_ENV_FILE_DST_PATH);

    fs::write(&src, release_env_content).map_err(|e| io_err(e, None))?;

    // Add context and files
    ctx.push_synced(src, Some(dst));
    ctx.set_release_env(release_env);

    info!("Installed 'litos.release_env' (CID: {})", release_env_cid);

    Ok(())
}
