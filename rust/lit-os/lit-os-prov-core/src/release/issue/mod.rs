use libsecp256k1::PublicKey;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::LitConfig;
use tracing::trace;

use crate::config::LitOsProvConfig;
use crate::error::{unexpected_err_code, validation_err_code, Result, EC};
use crate::release::common::keys::issue_host_identity;
use crate::release::common::load::load_release_assets;
use crate::release::common::types::Release;
use crate::release::issue::types::IssueReleaseRequest;

pub mod types;

pub async fn issue_release(
    cfg: &LitConfig, resolver: &ContractResolver, req: &IssueReleaseRequest,
    req_public_key: PublicKey, release: &Release,
) -> Result<Vec<String>> {
    trace!("issue_release");
    let body = req.body();
    body.verify()?;

    // Load (and verify) assets
    let (manifest, _release_env) = load_release_assets(cfg, Some(resolver), release, true)
        .await
        .map_err(|e| validation_err_code(e, EC::ProvReleaseValidation, None))?;

    // Load build env
    let build_env = manifest.load_build_env(cfg).await?;

    // Validation
    let guest_cpu_type = build_env.guest_cpu_type()?;
    if !guest_cpu_type.eq(body.vcpu_type()) {
        return Err(validation_err_code(
            format!(
                "release CPU type does not match the given vcpu_type: ({} vs {})",
                guest_cpu_type,
                body.vcpu_type()
            ),
            EC::ProvReleaseIdBlockIssueFailed,
            None,
        ));
    }

    // Issue id block
    let releases_dir = cfg.litos_prov_shared_release_path();
    let mut release_dir = releases_dir;
    release_dir.push(body.release_id());

    let (id_block, auth_info) = issue_host_identity(
        cfg,
        release_dir.as_path(),
        &manifest,
        &build_env,
        body.vcpus(),
        req_public_key,
    )
    .map_err(|e| unexpected_err_code(e, EC::ProvReleaseIdBlockIssueFailed, None))?;

    Ok(vec![id_block, auth_info])
}
