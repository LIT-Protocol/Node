use async_std::path::PathBuf;
use std::collections::HashMap;
use std::path::Path;
use url::Url;

use ethers::types::H160;
use lit_api_core::config::{http_section_key, https_section_key, LitApiConfig, CFG_KEY_IDENT};
use lit_api_core::config::{
    CFG_KEY_DOMAIN, CFG_KEY_ENABLED, CFG_KEY_PORT, CFG_KEY_REDIRECT_TO_HTTPS,
    CFG_KEY_TLS_AUTO_ENABLED,
};
use lit_blockchain::config::{
    LitBlockchainConfig, CFG_KEY_BLOCKCHAIN_CHAIN_ID, CFG_KEY_BLOCKCHAIN_CHAIN_NAME,
    CFG_KEY_BLOCKCHAIN_WALLET_DEFAULT_PRIVATE_KEY,
};
use lit_blockchain::resolver::rpc::{RpcHealthcheckPoller, ENDPOINT_MANAGER};
use lit_core::config::{LitConfig, LitConfigBuilder, ReloadableLitConfig};
use lit_logging::config::LitLoggingConfig;

use crate::error::{parser_err, validation_err, Result};

pub mod chain;

pub const CFG_SECTION_KEY: &str = "node";

// NB: Before adding keys here ensure they don't conflict with LitApiConfig
// - port, address, ident e.t.c. are all reserved.
pub static CFG_KEY_RPC_URL: &str = "rpc_url";
pub static CFG_KEY_STAKER_ADDRESS: &str = "staker_address";
pub static CFG_KEY_COMS_KEYS_SENDER_PRIVKEY: &str = "coms_keys_sender_privkey";
pub static CFG_KEY_COMS_KEYS_RECEIVER_PRIVKEY: &str = "coms_keys_receiver_privkey";
pub static CFG_KEY_ADMIN_ADDRESS: &str = "admin_address";
pub static CFG_KEY_ENABLE_PROXIED_HTTP_CLIENT: &str = "enable_proxied_http_client";
pub static CFG_KEY_ENABLE_RATE_LIMITING: &str = "enable_rate_limiting";
pub static CFG_KEY_ENABLE_RATE_LIMITING_ALLOCATION: &str = "enable_rate_limiting_allocation";
pub static CFG_KEY_ENABLE_ACTIONS_ALLOWLIST: &str = "enable_actions_allowlist";
pub static CFG_KEY_ENABLE_EPOCH_TRANSITIONS: &str = "enable_epoch_transitions";
pub static CFG_KEY_ECDSA_ROUND_TIMEOUT: &str = "ecdsa_round_timeout";
pub static CFG_KEY_WEBAUTHN_ALLOWED_ORIGINS: &str = "webauthn_allowed_origins";
pub static CFG_KEY_CHAIN_POLLING_INTERVAL_MS: &str = "chain_polling_interval";
pub static CFG_KEY_HTTP_CLIENT_TIMEOUT: &str = "http_client_timeout";
pub static CFG_KEY_ENTER_RESTORE_STATE: &str = "enter_restore_state";
pub static CFG_KEY_BLS_KEY_BLINDER: &str = "bls_key_blinder";
pub static CFG_KEY_ECDSA_KEY_BLINDER: &str = "ecdsa_key_blinder";
pub static CFG_KEY_ENABLE_SIWE_VALIDATION: &str = "enable_siwe_validation";
pub static CFG_KEY_RESTORE_LOG_INTERVAL_MS: &str = "restore_log_interval";
pub static CFG_KEY_ACTIONS_SOCKET: &str = "actions_socket";
pub static CFG_KEY_HEALTH_POLL_INTERVAL_MS: &str = "health_poll_interval";

// Defaults
pub static CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT: i64 = 30000;
pub static CFG_KEY_ECDSA_ROUND_TIMEOUT_MS_DEFAULT: i64 = 30000;
pub static CFG_KEY_RESTORE_LOG_INTERVAL_MS_DEFAULT: i64 = 1000 * 60 * 10;
pub static CFG_KEY_ACTIONS_SOCKET_DEFAULT: &str = "/tmp/lit_actions.sock";

