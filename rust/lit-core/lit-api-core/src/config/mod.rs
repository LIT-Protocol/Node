#[cfg(feature = "server")]
use zerossl::client::Client;

use lit_core::config::{LitConfig, LitConfigBuilder};

use crate::error::Result;

pub static CFG_SECTION_HTTP: &str = "http";
pub static CFG_SECTION_HTTPS: &str = "https";

pub static CFG_KEY_ENABLED: &str = "enabled";
pub static CFG_KEY_IDENT: &str = "ident";
pub static CFG_KEY_PORT: &str = "port";
pub static CFG_KEY_PORT_EXTERNAL: &str = "port_external";
pub static CFG_KEY_ADDRESS: &str = "address";
pub static CFG_KEY_WORKERS: &str = "workers";
pub static CFG_KEY_KEEP_ALIVE: &str = "keep_alive";
pub static CFG_KEY_DOMAIN: &str = "domain";
pub static CFG_KEY_REUSE_ADDRESS: &str = "reuse_address";
pub static CFG_KEY_REUSE_PORT: &str = "reuse_port";
pub static CFG_KEY_REDIRECT_TO_HTTPS: &str = "redirect_to_https";

pub static CFG_KEY_TLS_KEY: &str = "tls_key";
pub static CFG_KEY_TLS_CERTS: &str = "tls_certs";

pub static CFG_KEY_TLS_CSR_COUNTRY: &str = "tls_csr.country";
pub static CFG_KEY_TLS_CSR_ORG_NAME: &str = "tls_csr.org_name";
pub static CFG_KEY_TLS_CSR_ORG_UNIT: &str = "tls_csr.org_unit";
pub static CFG_KEY_TLS_CSR_SELF_SIGNED_CN: &str = "tls_csr.self_signed_cn";
pub static CFG_KEY_TLS_CSR_SELF_SIGNED_DAYS: &str = "tls_csr.self_signed_days";
pub static CFG_KEY_TLS_CSR_SELF_SIGNED_DESC: &str = "tls_csr.self_signed_desc";

pub static CFG_KEY_TLS_AUTO_ENABLED: &str = "tls_auto.enabled";
pub static CFG_KEY_TLS_AUTO_THRESHOLD_DAYS: &str = "tls_auto.threshold_days";
pub static CFG_KEY_TLS_AUTO_CHECK_INITIAL_SEC: &str = "tls_auto.check_initial_secs";
pub static CFG_KEY_TLS_AUTO_CHECK_INTERVAL_SEC: &str = "tls_auto.check_interval_secs";
pub static CFG_KEY_TLS_AUTO_PURGE_PENDING: &str = "tls_auto.purge_pending";
pub static CFG_KEY_TLS_AUTO_PURGE_ACTIVE: &str = "tls_auto.purge_active";
pub static CFG_KEY_TLS_AUTO_VERIFY_ATTEMPTS: &str = "tls_auto.verify_attempts";
pub static CFG_KEY_TLS_AUTO_VERIFY_RETRY_INTERVAL_SEC: &str = "tls_auto.verify_retry_interval_secs";
pub static CFG_KEY_TLS_AUTO_DOWNLOAD_ATTEMPTS: &str = "tls_auto.download_attempts";
pub static CFG_KEY_TLS_AUTO_DOWNLOAD_INITIAL_SEC: &str = "tls_auto.download_initial_secs";
pub static CFG_KEY_TLS_AUTO_DOWNLOAD_RETRY_INTERVAL_SEC: &str =
    "tls_auto.download_retry_interval_secs";

pub static CFG_KEY_ZEROSSL_API_KEY: &str = "zerossl.api_key";

