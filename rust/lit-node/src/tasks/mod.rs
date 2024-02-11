pub mod batch_transmissions;
pub mod beaver_manager;
pub mod endpoint_channels;
pub mod fsm;
pub mod fsm_worker;
pub mod realtime_metrics;
pub mod utils;

use crate::config::chain::ChainDataConfigManager;
use crate::config::LitNodeConfig;
use crate::error::Result;
use crate::models::AuthContextCache;
use crate::node_state::NodeState;
use crate::peers::peer_reviewer::{PeerComplaint, PeerReviewer};
use crate::peers::PeerState;
use crate::siwe_db::db;
use crate::siwe_db::rpc::EthBlockhashCache;
use crate::tasks::fsm::node_fsm_worker;
#[cfg(feature = "rtmetrics")]
use crate::tasks::realtime_metrics::real_time_metrics_worker;
use crate::tss::common::key_type::{DkgType, KeyType};
use crate::tss::common::models::{NodeTransmissionDetails, RoundData};
use crate::tss::common::peer_checker::peer_checker_worker;
use crate::tss::common::peer_reviewer_bridge::{self, peer_reviewer_bridge_worker};
use crate::tss::common::restore::RestoreState;
use crate::tss::common::traits::fsm_worker_metadata::FSMWorkerMetadata;
use crate::tss::common::tss_state::TssState;
use beaver_manager::models::{BeaverManager, BeaverMessage};
use lit_core::config::ReloadableLitConfig;
use reqwest::Client;
use std::cmp::max;
use std::future::Future;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::task;

use batch_transmissions::batch_transaction_worker;
use endpoint_channels::rounds_worker;

pub struct Handle {
    join: Vec<thread::JoinHandle<()>>,
    quit: Vec<mpsc::Sender<bool>>,
}

impl Handle {
    fn new(join: Vec<thread::JoinHandle<()>>, quit: Vec<mpsc::Sender<bool>>) -> Self {
        Self { join, quit }
    }