static REQUIRED_CFG_KEYS: [&str; 8] = [
    CFG_KEY_STAKER_ADDRESS,
    CFG_KEY_COMS_KEYS_SENDER_PRIVKEY,
    CFG_KEY_COMS_KEYS_RECEIVER_PRIVKEY,
    CFG_KEY_ADMIN_ADDRESS,
    CFG_KEY_ENABLE_RATE_LIMITING,
    CFG_KEY_ENABLE_ACTIONS_ALLOWLIST,
    CFG_KEY_ENABLE_EPOCH_TRANSITIONS,
    CFG_KEY_DOMAIN,
];

static USER_EDITABLE_KEYS: [&str; 16] = [
    CFG_KEY_RPC_URL,
    CFG_KEY_ADMIN_ADDRESS,
    CFG_KEY_STAKER_ADDRESS,
    CFG_KEY_ENABLE_PROXIED_HTTP_CLIENT,
    CFG_KEY_ENABLE_RATE_LIMITING,
    CFG_KEY_ENABLE_RATE_LIMITING_ALLOCATION,
    CFG_KEY_ENABLE_ACTIONS_ALLOWLIST,
    CFG_KEY_ENABLE_EPOCH_TRANSITIONS,
    CFG_KEY_ECDSA_ROUND_TIMEOUT,
    CFG_KEY_WEBAUTHN_ALLOWED_ORIGINS,
    CFG_KEY_CHAIN_POLLING_INTERVAL_MS,
    CFG_KEY_ENTER_RESTORE_STATE,
    CFG_KEY_BLS_KEY_BLINDER,
    CFG_KEY_ECDSA_KEY_BLINDER,
    CFG_KEY_ENABLE_SIWE_VALIDATION,
    CFG_KEY_HEALTH_POLL_INTERVAL_MS,
];

static USER_EDITABLE_KEYS_IN_SECTIONS: [&str; 3] = [
    CFG_KEY_BLOCKCHAIN_WALLET_DEFAULT_PRIVATE_KEY,
    CFG_KEY_BLOCKCHAIN_CHAIN_ID,
    CFG_KEY_BLOCKCHAIN_CHAIN_NAME,
];

pub trait LitNodeConfig {
    fn try_new() -> Result<LitConfig>;
    fn from_builder(builder: LitConfigBuilder) -> Result<LitConfig>;
    fn verify(&self) -> Result<()>;
    fn export_user_editable(&self) -> Result<HashMap<String, String>>;
    fn verify_user_editable(&self, data: &HashMap<String, String>) -> Result<()>;

    // Accessors
    fn external_port(&self) -> Result<u16>;
    fn external_addr(&self) -> Result<String>;
    fn http_prefix_when_talking_to_other_nodes(&self) -> String;
    fn internal_port(&self) -> Result<u16>;
    fn rpc_url(&self) -> Result<String>;
    fn staker_address(&self) -> Result<String>;
    fn coms_keys_sender_privkey(&self) -> Result<String>;
    fn coms_keys_receiver_privkey(&self) -> Result<String>;
    fn admin_address(&self) -> Result<H160>;
    fn webauthn_allowed_origins(&self) -> Result<Vec<Url>>;
    fn http_client_timeout(&self) -> Result<u64>;
    fn actions_socket(&self) -> Result<std::path::PathBuf>;

    // Feature flag bool accessors
    fn enable_proxied_http_client(&self) -> Result<bool>;
    fn enable_rate_limiting(&self) -> Result<bool>;
    fn enable_rate_limiting_allocation(&self) -> Result<bool>;
    fn enable_actions_allowlist(&self) -> Result<bool>;
    fn enable_epoch_transitions(&self) -> Result<bool>;
    fn enable_siwe_validation(&self) -> Result<bool>;

