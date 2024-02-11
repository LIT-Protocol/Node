use std::time::Duration;
use std::{env, thread};

use futures::future::LocalBoxFuture;
use log::{as_error, error, info, warn};

use lit_os_core::error::Result;

use crate::init::context::InitContext;
use crate::init::stage::unlock::deactivate_luks_volumes;
use crate::logging::{ENV_LOG_INIT_STAGE, ENV_LOG_INIT_SUB_STAGE};
use crate::utils::busybox::busybox_poweroff;

type StageHandler = fn(ctx: &mut InitContext) -> LocalBoxFuture<Result<Outcome>>;

pub(crate) const STAGES: [(&str, StageHandler); 8] = [
    ("PREPARE", |ctx: &mut InitContext| Box::pin(prepare::run(ctx))),
    ("SETUP", |ctx: &mut InitContext| Box::pin(setup::run(ctx))),
    ("ATTEST", |ctx: &mut InitContext| Box::pin(attest::run(ctx))),
    ("ONESHOT", |ctx: &mut InitContext| Box::pin(oneshot::run(ctx))),
    ("INIT", |ctx: &mut InitContext| Box::pin(init::run(ctx))),
    ("UNLOCK", |ctx: &mut InitContext| Box::pin(unlock::run(ctx))),
    ("VERIFY", |ctx: &mut InitContext| Box::pin(verify::run(ctx))),
    ("SYNC", |ctx: &mut InitContext| Box::pin(sync::run(ctx))),
];

pub(crate) mod attest;
pub(crate) mod cleanup;
pub(crate) mod debug;
pub(crate) mod init;
pub(crate) mod oneshot;
pub(crate) mod prepare;
pub(crate) mod setup;
pub(crate) mod sync;
pub(crate) mod unlock;
pub(crate) mod verify;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Outcome {
    Continue,
    Break,
    Diagnose,
    PowerOff,
    Halt,
}

pub(crate) async fn run_all(ctx: &mut InitContext) -> bool {
    // Run stages
    let mut outcome = Outcome::Continue;
    for (stage, fun) in STAGES {
        info!("In guest initrd - Running stage '{}'", stage);
        outcome = run(ctx, stage, fun).await;
        if outcome != Outcome::Continue {
            break;
        }
    }

    // Diagnose if requested
    if outcome == Outcome::Diagnose {
        let _ = run(ctx, "DEBUG", |ctx: &mut InitContext| Box::pin(debug::run(ctx))).await;
    }

    // Always run cleanup
    let _ = run(ctx, "CLEANUP", |ctx: &mut InitContext| Box::pin(cleanup::run(ctx))).await;

    // Ensure the system boot is disabled on failure
    if (outcome != Outcome::Continue && outcome != Outcome::PowerOff && outcome != Outcome::Halt)
        && ctx.is_activated()
    {
        securely_handle_failure(ctx);
        clear_env_log_prefixes();
    }

    match outcome {
        Outcome::PowerOff => {
            // Poweroff if requested
            if let Err(e) = busybox_poweroff() {
                error!(error = as_error!(e); "Failed to poweroff");
            };
        }
        Outcome::Halt | Outcome::Diagnose => {
            warn!("Halting boot");
            thread::sleep(Duration::from_secs(u64::MAX));
        }
        _ => {}
    }

    outcome == Outcome::Continue
}

async fn run(ctx: &mut InitContext, stage: &str, fun: StageHandler) -> Outcome {
    env::set_var(ENV_LOG_INIT_STAGE, stage);

    let res = fun(ctx).await;
    let outcome: Outcome;
    if let Err(e) = res {
        error!(error = as_error!(e); "Stage '{}' failed", stage);

        outcome = Outcome::Break;
    } else {
        outcome = res.unwrap();
    }

    clear_env_log_prefixes();
    outcome
}

fn clear_env_log_prefixes() {
    env::remove_var(ENV_LOG_INIT_STAGE);
    env::remove_var(ENV_LOG_INIT_SUB_STAGE);
}

fn securely_handle_failure(ctx: &mut InitContext) {
    env::set_var(ENV_LOG_INIT_STAGE, "DESTROY");

    info!("Tearing down system due to failure");
    if let Err(e) = deactivate_luks_volumes(ctx) {
        error!(error = as_error!(e); "Failed to deactivate_luks_volumes, halting boot");
        thread::sleep(Duration::from_secs(u64::MAX));
    }
}
