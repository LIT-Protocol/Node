use crate::error::{generic_err, io_err, serializer_err, validation_err, Result, Unexpected};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml::{map::Map, Value};

use once_cell::sync::Lazy;
use regex::Regex;

pub struct SimpleToml {
    data: HashMap<String, HashMap<String, String>>,
}

impl SimpleToml {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn apply_params(&mut self, params: &[String]) -> Result<&mut Self> {
        for param in params.iter() {
            let (key, value) =
                param.split_once('=').expect_or_err("Expected param to contain '='")?;

            validate_toml_key(key).map_err(|e| {
                validation_err(
                    e,
                    Some(format!("failed to validate key applying params to TOML: {}", param)),
                )
            })?;

            let (section, key) = key
                .rsplit_once('.')
                .expect_or_err("Expected param key to have at least one section")?;

            self.insert(section.to_string(), key.to_string(), value.to_string());
        }

        Ok(self)
    }

    pub fn data(&self) -> &HashMap<String, HashMap<String, String>> {
        &self.data
    }

    pub fn mut_data(&mut self) -> &mut HashMap<String, HashMap<String, String>> {
        &mut self.data
    }

    pub fn insert(&mut self, section: String, key: String, value: String) -> &mut Self {
        let mut item = if self.data.contains_key(&section) {
            self.data.remove(&section).unwrap()
        } else {
            HashMap::new()
        };

        item.insert(key, value);
        self.data.insert(section, item);
        self
    }

    pub fn get(&self, section: &str, key: &str) -> Option<&String> {
        if let Some(val) = self.data.get(section) {
            return val.get(key);
        }

        None
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn to_map(&self) -> Map<String, Value> {
        let mut map = Map::new();
        for (key, val) in self.data.iter() {
            let mut item = Map::new();
            for (ik, iv) in val.iter() {
                item.insert(ik.clone(), Value::String(iv.clone()));
            }

            map.insert(key.clone(), Value::Table(item));
        }

        map
    }

    pub fn write_file(&self, file: &Path) -> Result<()> {
        let toml_str = toml::to_string(&self.to_map()).map_err(|e| serializer_err(e, None))?;
        fs::write(file, toml_str).map_err(|e| io_err(e, None))?;

        Ok(())
    }
}

impl Default for SimpleToml {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&Path> for SimpleToml {
    type Error = crate::error::Error;

    fn try_from(path: &Path) -> std::result::Result<Self, Self::Error> {
        let data = fs::read(path)
            .map_err(|e| io_err(e, Some(format!("failed to read SimpleToml: {path:?}"))))?;

        let data: HashMap<String, HashMap<String, String>> =
            toml::from_slice(&data[..]).map_err(|e| {
                generic_err(e, Some(format!("failed to parse: {path:?} as SimpleToml")))
            })?;

        Ok(Self { data })
    }
}

pub fn validate_toml_key<S: AsRef<str>>(key: S) -> Result<()> {
    let key = key.as_ref();
    if key.is_empty() {
        return Err(validation_err("TOML key is empty", None));
    }

    if !VALID_TOML_KEY_RE.is_match(key) {
        return Err(validation_err("TOML key is invalid (must only contain a-z0-9.)", None));
    }

    Ok(())
}

pub static VALID_TOML_KEY_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-z0-9.]+").expect("failed to construct regex for label validation")
});
