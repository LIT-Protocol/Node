use lit_os_core::error::Result;

use crate::init::context::InitContext;
use crate::init::stage::setup::cloud_init::load_cloud_init_context;
use crate::init::stage::Outcome;

use crate::init::stage::setup::oneshot::load_oneshot_context;
use crate::init::stage::setup::release_env::install_release_env;

pub(crate) mod cloud_init;
pub(crate) mod common;
pub(crate) mod oneshot;
pub(crate) mod release_env;

pub(crate) async fn run(ctx: &mut InitContext) -> Result<Outcome> {
    // Load the cloud-init context
    let _cloud_init_ctx = load_cloud_init_context(ctx)?;

    // Load one-shot context
    let _oneshot_ctx = load_oneshot_context(ctx).await?;

    if ctx.is_release() {
        // Download and verify release env
        install_release_env(ctx).await?;
    }

    Ok(Outcome::Continue)
}
