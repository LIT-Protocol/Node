use crate::error::{unexpected_err, Result};
use crate::events::{EventHandler, Restart, Upgrade};
use ethers::middleware::Middleware;
use ethers::providers::StreamExt;
use lit_blockchain::contracts::host_commands::{HostCommands, HostCommandsEvents};
use lit_cli_os::guest::instance::GuestInstanceItem;
use tracing::{debug, error, info, trace, warn};
pub struct HostCommandsListener {
    host_commands_contract: HostCommands<ethers::providers::Provider<ethers::providers::Http>>,
    current_instance_item: GuestInstanceItem,
}

impl HostCommandsListener {
    pub fn new(
        host_commands_contract: lit_blockchain::contracts::host_commands::HostCommands<
            ethers::providers::Provider<ethers::providers::Http>,
        >,
        instance_item: GuestInstanceItem,
    ) -> Result<Self> {
        debug!(
            "HostCommandsListener created for Node ID {:?}",
            instance_item.instance_env.clone().instance_id.expect("instances must have an ID")
        );
        Ok(Self { host_commands_contract, current_instance_item: instance_item })
    }

    pub async fn listen_for_events(&self) -> Result<()> {
        let current_block = self
            .host_commands_contract
            .client()
            .get_block_number()
            .await
            .map_err(|e| unexpected_err(e, None))?;
        let events = self.host_commands_contract.events().from_block(current_block);
        let mut stream = events.stream().await.map_err(|e| unexpected_err(e, None))?;
        debug!("HostCommandsListener listening for host command events on {:?} starting from block {:?}", self.host_commands_contract, current_block);
        loop {
            tokio::select! {
                // watchdog time
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(60)) => {
                    let current_block = self.host_commands_contract.client().get_block_number().await.map_err(|e| unexpected_err(e, None))?;
                    debug!("No events received for 60 seconds, current block number: {:?}", current_block);
                }
                // event stream
                maybe_event = stream.next() => {
                    if let Some(event) = maybe_event {
                        match event {
                            Ok(log) => {
                                debug!("New event in event listener: {:?}", log);
                                match log.clone() {
                                    HostCommandsEvents::RestartFilter(restart_filter) => {
                                        debug!("Restart event");
                                        let restart_event = Restart::from(restart_filter);
                                        if restart_event.is_event_for_instance(&self.current_instance_item)
                                            && restart_event.is_unexpired()
                                        {
                                            info!("Received Restart event: {:?}", restart_event.clone());
                                            restart_event.handle(&self.current_instance_item)?
                                        }
                                    }
                                    HostCommandsEvents::UpgradeFilter(upgrade_filter) => {
                                        debug!("Upgrade event");
                                        let upgrade_event = Upgrade::from(upgrade_filter);
                                        if upgrade_event.is_event_for_instance(&self.current_instance_item)
                                            && upgrade_event.is_unexpired()
                                        {
                                            info!("Received Upgrade event: {:?}", upgrade_event.clone());
                                            upgrade_event.handle(&self.current_instance_item)?
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            Err(e) => {
                                error!("Error processing unknown HostCommands event: {:?}", e);
                                std::process::exit(1);
                            }
                        }
                    } else {
                        warn!("Stream yielded but no event found");
                    }
                }
            }
        }
    }
}