    pub fn shutdown(self) {
        self.quit.iter().for_each(|f| {
            let _ = f.try_send(true);
        });
        for f in self.join {
            let _ = f.join();
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn launch(
    cfg: ReloadableLitConfig,
    chain_data_manager: Arc<ChainDataConfigManager>,
    eth_blockhash_cache: Arc<EthBlockhashCache>,
    pr_tx: flume::Sender<PeerComplaint>,
    pr_rx: flume::Receiver<PeerComplaint>,
    auth_context_cache: Arc<AuthContextCache>,
    peer_state: Arc<PeerState>,
    tss_state: Arc<TssState>,
    node_state: Arc<Mutex<NodeState>>,
    restore_state: Arc<RwLock<RestoreState>>,
    tx_round_manager: Arc<flume::Sender<RoundData>>,
    rx_round_manager: flume::Receiver<RoundData>,
    rx_batch_manager: flume::Receiver<NodeTransmissionDetails>,
    rx_peer_reviewer_bridge: flume::Receiver<peer_reviewer_bridge::Message>,
    bm_rx: flume::Receiver<BeaverMessage>,
    bm_tx: flume::Sender<BeaverMessage>,
    http_client: Client,
    fsm_worker_metadata: Arc<dyn FSMWorkerMetadata<LifecycleId = u64>>,
    metrics_rx: flume::Receiver<realtime_metrics::MetricsMessage>,
) -> Result<Handle> {
    // Dedicated runtime just for tasks.  Give at least 4 threads, but
    // up to physical cpu count.
    let worker_threads = max(4, num_cpus::get_physical());

    let lit_config = cfg.load_full();
    let peer_state_for_tasks = peer_state.clone();
    let peer_state2 = peer_state.clone();
    let peer_state_for_metrics = peer_state.clone();
    let peer_state_for_peer_reviewer_bridge = peer_state.clone();
    let peer_state_for_peer_checker = peer_state.clone();
    let fsm_worker_metadata = fsm_worker_metadata.clone();
    info!("Starting node tasks (worker_threads: {worker_threads})");

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .thread_name("node-tasks")
        .worker_threads(worker_threads)
        .max_blocking_threads(512)
        .enable_all()
        .build()
        .expect("create tokio runtime");

    let (quit_tx, mut quit_rx) = mpsc::channel(1);

    let http_client_clone = http_client.clone();
    let j = thread::spawn(move || {
        runtime.block_on(async move {
            let mut tasks = vec![];

            let chain_data_manager_clone = chain_data_manager.clone();
            tasks.push(spawn(|q| async move {
                chain_data_manager_clone
                    .as_ref()
                    .watch_chain_data_config(q)
                    .await;
            }));

            tasks.push(spawn(|q| async move {
                eth_blockhash_cache
                    .as_ref()
                    .fetch_and_store_latest_blockhash(q)
                    .await;
            }));

            match cfg.load().external_port() {
                Ok(port) => {
                    tasks.push(spawn(move |q| async move {
                        if let Err(e) = db::init_fill_db(port, q).await {
                            error!("Error while fetching init blocks: {}", e);
                        }
                    }));
                }
                Err(e) => error!("Error getting external port from config: {}", e),
            }

            // start up FSM worker
            if matches!(cfg.load().enable_epoch_transitions(), Ok(true)) {
                // peer_reviewer
                let mut peer_reviewer =
                    PeerReviewer::new(pr_rx, cfg.clone(), chain_data_manager.clone());
                tasks.push(spawn(|q| async move {
                    peer_reviewer.receive_complaints(q).await;
                }));

                let beaver_manager =
                    BeaverManager::new(bm_rx, bm_tx, tss_state.clone(), http_client_clone);

                match beaver_manager {
                    Ok(mut beaver_manager) => {
                        let cfg_clone = cfg.clone();
                        tasks.push(spawn(|q| async move {
                            beaver_manager.listen(cfg_clone, q).await;
                        }));
                    }
                    Err(e) => error!("Error creating beaver manager: {}", e),
                }

                let recovery_epoch_managers = vec![
                    tss_state
                        .get_epoch_manager(KeyType::BLS, DkgType::RecoveryParty)
                        .expect("Could not resolve Recovery BLS Epoch Manager"),
                    tss_state
                        .get_epoch_manager(KeyType::EcdsaCaitSith, DkgType::RecoveryParty)
                        .expect("Could not resolve Recovery ECDSA Epoch Manager"),
                ];

                let standard_epoch_managers = vec![
                    tss_state
                        .get_epoch_manager(KeyType::BLS, DkgType::Standard)
                        .expect("Could not resolve Standard BLS Epoch Manager"),
                    tss_state
                        .get_epoch_manager(KeyType::EcdsaCaitSith, DkgType::Standard)
                        .expect("Could not resolve Standard ECDSA Epoch Manager"),
                ];

                tasks.push(spawn(|quit_rx| async move {
                    node_fsm_worker(
                        quit_rx,
                        cfg,
                        peer_state,
                        node_state,
                        restore_state,
                        recovery_epoch_managers,
                        standard_epoch_managers,
                        fsm_worker_metadata,
                    )
                    .await;
                }));

                tasks.push(spawn(|quit_channel_rx| async move {
                    let res = peer_state2.listen_for_events(quit_channel_rx).await;
                    if let Err(e) = res {
                        error!("Error listening for events: {}", e);
                    }
                }));
            } else {
                info!("Epoch transitions disabled, not starting FSM");
            }

            // TSS workers
            {
                tasks.push(spawn(move |q| async move {
                    peer_reviewer_bridge_worker(
                        q,
                        rx_peer_reviewer_bridge,
                        pr_tx,
                        peer_state_for_peer_reviewer_bridge,
                    )
                    .await;
                }));

                tasks.push(spawn(move |q| async move {
                    peer_checker_worker(q, peer_state_for_peer_checker).await;
                }));

                let lit_config_for_batch = lit_config.clone();
                tasks.push(spawn(move |q| async move {
                    batch_transaction_worker(
                        q,
                        lit_config_for_batch,
                        peer_state_for_tasks,
                        rx_batch_manager,
                        http_client.clone(),
                    )
                    .await;
                }));

                let lit_config_for_rounds_queue = lit_config.clone();
                tasks.push(spawn(|q| async move {
                    rounds_worker(
                        q,
                        lit_config_for_rounds_queue,
                        rx_round_manager,
                        tx_round_manager,
                    )
                    .await;
                }));

                #[cfg(feature = "rtmetrics")]
                tasks.push(spawn(|quit_channel_rx| async move {
                    real_time_metrics_worker(quit_channel_rx, metrics_rx, peer_state_for_metrics)
                        .await;
                }));
            };
            let _ = quit_rx.recv().await;
            shutdown(tasks);
        });
    });

    Ok(Handle::new(vec![j], vec![quit_tx]))
}

fn spawn<F, Fut, T>(f: F) -> (mpsc::Sender<bool>, task::JoinHandle<T>)
where
    F: FnOnce(mpsc::Receiver<bool>) -> Fut,
    F: Send + 'static,
    Fut: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let (quit_tx, quit_rx) = mpsc::channel(1);
    let j = tokio::spawn(f(quit_rx));

    (quit_tx, j)
}

fn shutdown<T>(tasks: Vec<(mpsc::Sender<bool>, task::JoinHandle<T>)>)
where
    T: Send + 'static,
{
    // Send shutdown signal
    for (q, _) in tasks.iter() {
        let _ = q.try_send(true);
    }

    // Wait for shutdown (up to 60 seconds)
    for _ in 0..6000 {
        let mut still_alive = false;
        for (_, j) in tasks.iter() {
            if !j.is_finished() {
                still_alive = true;
                break;
            }
        }

        if !still_alive {
            break;
        }

        thread::sleep(Duration::from_millis(10));
    }

    // Kill remaining
    for (_, j) in tasks {
        if !j.is_finished() {
            warn!("Task still alive after 60s, aborting");
            j.abort();
        }
    }
}
