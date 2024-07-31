use serde_json::{Map, Value};

use lit_core::config::LitConfig;

use crate::error::Result;
use crate::logging::types::LogRecord;

pub trait Plugin: Sync + Send {
    fn init(&mut self, cfg: &LitConfig) -> Result<()>;
    fn apply_fields(&self, fields: &mut Map<String, Value>) -> Result<()>;
    fn handle(&self, pkg: &str, record: &dyn LogRecord) -> Result<()>;
    fn flush(&self) -> Result<()>;
}
