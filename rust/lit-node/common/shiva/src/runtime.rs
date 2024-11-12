use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
    time::Duration,
};

use anyhow::Ok;

use tracing::{error, info};

use crate::{models::TestNetMessage, testnet_instance::TestnetInsance};

use super::models::{
    ContractAbis, TestNetCommand, TestNetCreateParams, TestNetInfo, TestNetState, TestnetHandler,
};

pub fn create_runtime(
    params: TestNetCreateParams,
    quit_rx: flume::Receiver<bool>,
    tnm_tx: flume::Sender<TestNetMessage>,
    rx: flume::Receiver<TestNetCommand>,
    tns: Arc<RwLock<HashMap<String, TestnetHandler>>>,
) -> Result<JoinHandle<()>, anyhow::Error> {
    let testnet_instance = thread::spawn(move || {
        let uuid = params.uuid.to_string();
        info!(
            "Got create runtime request, spawning new testnet: {:?}",
            params
        );
        // add tokio runtime for async tasks for spawning the testnet and validators
        // the runtime will allow the network to deploy / build and perform stand up operations
        // the thread will then remained in a polling loop waiting for signals.
        let tn_rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name(uuid.to_string())
            .build();

        if let Err(e) = tn_rt {
            error!("Error while attempting to create runtime {}", e);
            return;
        }
        let tn_rt = tn_rt.unwrap();

        let spawner = tn_rt.block_on(async {
            let instance = TestnetInsance::init(params.clone()).await;

            if let Err(e) = instance {
                return Err(anyhow::anyhow!("Error while creating testnet {}", e));
            }

            let mut instance = instance.unwrap();
            let my_tns_map = tns.clone();
            let state = TestNetState::Active;
            let map = my_tns_map.write();

            if let Err(e) = map {
                return Err(anyhow::anyhow!("Error while creating testnet {}", e));
            }

            let mut map = map.unwrap();
            let tn_s = map.get(&uuid);
            let tn_s = &mut tn_s.unwrap().to_owned();
            tn_s.state = state.clone();
            map.insert(uuid.to_string(), tn_s.clone());

            drop(map);

            // clone the channel reciever into the blocking task scope.
            let c_handler_rx = rx.clone();
            loop {
                let mut heartbeat = tokio::time::interval(Duration::from_millis(100));
                heartbeat.tick().await; // First tick is immediate

                tokio::select! {
                    _ = heartbeat.tick() => {
                       info!("Got heartbeat in testnet with id {} waiting commands", uuid.to_string());
                    }
                    _ = quit_rx.recv_async() => {
                        info!("Shutting down testnet with id {} ", uuid);
                        break;
                    }
                    handler_command = c_handler_rx.recv_async() => {
                        let handler_command = match handler_command {
                            std::result::Result::Ok(hc) => hc,
                            Err(e) => {
                               error!("Error on handler command {}", e);
                                continue;
                            }
                        };
                        match handler_command {
                            TestNetCommand::GetInfo(sender) => {
                                let active_validators =
                                    instance
                                    .validators
                                    .get_active_validators()
                                    .await;

                                if let Err(ref e) = active_validators {
                                    error!("There was an error getting active validators for {} {}", uuid.to_string(), e);
                                    let _ = sender.send_async(None).await;
                                }

                                let active_addresses  = active_validators
                                                        .unwrap()
                                                        .iter()
                                                        .map(|v| hex::encode(v.account().node_address.to_fixed_bytes()))
                                                        .collect();

                                let contract_addresses = crate::models::ContractAddresses::new(&instance.contracts.contract_addresses());
                                let contract_resolver = serde_json::to_string(instance.contracts.contracts().contract_resolver.abi());
                                if let Err(e) = contract_resolver {
                                    error!("There was an error getting testnet info {}", e);
                                    let _ = tnm_tx.send(TestNetMessage::Cleanup(uuid.to_string()));
                                    continue;
                                }
                                let contract_resolver = contract_resolver.unwrap();
                                let rpc_url = instance.test_net.rpcurl.clone();
                                let epoch_length = instance.epoch_length.clone();
                                let contract_abis = ContractAbis::new(&instance.contracts);
                                if let Err(e) = contract_abis {
                                    error!("There was an error getting abis for testnet contracts {}", e);
                                    let _ = tnm_tx.send(TestNetMessage::Cleanup(uuid.to_string()));
                                    continue;
                                }
                                let contract_abis = contract_abis.unwrap();

                                let _ = sender.send_async(Some(TestNetInfo {
                                    contract_addresses: contract_addresses.clone(),
                                    validator_addresses: active_addresses,
                                    contract_resolver_abi: contract_resolver,
                                    contract_abis: contract_abis,
                                    rpc_url: rpc_url,
                                    epoch_length: epoch_length,
                                })).await;
                            },

                            TestNetCommand::StopRandom(sender) => {
                                let res = instance.stop_random_node().await;
                                if res.is_ok() {
                                    let _ = sender.send_async(true).await;
                                } else {
                                    let _ = sender.send_async(false).await;
                                }
                            }

                            TestNetCommand::StopRandomAndWait(sender) => {
                                let current_epoch = instance.validators.actions().get_current_epoch().await;
                                let res = instance.stop_random_node().await;
                                if res.is_ok() {
                                    instance.validators.actions().wait_for_epoch(current_epoch + 1).await;
                                    let _ = sender.send_async(true).await;
                                } else {
                                    let _ = sender.send_async(false).await;
                                }
                            }

                            TestNetCommand::Shutdown(sender) => {
                                info!("Stopping network");
                                let _ = sender.send_async(true).await;
                                drop(instance);
                                info!("Cleaned up testnet");
                                break; // break out of the loop and let clean up occur on the thread.
                            }

                            TestNetCommand::TransitionEpochAndWait(sender) => {
                                let current_epoch = instance.validators.actions().get_current_epoch().await;
                                instance.validators.actions().wait_for_epoch(current_epoch + 1).await;
                                let _ = sender.send_async(true).await;
                            }
                        }
                    }
                }
            }

            Ok(())
        });
        info!("Dropping runtime for {}", uuid.to_string());
        if let Err(e) = spawner {
            error!("Error from testnet {} runnnig cleanup", e);
            let _ = tnm_tx.send(TestNetMessage::Cleanup(uuid.to_string()));
        }

        // Shut down the background tasks as we are not interested in letting any of them run to completion.
        // Might cause resource leakage but currently is not totally known.
        tn_rt.shutdown_background();
    });

    Ok(testnet_instance)
}
