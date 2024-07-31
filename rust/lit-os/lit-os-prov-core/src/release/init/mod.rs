use async_std::fs;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::LitConfig;

use crate::config::LitOsProvConfig;
use crate::error::{io_err_code, serializer_err_code, validation_err_code, Result, EC};
use crate::release::common::load::load_release_assets;
use crate::release::common::manifest::RELEASE_FILE_INITIAL_PASSWORD;
use crate::release::common::types::Release;
use crate::release::init::types::InitReleaseRequest;

pub mod types;

pub async fn init_release(
    cfg: &LitConfig, resolver: &ContractResolver, req: &InitReleaseRequest, release: &Release,
) -> Result<Vec<u8>> {
    let body = req.body();
    body.verify()?;

    // Load (and verify) assets
    let (_manifest, _release_env) =
        load_release_assets(cfg, Some(resolver), release, true)
            .await
            .map_err(|e| validation_err_code(e, EC::ProvReleaseValidation, None))?;

    // Load passphrase
    let releases_dir = cfg.litos_prov_shared_release_path();
    let mut release_dir = releases_dir.clone();
    release_dir.push(body.release_id());

    let mut initial_passwd_path = release_dir.clone();
    initial_passwd_path.push(RELEASE_FILE_INITIAL_PASSWORD);

    let passphrase_bytes = fs::read(initial_passwd_path.as_path())
        .await
        .map_err(|e| io_err_code(e, EC::ProvReleaseInvalid, None))?;

    let passphrase = base64::decode(&passphrase_bytes[..])
        .map_err(|e| serializer_err_code(e, EC::ProvReleaseInvalid, None))?;

    Ok(passphrase)
}
