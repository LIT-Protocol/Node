pub mod error;

use crate::error::Result;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_cli_os::config::LitCliOsConfig;
use lit_core::config::LitConfig;
use lit_core::config::LitConfigBuilder;
use lit_node_operator::get_instance_item;
use lit_node_operator::host_commands_listener::HostCommandsListener;
use nix::unistd::Uid;
use tracing::{debug, error};
use tracing_subscriber::{prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    match tracing_journald::layer() {
        Ok(layer) => {
            tracing_subscriber::registry().with(EnvFilter::from_default_env()).with(layer).init();
        }
        Err(e) => {
            eprintln!("Failed to initialize journald logging: {}", e);
            tracing_subscriber::fmt::init(); // Fallback to standard logging
        }
    }
    if !Uid::effective().is_root() {
        error!("lit-node-operator must be run as root, exiting");
        std::process::exit(1);
    }
    let cfg_builder = LitConfigBuilder::default();
    let cfg = LitConfig::from_builder(cfg_builder).expect("failed to load lit config");

    // get the only instance on this node
    let item = get_instance_item(&cfg);

    // instantiate listener
    let subnet = item.instance_env.subnet_id.clone().expect("node-type nodes must have a subnet");
    let env = item.build_env.env().expect("node-type nodes must have an env");
    debug!("Instantiating listener for subnet {:?} on env {:?}", subnet.clone(), env);
    let resolver = ContractResolver::new(subnet, env, None);
    let host_commands_contract = resolver
        .host_commands_contract(&cfg)
        .await
        .expect("Datil and later must have a HostCommands contract");

    let listener = HostCommandsListener::new(host_commands_contract, item)?;
    if let Err(e) = listener.listen_for_events().await {
        error!("{}", e);
        std::process::exit(1);
    }
    Ok(())
}
