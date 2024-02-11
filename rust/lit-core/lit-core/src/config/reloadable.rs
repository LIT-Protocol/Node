use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use arc_swap::{ArcSwap, DefaultStrategy, Guard};

use crate::config::LitConfig;
use crate::error::Result;

pub(crate) type ConfigLoader = Arc<dyn Send + Sync + Fn() -> Result<LitConfig>>;

#[derive(Clone)]
pub struct ReloadableLitConfig(Arc<ArcSwap<LitConfig>>, ConfigLoader);

impl ReloadableLitConfig {
    pub fn new(reloader: impl Send + Sync + Fn() -> Result<LitConfig> + 'static) -> Result<Self> {
        let first = Arc::new(reloader()?);
        Ok(Self(Arc::new(ArcSwap::from(first)), Arc::new(reloader)))
    }

    pub fn reload(&self) -> Result<Arc<LitConfig>> {
        let next = Arc::new(self.1()?);
        self.0.store(next.clone());

        Ok(next)
    }

    /// Use this when you are only accessing in the current method
    /// DO NOT use this when you are passing it another async function.
    pub fn load(&self) -> Guard<Arc<LitConfig>, DefaultStrategy> {
        self.0.load()
    }

    /// Generally you'll want to use this at the top of your method and clone
    /// it to pass it around.
    pub fn load_full(&self) -> Arc<LitConfig> {
        self.0.load_full()
    }
}

impl Debug for ReloadableLitConfig {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        self.0.load().fmt(fmt)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::envs::LitEnv;
    use crate::config::{LitConfigBuilder, ReloadableLitConfig};
    use std::path::PathBuf;

    const RESOURCES_TEST_DIR: &str = "resources/test/config";

    #[test]
    fn reloadable_reload_test() {
        let reloader = ReloadableLitConfig::new(|| {
            let system_path = get_test_path("default");
            let system_path_str = system_path.to_str().unwrap();

            let cfg = LitConfigBuilder::new_with_paths(
                None,
                Some("/tmp/fake/nope".to_string()),
                system_path_str,
                "/tmp/fake/nope",
            )
            .build()
            .expect("failed to load config");

            Ok(cfg)
        })
        .expect("failed to construct ReloadableLitConfig");

        let cfg = reloader.load();

        assert_eq!(cfg.env().clone(), LitEnv::Dev);
        assert_eq!(cfg.is_dev(), true);
        assert_eq!(cfg.config().get_string("simple.dummy").unwrap(), "default".to_string());
        assert_eq!(cfg.config().get_bool("simple.other").unwrap(), true);

        let cfg = reloader.reload().unwrap();

        assert_eq!(cfg.env().clone(), LitEnv::Dev);
        assert_eq!(cfg.is_dev(), true);
        assert_eq!(cfg.config().get_string("simple.dummy").unwrap(), "default".to_string());
        assert_eq!(cfg.config().get_bool("simple.other").unwrap(), true);
    }

    // Util
    fn get_test_path(path: &str) -> PathBuf {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push(RESOURCES_TEST_DIR);
        test_path.push(path);
        test_path
    }
}
