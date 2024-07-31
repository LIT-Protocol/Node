use ethers::types::{Address, Bytes, U256};
use lit_core::error::Unexpected;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;

use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::ReloadableLitConfig;

use crate::config::chain::ChainDataConfigManager;
use crate::error::Result;
use crate::peers::peer_state::models::NetworkState;
use crate::utils::contract::decode_revert;

use super::PeerState;

// keep this updated with the max Issue value below
pub const MAX_COMPLAINT_REASON_VALUE: u8 = 4;

#[derive(Debug)]
pub enum Issue {
    /// This is when a peer is not responding by sending packets back over the network.
    Unresponsive,
    /// This is when a peer is not participating in a protocol.
    NonParticipation,
    /// This is when peer data returned from their handshake does not match the data on chain.
    IncorrectInfo {
        err: anyhow::Error,
    },
    Error {
        err: anyhow::Error,
    },
}

impl Issue {
    pub fn value(&self) -> u8 {
        match *self {
            Issue::Unresponsive => 1,
            Issue::NonParticipation => 2,
            Issue::IncorrectInfo { .. } => 3,
            _ => 4,
        }
    }
}

impl PartialEq for Issue {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

#[derive(Debug)]
pub struct PeerComplaint {
    pub complainer: String,
    pub issue: Issue,
    pub peer_node_staker_address: Address,
}

#[derive(Debug, Default)]
/// PeerComplaints tracks all the complaints against a single peer for various reasons.
pub struct PeerComplaintsTracker {
    complaint_reason_to_count: HashMap<u8, u64>,
}

pub struct PeerReviewer {
    rx: flume::Receiver<PeerComplaint>,

    /// A map of peer keys to complaints against these peers.
    peer_key_to_complaint_maps: HashMap<String, PeerComplaintsTracker>,

    config: ReloadableLitConfig,
    chain_data_manager: Arc<ChainDataConfigManager>,
    peer_state: Arc<PeerState>,
}

impl PeerReviewer {
    pub fn new(
        rx: flume::Receiver<PeerComplaint>,
        config: ReloadableLitConfig,
        chain_data_manager: Arc<ChainDataConfigManager>,
        peer_state: Arc<PeerState>,
    ) -> PeerReviewer {
        PeerReviewer {
            rx,
            peer_key_to_complaint_maps: HashMap::new(),
            config,
            chain_data_manager,
            peer_state,
        }
    }

    // Should be spawned with a channel, with tx handles given to all tasks that need to complain
    pub async fn receive_complaints(&mut self, mut quit_rx: mpsc::Receiver<bool>) {
        // Start a timer and clear the issue counter every interval seconds
        let mut now = Instant::now();
        info!("Starting: PeerReviewer::receive_complaints");

        info!("Getting interval limit");
        let config = self.config.load_full();
        let resolver =
            ContractResolver::try_from(config.as_ref()).expect("failed to load ContractResolver");

        // TODO: As a quick and hacky solution, we will only use the Unresponsive complaint interval for now.
        // This is what we should be doing:
        // - Implement proper rolling window for each complaint reason.
        // - Clear complaints for each reason based on the respective interval.
        let mut interval = {
            let complaint_config = self
                .chain_data_manager
                .complaint_reason_to_config
                .get(&U256::from(1))
                .expect("No config found for complaint reason 1");
            complaint_config.interval_secs
        };

        loop {
            if now.elapsed().as_secs() > interval {
                self.peer_key_to_complaint_maps.clear();
                now = Instant::now();
                interval = {
                    let complaint_config = self
                        .chain_data_manager
                        .complaint_reason_to_config
                        .get(&U256::from(1))
                        .expect("No config found for complaint reason 1");
                    complaint_config.interval_secs
                };
            }

            tokio::select! {
                _ = quit_rx.recv() => {
                    break;
                }
                Ok(complaint) = self.rx.recv_async() => {
                    info!("Received complaint ({complaint:?})");
                    if let Err(e) = self.remember_complaint(complaint).await {
                        error!("Failed to remember complaint: {:?}", e);
                    }
                }
            }
        }

        info!("Stopped: PeerReviewer::receive_complaints");
    }

