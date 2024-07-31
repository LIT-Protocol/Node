use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::LitConfig;
use lit_os_prov_core::release::common::types::Release;
use lit_os_prov_core::release::init::init_release as core_init_release;
use lit_os_prov_core::release::init::types::InitReleaseRequest;

use crate::error::Result;

pub(crate) async fn init_release(
    cfg: &LitConfig, resolver: &ContractResolver, request: &InitReleaseRequest, release: &Release,
) -> Result<Vec<u8>> {
    core_init_release(cfg, resolver, request, release).await
}
