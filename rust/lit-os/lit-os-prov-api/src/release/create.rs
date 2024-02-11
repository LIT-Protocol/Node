use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::LitConfig;
use lit_os_prov_core::error::Result;
use lit_os_prov_core::release::create::create_release as core_create_release;
use lit_os_prov_core::release::create::types::CreateReleaseRequest;

pub(crate) async fn create_release(
    cfg: &LitConfig, resolver: &ContractResolver, request: &CreateReleaseRequest,
) -> Result<()> {
    let _ = core_create_release(cfg, resolver, request.body(), None).await?;

    Ok(())
}
