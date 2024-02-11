use lit_os_core::error::Result;
use lit_os_core::guest::oneshot::context::OneShotContext;

use crate::init::context::InitContext;
use crate::init::stage::oneshot::action::actions_map;

pub(crate) async fn apply_oneshot_requires(
    _ctx: &mut InitContext, oneshot_ctx: &mut OneShotContext,
) -> Result<()> {
    let actions_map = actions_map().await?;

    let mut requires: Vec<String> = Vec::new();
    for action in oneshot_ctx.config().actions().keys() {
        if let Some(action) = actions_map.get(action) {
            for require in action.requires() {
                requires.push(require.clone());
            }
        }
    }

    for require in requires {
        oneshot_ctx.add_require(require);
    }

    Ok(())
}
