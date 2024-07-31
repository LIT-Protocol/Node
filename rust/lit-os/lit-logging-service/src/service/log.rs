use char_device::CharDevice;
use std::io::Write;
use std::path::Path;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime};

use crossbeam_channel::{bounded, select, Receiver, Sender};
use serde_json::Value;
use tokio::runtime::Runtime;

use lit_core::config::LitConfig;

use crate::config::LitLoggingServiceConfig;
use crate::error::{timeout_err, unexpected_err, Result};

const INTERNAL_LOG_PREFIX: &str = "lit_logging_service::service::log";

const FLUSH_WAIT_SLEEP_MS: u64 = 50;
const FLUSH_TIMEOUT_SLEEP_MS: u64 = 2000;
const LOG_BACKLOG_LEN: usize = 50000;
const DEQUEUE_WAIT_MS: u64 = 500;

pub(crate) struct LogService {
    queue_tx: Sender<Value>,
    queue_rx: Receiver<Value>,
    queue_quit: Option<Sender<bool>>,
    queue_handle: Option<JoinHandle<Result<()>>>,
}

impl LogService {
    pub fn new() -> Self {
        let (tx, rx) = bounded(LOG_BACKLOG_LEN);

        Self { queue_tx: tx, queue_rx: rx, queue_quit: None, queue_handle: None }
    }

    pub fn start(mut self, cfg: &LitConfig) -> Result<Self> {
        let dev_path = cfg.log_service_device();

        let (quit_tx, quit_rx) = bounded(1);

        let tx = self.queue_tx.clone();
        let rx = self.queue_rx.clone();

        self.queue_quit = Some(quit_tx);
        self.queue_handle = Some(thread::spawn(move || {
            let rt = Runtime::new().map_err(|e| unexpected_err(e, None))?;
            rt.block_on(queue_worker(tx, rx, quit_rx, &dev_path));

            Ok(())
        }));

        Ok(self)
    }

    pub fn send(&self, entry: Value) -> Result<()> {
        self.queue_tx.try_send(entry).map_err(|e| unexpected_err(e, None))
    }

    #[allow(dead_code)]
    pub fn send_all(&self, entries: Vec<Value>) -> Result<()> {
        for entry in entries.into_iter() {
            self.send(entry)?;
        }

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        eprintln!("{INTERNAL_LOG_PREFIX}: Waiting for log service entries to flush...");

        let start = SystemTime::now();

        // Wait for any logs to be pushed into the queue.
        thread::sleep(Duration::from_millis(FLUSH_WAIT_SLEEP_MS));

        loop {
            let sofar =
                SystemTime::now().duration_since(start).map_err(|e| unexpected_err(e, None))?;

            if sofar.as_millis() >= FLUSH_TIMEOUT_SLEEP_MS as u128 {
                return Err(timeout_err("timed out waiting for log msg queue to drain", None));
            }

            if self.queue_rx.is_empty() {
                return Ok(());
            }

            // Still have items
            thread::sleep(Duration::from_millis(DEQUEUE_WAIT_MS));
        }
    }
}

impl Drop for LogService {
    fn drop(&mut self) {
        if let Err(e) = self.flush() {
            // DO _NOT_ use log* functions here.
            eprintln!("{INTERNAL_LOG_PREFIX}: Failed to flush log service message queue: {e:?}");
        }

        if let Some(quit) = self.queue_quit.as_ref() {
            let _ = quit.send(true);

            if let Some(handle) = self.queue_handle.take() {
                let _ = handle.join();
            }
        }
    }
}

// Worker

async fn queue_worker(
    _tx: Sender<Value>, rx: Receiver<Value>, quit_rx: Receiver<bool>, dev_path: &Path,
) {
    let mut dev = CharDevice::open(dev_path)
        .unwrap_or_else(|_| panic!("failed to open device: {dev_path:?}"));

    loop {
        select! {
            recv(quit_rx) -> _ => {
                // Shutdown.
                break;
            }
            recv(rx) -> res => {
                if let Ok(entry) = res {
                    match serde_json::to_string(&entry) {
                        Ok(json) =>
                            if let Err(e) = writeln!(dev, "{json}") {
                                eprintln!("{INTERNAL_LOG_PREFIX}: Failed to write log entry to device (dropping) - {e:?}")
                            },
                        Err(e) => {
                            eprintln!(
                                "{INTERNAL_LOG_PREFIX}: Failed to serialize log entry (dropping) - {e:?}"
                            )
                        }
                    }
                }
            }
        }
    }
}
