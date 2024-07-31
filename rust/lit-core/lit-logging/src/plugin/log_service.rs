use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime};

use chrono::{DateTime, Utc};
use crossbeam_channel::{bounded, Receiver, Sender};
use serde_json::{Map, Value};
use tokio::runtime::Runtime;

use crate::config::LitLoggingConfig;
use lit_core::config::LitConfig;
use lit_core::logging::plugin::Plugin;
use lit_core::logging::types::LogRecord;

use crate::error::{timeout_err, unexpected_err, Result};
use crate::service::client::{LoggingServiceClient, LOGGING_SERVICE_CLIENT};
use crate::service::types::SubmitReq;

const INTERNAL_LOG_PREFIX: &str = "lit_core::logging::plugin::log_service";

// TODO: Make configurable.
const BATCH_SUBMIT_SLEEP_MS: u64 = 250;
const BATCH_SIZE_MAX: usize = 200;
const FLUSH_WAIT_SLEEP_MS: u64 = 50;
const FLUSH_TIMEOUT_SLEEP_MS: u64 = 2000;
const LOG_BACKLOG_LEN: usize = 5000;

pub struct LogServicePlugin {
    enqueue_tx: Option<Sender<Value>>,
    dequeue_rx: Option<Receiver<Value>>,
    dequeue_quit: Option<Sender<bool>>,
    dequeue_handle: Option<JoinHandle<Result<()>>>,
}

impl LogServicePlugin {
    pub fn new() -> Self {
        Self { enqueue_tx: None, dequeue_rx: None, dequeue_quit: None, dequeue_handle: None }
    }
}

impl Plugin for LogServicePlugin {
    fn init(&mut self, cfg: &LitConfig) -> Result<()> {
        if !cfg.logging_service_enabled() {
            // Bypassed.
            return Ok(());
        }

        let (tx, rx) = bounded(LOG_BACKLOG_LEN);
        let (quit_tx, quit_rx) = bounded(1);

        self.enqueue_tx = Some(tx.clone());
        self.dequeue_rx = Some(rx.clone());
        self.dequeue_quit = Some(quit_tx);
        self.dequeue_handle = Some(thread::spawn(move || {
            let rt = Runtime::new().map_err(|e| unexpected_err(e, None))?;
            rt.block_on(queue_worker(tx, rx, quit_rx));

            Ok(())
        }));

        Ok(())
    }

    fn apply_fields(&self, _fields: &mut Map<String, Value>) -> Result<()> {
        Ok(())
    }

    fn handle(&self, pkg: &str, record: &dyn LogRecord) -> Result<()> {
        if self.enqueue_tx.is_none() {
            // Bypassed.
            return Ok(());
        }

        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let now = now.to_rfc3339();

        if let Some(tx) = self.enqueue_tx.as_ref() {
            let mut entry: Map<String, Value> = Map::new();
            entry.insert("service".into(), Value::String(pkg.to_string()));
            entry.insert("level".into(), Value::String(record.level().to_string()));
            entry.insert("target".into(), Value::String(record.target().to_owned()));
            if let Some(name) = record.name() {
                entry.insert("name".into(), Value::String(name.to_owned()));
            }
            entry.insert("msg".into(), Value::String(format!("{}", record.display_args())));
            entry.insert("fields".into(), Value::Object(record.fields().clone()));
            entry.insert("timestamp".into(), Value::String(now));

            tx.try_send(Value::Object(entry)).map_err(|e| unexpected_err(e, None))?;
        }

        Ok(())
    }

    // Intended to be called during panic/drop only.
    fn flush(&self) -> Result<()> {
        if self.enqueue_tx.is_none() || self.dequeue_rx.is_none() {
            // Bypassed.
            return Ok(());
        }

        eprintln!("{INTERNAL_LOG_PREFIX}: Waiting for logs to flush...");

        let start = SystemTime::now();

        // Wait for any logs to be pushed into the queue.
        thread::sleep(Duration::from_millis(FLUSH_WAIT_SLEEP_MS));

        loop {
            let sofar =
                SystemTime::now().duration_since(start).map_err(|e| unexpected_err(e, None))?;

            if sofar.as_millis() >= FLUSH_TIMEOUT_SLEEP_MS as u128 {
                return Err(timeout_err("timed out waiting for log msg queue to drain", None));
            }

            if let Some(rx) = self.dequeue_rx.as_ref() {
                if rx.is_empty() {
                    return Ok(());
                }
            } else {
                return Ok(());
            }

            // Still have items
            thread::sleep(Duration::from_millis(BATCH_SUBMIT_SLEEP_MS));
        }
    }
}

impl Default for LogServicePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for LogServicePlugin {
    // TODO: Never gets called? I guess the log fmt thing holds a reference open?
    fn drop(&mut self) {
        if let Err(e) = self.flush() {
            // DO _NOT_ use log* functions here.
            eprintln!("{INTERNAL_LOG_PREFIX}: failed to flush log message queue: {e:?}");
        }

        if let Some(quit) = self.dequeue_quit.as_ref() {
            let _ = quit.send(true);

            if let Some(handle) = self.dequeue_handle.take() {
                let _ = handle.join();
            }
        }
    }
}

// Worker

async fn queue_worker(tx: Sender<Value>, rx: Receiver<Value>, quit_rx: Receiver<bool>) {
    loop {
        let mut skip_sleep = false;
        let next_req: Option<SubmitReq> = {
            let entries = rx.try_iter().take(BATCH_SIZE_MAX);
            let entries: Vec<Value> = entries.collect();
            if !entries.is_empty() {
                Some(SubmitReq::new(entries))
            } else {
                None
            }
        };

        if let Some(req) = next_req {
            let entries_len = req.entries.len();
            let mut requeue_all = false;
            let mut submitted_ok = false;

            match LOGGING_SERVICE_CLIENT.load().submit(&req).await {
                Ok(res) => {
                    if res.submitted == entries_len {
                        submitted_ok = true;
                    } else {
                        // DO _NOT_ use log* functions here.
                        eprintln!(
                            "{INTERNAL_LOG_PREFIX}: failed to submit some entries logging-service: {} of {}",
                            res.submitted, entries_len
                        );
                    }

                    if res.submitted == 0 {
                        requeue_all = true;
                    }
                }
                Err(e) => {
                    // DO _NOT_ use log* functions here.
                    eprintln!("{INTERNAL_LOG_PREFIX}: failed to submit to logging-service: {e:?}");

                    requeue_all = true;
                }
            }

            if submitted_ok {
                if entries_len == BATCH_SIZE_MAX {
                    skip_sleep = true;
                }
            } else if requeue_all {
                for entry in req.entries.into_iter() {
                    if let Err(e) = tx.try_send(entry) {
                        eprintln!(
                            "{INTERNAL_LOG_PREFIX}: failed send failed msg back into channel: {e:?}"
                        );
                    }
                }
            }
        }

        if let Ok(_val) = quit_rx.try_recv() {
            // Shutdown.
            break;
        }

        if !skip_sleep {
            // Sleep between submits.
            thread::sleep(Duration::from_millis(BATCH_SUBMIT_SLEEP_MS));
        }
    }
}