    // communications parameters for ECDSA rounds
    fn ecdsa_round_timeout(&self) -> Result<i64>;
    fn chain_polling_interval_ms(&self) -> Result<i64>;

    // restore state parameters
    fn enter_restore_state(&self) -> Result<bool>;
    fn bls_key_blinder(&self) -> Result<String>;
    fn ecdsa_key_blinder(&self) -> Result<String>;
    fn restore_log_interval(&self) -> Result<i64>;

    // endpoint polling and healthcheck
    fn rpc_health_poll_interval(&self) -> Result<i64>;
}

impl LitNodeConfig for LitConfig {
    fn try_new() -> Result<Self> {
        <LitConfig as LitNodeConfig>::from_builder(LitConfigBuilder::default())
    }

    fn from_builder(mut builder: LitConfigBuilder) -> Result<LitConfig> {
        // Set defaults
        builder = builder
            .set_key(Some(CFG_SECTION_KEY.into()))
            .set_section_default(CFG_KEY_IDENT, "Lit Node")
            // See LitApiConfig for other API options.
            .set_section_default(http_section_key(CFG_KEY_REDIRECT_TO_HTTPS), "true")
            .set_section_default(http_section_key(CFG_KEY_PORT), "8080")
            .set_section_default(http_section_key(CFG_KEY_ENABLED), "true")
            .set_section_default(https_section_key(CFG_KEY_PORT), "8443")
            .set_section_default(https_section_key(CFG_KEY_ENABLED), "false")
            .set_section_default(CFG_KEY_TLS_AUTO_ENABLED, "false")
            .set_section_default(
                CFG_KEY_ECDSA_ROUND_TIMEOUT,
                CFG_KEY_ECDSA_ROUND_TIMEOUT_MS_DEFAULT.to_string(),
            )
            .set_section_default(CFG_KEY_HTTP_CLIENT_TIMEOUT, "30")
            .set_section_default(CFG_KEY_ENABLE_PROXIED_HTTP_CLIENT, "false")
            .set_section_default(CFG_KEY_WEBAUTHN_ALLOWED_ORIGINS, "http://*/,https://*/")
            .set_section_default(
                CFG_KEY_CHAIN_POLLING_INTERVAL_MS,
                CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT.to_string(),
            )
            .set_section_default(
                CFG_KEY_RESTORE_LOG_INTERVAL_MS,
                CFG_KEY_RESTORE_LOG_INTERVAL_MS_DEFAULT.to_string(),
            )
            .set_section_default(CFG_KEY_ENTER_RESTORE_STATE, "false")
            .set_section_default(CFG_KEY_ENABLE_SIWE_VALIDATION, "true")
            .set_section_default(CFG_KEY_ACTIONS_SOCKET, CFG_KEY_ACTIONS_SOCKET_DEFAULT)
            .set_section_default(CFG_KEY_HEALTH_POLL_INTERVAL_MS, "60000")
            .set_section_default(CFG_KEY_ENABLE_RATE_LIMITING_ALLOCATION, "false");

        // Apply others
        builder = <LitConfig as LitBlockchainConfig>::apply_defaults(builder)?;
        builder = <LitConfig as LitLoggingConfig>::apply_defaults(builder)?;

        <LitConfig as LitApiConfig>::from_builder(builder)
    }

    fn verify(&self) -> Result<()> {
        for req in REQUIRED_CFG_KEYS {
            self.get_section_checked_string(req)?;
        }

        self.rpc_url()?;
        self.api_port(self.https_enabled())?;
        self.api_port_external(self.https_enabled())?;
        self.blockchain_wallet_private_key(None)?;
        self.blockchain_chain_id()?;
        self.blockchain_chain_name()?;

        Ok(())
    }

