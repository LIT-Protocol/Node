extern crate dotenv;

use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::LitConfig;
use lit_node::config::load_cfg;

use std::sync::Arc;

pub fn load_config() -> (Arc<LitConfig>, Arc<ContractResolver>) {
    // Load config
    let cfg = load_cfg().expect("failed to load LitConfig");
    let loaded_config = cfg.load_full();

    let resolver = Arc::new(
        ContractResolver::try_from(cfg.load().as_ref()).expect("failed to load ContractResolver"),
    );

    (loaded_config, resolver)
}
