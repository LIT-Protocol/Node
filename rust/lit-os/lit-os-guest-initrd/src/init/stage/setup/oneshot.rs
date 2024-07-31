use std::fs;
use std::path::PathBuf;

use log::info;

use lit_os_core::config::LitOsGuestConfig;
use lit_os_core::error::{io_err, Result};
use lit_os_core::guest::oneshot::config::OneShotConfig;
use lit_os_core::guest::oneshot::context::OneShotContext;
use lit_os_core::guest::oneshot::{ALLOWED_ONESHOT_FILES, ONESHOT_FILE_CONFIG};
use lit_os_core::utils::mount::mount;

use crate::init::context::{InitContext, CTX_KEY_ONESHOT_CTX};
use crate::init::stage::oneshot::setup::apply_oneshot_requires;
use crate::init::stage::setup::common::verify_allowed_in_mount;

pub(crate) fn mount_oneshot(ctx: &mut InitContext) -> Result<Option<PathBuf>> {
    let dev_path = ctx.cfg().litos_oneshot_dev();
    if !dev_path.exists() {
        return Ok(None);
    }

    let fs_type = "ext4";

    let mnt_path = ctx.cfg().litos_oneshot_mnt();
    if !mnt_path.exists() {
        fs::create_dir_all(&mnt_path)
            .map_err(|e| io_err(e, Some(format!("failed to make dir: {:?}", &mnt_path))))?;
    }

    info!("Mounting {:?} on {:?} (type: {}, read-only: {})", &dev_path, &mnt_path, &fs_type, false);
    mount(fs_type, &dev_path, &mnt_path, Some("rw"))?;

    Ok(Some(mnt_path))
}

pub(crate) async fn load_oneshot_context(ctx: &mut InitContext) -> Result<Option<OneShotContext>> {
    // Mount cloud-init
    let mnt = match mount_oneshot(ctx)? {
        Some(val) => val,
        _ => return Ok(None),
    };

    // Verify allowed files
    verify_allowed_in_mount(mnt.as_path(), ALLOWED_ONESHOT_FILES.as_slice(), "one shot")?;

    // Verify one shot config
    let mut config_path = mnt.clone();
    config_path.push(ONESHOT_FILE_CONFIG);

    if !config_path.exists() {
        return Err(io_err(format!("cloud-init file: {config_path:?} is missing"), None));
    }

    let config = OneShotConfig::try_from(config_path.as_path())?;
    config.verify()?;

    // Load context
    let mut oneshot_ctx = OneShotContext::new(mnt, config);

    // Apply requires
    apply_oneshot_requires(ctx, &mut oneshot_ctx).await?;

    ctx.insert(CTX_KEY_ONESHOT_CTX, Box::new(oneshot_ctx.clone()));

    Ok(Some(oneshot_ctx))
}
