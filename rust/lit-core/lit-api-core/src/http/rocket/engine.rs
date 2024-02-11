use std::result::Result as StdResult;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use futures::future::BoxFuture;
use log::as_error;
use once_cell::sync::Lazy;
use rocket::async_main;
use rocket::Error as RocketError;
use sd_notify::NotifyState;
use tokio::runtime::Runtime;

use crate::http::rocket::launcher::{Launcher, Shutdown};
use crate::Event;

static LOCAL_RUNTIME: Lazy<Arc<Runtime>> =
    Lazy::new(|| Arc::new(tokio::runtime::Runtime::new().unwrap()));

const RESTARTER_SLEEP_MS: u64 = 500;
const SHUTDOWN_WAIT_SECS: u64 = 20;
const SHUTDOWN_WAIT_INTERVAL_MS: u64 = 100;

pub(crate) type CreateLauncher = Arc<dyn Send + Sync + Fn() -> BoxFuture<'static, Launcher>>;

pub struct Engine {
    create_launcher: CreateLauncher,
    launcher_join_handles: Arc<Mutex<Vec<JoinHandle<()>>>>,
    restarter_join_handle: Option<JoinHandle<()>>,
    shutdown_handles: Arc<Mutex<Vec<Shutdown>>>,
    // State
    started: bool,
    quit_mu: Arc<Mutex<bool>>,
}

impl Engine {
    pub fn new(
        create_launcher: impl Send + Sync + Fn() -> BoxFuture<'static, Launcher> + 'static,
    ) -> Self {
        Self {
            create_launcher: Arc::new(create_launcher),
            launcher_join_handles: Arc::new(Mutex::new(Vec::new())),
            restarter_join_handle: None,
            shutdown_handles: Arc::new(Mutex::new(Vec::new())),
            started: false,
            quit_mu: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&mut self) {
        if self.started {
            panic!("start may not be called more than once!");
        }
        self.started = true;

        let (restarter_tx, restarter_rx) = mpsc::sync_channel(10);

        self.spawn_launcher(restarter_tx.clone());
        self.spawn_restarter(restarter_tx, restarter_rx);
        self.notify_started();
        self.join();
    }

    fn spawn_launcher(&mut self, restarter_tx: SyncSender<()>) {
        spawn_launcher(
            restarter_tx,
            self.create_launcher.clone(),
            self.launcher_join_handles.clone(),
            self.shutdown_handles.clone(),
        )
        .unwrap();
    }

    fn spawn_restarter(&mut self, restarter_tx: SyncSender<()>, restarter_rx: Receiver<()>) {
        let create_launcher = self.create_launcher.clone();
        let launcher_join_handles = self.launcher_join_handles.clone();
        let shutdown_handles = self.shutdown_handles.clone();
        let quit_mu = self.quit_mu.clone();

        self.restarter_join_handle = Some(thread::spawn(move || {
            let mut quit = *quit_mu.lock().unwrap();
            while !quit {
                if restarter_rx.try_recv().is_ok() {
                    info!("rocket engine - restart requested");

                    // Collect shutdown handles
                    let handles = take_shutdown_handles(shutdown_handles.clone());

                    // Start new instance
                    spawn_launcher(
                        restarter_tx.clone(),
                        create_launcher.clone(),
                        launcher_join_handles.clone(),
                        shutdown_handles.clone(),
                    )
                    .unwrap();

                    // Trigger shutdown
                    trigger_shutdown(handles);

                    // Wait for shutdown
                    if !wait_for_launcher_join_handles_len(launcher_join_handles.clone(), 1) {
                        panic!("failed to wait for shutdown during rocket restart");
                    }
                }

                thread::sleep(Duration::from_millis(RESTARTER_SLEEP_MS));
                quit = *quit_mu.lock().unwrap();
            }
        }));
    }

    fn notify_started(&self) {
        if let Err(e) = sd_notify::notify(true, &[NotifyState::Ready]) {
            warn!(error = as_error!(e); "failed to send systemd notify");
        }
    }

    fn join(&mut self) {
        let mut cur_handle: Option<JoinHandle<()>> =
            self.launcher_join_handles.lock().unwrap().pop();
        while cur_handle.is_some() {
            let _ = cur_handle.take().unwrap().join();
            cur_handle = self.launcher_join_handles.lock().unwrap().pop();
        }

        // Launchers have finished, trigger quit.
        {
            let mut quit = self.quit_mu.lock().unwrap();
            *quit = true;
        }

        if let Some(cur_handle) = self.restarter_join_handle.take() {
            let _ = cur_handle.join();
        }
    }
}

fn spawn_launcher(
    restart_tx: SyncSender<()>, create_launcher: CreateLauncher,
    launcher_join_handles: Arc<Mutex<Vec<JoinHandle<()>>>>,
    shutdown_handles: Arc<Mutex<Vec<Shutdown>>>,
) -> StdResult<(), RocketError> {
    let mut launcher: Launcher = LOCAL_RUNTIME.block_on((create_launcher)());

    launcher = launcher.on_event(Event::CertChange, move |_data| {
        let restart_tx = restart_tx.clone();

        Box::pin(async move {
            restart_tx.send(()).unwrap();

            Ok(())
        })
    });

    let mut shutdown_handles = shutdown_handles.lock().unwrap();

    shutdown_handles.push(LOCAL_RUNTIME.block_on(launcher.ignite())?);

    let mut launcher_join_handles = launcher_join_handles.lock().unwrap();

    launcher_join_handles.push(thread::spawn(move || {
        let _ = async_main(launcher.launch());
    }));

    Ok(())
}

fn take_shutdown_handles(shutdown_handles: Arc<Mutex<Vec<Shutdown>>>) -> Vec<Shutdown> {
    let mut res = Vec::new();

    let mut shutdown_handles = shutdown_handles.lock().unwrap();
    while shutdown_handles.len() > 0 {
        res.push(shutdown_handles.remove(0));
    }

    res
}

fn trigger_shutdown(mut shutdown_handles: Vec<Shutdown>) {
    while !shutdown_handles.is_empty() {
        let handle = shutdown_handles.remove(0);
        handle.shutdown();
    }
}

fn wait_for_launcher_join_handles_len(
    launcher_join_handles: Arc<Mutex<Vec<JoinHandle<()>>>>, expected_len: usize,
) -> bool {
    let mut attempts = 0;
    let mut cur_len = launcher_join_handles.lock().unwrap().len();
    while cur_len != expected_len {
        if attempts >= (SHUTDOWN_WAIT_SECS * 1000) / SHUTDOWN_WAIT_INTERVAL_MS {
            return false;
        }

        thread::sleep(Duration::from_millis(SHUTDOWN_WAIT_INTERVAL_MS));
        cur_len = launcher_join_handles.lock().unwrap().len();
        attempts += 1;
    }

    true
}