pub trait LitApiConfig {
    fn from_builder(builder: LitConfigBuilder) -> Result<LitConfig>;
    fn apply_defaults(builder: LitConfigBuilder) -> Result<LitConfigBuilder>;
    fn api_domain(&self) -> Result<String>;
    fn api_ident(&self) -> Result<String>;
    fn api_port(&self, is_https: bool) -> Result<i64>;
    fn api_http_port(&self) -> Result<i64>;
    fn api_https_port(&self) -> Result<i64>;
    fn api_port_external(&self, is_https: bool) -> Result<i64>;
    fn api_http_port_external(&self) -> Result<i64>;
    fn api_https_port_external(&self) -> Result<i64>;
    fn get_http_section_string(&self, key: &str, is_https: bool) -> Result<String>;
    fn get_http_section_int(&self, key: &str, is_https: bool) -> Result<i64>;
    fn get_http_section_float(&self, key: &str, is_https: bool) -> Result<f64>;
    fn get_http_section_bool(&self, key: &str, is_https: bool) -> Result<bool>;
    fn get_http_enabled(&self, is_https: bool) -> bool;
    fn http_enabled(&self) -> bool;
    fn https_enabled(&self) -> bool;
    fn tls_auto(&self) -> bool;
    fn tls_key(&self) -> Option<String>;
    fn tls_certs(&self) -> Option<String>;
    #[cfg(feature = "server")]
    fn zerossl_api_key(&self) -> Option<String>;
    #[cfg(feature = "server")]
    fn zerossl_client(&self) -> Result<Client>;
}

impl LitApiConfig for LitConfig {
    fn from_builder(mut builder: LitConfigBuilder) -> Result<LitConfig> {
        builder = Self::apply_defaults(builder)?;
        builder.build()
    }

    fn apply_defaults(mut builder: LitConfigBuilder) -> Result<LitConfigBuilder> {
        if builder.key().is_none() {
            builder = builder.set_key(Some("api".to_string()));
        }

        // Set defaults
        builder = builder.set_section_default(CFG_KEY_IDENT, "Lit API")
            .set_section_default(CFG_KEY_REUSE_ADDRESS, "true")
            .set_section_default(CFG_KEY_REUSE_PORT, "true")
            .set_section_default(CFG_KEY_WORKERS, (num_cpus::get_physical() * 8).to_string())
            .set_section_default(http_section_key(CFG_KEY_ENABLED), "true")
            .set_section_default(http_section_key(CFG_KEY_PORT), "8080")
            .set_section_default(http_section_key(CFG_KEY_REDIRECT_TO_HTTPS), "false")
            .set_section_default(https_section_key(CFG_KEY_ENABLED), "false")
            .set_section_default(https_section_key(CFG_KEY_PORT), "8443")
            .set_section_default(https_section_key(CFG_KEY_TLS_KEY), "./cert/key.pem")
            .set_section_default(https_section_key(CFG_KEY_TLS_CERTS), "./cert/certs.pem")
            .set_section_default(https_section_key(CFG_KEY_TLS_CSR_COUNTRY), "US")
            .set_section_default(https_section_key(CFG_KEY_TLS_CSR_ORG_NAME), "Lit Protocol")
            .set_section_default(https_section_key(CFG_KEY_TLS_CSR_ORG_UNIT), "Node Devs")
            .set_section_default(https_section_key(CFG_KEY_TLS_CSR_SELF_SIGNED_CN), "rogue.litnet.io")
            .set_section_default(https_section_key(CFG_KEY_TLS_CSR_SELF_SIGNED_DAYS), "36500")
            .set_section_default(https_section_key(CFG_KEY_TLS_CSR_SELF_SIGNED_DESC), "Temporary / internal (self-signed).")
            .set_section_default(https_section_key(CFG_KEY_TLS_AUTO_ENABLED), "false")
            .set_section_default(https_section_key(CFG_KEY_TLS_AUTO_THRESHOLD_DAYS), "28")
            .set_section_default(https_section_key(CFG_KEY_TLS_AUTO_CHECK_INITIAL_SEC), "15")
            .set_section_default(https_section_key(CFG_KEY_TLS_AUTO_CHECK_INTERVAL_SEC), "86400")
            .set_section_default(https_section_key(CFG_KEY_TLS_AUTO_PURGE_PENDING), "true")
            .set_section_default(https_section_key(CFG_KEY_TLS_AUTO_PURGE_ACTIVE), "true")
            .set_section_default(https_section_key(CFG_KEY_TLS_AUTO_VERIFY_ATTEMPTS), "10")
            .set_section_default(https_section_key(CFG_KEY_TLS_AUTO_VERIFY_RETRY_INTERVAL_SEC), "5")
            .set_section_default(https_section_key(CFG_KEY_TLS_AUTO_DOWNLOAD_ATTEMPTS), "10")
            // Wait even longer for download (generation takes a while)
            .set_section_default(https_section_key(CFG_KEY_TLS_AUTO_DOWNLOAD_INITIAL_SEC), "10")
            .set_section_default(https_section_key(CFG_KEY_TLS_AUTO_DOWNLOAD_RETRY_INTERVAL_SEC), "60");

        Ok(builder)
    }

