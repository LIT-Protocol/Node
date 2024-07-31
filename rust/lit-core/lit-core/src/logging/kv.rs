#[allow(unused_imports)]
use std::io::Write;

#[cfg(feature = "logging-core")]
use env_logger::fmt::Formatter;
use log::kv::{Error as KVError, Key, Value, Visitor};
use serde_json::{Map, Value as JsonValue};

use crate::error::Error;

#[cfg(feature = "logging-core")]
pub struct JsonKVVisitor<'a>(pub &'a mut Formatter);

#[cfg(feature = "logging-core")]
impl<'a, 'kvs> Visitor<'kvs> for JsonKVVisitor<'a> {
    fn visit_pair(&mut self, key: Key<'kvs>, value: Value<'kvs>) -> Result<(), KVError> {
        let json = serde_json::to_string(&value).map_err(KVError::boxed)?;

        write!(self.0, " \"{key}\":{json}").map_err(|e| e.into())
    }
}

#[cfg(feature = "logging-core")]
pub struct InlineKVVisitor<'a>(pub &'a mut Formatter);

#[cfg(feature = "logging-core")]
impl<'a, 'kvs> Visitor<'kvs> for InlineKVVisitor<'a> {
    fn visit_pair(&mut self, key: Key<'kvs>, value: Value<'kvs>) -> Result<(), KVError> {
        write!(self.0, " {key}={value}").map_err(|e| e.into())
    }
}

pub struct FieldCollectorKVVisitor<'a>(pub &'a mut Map<String, JsonValue>);

impl<'a, 'kvs> Visitor<'kvs> for FieldCollectorKVVisitor<'a> {
    fn visit_pair(&mut self, key: Key<'kvs>, value: Value<'kvs>) -> Result<(), KVError> {
        let value = if let Some(err) = value.to_borrowed_error() {
            if let Some(err) = err.downcast_ref::<Error>() {
                // Apply the fields in the error
                err.apply_fields(self.0);

                // Serialize the error
                serde_json::to_value(err).map_err(KVError::boxed)?
            } else {
                // Use value instead of err.
                serde_json::to_value(&value).map_err(KVError::boxed)?
            }
        } else {
            serde_json::to_value(&value).map_err(KVError::boxed)?
        };

        self.0.insert(key.to_string(), value);

        Ok(())
    }
}

#[cfg(feature = "tracing-support")]
impl<'a> tracing::field::Visit for FieldCollectorKVVisitor<'a> {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        if let Ok(value) = serde_json::to_value(value) {
            self.0.insert(field.name().to_string(), value);
        }
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        if let Ok(value) = serde_json::to_value(value) {
            self.0.insert(field.name().to_string(), value);
        }
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        if let Ok(value) = serde_json::to_value(value) {
            self.0.insert(field.name().to_string(), value);
        }
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        if let Ok(value) = serde_json::to_value(value) {
            self.0.insert(field.name().to_string(), value);
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if let Ok(value) = serde_json::to_value(value) {
            self.0.insert(field.name().to_string(), value);
        }
    }

    fn record_error(
        &mut self, field: &tracing::field::Field, value: &(dyn std::error::Error + 'static),
    ) {
        if let Some(err) = value.downcast_ref::<Error>() {
            // Apply the fields in the error
            err.apply_fields(self.0);

            // Serialize the error
            if let Ok(value) = serde_json::to_value(err) {
                self.0.insert(field.name().to_string(), value);
            }
        } else {
            // Use value instead of err.
            self.0
                .insert(field.name().to_string(), serde_json::Value::String(format!("{}", value)));
        }
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0.insert(field.name().to_string(), serde_json::Value::String(format!("{:?}", value)));
    }
}
