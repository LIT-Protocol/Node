use reqwest::{Client, RequestBuilder};
use serde_json::Value;

use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::LitConfig;

use crate::error::{blockchain_err, conversion_err, http_client_err, Result};

pub struct ProvApiClient {
    pub(crate) client: Client,
    pub(crate) domain: String,
}

impl ProvApiClient {
    pub async fn new(
        cfg: &LitConfig, resolver: Option<&ContractResolver>,
    ) -> Result<ProvApiClient> {
        let domain = match resolver {
            Some(resolver) => lookup_api_domain(cfg, resolver).await?,
            None => lookup_api_domain(cfg, &ContractResolver::try_from(cfg)?).await?,
        };

        Ok(Self { client: create_http_client()?, domain })
    }

    pub fn domain(&self) -> &String {
        &self.domain
    }

    #[allow(dead_code)]
    pub(crate) fn get<U>(&self, uri: U) -> RequestBuilder
    where
        U: AsRef<str>,
    {
        self.client.get(self.url(uri.as_ref()))
    }

    #[allow(dead_code)]
    pub(crate) fn post<U>(&self, uri: U) -> RequestBuilder
    where
        U: AsRef<str>,
    {
        self.client.post(self.url(uri.as_ref()))
    }

    #[allow(dead_code)]
    pub(crate) fn put<U>(&self, uri: U) -> RequestBuilder
    where
        U: AsRef<str>,
    {
        self.client.put(self.url(uri.as_ref()))
    }

    #[allow(dead_code)]
    pub(crate) fn patch<U>(&self, uri: U) -> RequestBuilder
    where
        U: AsRef<str>,
    {
        self.client.patch(self.url(uri.as_ref()))
    }

    #[allow(dead_code)]
    pub(crate) fn delete<U>(&self, uri: U) -> RequestBuilder
    where
        U: AsRef<str>,
    {
        self.client.delete(self.url(uri.as_ref()))
    }

    pub(crate) fn url<U>(&self, uri: U) -> String
    where
        U: AsRef<str>,
    {
        format!("https://{}/{}", self.domain, uri.as_ref())
    }
}

async fn lookup_api_domain(cfg: &LitConfig, resolver: &ContractResolver) -> Result<String> {
    let contract = resolver.release_register_contract(cfg).await?;
    let domain_bytes = contract.get_creator_domain().call().await.map_err(|e| {
        blockchain_err(e, None).add_field("subnet_id", Value::from(resolver.subnet_id().clone()))
    })?;

    if domain_bytes.is_empty() {
        return Err(blockchain_err(
            "empty domain bytes returned from release_register_contract", None,
        )
        .add_field("subnet_id", Value::from(resolver.subnet_id().clone())));
    }

    String::from_utf8(domain_bytes.to_vec()).map_err(|e| {
        conversion_err(e, Some("failed to parse creator domain bytes as utf8".into()))
            .add_field("subnet_id", Value::from(resolver.subnet_id().clone()))
    })
}

#[cfg(feature = "trust-dns")]
fn create_http_client() -> Result<Client> {
    let mut client = Client::builder();
    client = client.trust_dns(true);

    let client = client
        .build()
        .map_err(|e| http_client_err(e, Some("failed to construct reqwest client".into())))?;

    Ok(client)
}

#[cfg(not(feature = "trust-dns"))]
fn create_http_client() -> Result<Client> {
    let client = Client::builder()
        .build()
        .map_err(|e| http_client_err(e, Some("failed to construct reqwest client".into())))?;

    Ok(client)
}