    /// Export user editable config
    fn export_user_editable(&self) -> Result<HashMap<String, String>> {
        let mut map: HashMap<String, String> = HashMap::new();

        for key in USER_EDITABLE_KEYS {
            map.insert(
                format!("{}.{}", CFG_SECTION_KEY, key),
                self.get_section_string(key).unwrap_or("".into()),
            );
        }

        for key in USER_EDITABLE_KEYS_IN_SECTIONS {
            let val = if CFG_KEY_BLOCKCHAIN_WALLET_DEFAULT_PRIVATE_KEY.eq(key) {
                self.blockchain_wallet_private_key(None)
                    .unwrap_or("".into())
            } else if CFG_KEY_RPC_URL.eq(key) {
                self.rpc_url().unwrap_or("".into())
            } else {
                self.get_string(key).unwrap_or("".into())
            };

            map.insert(key.to_owned(), val);
        }

        Ok(map)
    }

    /// Verify the user editable config map (ensure keys are valid)
    fn verify_user_editable(&self, data: &HashMap<String, String>) -> Result<()> {
        for (full_key, _) in data.iter() {
            if let Some((section_key, key)) = full_key.split_once('.') {
                if (section_key != CFG_SECTION_KEY || !USER_EDITABLE_KEYS.contains(&key))
                    && !USER_EDITABLE_KEYS_IN_SECTIONS.contains(&full_key.as_str())
                {
                    return Err(validation_err(
                        format!("user editing of config key '{}' not allowed", full_key),
                        None,
                    ));
                }
            } else {
                return Err(validation_err(
                    format!("user editing of config key '{}' not allowed", full_key),
                    None,
                ));
            }
        }

        Ok(())
    }

    // Accessors

    /// The external port used to access the node (rocket may bind to a different port but iptables will
    /// forward to it).
    fn external_port(&self) -> Result<u16> {
        Ok(self.api_port_external(self.https_enabled())? as u16)
    }

    fn external_addr(&self) -> Result<String> {
        Ok(format!("{}:{}", self.api_domain()?, self.external_port()?))
    }

    fn http_prefix_when_talking_to_other_nodes(&self) -> String {
        if self.https_enabled() {
            "https://".to_string()
        } else {
            "http://".to_string()
        }
    }

    fn internal_port(&self) -> Result<u16> {
        Ok(self.api_port(self.https_enabled())? as u16)
    }

    fn rpc_url(&self) -> Result<String> {
        if let Ok(url) = self.get_section_string(CFG_KEY_RPC_URL) {
            return Ok(url);
        }

        ENDPOINT_MANAGER.rpc_url(self.blockchain_chain_name()?)
    }

    fn staker_address(&self) -> Result<String> {
        self.get_section_string(CFG_KEY_STAKER_ADDRESS)
    }

    fn coms_keys_sender_privkey(&self) -> Result<String> {
        self.get_section_string(CFG_KEY_COMS_KEYS_SENDER_PRIVKEY)
    }

    fn coms_keys_receiver_privkey(&self) -> Result<String> {
        self.get_section_string(CFG_KEY_COMS_KEYS_RECEIVER_PRIVKEY)
    }

    fn admin_address(&self) -> Result<H160> {
        self.get_section_string(CFG_KEY_ADMIN_ADDRESS)?
            .parse::<H160>()
            .map_err(|e| {
                parser_err(
                    e,
                    Some("Could not convert admin_address to H160".to_string()),
                )
            })
    }

    fn webauthn_allowed_origins(&self) -> Result<Vec<Url>> {
        let origins: Vec<String> = self
            .get_section_string(CFG_KEY_WEBAUTHN_ALLOWED_ORIGINS)
            .map(|s| s.split(',').map(|s| s.to_string()).collect())?;

        let origins: Vec<Url> = origins
            .iter()
            .map(|s| {
                Url::parse(s).map_err(|e| {
                    parser_err(
                        e,
                        Some(format!(
                            "Could not parse webauthn_allowed_origins url: {}",
                            s
                        )),
                    )
                })
            })
            .collect::<Result<Vec<Url>>>()?;

        Ok(origins)
    }

