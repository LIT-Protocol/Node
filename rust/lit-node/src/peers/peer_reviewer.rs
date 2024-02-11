use ethers::types::{Address, Bytes, U256};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;

use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::ReloadableLitConfig;

use crate::config::chain::ChainDataConfigManager;
use crate::utils::contract::decode_revert;

#[derive(Debug)]
pub enum Issue {
    /// This is when a peer is not responding by sending packets back over the network.
    Unresponsive,
    /// This is when a peer is not participating in a protocol.
    NonParticipation,
    Error {
        err: anyhow::Error,
    },
}

impl Issue {
    fn value(&self) -> u8 {
        match *self {
            Issue::Unresponsive => 1,
            Issue::NonParticipation => 2,
            _ => 3,
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

pub struct PeerReviewer {
    rx: flume::Receiver<PeerComplaint>,
    counter: HashMap<String, u64>,

    // A counter for tracking ONLY non-participation complaints per each interval
    // TODO: This is a temporary solution until we have a better way to track and action against
    // various types of complaints.
    non_participation_counter: HashMap<String, u64>,
    config: ReloadableLitConfig,
    chain_data_manager: Arc<ChainDataConfigManager>,
}

impl PeerReviewer {
    pub fn new(
        rx: flume::Receiver<PeerComplaint>,
        config: ReloadableLitConfig,
        chain_data_manager: Arc<ChainDataConfigManager>,
    ) -> PeerReviewer {
        PeerReviewer {
            rx,
            counter: HashMap::new(),
            non_participation_counter: HashMap::new(),
            config,
            chain_data_manager,
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

        let mut interval = {
            let complaint_config = self.chain_data_manager.generic_config.read().await;
            complaint_config.complaint_interval_secs
        };

        loop {
            if now.elapsed().as_secs() > interval {
                self.counter.clear();
                self.non_participation_counter.clear();
                now = Instant::now();
                interval = {
                    let complaint_config = self.chain_data_manager.generic_config.read().await;
                    complaint_config.complaint_interval_secs
                };
            }

            tokio::select! {
                _ = quit_rx.recv() => {
                    break;
                }
                Ok(complaint) = self.rx.recv_async() => {
                    warn!("Received complaint ({complaint:?})");
                    self.remember_complaint(complaint).await;
                }
            }
        }

        info!("Stopped: PeerReviewer::receive_complaints");
    }

    pub async fn remember_complaint(&mut self, complaint: PeerComplaint) {
        // TODO track reason here
        let peer_key = complaint.peer_node_staker_address;
        let number_of_complaints = self.counter.entry(peer_key.to_string()).or_insert(0);
        *number_of_complaints += 1;

        let complaint_tolerance = {
            let complaint_config = self.chain_data_manager.generic_config.read().await;
            complaint_config.complaint_tolerance
        };

        info!(
            "Remembering complaint #{:?} for peer {:?}.  Tolerance is {:?}",
            number_of_complaints, peer_key, complaint_tolerance
        );

        // Otherwise, we escalate if we've received enough complaints about this peer.
        if *number_of_complaints >= complaint_tolerance {
            self.escalate_complaint(&complaint).await;
        }

        if complaint.issue == Issue::NonParticipation {
            // If we've received enough complaints about this peer not participating in a protocol, we escalate.
            let number_of_non_participation_complaints = self
                .non_participation_counter
                .entry(peer_key.to_string())
                .or_insert(0);
            *number_of_non_participation_complaints += 1;

            if *number_of_non_participation_complaints >= 3 {
                self.escalate_complaint(&complaint).await;
            }
        }
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
