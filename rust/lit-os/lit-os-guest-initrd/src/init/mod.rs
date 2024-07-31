use log::{as_error, error, info};
use nix::unistd::Uid;
use std::process::exit;

use crate::init::context::InitContext;

pub mod context;
pub(crate) mod stage;

pub async fn init() {
    if !Uid::effective().is_root() {
        error!("must be root to run this command");
        exit(255);
    }

    // Init context
    let mut ctx = match InitContext::new(false) {
        Err(e) => {
            error!(error = as_error!(e); "InitContext->new() failed");
            exit(255);
        }
        Ok(ctx) => ctx,
    };

    // Run stages
    if !stage::run_all(&mut ctx).await {
        exit(255);
    }

    info!("Init complete: Booting");
}
