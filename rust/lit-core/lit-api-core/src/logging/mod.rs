use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value};

use lit_core::config::LitConfig;
use lit_core::logging::plugin::Plugin;
use lit_core::logging::types::LogRecord;

#[cfg(feature = "server")]
use crate::context::TRACING;
use crate::error::{Error, Result};

pub const LOG_FIELD_KEY_CORRELATION_ID: &str = "correlation_id";

pub struct TracingPlugin {}

impl TracingPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for TracingPlugin {
    fn init(&mut self, _cfg: &LitConfig) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn apply_fields(&self, fields: &mut Map<String, Value>) -> Result<()> {
        #[allow(unused_mut)]
        let mut err: Option<Error> = None;

        // Ignore errors (it's optional).
        #[cfg(feature = "server")]
        let _ = TRACING.try_with(|tracing| {
            fields.insert(
                LOG_FIELD_KEY_CORRELATION_ID.into(),
                serde_json::Value::String(tracing.correlation_id().clone()),
            );

            if let Err(e) = tracing.apply_fields(fields) {
                let _ = err.insert(e);
            }
        });

        if let Some(e) = err {
            return Err(e);
        }

        Ok(())
    }

    fn handle(&self, _pkg: &str, _record: &dyn LogRecord) -> Result<()> {
        Ok(())
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }
}

impl Default for TracingPlugin {
    fn default() -> Self {
        TracingPlugin::new()
    }
}

pub struct FileLoggingPlugin {
    path: Option<PathBuf>,
}

impl FileLoggingPlugin {
    pub fn new() -> Self {
        Self { path: None }
    }
}

impl Plugin for FileLoggingPlugin {
    fn init(&mut self, cfg: &LitConfig) -> Result<()> {
        // make test_logs folder if it doesn't exist
        let path = Path::new("./test_logs");
        if !path.exists() {
            let created = std::fs::create_dir(path);
            if let Err(e) = created {
                eprintln!("Error creating test_logs folder: {:?}", e);
            }
        }
        let path = PathBuf::from(format!(
            "./test_logs/{}",
            cfg.get_string("node.staker_address")
                .expect("Could not load staker address for test FileLoggingPlugin")
                .to_lowercase()
        ));
        // create the file
        File::create(&path).expect("Could not create file for test FileLoggingPlugin");
        self.path = Some(path);
        Ok(())
    }

    #[allow(unused_variables)]
    fn apply_fields(&self, fields: &mut Map<String, Value>) -> Result<()> {
        Ok(())
    }

    // FIXME: the way this opens the file each time is really inefficient.  instead, look at the lit-core/lit-logging-src-pluging/log_service.rs does it with a queue and a thread for a better option
    fn handle(&self, _pkg: &str, record: &dyn LogRecord) -> Result<()> {
        if let Some(path) = &self.path {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(path)
                .expect("Could not open file for test FileLoggingPlugin");
            let wrote = writeln!(file, "{}", record.display_args());
            if let Err(e) = wrote {
                eprintln!("Error writing to file: {:?}", e);
            }
        }

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }
}

impl Default for FileLoggingPlugin {
    fn default() -> Self {
        FileLoggingPlugin::new()
    }
}
