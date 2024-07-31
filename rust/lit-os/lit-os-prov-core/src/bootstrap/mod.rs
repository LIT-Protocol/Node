use ethers::types::Bytes;
use log::{as_serde, info};
use sev_snp_utilities::fingerprint_id_key;
use std::path::PathBuf;

use lit_attestation::verification::Policy;
use lit_attestation::AttestedRequest;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_blockchain::util::string_to_eth_address;
use lit_core::config::LitConfig;

use crate::config::LitOsProvConfig;
use crate::error::{
    attestation_err, blockchain_err, conversion_err, unexpected_err, validation_err, Result,
};
use crate::release::common::keys::{create_id_pem_key, issue_host_identity, AUTHOR_KEY_FILE_NAME};
use crate::release::create::types::CreateReleaseRequest;
use crate::release::create::{create_release, verify_has_allowed_env};

pub async fn bootstrap(
    cfg: &LitConfig, resolver: &ContractResolver, req: &CreateReleaseRequest, guest_vcpus: u16,
    mut updates: Option<&mut Vec<PathBuf>>,
) -> Result<(String, String)> {
    req.verify(cfg, Some(resolver), Some(Policy::Admin)).await?;

    let public_key = req
        .auth()
        .last_public_key()
        .map_err(|e| attestation_err(e, Some("expected request to be signed".into())))?;

    // Shared
    let contract = resolver
        .release_register_contract_with_signer(cfg)
        .await
        .map_err(|e| blockchain_err(e, None))?;

    let subnet_id = cfg.subnet_id()?;
    let domain = cfg.get_string("api.prov.domain")?;

    // Create the author key
    info!(env = as_serde!(cfg.env()),
        subnet_id = as_serde!(&subnet_id),
        domain = as_serde!(&domain);
        "Creating author key: {}", AUTHOR_KEY_FILE_NAME);

    let (_author_key, author_key_digest) = create_author_pem_key(cfg, &mut updates)?;

    // Check allowed env
    verify_has_allowed_env(&contract, cfg.env(), &subnet_id).await?;

    if !cfg.is_dev() {
        // Check we haven't already called init (done in contract, but lets be safe).
        if contract.has_creator_init().call().await.map_err(|e| {
            blockchain_err(e, Some("failed to call 'has_creator_init' on chain".into()))
        })? {
            return Err(validation_err(
                "unable to bootstrap already initialized contract ('init_creator' already called)",
                None,
            ));
        }
    }

    // Init the releases contract
    info!(env = as_serde!(cfg.env()),
        subnet_id = as_serde!(&subnet_id),
        domain = as_serde!(&domain);
        "Initializing ReleaseRegister contract as Creator");

    resolver
        .send(
            cfg,
            contract.init_creator(
                *cfg.env() as u8,
                string_to_eth_address(&subnet_id)?,
                Bytes::from(domain.as_bytes().to_vec()),
                Bytes::from(author_key_digest),
            ),
            Some("tx for 'init_creator' failed"),
        )
        .await?;

    // Create the release
    let (manifest, build_env, release_dir) =
        create_release(cfg, resolver, req.body(), updates).await?;

    // Issue id block
    issue_host_identity(cfg, release_dir.as_path(), &manifest, &build_env, guest_vcpus, public_key)
}

fn create_author_pem_key(
    cfg: &LitConfig, updates: &mut Option<&mut Vec<PathBuf>>,
) -> Result<(PathBuf, Vec<u8>)> {
    let key_path = cfg.litos_prov_shared_author_key_path();

    if key_path.exists() {
        return Err(unexpected_err(
            format!("author key already exists: {:?}", key_path.as_path()),
            None,
        ));
    }

    create_id_pem_key(key_path.as_path())?;

    if let Some(updates) = updates {
        updates.push(key_path.clone());
    }

    // Extract the author key digest
    let author_key_digest =
        fingerprint_id_key(key_path.as_path()).map_err(|e| conversion_err(e, None))?;

    Ok((key_path, author_key_digest))
}