    /// If any of the complaints have reached the threshold for that corresponding reason, escalate the complaint.
    pub async fn remember_complaint(&mut self, complaint: PeerComplaint) -> Result<()> {
        let network_state = match self.peer_state.network_state().await {
            Ok(state) => state,
            Err(e) => {
                error!(
                    "Error getting network state in peer_checker_worker: {:?}",
                    e
                );
                return Err(e);
            }
        };
        if network_state == NetworkState::Restore || network_state == NetworkState::Paused {
            info!("Network state is {:?}. Skipping complaint.", network_state);
            return Ok(()); // don't complain about peers if we are restoring or paused
        }

        let peer_key = complaint.peer_node_staker_address;
        let peer_complaints_tracker = self
            .peer_key_to_complaint_maps
            .entry(peer_key.to_string())
            .or_default();
        let number_of_complaints = peer_complaints_tracker
            .complaint_reason_to_count
            .entry(Issue::Unresponsive.value())
            .or_insert(0);
        *number_of_complaints += 1;

        let complaint_reason_tolerance = {
            let complaint_config = self
                .chain_data_manager
                .complaint_reason_to_config
                .get(&U256::from(complaint.issue.value()))
                .expect_or_err(format!(
                    "No config found for complaint reason number {} which contains {:?}",
                    complaint.issue.value(),
                    complaint.issue
                ))?;
            complaint_config.tolerance
        };

        info!(
            "Remembering complaint #{:?} for peer {:?}.  Tolerance is {:?}",
            number_of_complaints, peer_key, complaint_reason_tolerance
        );

        if *number_of_complaints >= complaint_reason_tolerance {
            self.escalate_complaint(&complaint).await;
        }

        Ok(())
    }

    // post to blockchain
    pub async fn escalate_complaint(&self, complaint: &PeerComplaint) {
        info!("Escalating complaint ({:?})", complaint);
        let config = self.config.load_full();
        let resolver =
            ContractResolver::try_from(config.as_ref()).expect("failed to load ContractResolver");

        match resolver.staking_contract_with_signer(config.as_ref()).await {
            Ok(staking_contract) => match staking_contract
                .kick_validator_in_next_epoch(
                    complaint.peer_node_staker_address,
                    U256::from(complaint.issue.value()),
                    Bytes::from(vec![]),
                )
                .send()
                .await
            {
                Ok(_) => trace!("voted to kick peer"),
                Err(e) => trace!(
                    "failed to vote to kick peer w/ err {:?}",
                    decode_revert(&e, staking_contract.abi())
                ),
            },
            Err(e) => {
                error!("Failed to load staking contract: {:?}", e);
            }
        }
    }
}

// commented out until fixed or replaced
// #[cfg(test)]
// mod tests {
//     use std::path::PathBuf;

//     use ethers::types::Address;
//     use lit_core::config::{LitConfigBuilder, ReloadableLitConfig};
//     use tokio::sync::mpsc;

//     use super::{Issue, PeerComplaint, PeerReviewer};

//     fn generate_test_complaint(address: Address) -> PeerComplaint {
//         PeerComplaint {
//             complainer: String::from("complainer"),
//             issue: Issue::Unresponsive,
//             peer_node_staker_address: address,
//         }
//     }

//     fn generate_test_config() -> ReloadableLitConfig {
//         ReloadableLitConfig::new(|| {
//             let system_path = PathBuf::from("default");
//             let system_path_str = system_path.to_str().unwrap();

//             let cfg = LitConfigBuilder::new_with_paths(
//                 None,
//                 Some("/tmp/fake/nope".to_string()),
//                 system_path_str,
//                 "/tmp/fake/nope",
//             )
//             .build()
//             .expect("failed to load config");

//             Ok(cfg)
//         })
//         .unwrap()
//     }

//     #[tokio::test]
//     async fn test_receive_complaints() {
//         // create channel for passing complaint
//         let (pr_tx, pr_rx) = mpsc::channel(3);
//         let (quit_tx, quit_rx) = mpsc::channel(3);

//         // create new peerreviewer
//         let mut peer_reviewer = PeerReviewer::new(pr_rx, generate_test_config());

//         let res = tokio::time::timeout(
//             std::time::Duration::from_secs(1),
//             peer_reviewer.receive_complaints(quit_rx),
//         );

//         let num_requests: u64 = 3;
//         let addr = Address::random();
//         // pass message in channel
//         for i in 0..num_requests {
//             let send_complaint = pr_tx.send(generate_test_complaint(addr.clone())).await;
//             assert!(send_complaint.is_ok());
//         }

//         let _ = res.await;

//         // check received
//         let count = *peer_reviewer.counter.get(&addr.to_string()).unwrap();
//         assert!(count == num_requests);
//     }
// }
