use lit_core::config::LitConfig;
use lit_core::error::Result;
use reqwest::Client;
#[cfg(all(feature = "proxy_http", feature = "testing"))]
use reqwest::Url;
#[cfg(all(feature = "proxy_http", feature = "testing"))]
use std::path::PathBuf;

use crate::config::LitNodeConfig;
#[cfg(all(feature = "proxy_http", feature = "testing"))]
use crate::error::parser_err;
use crate::error::unexpected_err;
#[cfg(all(feature = "proxy_http", feature = "testing"))]
use crate::networking::http::proxy::mapping::{HttpClientProxyMapping, PROXY_MAPPING_PATH};

pub struct HttpClientFactory;

impl HttpClientFactory {
    pub fn new_client(cfg: &LitConfig) -> Result<Client> {
        #[cfg(not(all(feature = "proxy_http", feature = "testing")))]
        {
            trace!("Using default HTTP client");
            HttpClientFactory::new_default_client(cfg)
        }

        #[cfg(all(feature = "proxy_http", feature = "testing"))]
        {
            if cfg.enable_proxied_http_client()? {
                trace!("Using proxied HTTP client");
                HttpClientFactory::new_proxied_client(cfg)
            } else {
                trace!("Using default HTTP client");
                HttpClientFactory::new_default_client(cfg)
            }
        }
    }

    fn new_default_client(cfg: &LitConfig) -> Result<Client> {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(cfg.http_client_timeout()?))
            .use_rustls_tls()
            .build()
            .map_err(|e| unexpected_err(e, Some("Unable to init default client".into())))
    }

    #[cfg(all(feature = "proxy_http", feature = "testing"))]
    fn new_proxied_client(cfg: &LitConfig) -> Result<Client> {
        let mut builder = reqwest::Client::builder()
            .use_rustls_tls()
            .timeout(std::time::Duration::from_secs(cfg.http_client_timeout()?));

        // Check if config file for proxy mappings exists.
        let path = PathBuf::from(PROXY_MAPPING_PATH);

        if path.exists() {
            // Read config file for proxy mappings.
            let proxy_config = HttpClientProxyMapping::try_from(path.as_path())?;

            // Check if our address is in the proxy mappings.
            let our_addr = Url::parse(&format!("http://{}", cfg.external_addr()?))
                .map_err(|e| parser_err(e, Some("Unable to parse our external addr".into())))?;
            let maybe_our_proxy_config = proxy_config.proxy_mappings().get(&our_addr).cloned();
            if let Some(our_proxy_config) = maybe_our_proxy_config {
                builder = builder.proxy(reqwest::Proxy::custom(move |url| {
                    // Attempt to get the proxy URL for the given URL.
                    let dest_url = match our_proxy_config.get(url) {
                        Some(dest_url) => dest_url,
                        None => {
                            trace!("No proxy URL found for {:?}", url);
                            return None;
                        }
                    };

                    Some(dest_url.to_owned())
                }))
            }
        }

        builder
            .build()
            .map_err(|e| unexpected_err(e, Some("Unable to init proxied client".into())))
    }
}
