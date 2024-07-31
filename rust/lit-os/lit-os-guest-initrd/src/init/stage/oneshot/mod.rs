use std::{env, fs};

use log::{as_error, error, info};

use lit_os_core::config::LitOsGuestConfig;
use lit_os_core::error::{config_err, io_err, validation_err, Result};
use lit_os_core::guest::cloud_init::CLOUD_INIT_FILE_INIT_PW;
use lit_os_core::guest::oneshot::context::{OneShotContext, REQUIRE_SYNC};

use crate::init::context::{
    InitContext, CTX_KEY_PASSPHRASE_INIT, CTX_KEY_ROOT_PASSPHRASE_BOOT, CTX_KEY_VAR_PASSPHRASE_BOOT,
};
use crate::init::stage::init::{prepare_boot_passphrases, volume_paths_and_labels};
use crate::init::stage::oneshot::action::Outcome as ActionOutcome;
use crate::init::stage::oneshot::action::{actions_map, ACTIONS};
use crate::init::stage::unlock::{
    activate_luks_root_volume, activate_luks_var_volume, mount_root_volume, mount_var_volume,
};
use crate::init::stage::{cleanup, sync, Outcome};
use crate::logging::ENV_LOG_INIT_SUB_STAGE;
#[cfg(not(target_os = "macos"))]
use crate::utils::libcryptsetup::{cryptsetup_has_slot_active, cryptsetup_init_dev};

pub(crate) mod action;
pub(crate) mod prepare;
pub(crate) mod setup;

pub(crate) async fn run(ctx: &mut InitContext) -> Result<Outcome> {
    let oneshot_ctx = ctx.get_oneshot_ctx();
    if oneshot_ctx.is_none() {
        return Ok(Outcome::Continue);
    }

    // Clone to avoid mutable borrow error on ctx.
    let oneshot_ctx = oneshot_ctx.unwrap().clone();
    let config = oneshot_ctx.config();
    let available = actions_map().await?;

    // Dry run to verify
    for (key, _entry) in config.actions().iter() {
        if !available.contains_key(key) {
            return Err(config_err(format!("one shot action '{key}' is invalid"), None));
        }
    }
    info!("Oneshot dry run verified");

    // Before
    match before_execute(ctx, &oneshot_ctx).await? {
        Outcome::Break => unreachable!("invalid outcome for before_execute (Break)"),
        Outcome::PowerOff => return Ok(Outcome::PowerOff),
        _ => {}
    }
    info!("Oneshot before_execute executed");

    // Go for it - use ACTIONS to iterate in correct order.
    let actions = ACTIONS.read().await.clone();
    for action in actions.iter() {
        if let Some(entry) = config.actions().get(action.action()) {
            env::set_var(ENV_LOG_INIT_SUB_STAGE, action.action());
            info!("Running oneshot action '{:?}'", entry);
            match action.execute(ctx, &oneshot_ctx, entry).await {
                Ok(ActionOutcome::Continue) => continue,
                Ok(ActionOutcome::Break) => break,
                Err(e) => {
                    error!(error = as_error!(e); "one shot action '{}' failed", action.action());
                    env::remove_var(ENV_LOG_INIT_SUB_STAGE);
                    return Ok(Outcome::Diagnose);
                }
            }
        }
    }

    // Clear sub stage.
    env::remove_var(ENV_LOG_INIT_SUB_STAGE);

    // After
    match after_execute(ctx, &oneshot_ctx).await? {
        Outcome::Break => unreachable!("invalid outcome for after_execute (Break)"),
        Outcome::PowerOff => {} // NB: Ensure nothing happens below.
        _ => {}
    }

    Ok(Outcome::PowerOff)
}

pub(crate) async fn before_execute(
    ctx: &mut InitContext, oneshot_ctx: &OneShotContext,
) -> Result<Outcome> {
    // Process requires
    if oneshot_ctx.has_require(&REQUIRE_SYNC.into()) {
        info!("Oneshot requires '{}': Mounting volumes", REQUIRE_SYNC);

        // Prepare for mount
        prepare_for_mount(ctx).await?;

        // Mount
        mount_root(ctx, oneshot_ctx)?; // root is required for var
        mount_var(ctx, oneshot_ctx)?;
    }

    Ok(Outcome::Continue)
}

pub(crate) async fn after_execute(
    ctx: &mut InitContext, oneshot_ctx: &OneShotContext,
) -> Result<Outcome> {
    // Process requires
    if oneshot_ctx.has_require(&REQUIRE_SYNC.into()) {
        info!("Oneshot requires '{}': Running sync", REQUIRE_SYNC);

        sync::run(ctx).await?;
    }

    // Write status
    let mut status_path = ctx.cfg().litos_oneshot_mnt();
    status_path.push("status");

    fs::write(status_path.as_path(), "1").map_err(|e| io_err(e, None))?;

    // Clean up
    cleanup::run(ctx).await?;

    Ok(Outcome::Continue)
}

// Utils
async fn prepare_for_mount(ctx: &mut InitContext) -> Result<()> {
    set_is_first_boot(ctx)?;

    if ctx.is_first_boot() {
        if let Some(passphrase) = ctx.get_bin(CTX_KEY_PASSPHRASE_INIT).cloned() {
            ctx.insert(CTX_KEY_ROOT_PASSPHRASE_BOOT, Box::new(passphrase.clone()));
            ctx.insert(CTX_KEY_VAR_PASSPHRASE_BOOT, Box::new(passphrase));
        } else {
            return Err(config_err(format!("this oneshot requires mounting /var and no '{CLOUD_INIT_FILE_INIT_PW}' file was found in the cloud-init"), None));
        }
    } else {
        prepare_boot_passphrases(ctx).await?;
    }

    Ok(())
}

#[allow(unused_variables, unreachable_code)]
fn set_is_first_boot(ctx: &mut InitContext) -> Result<()> {
    #[cfg(target_os = "macos")]
    unimplemented!("cryptsetup not supported on MacOs");

    // Load LUKS devices
    let (root_dev_path, root_dev_label, _var_dev_path, _var_dev_label) =
        volume_paths_and_labels(ctx)?;

    #[cfg(not(target_os = "macos"))]
    let mut root_dev = cryptsetup_init_dev(root_dev_path.as_path(), root_dev_label.as_str())?;

    #[cfg(not(target_os = "macos"))]
    if cryptsetup_has_slot_active(&mut root_dev, root_dev_label.as_str(), 0)? {
        // First boot (or not init in other words).
        ctx.set_is_first_boot(true);
    }

    Ok(())
}

#[allow(dead_code)]
fn mount_root(ctx: &mut InitContext, _oneshot_ctx: &OneShotContext) -> Result<()> {
    activate_luks_root_volume(ctx)?;
    mount_root_volume(ctx)?;

    Ok(())
}

#[allow(dead_code)]
fn mount_var(ctx: &mut InitContext, _oneshot_ctx: &OneShotContext) -> Result<()> {
    if !ctx.is_first_boot() {
        // TODO: We could support it, but will we need to?
        return Err(validation_err("this oneshot requires mounting /var however this is only supported before init has occurred", None));
    }

    activate_luks_var_volume(ctx)?;
    mount_var_volume(ctx)?;

    Ok(())
}
