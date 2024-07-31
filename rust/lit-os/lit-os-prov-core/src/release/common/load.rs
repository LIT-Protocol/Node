use lit_attestation::verification::Policy;
use log::{as_serde, trace};
use serde_json::Value;

use lit_blockchain::contracts::release::{ReleasePlatform, ReleaseStatus, ReleaseType};
use lit_blockchain::resolver::contract::ContractResolver;
use lit_blockchain::util::release::release_id_bin_from_string;
use lit_core::config::envs::LitEnv;
use lit_core::config::LitConfig;
use lit_core::utils::ipfs::bytes_to_ipfs_cid;
use lit_os_core::guest::env::release::GuestReleaseEnv;

use crate::error::{blockchain_err_code, validation_err, validation_err_code, Result, EC};
use crate::release::common::manifest::{load_release_manifest, ReleaseManifest};
use crate::release::common::types::Release;

pub async fn load_release<R>(
    cfg: &LitConfig, resolver: &ContractResolver, id: R, require_active: bool,
) -> Result<Release>
where
    R: AsRef<str> + Into<String>,
{
    let contract = resolver.release_register_contract(cfg).await?;

    let release = contract
        .get_release(release_id_bin_from_string(id.as_ref())?)
        .call()
        .await
        .map_err(|e| blockchain_err_code(e, EC::ProvReleaseLoadFailed, None))?;
    if release.status == (ReleaseStatus::Null as u8) {
        return Err(validation_err_code(
            format!("release id ({}) not found", id.as_ref()),
            EC::ProvReleaseNotFound,
            None,
        )
        .add_source_to_details()
        .add_field("release_id", Value::from(id.as_ref())));
    }

    let status = ReleaseStatus::try_from(release.status)
        .map_err(|e| validation_err_code(e, EC::ProvReleaseInvalid, None))?;

    if require_active && status != ReleaseStatus::Active {
        return Err(validation_err_code(
            format!("release id ({}) not active (status: {})", id.as_ref(), status),
            EC::ProvReleaseNotActive,
            None,
        )
        .add_source_to_details()
        .add_field("release_id", Value::from(id.as_ref()))
        .add_field("release_status", Value::from(status.to_string())));
    }

    let env = LitEnv::try_from(release.env)
        .map_err(|e| validation_err_code(e, EC::ProvReleaseInvalid, None))?;
    let typ = ReleaseType::try_from(release.typ)
        .map_err(|e| validation_err_code(e, EC::ProvReleaseInvalid, None))?;
    let platform = ReleasePlatform::try_from(release.platform)
        .map_err(|e| validation_err_code(e, EC::ProvReleaseInvalid, None))?;
    let manifest_cid = bytes_to_ipfs_cid(release.cid.as_ref())
        .map_err(|e| validation_err_code(e, EC::ProvReleaseInvalid, None))?;

    Ok(Release::new(id.into(), env, status, typ, platform, manifest_cid))
}

pub async fn load_release_assets(
    cfg: &LitConfig, resolver: Option<&ContractResolver>, release: &Release,
    verify_measurements: bool,
) -> Result<(ReleaseManifest, GuestReleaseEnv)> {
    trace!(release_id = as_serde!(release.id()), manifest_cid = as_serde!(release.manifest_cid());
        "load_release_assets: loading release assets");

    let manifest = load_release_manifest(cfg, release.manifest_cid()).await?;

    // Verify the manifest
    manifest.verify(cfg, resolver, Some(Policy::AdminOrOperator), verify_measurements).await?;

    // Load & prepare release env
    let release_env = manifest.load_release_env(cfg).await?;
    release_env.verify().map_err(|e| validation_err(e, None))?;

    // Verify provided release_id matches
    let rel_release_id = release_env.release_id.as_ref().unwrap();
    if !release.id().eq(rel_release_id) {
        return Err(validation_err_code(
            format!(
                "provided release id ({}) does not match manifested release id ({})",
                release.id(),
                rel_release_id
            )
            .as_str(),
            EC::ProvReleaseValidation,
            None,
        )
        .add_detail("The release id provided is invalid (does not match the manifest)"));
    }

    Ok((manifest, release_env))
}
