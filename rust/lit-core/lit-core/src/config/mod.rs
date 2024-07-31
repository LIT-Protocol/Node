use std::collections::HashMap;
use std::env;
#[cfg(feature = "cli")]
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::str::FromStr;

use config::{Config, Environment, File, Map, Value};
#[cfg(feature = "ipfs")]
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient, TryFromUri};
#[cfg(feature = "pinata")]
use pinata_sdk::PinataApi;
#[cfg(feature = "cli")]
use termion::input::TermRead;

use crate::config::envs::LitEnv;
pub use crate::config::reloadable::ReloadableLitConfig;
use crate::error::unexpected::Unexpected;
#[allow(unused_imports)]
use crate::error::{config_err, io_err, ipfs_err, unexpected_err, validation_err, Result};
use crate::utils::binary::remove_0x_prefix;
use crate::utils::toml::SimpleToml;

pub mod envs;
pub mod helper;
pub mod reloadable;

pub const CFG_DIR_DEFAULT: &str = "/etc/lit";
pub const CFG_DIR_GUEST_INIT: &str = "/var/lit/os/guest-init/etc";
pub const CFG_NAME: &str = "config";
pub const CFG_EXT: &str = "toml";

pub const CFG_ADMIN_OVERRIDE_NAME: &str = "config_admin_override";
pub const CFG_RESTORE_OVERRIDE_NAME: &str = "config_restore_override";

pub const CFG_KEY_IPFS_API: &str = "ipfs.api";
pub const CFG_KEY_IPFS_GATEWAY: &str = "ipfs.gateway";
pub const CFG_KEY_IPFS_REMOTES: &str = "ipfs.remotes";

pub const CFG_KEY_SUBNET_ID: &str = "subnet.id";

// Keys that DO NOT relate to lit_core and yet are needed to be defined globally (for safety).
pub const CFG_KEY_GUEST_ACTIVE: &str = "guest.active";

pub const DEFAULT_SECTION_KEY: &str = "default";

pub const DEFAULT_IPFS_API: &str = "http://litos-host:5001";
pub const DEFAULT_IPFS_GATEWAY: &str = "http://litos-host:8080";

pub const ENV_LIT_CONFIG_FILE: &str = "LIT_CONFIG_FILE";

#[derive(Debug, Clone)]
pub struct LitConfigBuilder {
    key: Option<String>,
    home_dir: Option<String>,
    cfg_dir_default: String,
    cfg_dir_guest_init: String,
    defaults: Map<String, Value>,
    overrides: Map<String, Value>,
}

impl LitConfigBuilder {
    #[allow(dead_code)]
    pub fn new_with_paths(
        key: Option<String>, home_dir: Option<String>, cfg_dir_default: &str,
        cfg_dir_guest_init: &str,
    ) -> Self {
        Self {
            key,
            home_dir,
            cfg_dir_default: cfg_dir_default.to_string(),
            cfg_dir_guest_init: cfg_dir_guest_init.to_string(),
            ..Self::default()
        }
    }

    pub fn set_default<K, T>(mut self, key: K, value: T) -> Self
    where
        K: Into<String>,
        T: Into<Value>,
    {
        self.defaults.entry(key.into()).or_insert_with(|| value.into());
        self
    }

    pub fn force_set_default<K, T>(mut self, key: K, value: T) -> Self
    where
        K: Into<String>,
        T: Into<Value>,
    {
        let key: String = key.into();
        self.defaults.insert(key, value.into());
        self
    }

    pub fn set_override<K, T>(mut self, key: K, value: T) -> Self
    where
        K: Into<String>,
        T: Into<Value>,
    {
        self.overrides.entry(key.into()).or_insert_with(|| value.into());
        self
    }

    pub fn force_set_override<K, T>(mut self, key: K, value: T) -> Self
    where
        K: Into<String>,
        T: Into<Value>,
    {
        let key: String = key.into();
        self.overrides.insert(key, value.into());
        self
    }

