pub(crate) mod bootstrap;

use crate::init::context::InitContext;
use futures::future::LocalBoxFuture;
use lit_os_core::error::{config_err, Result};
use lit_os_core::guest::oneshot::config::ActionEntry;
use lit_os_core::guest::oneshot::context::OneShotContext;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub(crate) static ACTIONS: Lazy<RwLock<Vec<OneShotAction>>> = Lazy::new(|| RwLock::new(Vec::new()));

pub(crate) type ExecutorFn = for<'a> fn(
    ctx: &'a mut InitContext,
    oneshot_ctx: &'a OneShotContext,
    entry: &'a ActionEntry,
) -> LocalBoxFuture<'a, Result<Outcome>>;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) enum Outcome {
    Continue,
    Break,
}

#[derive(Clone)]
pub(crate) struct OneShotAction {
    action: String,
    executor: ExecutorFn,
    requires: Vec<String>,
}

impl OneShotAction {
    #[allow(dead_code)]
    pub(crate) fn new(action: String, executor: ExecutorFn) -> Self {
        Self { action, executor, requires: Vec::new() }
    }

    #[allow(dead_code)]
    pub fn action(&self) -> &String {
        &self.action
    }

    #[allow(dead_code)]
    pub(crate) fn with_require(mut self, require: String) -> Self {
        self.add_require(require);
        self
    }

    #[allow(dead_code)]
    pub fn requires(&self) -> &Vec<String> {
        &self.requires
    }

    #[allow(dead_code)]
    pub fn has_require(&self, require: &String) -> bool {
        self.requires.contains(require)
    }

    #[allow(dead_code)]
    pub fn add_require(&mut self, require: String) -> &mut Self {
        if !self.has_require(&require) {
            self.requires.push(require);
        }
        self
    }

    #[allow(dead_code)]
    pub async fn execute(
        &self, ctx: &mut InitContext, oneshot_ctx: &OneShotContext, entry: &ActionEntry,
    ) -> Result<Outcome> {
        (self.executor)(ctx, oneshot_ctx, entry).await
    }
}

pub(crate) async fn actions_map() -> Result<HashMap<String, OneShotAction>> {
    let actions = ACTIONS.read().await;

    let mut map: HashMap<String, OneShotAction> = HashMap::new();
    for action in actions.iter() {
        if map.contains_key(&action.action) {
            return Err(config_err(
                format!("more than one oneshot action found with the key: {}", &action.action),
                None,
            ));
        }

        map.insert(action.action.clone(), action.clone());
    }

    Ok(map)
}
