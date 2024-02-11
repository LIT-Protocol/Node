use crate::error::{config_err, Result};
use config::{Map, Value};

pub trait MapHelper {
    fn get_value(&self, key: &str) -> Result<&Value>;
    fn get_string(&self, key: &str) -> Result<String>;
    fn get_checked_string(&self, key: &str) -> Result<String>;
    fn get_int(&self, key: &str) -> Result<i64>;
    fn get_float(&self, key: &str) -> Result<f64>;
    fn get_bool(&self, key: &str) -> Result<bool>;
}

impl MapHelper for Map<String, Value> {
    fn get_value(&self, key: &str) -> Result<&Value> {
        self.get(key).ok_or_else(|| {
            config_err(format!("Config key '{}' not found in config map", key), None)
        })
    }

    fn get_string(&self, key: &str) -> Result<String> {
        self.get_value(key)?.clone().into_string().map_err(|e| config_err(e, None))
    }

    fn get_checked_string(&self, key: &str) -> Result<String> {
        self.get_string(key).map_err(|e| config_err(e, None)).and_then(|v| {
            if v.is_empty() {
                Err(config_err(format!("Config key: '{key}' is empty"), None))
            } else {
                Ok(v)
            }
        })
    }

    fn get_int(&self, key: &str) -> Result<i64> {
        self.get_value(key)?.clone().into_int().map_err(|e| config_err(e, None))
    }

    fn get_float(&self, key: &str) -> Result<f64> {
        self.get_value(key)?.clone().into_float().map_err(|e| config_err(e, None))
    }

    fn get_bool(&self, key: &str) -> Result<bool> {
        self.get_value(key)?.clone().into_bool().map_err(|e| config_err(e, None))
    }
}