    pub fn get_section_key<K>(&self, key: K) -> String
    where
        K: Into<String>,
    {
        let key: String = key.into();
        format!("{}.{}", self.key.as_ref().unwrap_or(&DEFAULT_SECTION_KEY.to_string()), key)
    }

    pub fn set_section_default<K, T>(self, key: K, value: T) -> Self
    where
        K: Into<String>,
        T: Into<Value>,
    {
        let key = self.get_section_key(key);
        self.set_default::<String, T>(key, value)
    }

    pub fn set_section_override<K, T>(self, key: K, value: T) -> Self
    where
        K: Into<String>,
        T: Into<Value>,
    {
        let key = self.get_section_key(key.into());
        self.set_override::<String, T>(key, value)
    }

    pub fn build(self) -> Result<LitConfig> {
        LitConfig::build(self)
    }

    pub fn build_cloned(&self) -> Result<LitConfig> {
        LitConfig::build(self.clone())
    }

    // Accessors
    pub fn key(&self) -> Option<String> {
        self.key.clone()
    }

    pub fn set_key(mut self, key: Option<String>) -> Self {
        self.key = key;
        self
    }
}

impl Default for LitConfigBuilder {
    fn default() -> Self {
        Self {
            key: None,
            home_dir: env::var("HOME").ok(),
            cfg_dir_default: CFG_DIR_DEFAULT.to_string(),
            cfg_dir_guest_init: CFG_DIR_GUEST_INIT.to_string(),
            defaults: Map::new(),
            overrides: Map::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LitConfig {
    config: Config,
    path: String,
    env: LitEnv,
    #[cfg(feature = "ipfs")]
    ipfs_remotes: Option<Vec<String>>,
    key: String,
}

impl LitConfig {
    fn build(lit_builder: LitConfigBuilder) -> Result<Self> {
        let default_key = DEFAULT_SECTION_KEY.to_string();
        let key = lit_builder.key.as_ref().unwrap_or(&default_key);
        let home_dir = lit_builder.home_dir.clone();
        let cfg_dir_default = lit_builder.cfg_dir_default.clone();
        let cfg_dir_guest_init = lit_builder.cfg_dir_guest_init.clone();

        let full_cfg_name = format!("{CFG_NAME}.{CFG_EXT}");
        let full_cfg_path = format!("{cfg_dir_default}/{full_cfg_name}");

        // 1: /etc/lit/config.toml
        // 2: /etc/lit/{key}.config.toml
        let mut builder = Config::builder()
            .add_source(File::with_name(&format!("{cfg_dir_default}/{CFG_NAME}")).required(false))
            .add_source(
                File::with_name(&format!("{cfg_dir_default}/{key}.{CFG_NAME}")).required(false),
            );

        // User defined config (these files have been verified).
        // 3. /var/lit/os/guest-init/etc/config.toml
        // 4. /var/lit/os/guest-init/etc/{key}.config.toml
        builder = builder
            .add_source(
                File::with_name(&format!("{cfg_dir_guest_init}/{CFG_NAME}")).required(false),
            )
            .add_source(
                File::with_name(&format!("{cfg_dir_guest_init}/{key}.{CFG_NAME}")).required(false),
            );

        // 5. ./config.toml
        builder = builder.add_source(File::with_name(&format!("./{CFG_NAME}")).required(false));

        // 6. ~/.lit/config.toml
        // 7. ~/.lit/{key}.config.toml
        if let Some(home_dir) = home_dir.as_ref() {
            builder = builder
                .add_source(File::with_name(&format!("{home_dir}/.lit/{CFG_NAME}")).required(false))
                .add_source(
                    File::with_name(&format!("{home_dir}/.lit/{key}.{CFG_NAME}")).required(false),
                );
        }

        // 8. config var defined by env var CONFIG_FILE
        if let Ok(config_file_name) = env::var(ENV_LIT_CONFIG_FILE) {
            builder = builder.add_source(File::with_name(&config_file_name).required(false));
        }

        // 9. ./config_admin_override.toml
        // 10. ./config_restore_override.toml
        builder = builder
            .add_source(File::with_name(&format!("./{CFG_ADMIN_OVERRIDE_NAME}")).required(false));
        builder = builder
            .add_source(File::with_name(&format!("./{CFG_RESTORE_OVERRIDE_NAME}")).required(false));

        builder = builder
            // 11. Environment
            .add_source(
                Environment::with_prefix("LIT")
                    .prefix_separator("_")
                    .keep_prefix(false)
                    //.try_parsing(true)
                    .separator("__")
                    .list_separator(" "),
            )
            // Defaults
            .set_default(CFG_KEY_IPFS_API, DEFAULT_IPFS_API)
            .map_err(|e| config_err(e, None))?
            .set_default(CFG_KEY_IPFS_GATEWAY, DEFAULT_IPFS_GATEWAY)
            .map_err(|e| config_err(e, None))?;

        for (k, v) in lit_builder.defaults.iter() {
            builder = builder.set_default(k, v.clone()).map_err(|e| config_err(e, None))?;
        }
        for (k, v) in lit_builder.overrides.iter() {
            builder = builder.set_override(k, v.clone()).map_err(|e| config_err(e, None))?;
        }

        let cfg = builder.build().map_err(|e| config_err(e, None))?;

        // Map env
        let env_str = cfg.get_string("lit.env").map_err(|e| config_err(e, None))?;
        let env = LitEnv::from_str(env_str.as_str()).expect_or_err(
            format!("failed to map env: {env_str} (valid: dev, staging, prod)").as_str(),
        )?;

        // Load ipfs remotes (from config)
        #[cfg(feature = "ipfs")]
        let mut ipfs_remotes: Option<Vec<String>> = None;
        #[cfg(feature = "ipfs")]
        if let Ok(values) = cfg.get_array(CFG_KEY_IPFS_REMOTES) {
            let mut remotes: Vec<String> = Vec::new();
            for i in values {
                let val = i.to_string();
                if !val.is_empty() && val != "none" {
                    remotes.push(val);
                }
            }
            ipfs_remotes = Some(remotes);
        }

        Ok(Self {
            config: cfg,
            path: full_cfg_path,
            env,
            #[cfg(feature = "ipfs")]
            ipfs_remotes,
            key: key.to_string(),
        })
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn config_file(&self) -> String {
        self.path.clone()
    }

    // Env

    #[inline]
    pub fn env(&self) -> &LitEnv {
        &self.env
    }

    #[inline]
    pub fn is_dev(&self) -> bool {
        self.env == LitEnv::Dev
    }

    #[inline]
    pub fn is_staging(&self) -> bool {
        self.env == LitEnv::Staging
    }

    #[inline]
    pub fn is_prod(&self) -> bool {
        self.env == LitEnv::Prod
    }

    // Accessors
    #[inline]
    pub fn get_string(&self, key: &str) -> Result<String> {
        self.config.get_string(key).map_err(|e| config_err(e, None))
    }

    #[inline]
    pub fn get_checked_string(&self, key: &str) -> Result<String> {
        self.config.get_string(key).map_err(|e| config_err(e, None)).and_then(|v| {
            if v.is_empty() {
                Err(config_err(format!("Config key: '{key}' is empty"), None))
            } else {
                Ok(v)
            }
        })
    }

    #[inline]
    pub fn get_int(&self, key: &str) -> Result<i64> {
        self.config.get_int(key).map_err(|e| config_err(e, None))
    }

    #[inline]
    pub fn get_float(&self, key: &str) -> Result<f64> {
        self.config.get_float(key).map_err(|e| config_err(e, None))
    }

    #[inline]
    pub fn get_bool(&self, key: &str) -> Result<bool> {
        self.config.get_bool(key).map_err(|e| config_err(e, None))
    }

    #[inline]
    pub fn get_array(&self, key: &str) -> Result<Vec<Value>> {
        self.config.get_array(key).map_err(|e| config_err(e, None))
    }

    // Section

    #[inline]
    pub fn key(&self) -> &String {
        &self.key
    }

    #[inline]
    pub fn get_section_key(&self, key: &str) -> String {
        format!("{}.{}", self.key, key)
    }

    #[inline]
    pub fn get_section_string(&self, key: &str) -> Result<String> {
        self.get_string(self.get_section_key(key).as_str())
    }

    #[inline]
    pub fn get_section_checked_string(&self, key: &str) -> Result<String> {
        self.get_checked_string(self.get_section_key(key).as_str())
    }

    #[inline]
    pub fn get_section_int(&self, key: &str) -> Result<i64> {
        self.get_int(self.get_section_key(key).as_str())
    }

    #[inline]
    pub fn get_section_float(&self, key: &str) -> Result<f64> {
        self.get_float(self.get_section_key(key).as_str())
    }

    #[inline]
    pub fn get_section_bool(&self, key: &str) -> Result<bool> {
        self.get_bool(self.get_section_key(key).as_str())
    }

    // Admin
    #[inline]
    #[cfg(not(feature = "cli"))]
    pub fn admin_key(&self) -> Result<String> {
        self.get_string("admin.key").map(remove_0x_prefix)
    }

    // Admin
    #[inline]
    #[cfg(feature = "cli")]
    pub fn admin_key(&self) -> Result<String> {
        match self.get_string("admin.key") {
            Ok(val) => Ok(remove_0x_prefix(val)),
            Err(_) => read_sensitive_stdin("admin key", Some(64), true),
        }
    }

    // IPFS
    #[cfg(feature = "ipfs")]
    #[inline]
    pub fn ipfs_api(&self) -> String {
        self.config.get_string(CFG_KEY_IPFS_API).unwrap_or(DEFAULT_IPFS_API.to_string())
    }

    #[cfg(feature = "ipfs")]
    #[inline]
    pub fn ipfs_gateway(&self) -> String {
        self.config.get_string(CFG_KEY_IPFS_GATEWAY).unwrap_or(DEFAULT_IPFS_GATEWAY.to_string())
    }

    #[cfg(feature = "ipfs")]
    #[inline]
    pub fn ipfs_client(&self) -> IpfsClient {
        IpfsClient::from_str(self.ipfs_api().as_str()).expect("failed to construct IPFS client")
    }

    #[cfg(feature = "ipfs")]
    #[inline]
    pub async fn ipfs_remotes(&mut self) -> Result<Vec<String>> {
        if let Some(remotes) = self.ipfs_remotes.as_ref() {
            Ok(remotes.clone())
        } else {
            let client = self.ipfs_client();
            let res = client.pin_remote_service_ls(false).await.map_err(|e| ipfs_err(e, None))?;

            let mut remotes: Vec<String> = Vec::new();
            for i in res.remote_services {
                if let Some(val) = i.service {
                    remotes.push(val);
                }
            }

            self.ipfs_remotes = Some(remotes.clone());

            Ok(remotes)
        }
    }

    #[cfg(feature = "pinata")]
    #[inline]
    pub fn pinata_client(&self) -> Result<PinataApi> {
        PinataApi::new(
            self.get_checked_string("pinata.api_key")?,
            self.get_checked_string("pinata.api_secret")?,
        )
        .map_err(|e| unexpected_err(e, None))
    }

    // Subnet
    #[inline]
    pub fn subnet_id(&self) -> Result<String> {
        self.get_string(CFG_KEY_SUBNET_ID).map(|v| v.to_lowercase())
    }

    // LitOS
    #[inline]
    pub fn litos_guest(&self) -> Result<bool> {
        self.get_bool(CFG_KEY_GUEST_ACTIVE)
    }

    #[inline]
    pub fn is_litos_guest(&self) -> bool {
        matches!(self.litos_guest(), Ok(true))
    }

    // Local config
    pub fn save_local_config(
        &self, config_file: &str, data: &HashMap<String, String>,
    ) -> Result<String> {
        let mut config_toml = SimpleToml::new();

        for (key, value) in data.iter() {
            match key.rsplit_once('.') {
                Some((section, key)) => {
                    config_toml
                        .mut_data()
                        .entry(section.to_string())
                        .and_modify(|m| {
                            let _ = m.insert(key.to_string(), value.to_string());
                        })
                        .or_insert_with(|| {
                            let mut section: HashMap<String, String> = HashMap::new();
                            section.insert(key.to_string(), value.to_string());
                            section
                        });
                }
                None => {
                    return Err(validation_err(
                        format!("Invalid data supplied: {key} (does not contain any '.')"),
                        None,
                    ))
                }
            }
        }

        self.save_toml_as_local_config(config_file, &config_toml)
    }

    // Local config
    pub fn save_toml_as_local_config(
        &self, config_file: &str, toml: &SimpleToml,
    ) -> Result<String> {
        let config_file = PathBuf::from(format!("./{}.{CFG_EXT}", config_file));
        toml.write_file(config_file.as_path())?;

        Ok(config_file.display().to_string())
    }
}

#[inline]
#[cfg(feature = "cli")]
pub fn read_sensitive_stdin<S>(label: S, expected_len: Option<usize>, trim: bool) -> Result<String>
where
    S: AsRef<str>,
{
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let stdin = stdin();
    let mut stdin = stdin.lock();

    loop {
        stdout
            .write_fmt(format_args!("Enter {}: ", label.as_ref()))
            .map_err(|e| io_err(e, None))?;
        stdout.flush().map_err(|e| io_err(e, None))?;

        match stdin.read_passwd(&mut stdout) {
            Ok(Some(mut value)) => {
                if trim {
                    value = remove_0x_prefix(value);
                }

                if let Some(len) = expected_len {
                    if value.len() != len {
                        stdout
                            .write_all(b"\nERROR: Value invalid length.\n")
                            .map_err(|e| io_err(e, None))?;
                        continue;
                    }
                }

                stdout.write_all(b"\n").map_err(|e| io_err(e, None))?;

                return Ok(value);
            }
            Ok(None) => {
                return Err(unexpected_err("no input", None));
            }
            Err(e) => {
                return Err(unexpected_err(e, None));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::PathBuf;

    use crate::config::envs::LitEnv;
    #[allow(unused_imports)]
    use crate::config::{LitConfigBuilder, DEFAULT_IPFS_API, DEFAULT_IPFS_GATEWAY};

    const RESOURCES_TEST_DIR: &str = "resources/test/config";

    #[test]
    fn failed_load_test() {
        let system_path = get_test_path("non-existant");
        let system_path_str = system_path.to_str().unwrap();
        let res = LitConfigBuilder::new_with_paths(
            None,
            Some("/tmp/fake/nope".to_string()),
            system_path_str,
            "/tmp/fake/nope",
        )
        .build();

        assert!(res.is_err());
    }

    #[test]
    fn default_load_test() {
        let system_path = get_test_path("default");
        let system_path_str = system_path.to_str().unwrap();

        let cfg = LitConfigBuilder::new_with_paths(
            None,
            Some("/tmp/fake/nope".to_string()),
            system_path_str,
            "/tmp/fake/nope",
        )
        .build()
        .expect("failed to load config");

        assert_eq!(cfg.env().clone(), LitEnv::Dev);
        assert_eq!(cfg.is_dev(), true);
        assert_eq!(cfg.config().get_string("simple.dummy").unwrap(), "default".to_string());
        assert_eq!(cfg.config().get_bool("simple.other").unwrap(), true);
        #[cfg(feature = "ipfs")]
        assert_eq!(cfg.ipfs_api(), DEFAULT_IPFS_API.to_string());
        #[cfg(feature = "ipfs")]
        assert_eq!(cfg.ipfs_gateway(), DEFAULT_IPFS_GATEWAY.to_string());
        #[cfg(feature = "ipfs")]
        assert!(cfg.ipfs_remotes.is_none());
    }

    #[test]
    fn keyed_load_test() {
        let system_path = get_test_path("default");
        let system_path_str = system_path.to_str().unwrap();

        let cfg = LitConfigBuilder::new_with_paths(
            Some("keyed".to_string()),
            Some("/tmp/fake/nope".to_string()),
            system_path_str,
            "/tmp/fake/nope",
        )
        .build()
        .expect("failed to load config");

        assert_eq!(cfg.env().clone(), LitEnv::Dev);
        assert_eq!(cfg.is_dev(), true);
        assert_eq!(cfg.config().get_string("simple.dummy").unwrap(), "keyed".to_string());
        assert_eq!(cfg.config().get_bool("simple.other").unwrap(), true);
        #[cfg(feature = "ipfs")]
        assert_eq!(cfg.ipfs_api(), "http://litos-host-other:2199".to_string());
        #[cfg(feature = "ipfs")]
        assert_eq!(cfg.ipfs_gateway(), "http://litos-host-other:2199".to_string());
        #[cfg(feature = "ipfs")]
        assert!(cfg.ipfs_remotes.is_some());
    }

    #[test]
    fn home_load_test() {
        let system_path = get_test_path("default");
        let system_path_str = system_path.to_str().unwrap();
        let home_path = get_test_path("home");
        let home_path_str = home_path.to_str().unwrap();

        let cfg = LitConfigBuilder::new_with_paths(
            Some("keyed".to_string()),
            Some(home_path_str.to_string()),
            system_path_str,
            "/tmp/fake/nope",
        )
        .build()
        .expect("failed to load config");

        assert_eq!(cfg.env().clone(), LitEnv::Dev);
        assert_eq!(cfg.is_dev(), true);
        assert_eq!(cfg.config().get_string("simple.dummy").unwrap(), "home".to_string());
        assert_eq!(cfg.config().get_bool("simple.other").unwrap(), true);
        #[cfg(feature = "ipfs")]
        assert_eq!(cfg.ipfs_api(), "http://litos-host-home:2199".to_string());
        #[cfg(feature = "ipfs")]
        assert_eq!(cfg.ipfs_gateway(), "http://litos-host-home:2199".to_string());
        #[cfg(feature = "ipfs")]
        assert!(cfg.ipfs_remotes.is_some());
    }

    #[test]
    fn guest_init_load_test() {
        let system_path = get_test_path("guest_init/system");
        let system_path_str = system_path.to_str().unwrap();

        let init_path = get_test_path("guest_init/injected");
        let init_path_str = init_path.to_str().unwrap();

        let cfg = LitConfigBuilder::new_with_paths(
            Some("keyed".to_string()),
            Some("/tmp/fake/nope".to_string()),
            system_path_str,
            init_path_str,
        )
        .build()
        .expect("failed to load config");

        assert_eq!(cfg.env().clone(), LitEnv::Dev);
        assert_eq!(cfg.is_dev(), true);
        assert_eq!(cfg.config().get_string("simple.dummy").unwrap(), "system".to_string());
        assert_eq!(cfg.config().get_bool("simple.other").unwrap(), false);
        assert_eq!(cfg.config().get_string("top.one").unwrap(), "1".to_string());
        assert_eq!(cfg.config().get_string("top.two").unwrap(), "2".to_string());
        assert_eq!(cfg.config().get_string("top.three").unwrap(), "3".to_string());
        assert_eq!(cfg.config().get_string("top.four").unwrap(), "4".to_string());
    }

    #[test]
    fn env_test() {
        let system_path = get_test_path("guest_init/system");
        let system_path_str = system_path.to_str().unwrap();

        env::set_var("LIT_TEST_VAL", "yes");
        env::set_var("LIT_LIT__SECTION__VAL", "yes");

        let cfg = LitConfigBuilder::new_with_paths(
            Some("keyed".to_string()),
            Some("/tmp/fake/nope".to_string()),
            system_path_str,
            "/tmp/fake/nope",
        )
        .build()
        .expect("failed to load config");

        assert_eq!(cfg.config().get_string("test_val").unwrap(), "yes".to_string());
        assert_eq!(cfg.config().get_string("lit.section.val").unwrap(), "yes".to_string());
    }

    #[test]
    fn defaulting_test() {
        let system_path = get_test_path("default");
        let system_path_str = system_path.to_str().unwrap();

        let mut builder = LitConfigBuilder::new_with_paths(
            Some("oh.yes".to_string()),
            Some("/tmp/fake/nope".to_string()),
            system_path_str,
            "/tmp/fake/nope",
        );

        builder = builder.set_default("simple.dummy", "defaulted").set_default("org.id", "1234");

        let cfg = builder.build().expect("failed to load config");

        assert_eq!(cfg.config().get_string("simple.dummy").unwrap(), "default".to_string());
        assert_eq!(cfg.config().get_string("org.id").unwrap(), "1234".to_string());
    }

    #[test]
    fn override_test() {
        let system_path = get_test_path("default");
        let system_path_str = system_path.to_str().unwrap();

        let mut builder = LitConfigBuilder::new_with_paths(
            Some("oh.yes".to_string()),
            Some("/tmp/fake/nope".to_string()),
            system_path_str,
            "/tmp/fake/nope",
        );

        builder = builder.set_override("simple.dummy", "override").set_override("org.id", "1234");

        let cfg = builder.build().expect("failed to load config");

        assert_eq!(cfg.config().get_string("simple.dummy").unwrap(), "override".to_string());
        assert_eq!(cfg.config().get_string("org.id").unwrap(), "1234".to_string());
    }

    #[test]
    fn section_get_test() {
        let system_path = get_test_path("default");
        let system_path_str = system_path.to_str().unwrap();

        let cfg = LitConfigBuilder::new_with_paths(
            Some("oh.yes".to_string()),
            Some("/tmp/fake/nope".to_string()),
            system_path_str,
            "/tmp/fake/nope",
        )
        .build()
        .expect("failed to load config");

        assert_eq!(cfg.get_section_string("moo").unwrap(), "cow".to_string());
    }

    #[test]
    fn section_defaulting_test() {
        let system_path = get_test_path("default");
        let system_path_str = system_path.to_str().unwrap();

        let mut builder = LitConfigBuilder::new_with_paths(
            Some("oh.yes".to_string()),
            Some("/tmp/fake/nope".to_string()),
            system_path_str,
            "/tmp/fake/nope",
        );

        builder = builder.set_section_default("moo", "duck").set_section_default("bark", "dog");

        let cfg = builder.build().expect("failed to load config");

        assert_eq!(cfg.get_section_string("moo").unwrap(), "cow".to_string());
        assert_eq!(cfg.get_section_string("bark").unwrap(), "dog".to_string());
    }

    #[test]
    fn section_override_test() {
        let system_path = get_test_path("default");
        let system_path_str = system_path.to_str().unwrap();

        let mut builder = LitConfigBuilder::new_with_paths(
            Some("oh.yes".to_string()),
            Some("/tmp/fake/nope".to_string()),
            system_path_str,
            "/tmp/fake/nope",
        );

        builder = builder.set_section_override("moo", "duck").set_section_override("bark", "dog");

        let cfg = builder.build().expect("failed to load config");

        assert_eq!(cfg.get_section_string("moo").unwrap(), "duck".to_string());
        assert_eq!(cfg.get_section_string("bark").unwrap(), "dog".to_string());
    }

    // Util
    fn get_test_path(path: &str) -> PathBuf {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push(RESOURCES_TEST_DIR);
        test_path.push(path);
        test_path
    }
}