    fn http_client_timeout(&self) -> Result<u64> {
        self.get_section_int(CFG_KEY_HTTP_CLIENT_TIMEOUT)
            .map(|i| i as u64)
    }

    // Feature flag bool accessors
    fn enable_proxied_http_client(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_PROXIED_HTTP_CLIENT)
    }

    fn enable_rate_limiting(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_RATE_LIMITING)
    }

    fn enable_rate_limiting_allocation(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_RATE_LIMITING_ALLOCATION)
    }

    fn enable_actions_allowlist(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_ACTIONS_ALLOWLIST)
    }

    fn enable_epoch_transitions(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_EPOCH_TRANSITIONS)
    }

    fn enable_siwe_validation(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_SIWE_VALIDATION)
    }

    fn ecdsa_round_timeout(&self) -> Result<i64> {
        self.get_section_int(CFG_KEY_ECDSA_ROUND_TIMEOUT)
    }

    fn chain_polling_interval_ms(&self) -> Result<i64> {
        self.get_section_int(CFG_KEY_CHAIN_POLLING_INTERVAL_MS)
    }

    fn enter_restore_state(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENTER_RESTORE_STATE)
    }

    fn bls_key_blinder(&self) -> Result<String> {
        self.get_section_string(CFG_KEY_BLS_KEY_BLINDER)
    }

    fn ecdsa_key_blinder(&self) -> Result<String> {
        self.get_section_string(CFG_KEY_ECDSA_KEY_BLINDER)
    }

    fn restore_log_interval(&self) -> Result<i64> {
        self.get_section_int(CFG_KEY_RESTORE_LOG_INTERVAL_MS)
    }

    fn actions_socket(&self) -> Result<std::path::PathBuf> {
        self.get_section_string(CFG_KEY_ACTIONS_SOCKET)
            .map(Into::into)
    }

    fn rpc_health_poll_interval(&self) -> Result<i64> {
        self.get_section_int(CFG_KEY_HEALTH_POLL_INTERVAL_MS)
    }
}

pub(crate) fn key_path(staker_address: &str) -> PathBuf {
    let staker_address = match staker_address.starts_with("0x") {
        true => staker_address.to_string(),
        false => format!("0x{}", staker_address),
    };
    let path_root = format!("./node_keys/{}", staker_address.to_lowercase());
    PathBuf::from(&path_root)
}

pub(crate) fn backup_key_path(staker_address: &str) -> PathBuf {
    let mut path = key_path(staker_address);
    path.push("backup");
    path
}

pub(crate) fn encrypted_key_path(staker_address: &str) -> PathBuf {
    let mut path = key_path(staker_address);
    path.push("encrypted");
    path
}
pub(crate) fn beaver_triple_path(staker_address: &str) -> PathBuf {
    let mut path = key_path(staker_address);
    path.push("beaver_triples");
    path
}

pub(crate) fn typed_key_path(key_type: &str, staker_address: &str) -> PathBuf {
    let mut path = key_path(staker_address);
    path.push(key_type);
    path
}

pub fn segmented_paths(
    path: impl AsRef<Path>,
    key: &str,
    levels: usize,
    from_end: bool,
) -> Result<PathBuf> {
    let mut keys: Vec<&str> = key.split("").collect();
    if keys.len() < levels {
        return Err(validation_err(
            "segmented_paths: provided key is not long enough for the levels required",
            None,
        ));
    }
    if from_end {
        keys.reverse();
    }

    let mut new = PathBuf::from(path.as_ref());
    for i in 0..levels {
        if let Some(k) = keys.get(i) {
            new.push(k);
        }
    }

    Ok(new)
}

pub fn load_cfg() -> Result<ReloadableLitConfig> {
    ReloadableLitConfig::new(|| {
        let cfg = <LitConfig as LitNodeConfig>::try_new()?;

        // Verify every load (will not replace running config unless it works)
        cfg.verify()?;

        Ok(cfg)
    })
}
