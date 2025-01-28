use signal_hook::iterator::exfiltrator::WithOrigin;
use signal_hook::{
    consts::{SIGCONT, SIGHUP, SIGTSTP, SIGWINCH, TERM_SIGNALS},
    flag,
    iterator::SignalsInfo,
};

use crate::client::ShivaClient;
use crate::models::{TestNetCommand, TestNetInfo, TestNetMessage, TestNetState, TestnetHandler};
use crate::transport::{HttpTransport, Transport, TransportType};

use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, Arc, RwLock},
    thread::{self, JoinHandle},
    time::Duration,
};
use tracing::{error, info};

use tokio::sync::broadcast;
extern crate rocket;

pub fn start_runtime(transport: TransportType) -> Result<(), anyhow::Error> {
    info!("setting up testnet manager env");

    // Make sure double CTRL+C and similar kills
    // If we use a single CTRL+C we will run into case
    // which will cause managed testnets to be dropped before it formally leaves
    // scope of the runtime instance. Which causes undefined behaviors
    let term_now = Arc::new(AtomicBool::new(false));
    for sig in TERM_SIGNALS {
        // When terminated by a second term signal, exit with exit code 1.
        // This will do nothing the first time (because term_now is false).
        flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
        // But this will "arm" the above for the second time, by setting it to true.
        // The order of registering these is important, if you put this one first, it will
        // first arm and then terminate ‒ all in the first round.
        flag::register(*sig, Arc::clone(&term_now))?;
    }

    // Subscribe to all these signals with information about where they come from. We use the
    // extra info only for logging in this example (it is not available on all the OSes or at
    // all the occasions anyway, it may return `Unknown`).
    let mut sigs = vec![
        // Some terminal handling
        SIGTSTP, SIGCONT, SIGWINCH,
        // Reload of configuration for daemons ‒ um, is this example for a TUI app or a daemon
        // O:-)? You choose...
        SIGHUP,
    ];

    sigs.extend(TERM_SIGNALS);
    let mut signals = SignalsInfo::<WithOrigin>::new(&sigs)?;

    // use current thread context for the web server as we spawn the runtime in a new background thread
    let web_rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .thread_name("Testnet Manager http")
        .build()
        .map_err(|e| anyhow::anyhow!("Could not create tokio runtime {}", e))?;

    // Deno 1.46 pins Tokio to 1.36.0, which doesn't support metrics.
    // TODO: Update to Deno 2.0.4+, which lifts this restriction.
    // See https://github.com/denoland/deno/pull/26457
    // let web_metrics = web_rt.metrics();

    let (rt_quit_tx, _rt_quit_rx) = broadcast::channel::<bool>(1);
    let mut quit_rx_web = rt_quit_tx.subscribe();
    let (tnm_tx, tnm_rx) = flume::unbounded::<TestNetMessage>();

    let s_quit_tx = rt_quit_tx.clone();

    let c_tnm_tx = tnm_tx.clone();
    let t_tnm_tx = tnm_tx.clone();
    // setup a runtime for managing the testnet runtimes (a runtime runtime manager).
    let tnm_rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("Testnet Manager")
        .build()
        .map_err(|e| anyhow::anyhow!("Could not create tokio runtime {}", e))?;
    // let tnm_metrics = tnm_rt.metrics();
    let client = ShivaClient::new();

    let tnm_handler = thread::spawn(move || {
        tnm_rt.block_on(async {
                let tns_map: Arc<RwLock<HashMap<String, TestnetHandler>>> = Arc::new(RwLock::new(HashMap::new()));
                let mut tnh_map: HashMap<String, JoinHandle<()>> = HashMap::new();

                loop {
                    let mut heartbeat = tokio::time::interval(Duration::from_millis(100));
                    heartbeat.tick().await; // First tick is immediate
                    tokio::select! {
                        _ = quit_rx_web.recv() => {
                            info!("Got singal to exit, closing testnet manager");
                            info!("Shutting down testnets");
                            let h_map = tns_map.read().unwrap();
                            for tn in h_map.keys() {
                                info!("cleaning up testnet with id {}", tn);
                                let handle = h_map.get(tn).unwrap();
                                let _ = handle.term_channel.send(true);
                            }
                            break;
                        }
                        _ = heartbeat.tick() => {
                        }
                        command = tnm_rx.recv_async() => {
                            let command = match command {
                                Ok(c) => c,
                                Err(e) => {
                                    println!("error on commnand recieve {}", e);
                                    continue;
                                }
                            };
                            match command {
                                TestNetMessage::Create(params) => {
                                    info!("starting Testnet {}", params.uuid.to_string());
                                    let cloned_id = params.uuid.to_string();
                                    let (handler_tx, handler_rx)  = flume::unbounded::<TestNetCommand>();
                                    let (quit_tx, quit_rx) = flume::unbounded::<bool>();
                                    let handler = TestnetHandler {
                                        state: TestNetState::Busy.clone(),
                                        channel: handler_tx.clone(),
                                        term_channel: quit_tx.clone(),
                                    };
                                    let my_tns_map = tns_map.clone();
                                    let testnet_instance = crate::runtime::create_runtime(
                                        params.clone(),
                                        quit_rx,
                                        c_tnm_tx.clone(),
                                        handler_rx,
                                        my_tns_map
                                    );
                                    if let Err(e) = testnet_instance {
                                        error!("Error while atempting to create new testnet: {} aborting testnet standup", e);
                                    } else {
                                        let testnet_instance = testnet_instance.unwrap();
                                        tnh_map.insert(cloned_id.to_string(), testnet_instance);
                                        tns_map.write().unwrap().insert(cloned_id.to_string(), handler);
                                    }
                                }

                                TestNetMessage::Delete(uuid, tx) => {
                                    let map = tns_map.read().unwrap();
                                    let handler = map.get(&uuid);
                                    if let Some(h) = handler {
                                        let (shutdown_tx, shutdown_rx) = flume::unbounded::<bool>();
                                        info!("sending command to shutdown testnet");
                                        let _ = h.channel.send(TestNetCommand::Shutdown(shutdown_tx));
                                        let _ = shutdown_rx.recv_async().await;
                                        let _ = tx.send(true); // send a response once we receive from the runtime
                                    } else {
                                        let _ = tx.send(false);
                                        continue; // if the handler doesnt exist just respond to the channel and continue processing
                                    }
                                    drop(map);

                                    info!("found testnet shutting down after confirmation send");
                                    let map = tns_map.write();
                                    let mut map = map.unwrap();
                                    info!("got map read lock deleting testnet handler");
                                    let t_handler = map.remove_entry(&uuid);
                                    let join_handle = tnh_map.remove_entry(&uuid);

                                    let join_handle = join_handle.unwrap().1;
                                    let res = join_handle.join();
                                    info!("got result from {} shutdown response {:?}", uuid, res);
                                    drop(t_handler);
                                    drop(map);
                                }

                                TestNetMessage::Poke(uuid, tx) => {
                                    let map = tns_map.read().unwrap();
                                    let handler = map.get(&uuid);

                                    if let Some(h) = handler {
                                        let _ = tx.send(h.state.clone());
                                    }
                                    drop(map);
                                }

                                TestNetMessage::GetInfo(uuid, tx) => {
                                    let map = tns_map.read();
                                    if let Err(e) = map {
                                        error!("Lock is posioned aborting command operation {}", e);
                                        let _ = tx.send(None);
                                        continue;
                                    }
                                    let map = map.unwrap();
                                    let handler = map.get(&uuid);

                                    if let Some(h) = handler {
                                        if h.state == TestNetState::Active {
                                            let (info_tx, info_rx) = flume::unbounded::<Option<TestNetInfo>>();
                                            let _ = h.channel.send(TestNetCommand::GetInfo(info_tx));
                                            let info_res = info_rx.recv_async().await;
                                            if let Err(e) = info_res {
                                                error!("Error while attempting to get testnet info {}", e);
                                                let _ = tx.send(None);
                                            } else if let Ok(info) = info_res {
                                                let _ = tx.send(info);
                                            }
                                        } else {
                                            info!("TestNet is busy, come back later");
                                            let _ = tx.send(None);
                                        }
                                    } else {
                                        info!("Testnet does not exist");
                                        let _ = tx.send(None);
                                    }
                                }

                                TestNetMessage::StopRandom(uuid, tx) => {
                                    let map = tns_map.write();
                                    if let Err(e) = map {
                                        error!("Lock is posioned aborting command operation {}", e);
                                        let _ = tx.send(None);
                                        continue;
                                    }
                                    let mut map = map.unwrap();
                                    let handler = map.get(&uuid);

                                    if let Some(h) = handler {
                                        if h.state == TestNetState::Active {
                                            let state = TestNetState::Mutating;
                                            let mut ch = h.clone();
                                            ch.state = state.clone();
                                            map.insert(uuid.to_string(), ch.clone());

                                            let (info_tx, info_rx) = flume::unbounded::<bool>();
                                            let _shutdown_res = ch.channel.send(TestNetCommand::StopRandom(info_tx));

                                            let stop_res = info_rx.recv_async().await;
                                            let _stop_res = stop_res.unwrap();
                                            let _ = tx.send(Some(true));

                                            let state = TestNetState::Active;
                                            ch.state = state;
                                            map.insert(uuid.to_string(), ch.clone());

                                            drop(map);
                                        }
                                    }
                                }

                                TestNetMessage::StopRandomAndWait(uuid, tx) => {
                                    let map = tns_map.write();
                                    if let Err(e) = map {
                                        error!("Lock is posioned aborting command operation {}", e);
                                        let _ = tx.send(None);
                                        continue;
                                    }
                                    let mut map = map.unwrap();

                                    let handler = map.get(&uuid);

                                    if let Some(h) = handler {
                                        if h.state == TestNetState::Active {
                                            let state = TestNetState::Mutating;
                                            let mut ch = h.clone();
                                            ch.state = state.clone();
                                            map.insert(uuid.to_string(), ch.clone());

                                            let (info_tx, info_rx) = flume::unbounded::<bool>();
                                            let _shutdown_res = ch.channel.send(TestNetCommand::StopRandomAndWait(info_tx));

                                            let stop_res = info_rx.recv_async().await;
                                            let _stop_res = stop_res.unwrap();
                                            let _ = tx.send(Some(true));

                                            let state = TestNetState::Active;
                                            ch.state = state;
                                            map.insert(uuid.to_string(), ch.clone());

                                            drop(map);
                                        }
                                    }
                                }

                                TestNetMessage::TransitionEpochAndWait(uuid, tx) => {
                                    let map = tns_map.write();
                                    if let Err(e) = map {
                                        error!("Lock is posioned aborting command operation {}", e);
                                        let _ = tx.send(false);
                                        continue;
                                    }
                                    let mut map = map.unwrap();

                                    let handler = map.get(&uuid);

                                    if let Some(h) = handler {
                                        if h.state == TestNetState::Active {
                                            let state = TestNetState::Mutating;
                                            let mut ch = h.clone();
                                            ch.state = state.clone();
                                            map.insert(uuid.to_string(), ch.clone());

                                            let (info_tx, info_rx) = flume::unbounded::<bool>();
                                            let _epoch_transition_res = ch.channel.send(TestNetCommand::TransitionEpochAndWait(info_tx));
                                            let _transition_res = info_rx.recv_async().await;
                                            let _ = tx.send(true);

                                            let state = TestNetState::Active;
                                            ch.state = state;
                                            map.insert(uuid.to_string(), ch.clone());

                                            drop(map);
                                        }
                                    }
                                }

                                TestNetMessage::GetTestnets(tx) => {
                                    let map = tns_map.write();
                                    if let Err(e) = map {
                                        error!("Lock is posioned aborting command operation {}", e);
                                        let _ = tx.send(vec![]);
                                        continue;
                                    }
                                    let map = map.unwrap();

                                    let keys = map.keys();
                                    let mut ids: Vec<String> = Vec::with_capacity(map.len());
                                    for key in keys {
                                        ids.push(String::from(key));
                                    }
                                    let _ = tx.send(ids);
                                }

                                TestNetMessage::Cleanup(uuid) => {
                                    info!("Cleaning up testnet with id {}", uuid.to_string()); 
                                    info!("Found testnet handler, performing cleanup");
                                    let map = tns_map.write();
                                    // TODO: If the lock is found to be posioned can we still clean up?
                                    if let Err(e) = map {
                                        error!("Lock is posioned aborting command operation {}", e);
                                        continue;
                                    }
                                    let mut map = map.unwrap();
                                    info!("Got map read lock deleting testnet handler");
                                    let t_handler = map.remove_entry(&uuid);
                                    let _ = tnh_map.remove_entry(&uuid);

                                    drop(t_handler);
                                    drop(map);
                                }

                                _ => {
                                    info!("unknown command");
                                }
                            }
                        }
                    }
                }
            });
    });

    let web_handler: JoinHandle<()>;
    match transport {
        TransportType::HTTP => {
            info!("Found transport type to be http");
            let port_env = std::env::var("PORT");
            let mut port = "8000".to_string();
            if let Err(_) = port_env {
                error!("Port not found in ENV using default 8000");
            } else if let Ok(p) = port_env {
                port = p.to_string();
            }

            let transport = HttpTransport {
                address: "0.0.0.0".parse()?,
                port: port.parse()?,
            };

            web_handler = transport.bind(web_rt, client, s_quit_tx.clone(), t_tnm_tx)?;
            info!("Done binding http transport type");
        }
    }
    info!("Started manager and web server runtimes");
    // info!(
    //     "Manager runtime worker count {:?}",
    //     tnm_metrics.num_workers()
    // );
    // info!("> Web server worker count: {:?}", web_metrics.num_workers());
    // Consume all the incoming signals. This happens in "normal" Rust thread, not in the
    // signal handlers. This means that we are allowed to do whatever we like in here, without
    // restrictions, but it also means the kernel believes the signal already got delivered, we
    // handle them in delayed manner. This is in contrast with eg the above
    // `register_conditional_shutdown` where the shutdown happens *inside* the handler.
    for info in &mut signals {
        // Check for SIGINT
        if info.signal == 2 {
            // if we get a signal for shutdown tell the
            // manager runtime it is time to clean up
            let _ = s_quit_tx.clone().send(true);
        }
    }

    let _ = web_handler.join();
    let _ = tnm_handler.join();
    info!("Done joining threads, completing shutdown");
    Ok(())
}
