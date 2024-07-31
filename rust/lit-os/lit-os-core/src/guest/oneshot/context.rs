use std::path::PathBuf;

use crate::guest::oneshot::config::OneShotConfig;

pub const REQUIRE_SYNC: &str = "sync";

#[derive(Debug, Clone)]
pub struct OneShotContext {
    path: PathBuf,
    config: OneShotConfig,
    requires: Vec<String>,
}

impl OneShotContext {
    pub fn new(path: PathBuf, config: OneShotConfig) -> Self {
        Self { path, config, requires: Vec::new() }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn config(&self) -> &OneShotConfig {
        &self.config
    }

    pub fn requires(&self) -> &Vec<String> {
        &self.requires
    }

    pub fn has_require(&self, require: &String) -> bool {
        self.requires.contains(require)
    }

    pub fn add_require(&mut self, require: String) -> &mut Self {
        if !self.has_require(&require) {
            self.requires.push(require);
        }
        self
    }
}
