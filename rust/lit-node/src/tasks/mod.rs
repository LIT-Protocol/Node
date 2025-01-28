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
use crate::tss::common::models::RoundData;
use crate::tss::common::peer_checker::peer_checker_worker;
use crate::tss::common::restore::RestoreState;
use crate::tss::common::traits::fsm_worker_metadata::FSMWorkerMetadata;
use crate::tss::common::tss_state::TssState;
use crate::tss::common::{curve_type::CurveType, dkg_type::DkgType};

use lit_core::config::ReloadableLitConfig;
use std::cmp::max;
use std::future::Future;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::task;

use endpoint_channels::rounds_worker;
use lit_blockchain::resolver::rpc::{RpcHealthcheckPoller, ENDPOINT_MANAGER};

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
    fsm_worker_metadata: Arc<dyn FSMWorkerMetadata<LifecycleId = u64>>,
    metrics_rx: flume::Receiver<realtime_metrics::MetricsMessage>,
) -> Result<Handle> {
    // Dedicated runtime just for tasks.  Give at least 4 threads, but
    // up to physical cpu count.
    let worker_threads = max(4, num_cpus::get_physical());

    let lit_config = cfg.load_full();
    let lit_config_clone = lit_config.clone();
    let peer_state_for_tasks = peer_state.clone();
    let peer_state2 = peer_state.clone();
    let peer_state_for_peer_reviewer = peer_state.clone();
    let peer_config_for_epoch_worker = peer_state.clone();
    let peer_state_for_peer_checker = peer_state.clone();
    #[cfg(feature = "rtmetrics")]
    let peer_state_for_metrics = peer_state.clone();
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

    let j = thread::spawn(move || {
        runtime.block_on(async move {
            let mut tasks = vec![];

            let chain_data_manager_clone = chain_data_manager.clone();
            let chain_data_manager_clone2 = chain_data_manager.clone();
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

            tasks.push(spawn(|mut q| async move {
                let poll_interval =
                    lit_config_clone.rpc_health_poll_interval().unwrap_or(60000) as u64;
                let mut interval = tokio::time::interval(Duration::from_millis(poll_interval));
                loop {
                    tokio::select! {
                        _ = q.recv() => {
                            break;
                        }
                        _ = interval.tick() => {
                            // Continue below.
                        }
                    }
                    if !chain_data_manager_clone2
                        .generic_config
                        .read()
                        .await
                        .rpc_healthcheck_enabled
                    {
                        continue;
                    }
                    ENDPOINT_MANAGER.poll_rpcs_for_latency().await;
                }
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
                let mut peer_reviewer = PeerReviewer::new(
                    pr_rx,
                    cfg.clone(),
                    chain_data_manager.clone(),
                    peer_state_for_peer_reviewer,
                );
                tasks.push(spawn(|q| async move {
                    peer_reviewer.receive_complaints(q).await;
                }));

                let recovery_epoch_managers = vec![
                    tss_state
                        .get_epoch_manager(CurveType::BLS, DkgType::RecoveryParty)
                        .expect("Could not resolve Recovery BLS Epoch Manager"),
                    tss_state
                        .get_epoch_manager(CurveType::K256, DkgType::RecoveryParty)
                        .expect("Could not resolve Recovery ECDSA Epoch Manager"),
                ];

                let standard_epoch_managers = vec![
                    tss_state
                        .get_epoch_manager(CurveType::BLS, DkgType::Standard)
                        .expect("Could not resolve Standard BLS Epoch Manager"),
                    tss_state
                        .get_epoch_manager(CurveType::K256, DkgType::Standard)
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
                    peer_checker_worker(q, peer_state_for_peer_checker).await;
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
            }

            #[cfg(feature = "lit-actions-server")]
            tasks.push(spawn(|mut quit_channel_rx| async move {
                let socket = lit_config
                    .actions_socket()
                    .expect("invalid socket path in config");

                info!("Starting: lit_actions server listening on {socket:?}");

                let signal = async move {
                    quit_channel_rx.recv().await;
                    info!("Stopped: lit_actions server");
                };

                if let Err(e) = lit_actions_server::start_server_with_shutdown(socket, signal).await
                {
                    error!("Error starting lit_actions server: {e:#}");
                }
            }));

            let _ = quit_rx.recv().await;
            shutdown(tasks).await;
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

async fn shutdown<T>(tasks: Vec<(mpsc::Sender<bool>, task::JoinHandle<T>)>)
where
    T: Send + 'static,
{
    // Send shutdown signal
    for (q, _) in tasks.iter() {
        let _ = q.try_send(true);
    }

    let shutdown_timeout = tokio::time::sleep(Duration::from_secs(60));
    tokio::pin!(shutdown_timeout);

    loop {
        let still_alive = tasks.iter().any(|(_, j)| !j.is_finished());
        if !still_alive {
            break;
        }

        tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(1)) => {}
            _ = &mut shutdown_timeout => {
                break;
            }
            _ = tokio::signal::ctrl_c() => {
                warn!("Ctrl-C received, forcing shutdown");
                break;
            }
        }
    }

    // Kill remaining
    for (_, j) in tasks {
        if !j.is_finished() {
            warn!("Task still alive after 60s, aborting");
            j.abort();
        }
    }
}
