use crate::error::unexpected_err;
use crate::error::Result;
use crate::events::EventHandler;
use ethers::types::{Address, U256};
use lit_blockchain::contracts::host_commands::UpgradeFilter;
use lit_cli_os::config::LitCliOsConfig;
use lit_cli_os::guest::instance::GuestInstanceItem;
use lit_core::config::LitConfig;
use lit_core::config::LitConfigBuilder;
use serde_yaml::Value;
use std::path::PathBuf;
use std::process::Command;
use tracing::{info, trace};

#[derive(Debug, Clone)]
pub struct Upgrade {
    pub stake_address: Address,
    pub expiration_time: U256,
    pub release_branch_name: String,
    pub release_id: String,
    pub force: bool,
}
impl Upgrade {
    /// Update the lit os branches as per the event in the git.sls file, run `lit os update`
    pub fn update_litos(&self) -> Result<()> {
        trace!("Updating lit os ...");
        self.update_branches_in_git_sls()?;

        // NOTE: We run lit os update whether or not the branches actually changed as sometimes release branches are updated multiple times for a single release
        let output = Command::new("lit").args(["os", "update"]).output().map_err(|e| {
            unexpected_err(format!("Failed to run lit os update command with error {}", e), None)
        })?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(unexpected_err(
                format!("lit os update command failed with output: {}", error),
                None,
            ));
        }
        Ok(())
    }

    fn update_branches_in_git_sls(&self) -> Result<()> {
        let git_sls_path = PathBuf::from("/root/.salt-local/git.sls");
        let new_branch = self.release_branch_name.clone();
        let file_content =
            std::fs::read_to_string(&git_sls_path).map_err(|e| unexpected_err(e, None))?;
        let mut yaml_data: Value = serde_yaml::from_str(&file_content)
            .map_err(|e| unexpected_err(format!("Parse error: {}", e), None))?;
        let mut os_branch_found = false;
        let lit_os_branch = yaml_data
            .get_mut("git_repo")
            .and_then(|git_repo| git_repo.get_mut("lit-os"))
            .and_then(|lit_os| lit_os.get_mut("branch"))
            .ok_or_else(|| {
                unexpected_err("git_repo.lit-os.branch key does not exist in git.sls", None)
            })?;
        for env in ["prod", "staging", "dev"].iter() {
            if let Some(branch) = lit_os_branch.get_mut(env) {
                if os_branch_found {
                    return Err(unexpected_err(
                        "git.sls lit-os.branches contains more than one of: dev, staging, prod",
                        None,
                    ));
                }
                *branch = Value::String(new_branch.clone());
                os_branch_found = true;
            }
        }
        if !os_branch_found {
            return Err(unexpected_err(
                "git.sls lit-os.branches contains none of: dev, staging, prod",
                None,
            ));
        }
        let mut assets_branch_found = false;
        let lit_assets_branch = yaml_data
            .get_mut("git_repo")
            .and_then(|git_repo| git_repo.get_mut("lit-assets"))
            .and_then(|lit_os| lit_os.get_mut("branch"))
            .ok_or_else(|| {
                unexpected_err("git_repo.lit-assets.branch key does not exist in git.sls", None)
            })?;
        let envs = ["prod", "staging", "dev"];
        for env in &envs {
            if let Some(branch) = lit_assets_branch.get_mut(env) {
                if assets_branch_found {
                    return Err(unexpected_err(
                        "git.sls lit-assets.branches contains more than one of: dev, staging, prod",
                        None,
                    ));
                }
                *branch = Value::String(new_branch.clone());
                assets_branch_found = true;
            }
        }
        if !assets_branch_found {
            return Err(unexpected_err(
                "git.sls lit-assets.branches contains none of: dev, staging, prod",
                None,
            ));
        }
        let updated_content =
            serde_yaml::to_string(&yaml_data).map_err(|e| unexpected_err(e, None))?;
        std::fs::write(&git_sls_path, updated_content).map_err(|e| unexpected_err(e, None))?;
        Ok(())
    }

    pub fn recreate_node(&self, instance_item: &GuestInstanceItem) -> Result<()> {
        // assemble args for lit os recreate command
        let mut recreate_args = if self.force {
            vec!["os", "guest", "instance", "recreate", "--force"]
        } else {
            vec!["os", "guest", "instance", "recreate"]
        };
        // add node id to recreate command
        let instance_id = instance_item.instance_env.instance_id.clone().ok_or_else(|| {
            unexpected_err("node type node must have an instance id".to_string(), None)
        })?;
        recreate_args.push(&instance_id);
        // run it
        let output = Command::new("lit").args(recreate_args).output().map_err(|e| {
            unexpected_err(format!("Failed to run recreate command with error {}", e), None)
        })?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(unexpected_err(
                format!("Upgrade command failed with output: {}", error),
                None,
            ));
        }

        // Verify that the newly recreated node is on the correct release ID
        let cfg_builder = LitConfigBuilder::default();
        let cfg = LitConfig::from_builder(cfg_builder).expect("failed to load lit config");
        let new_release_id = crate::get_instance_item(&cfg)
            .release_env
            .expect("lit node only downloads releases which have a release_env")
            .release_id
            .expect("releases have an id");
        if new_release_id == self.release_id {
            Ok(())
        } else {
            Err(unexpected_err(
                format!(
                    "Failed to upgrade to target version {:?}, node is on release ID {:?}",
                    self.release_id, new_release_id
                ),
                None,
            ))
        }
    }
}

impl EventHandler for Upgrade {
    /// Call lit os to recreate/upgrade the node
    /// NOTE: All removal checks are delegated to lit os
    fn handle(&self, instance_item: &GuestInstanceItem) -> Result<()> {
        trace!("Executing upgrade operation...");
        self.update_litos()?;
        self.recreate_node(instance_item)?;
        info!("Upgrade operation completed successfully. Restarting lit-node-operator to start monitoring the new node");
        std::process::exit(0);
    }
    fn stake_address(&self) -> Address {
        self.stake_address
    }
    fn expiration_time(&self) -> U256 {
        self.expiration_time
    }
}

impl From<UpgradeFilter> for Upgrade {
    fn from(filter: UpgradeFilter) -> Self {
        Self {
            stake_address: filter.stake_address,
            expiration_time: filter.expiration_time,
            release_branch_name: filter.release_branch_name,
            release_id: filter.release_id,
            force: filter.force,
        }
    }
}
