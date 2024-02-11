use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::LitConfig;
use lit_os_core::guest::types::GuestType;
use lit_os_prov_core::release::common::types::Release;
use lit_os_prov_core::release::query::query_releases as core_query_releases;
use lit_os_prov_core::release::query::types::QueryReleaseRequest;
use std::collections::HashMap;

use crate::error::Result;

pub(crate) async fn query_releases(
    cfg: &LitConfig, resolver: &ContractResolver, request: &QueryReleaseRequest,
) -> Result<HashMap<GuestType, Release>> {
    core_query_releases(cfg, resolver, request).await
}
