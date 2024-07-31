use log::{Level, Record};
use serde_json::{Map, Value};
use std::sync::Arc;

#[allow(unused_imports)]
use crate::error::unexpected::Unexpected;
use crate::error::{unexpected_err, Result};
use crate::logging::kv::FieldCollectorKVVisitor;
use crate::logging::plugin::Plugin;

#[allow(dead_code)]
const TRACING_FIELD_MESSAGE: &str = "message";

pub trait LogRecord {
    fn fields(&self) -> &Map<String, Value>;
    fn debug_args(&self) -> &dyn std::fmt::Debug;
    fn display_args(&self) -> &dyn std::fmt::Display;
    fn level(&self) -> Level;
    fn target(&self) -> &str;
    fn name(&self) -> Option<&str>;
}

pub struct DefaultLogRecord<'a> {
    record: &'a Record<'a>,
    fields: Map<String, Value>,
}

impl<'a> DefaultLogRecord<'a> {
    pub fn try_new(
        record: &'a Record, fields: Arc<Option<Map<String, Value>>>,
        plugins: Arc<Vec<Box<dyn Plugin>>>,
    ) -> Result<Self> {
        let mut fields: Map<String, Value> =
            if let Some(fields) = fields.as_ref() { fields.clone() } else { Map::new() };

        let kvs = record.key_values();
        if kvs.count() > 0 {
            kvs.visit(&mut FieldCollectorKVVisitor(&mut fields))
                .map_err(|e| unexpected_err(e, None))?;
        }

        // Apply plugin fields
        for plugin in plugins.iter() {
            plugin.apply_fields(&mut fields)?;
        }

        Ok(Self { record, fields })
    }
}

impl<'a> LogRecord for DefaultLogRecord<'a> {
    fn fields(&self) -> &Map<String, Value> {
        &self.fields
    }

    fn debug_args(&self) -> &dyn std::fmt::Debug {
        self.record.args()
    }

    fn display_args(&self) -> &dyn std::fmt::Display {
        self.record.args()
    }

    fn level(&self) -> Level {
        self.record.level()
    }

    fn target(&self) -> &str {
        self.record.target()
    }

    fn name(&self) -> Option<&str> {
        None
    }
}

#[cfg(feature = "tracing-support")]
pub struct TracingLogRecord<'a> {
    record: &'a tracing::Event<'a>,
    fields: Map<String, Value>,
    message: Option<String>,
}

#[cfg(feature = "tracing-support")]
impl<'a> TracingLogRecord<'a> {
    pub fn try_new(
        record: &'a tracing::Event<'a>, fields: Arc<Option<Map<String, Value>>>,
        plugins: Arc<Vec<Box<dyn Plugin>>>,
    ) -> Result<Self> {
        let mut fields: Map<String, Value> =
            if let Some(fields) = fields.as_ref() { fields.clone() } else { Map::new() };

        record.record(&mut FieldCollectorKVVisitor(&mut fields));

        // Apply plugin fields
        for plugin in plugins.iter() {
            plugin.apply_fields(&mut fields)?;
        }

        let message = if let Some(msg) = fields.remove(TRACING_FIELD_MESSAGE) {
            Some(
                msg.as_str()
                    .expect_or_err(format!(
                        "expected tracing field '{TRACING_FIELD_MESSAGE}' to be str"
                    ))?
                    .to_string(),
            )
        } else {
            None
        };

        Ok(Self { record, fields, message })
    }
}

#[cfg(feature = "tracing-support")]
impl<'a> LogRecord for TracingLogRecord<'a> {
    fn fields(&self) -> &Map<String, Value> {
        &self.fields
    }

    fn debug_args(&self) -> &dyn std::fmt::Debug {
        if let Some(msg) = self.message.as_ref() {
            msg
        } else {
            &""
        }
    }

    fn display_args(&self) -> &dyn std::fmt::Display {
        if let Some(msg) = self.message.as_ref() {
            msg
        } else {
            &""
        }
    }

    fn level(&self) -> Level {
        match *self.record.metadata().level() {
            tracing::Level::TRACE => Level::Trace,
            tracing::Level::DEBUG => Level::Debug,
            tracing::Level::INFO => Level::Info,
            tracing::Level::WARN => Level::Warn,
            tracing::Level::ERROR => Level::Error,
        }
    }

    fn target(&self) -> &str {
        self.record.metadata().target()
    }

    fn name(&self) -> Option<&str> {
        Some(self.record.metadata().name())
    }
}
