use std::fs;
use std::path::{Path, PathBuf};

use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::providers::Provider;
use lit_attestation::verification::Policy;
use log::{as_serde, info};

use lit_blockchain::contracts::release::{ReleasePlatform, ReleaseStatus};
use lit_blockchain::resolver::contract::ContractResolver;
use lit_blockchain::util::string_to_eth_address;
use lit_blockchain::ReleaseRegister;
use lit_core::config::envs::LitEnv;
use lit_core::config::LitConfig;
use lit_core::utils::ipfs::ipfs_cid_to_bytes;
use lit_os_core::guest::env::build::GuestBuildEnv;
use lit_os_core::guest::env::release::GuestReleaseEnv;

use crate::config::LitOsProvConfig;
use crate::error::{
    blockchain_err, blockchain_err_code, io_err_code, unexpected_err_code, validation_err,
    validation_err_code, Result, EC,
};
use crate::release::common::keys::{
    create_id_pem_key, extract_host_identity_fingerprint, ID_KEY_FILE_NAME,
};
use crate::release::common::manifest::{
    load_release_manifest, ReleaseManifest, RELEASE_FILE_INITIAL_PASSWORD,
};
use crate::release::create::types::CreateRelease;

pub mod types;

pub async fn create_release(
    cfg: &LitConfig, resolver: &ContractResolver, req: &CreateRelease,
    mut updates: Option<&mut Vec<PathBuf>>,
) -> Result<(ReleaseManifest, GuestBuildEnv, PathBuf)> {
    // Shared
    let contract = resolver
        .release_register_contract_with_signer(cfg)
        .await
        .map_err(|e| blockchain_err_code(e, EC::ProvFault, None))?;

    info!(release_id = as_serde!(req.release_id()); "Creating release");

    if !cfg.is_dev() {
        // Initial chain validations
        verify_release_not_exist(&contract, req).await?;
    }

    // Verify request body
    let (manifest, release_env) = verify_and_load_release_from_request(cfg, Some(resolver), req)
        .await
        .map_err(|e| validation_err_code(e, EC::ProvReleaseValidation, None))?;

    // Load build env
    let build_env = manifest.load_build_env(cfg).await?;

    // Verify env and subnet
    let subnet_id = release_env.release_subnet_id.as_ref().ok_or_else(|| {
        unexpected_err_code("release env missing 'release_subnet_id'", EC::ProvFault, None)
    })?;

    verify_has_allowed_env(&contract, &build_env.env()?, subnet_id).await?;
    verify_has_allowed_subnet(&contract, subnet_id).await?;

    // Create release dir
    let release_dir = create_release_dir(cfg, req, &mut updates)?;

    // Store initial password
    save_initial_password(req, release_dir.as_path(), &mut updates)?;

    // Create ID Pem
    let _id_key_path =
        create_release_id_pem_key(req.release_id(), release_dir.as_path(), &mut updates)?;

    // Call contract to create release
    info!(release_id = as_serde!(req.release_id()),
        env = as_serde!(build_env.env()?),
        guest_type = as_serde!(build_env.guest_type()?),
        status = as_serde!(ReleaseStatus::Active),
        platform = as_serde!(build_env.guest_cpu_type()?);
        "Sending release to ReleaseRegister contract");

    resolver
        .send(
            cfg,
            contract.create_release(
                req.release_id_as_b32_slice()?,
                ReleaseStatus::Active as u8,
                build_env.env()? as u8,
                build_env.guest_type()?.to_release_type() as u8,
                Bytes::from(build_env.guest_kind_to_vec()),
                Into::<ReleasePlatform>::into(build_env.guest_cpu_type()?) as u8,
                U256::from(build_env.build_options_bits()),
                Bytes::from(extract_host_identity_fingerprint(release_dir.as_path())?),
                Bytes::from(req.public_key().to_vec()),
                Bytes::from(ipfs_cid_to_bytes(req.manifest_cid())?),
                U256::from(0),
            ),
            Some("failed send TX for create release"),
        )
        .await?;

    Ok((manifest, build_env, release_dir))
}

async fn verify_release_not_exist(
    contract: &ReleaseRegister<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    req: &CreateRelease,
) -> Result<()> {
    let release = contract
        .get_release(req.release_id_as_b32_slice()?)
        .call()
        .await
        .map_err(|e| blockchain_err(e, None))?;
    if release.status != (ReleaseStatus::Null as u8) {
        return Err(validation_err_code(
            format!("release id ({}) already exists on chain", req.release_id()),
            EC::ProvReleaseExists,
            None,
        ));
    }

    Ok(())
}