    // Accessors
    #[inline]
    fn api_domain(&self) -> Result<String> {
        self.get_section_checked_string(CFG_KEY_DOMAIN)
    }

    #[inline]
    fn api_ident(&self) -> Result<String> {
        self.get_section_checked_string(CFG_KEY_IDENT)
    }

    #[inline]
    fn api_port(&self, is_https: bool) -> Result<i64> {
        self.get_http_section_int(CFG_KEY_PORT, is_https)
    }

    #[inline]
    fn api_http_port(&self) -> Result<i64> {
        self.api_port(false)
    }

    #[inline]
    fn api_https_port(&self) -> Result<i64> {
        self.api_port(true)
    }

    #[inline]
    fn api_port_external(&self, is_https: bool) -> Result<i64> {
        self.get_http_section_int(CFG_KEY_PORT_EXTERNAL, is_https)
            .or_else(|_| self.api_port(is_https))
    }

    #[inline]
    fn api_http_port_external(&self) -> Result<i64> {
        self.api_port_external(false)
    }

    #[inline]
    fn api_https_port_external(&self) -> Result<i64> {
        self.api_port_external(true)
    }

    // HTTP/HTTPS Section

    #[inline]
    fn get_http_section_string(&self, key: &str, is_https: bool) -> Result<String> {
        let sub_key = if is_https { https_section_key(key) } else { http_section_key(key) };

        self.get_section_checked_string(sub_key.as_str()).or(self.get_section_checked_string(key))
    }

    #[inline]
    fn get_http_section_int(&self, key: &str, is_https: bool) -> Result<i64> {
        let sub_key = if is_https { https_section_key(key) } else { http_section_key(key) };

        self.get_section_int(sub_key.as_str()).or(self.get_section_int(key))
    }

    #[inline]
    fn get_http_section_float(&self, key: &str, is_https: bool) -> Result<f64> {
        let sub_key = if is_https { https_section_key(key) } else { http_section_key(key) };

        self.get_section_float(sub_key.as_str()).or(self.get_section_float(key))
    }

    #[inline]
    fn get_http_section_bool(&self, key: &str, is_https: bool) -> Result<bool> {
        let sub_key = if is_https { https_section_key(key) } else { http_section_key(key) };

        self.get_section_bool(sub_key.as_str()).or(self.get_section_bool(key))
    }

    #[inline]
    fn get_http_enabled(&self, is_https: bool) -> bool {
        if let Ok(val) = self.get_http_section_bool(CFG_KEY_ENABLED, is_https) {
            return val;
        }

        false
    }

    #[inline]
    fn http_enabled(&self) -> bool {
        self.get_http_enabled(false)
    }

    #[inline]
    fn https_enabled(&self) -> bool {
        self.get_http_enabled(true)
    }

    // TLS

    #[inline]
    fn tls_auto(&self) -> bool {
        if let Ok(val) = self.get_http_section_bool(CFG_KEY_TLS_AUTO_ENABLED, true) {
            return val;
        }

        false
    }

    #[inline]
    fn tls_key(&self) -> Option<String> {
        self.get_http_section_string(CFG_KEY_TLS_KEY, true).ok()
    }

    #[inline]
    fn tls_certs(&self) -> Option<String> {
        self.get_http_section_string(CFG_KEY_TLS_CERTS, true).ok()
    }

    // ZeroSSL

    #[inline]
    #[cfg(feature = "server")]
    fn zerossl_api_key(&self) -> Option<String> {
        self.get_checked_string(CFG_KEY_ZEROSSL_API_KEY).ok()
    }

    #[inline]
    #[cfg(feature = "server")]
    fn zerossl_client(&self) -> Result<Client> {
        if let Some(api_key) = self.zerossl_api_key() {
            return Ok(Client::new(api_key));
        }

        Err(crate::error::certs_err(
            format!("missing zerossl api key config '{CFG_KEY_ZEROSSL_API_KEY}'"),
            None,
        ))
    }
}

// Util

#[inline]
pub fn http_section_key<K>(key: K) -> String
where
    K: Into<String>,
{
    format!("{}.{}", CFG_SECTION_HTTP, key.into())
}

#[inline]
pub fn https_section_key<K>(key: K) -> String
where
    K: Into<String>,
{
    format!("{}.{}", CFG_SECTION_HTTPS, key.into())
}
