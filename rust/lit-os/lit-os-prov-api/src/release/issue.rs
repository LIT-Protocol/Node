use libsecp256k1::PublicKey;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::LitConfig;
use lit_os_prov_core::release::common::types::Release;
use lit_os_prov_core::release::issue::issue_release as core_issue_release;
use lit_os_prov_core::release::issue::types::IssueReleaseRequest;

use crate::error::Result;

pub(crate) async fn issue_release(
    cfg: &LitConfig, resolver: &ContractResolver, req: &IssueReleaseRequest,
    req_public_key: PublicKey, release: &Release,
) -> Result<Vec<String>> {
    core_issue_release(cfg, resolver, req, req_public_key, release).await
}
