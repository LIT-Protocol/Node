use crate::error::unexpected_err;
use crate::error::Result;
use crate::events::EventHandler;
use lit_blockchain::contracts::host_commands::RestartFilter;
use lit_cli_os::guest::instance::GuestInstanceItem;
use std::process::Command;
use tracing::{info, trace};

use ethers::types::{Address, U256};

#[derive(Debug, Clone)]
pub struct Restart {
    pub stake_address: Address,
    pub expiration_time: U256,
    pub force: bool,
}
impl EventHandler for Restart {
    /// Call lit os to restart the node
    /// NOTE: All removal checks are delegated to lit os
    fn handle(&self, instance_item: &GuestInstanceItem) -> Result<()> {
        trace!("Executing restart operation...");
        // assemble args for lit os restart command
        let mut restart_args = if self.force {
            vec!["os", "guest", "instance", "restart", "--force"]
        } else {
            vec!["os", "guest", "instance", "restart"]
        };
        // add node id to restart command
        let instance_id = instance_item.instance_env.instance_id.clone().ok_or_else(|| {
            unexpected_err("node type node must have an instance id".to_string(), None)
        })?;
        restart_args.push(&instance_id);

        // run it
        let output = Command::new("lit").args(restart_args).output().map_err(|e| {
            unexpected_err(format!("Failed to run restart command with error {}", e), None)
        })?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(unexpected_err(
                format!("Restart command failed with output: {}", error),
                None,
            ));
        }

        info!("Restart operation completed successfully");
        Ok(())
    }
    fn stake_address(&self) -> Address {
        self.stake_address
    }
    fn expiration_time(&self) -> U256 {
        self.expiration_time
    }
}

impl From<RestartFilter> for Restart {
    fn from(filter: RestartFilter) -> Self {
        Self {
            stake_address: filter.stake_address,
            expiration_time: filter.expiration_time,
            force: filter.force,
        }
    }
}
