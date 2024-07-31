#[allow(unused_imports)]
use lit_os_core::guest::oneshot::config::{ActionEntry, ACTION_TYPE_BOOTSTRAP};
#[allow(unused_imports)]
use lit_os_core::guest::oneshot::context::{OneShotContext, REQUIRE_SYNC};

use crate::init::context::InitContext;
#[allow(unused_imports)]
use crate::init::stage::oneshot::action::bootstrap;
#[allow(unused_imports)]
use crate::init::stage::oneshot::action::{OneShotAction, ACTIONS};

#[allow(unused_variables)]
pub(crate) async fn prepare_oneshot_actions(_ctx: &mut InitContext) {
    #[allow(unused_mut)]
    let mut actions = ACTIONS.write().await;

    #[cfg(feature = "type-prov")]
    actions.push(
        OneShotAction::new(
            ACTION_TYPE_BOOTSTRAP.into(),
            |ctx: &mut InitContext, oneshot_ctx: &OneShotContext, entry: &ActionEntry| {
                Box::pin(bootstrap::run(ctx, oneshot_ctx, entry))
            },
        )
        .with_require(REQUIRE_SYNC.into()),
    );
}