pub(crate) async fn verify_has_allowed_env(
    contract: &ReleaseRegister<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, env: &LitEnv,
    subnet_id: &String,
) -> Result<()> {
    if !contract
        .has_allowed_env(*env as u8)
        .call()
        .await
        .map_err(|e| blockchain_err(e, Some("failed to call 'has_allowed_env' on chain".into())))?
    {
        return Err(validation_err_code(
            format!(
                "env '{}' is not allowed for this subnet ('{}') in the release contract",
                env, &subnet_id
            ),
            EC::ProvReleaseValidation,
            None,
        )
        .add_detail("This ENV is not allowed"));
    };

    Ok(())
}

pub(crate) async fn verify_has_allowed_subnet(
    contract: &ReleaseRegister<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    subnet_id: &String,
) -> Result<()> {
    if !contract.has_allowed_subnet(string_to_eth_address(subnet_id)?).call().await.map_err(
        |e| blockchain_err(e, Some("failed to call 'has_allowed_subnet' on chain".into())),
    )? {
        return Err(validation_err_code(
            format!("this subnet ('{}') is not allowed in the release contract", &subnet_id),
            EC::ProvReleaseValidation,
            None,
        )
        .add_detail("This subnet is not allowed"));
    };

    Ok(())
}

async fn verify_and_load_release_from_request(
    cfg: &LitConfig, resolver: Option<&ContractResolver>, req: &CreateRelease,
) -> Result<(ReleaseManifest, GuestReleaseEnv)> {
    // Verify request
    req.verify()?;

    let manifest = load_release_manifest(cfg, req.manifest_cid().as_str()).await?;

    // Verify the manifest
    manifest.verify(cfg, resolver, Some(Policy::Admin), true).await?;

    // Load & prepare release env
    let release_env = manifest.load_release_env(cfg).await?;
    release_env.verify().map_err(|e| validation_err(e, None))?;

    // Verify provided release_id matches
    let rel_release_id = release_env.release_id.as_ref().unwrap();
    if !req.release_id().eq(rel_release_id) {
        return Err(validation_err_code(format!("CreateReleaseRequest invalid - provided release id ({}) does not match manifested release id ({})",
                                               req.release_id(), rel_release_id).as_str(), EC::ProvReleaseValidation, None)
            .add_detail("The release id provided is invalid (does not match the manifest)"));
    }

    Ok((manifest, release_env))
}

fn create_release_dir(
    cfg: &LitConfig, req: &CreateRelease, updates: &mut Option<&mut Vec<PathBuf>>,
) -> Result<PathBuf> {
    let releases_dir = cfg.litos_prov_shared_release_path();
    let mut release_dir = releases_dir;
    release_dir.push(req.release_id());

    if release_dir.exists() {
        if cfg.is_dev() {
            if let Some(updates) = updates {
                updates.push(release_dir.clone());
            }

            return Ok(release_dir);
        }

        return Err(io_err_code(
            format!("release dir already exists: {:?}", &release_dir),
            EC::ProvReleaseExists,
            None,
        ));
    }

    fs::create_dir_all(&release_dir).map_err(|e| {
        io_err_code(
            e,
            EC::ProvFault,
            Some(format!("failed to create release dir: {:?}", &release_dir)),
        )
    })?;

    if let Some(updates) = updates {
        updates.push(release_dir.clone());
    }

    Ok(release_dir)
}

fn save_initial_password(
    req: &CreateRelease, release_dir: &Path, updates: &mut Option<&mut Vec<PathBuf>>,
) -> Result<()> {
    let initial_passwd = base64::encode(req.password().as_slice());
    let mut initial_passwd_path = release_dir.to_path_buf();
    initial_passwd_path.push(RELEASE_FILE_INITIAL_PASSWORD);

    fs::write(&initial_passwd_path, initial_passwd).map_err(|e| {
        io_err_code(
            e,
            EC::ProvFault,
            Some(format!("failed to write password file to: {:?}", &initial_passwd_path)),
        )
    })?;

    if let Some(updates) = updates {
        updates.push(initial_passwd_path);
    }

    Ok(())
}

fn create_release_id_pem_key(
    release_id: &String, release_dir: &Path, updates: &mut Option<&mut Vec<PathBuf>>,
) -> Result<PathBuf> {
    info!(release_id = as_serde!(release_id); "Creating release id key: {}", ID_KEY_FILE_NAME);

    let mut id_key_path = release_dir.to_path_buf();
    id_key_path.push(ID_KEY_FILE_NAME);

    create_id_pem_key(id_key_path.as_path())?;

    if let Some(updates) = updates {
        updates.push(id_key_path.clone());
    }

    Ok(id_key_path)
}
