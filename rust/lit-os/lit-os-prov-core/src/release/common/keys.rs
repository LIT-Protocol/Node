use libsecp256k1::PublicKey;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use log::{as_serde, info};
use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use serde_json::Value;
use sev_snp_utilities::{fingerprint_id_key, LaunchDigest};

use lit_attestation::utils::sev_snp::{FamilyIdBuilder, ImageIdBuilder};
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use lit_core::utils::hash::sha512;
use lit_os_core::guest::env::build::GuestBuildEnv;
use lit_os_core::guest::types::GuestCpuType;
use lit_os_core::utils::openssl::openssl_genpkey;
use lit_os_core::utils::sev_snp::{sev_snp_host_identity, DEFAULT_SEV_SNP_POLICY};

use crate::config::LitOsProvConfig;
use crate::error::{
    conversion_err, io_err, unexpected_err, validation_err, validation_err_code, Result, EC,
};
use crate::release::common::manifest::ReleaseManifest;

pub const IDENTITY_PATH_NAME: &str = "identity";
pub const AUTHOR_KEY_FILE_NAME: &str = "author-key.pem";
pub const ID_KEY_FILE_NAME: &str = "id-key.pem";
pub const AUTH_INFO_FILE_NAME: &str = "auth_info.b64";
pub const ID_BLOCK_FILE_NAME: &str = "id_block.b64";

pub fn create_id_pem_key(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            io_err(e, Some(format!("create_id_pem_key: failed to create parent dir for: {path:?}")))
        })?;
    } else {
        return Err(io_err(
            format!("create_id_pem_key: unable to determine parent of dir: {path:?}"),
            None,
        ));
    }

    openssl_genpkey(path, None, None).map_err(|e| io_err(e, None))
}

pub fn issue_host_identity(
    cfg: &LitConfig, release_dir: &Path, manifest: &ReleaseManifest, build_env: &GuestBuildEnv,
    vcpus: u16, public_key: PublicKey,
) -> Result<(String, String)> {
    let vcpus_type = build_env.guest_cpu_type()?;

    let measurement = manifest.find_measurement(vcpus_type.to_string(), vcpus)
        .ok_or_else(|| validation_err_code(
            format!("failed to find a valid measurement in the release manifest matching; vcpus_type: {vcpus_type}, vcpus: {vcpus}"),
            EC::ProvReleaseIdBlockIssueInvalid, None)
            .add_field("release_id", Value::from(manifest.release().id.clone())))?;

    let author_key = cfg.litos_prov_shared_author_key_path();
    if !author_key.exists() {
        return Err(unexpected_err(format!("expected author key to exist: {author_key:?}"), None));
    }

    let mut id_key = release_dir.to_path_buf();
    id_key.push(ID_KEY_FILE_NAME);
    if !id_key.exists() {
        return Err(unexpected_err(format!("expected id key to exist: {id_key:?}"), None));
    }

    let measurement_ld = LaunchDigest::from_str(measurement.as_str()).map_err(|e| {
        conversion_err(
            e,
            Some(format!("failed to convert measurement hex from: {}", measurement.as_str())),
        )
    })?;

    let image_id = ImageIdBuilder::new(manifest.release().id.clone()).build()?;

    let public_key_bytes = &public_key.serialize();
    let public_key_hash = sha512(public_key_bytes).to_vec();

    let fingerprint: [u8; 4] = public_key_hash[..4].try_into().map_err(|e| {
        conversion_err(e, Some("failed to convert public hash key to fixed slice".into()))
    })?;

    let mut seed: [u8; 2] = [0; 2];

    let mut rng = ChaChaRng::from_entropy();
    rng.fill_bytes(&mut seed);

    let family_id = FamilyIdBuilder::new(
        manifest.meta().build_release,
        manifest.meta().build_type.to_release_type(),
        vcpus_type.into(),
        vcpus_type.into(),
        vcpus,
        build_env.guest_os_type()? as u8,
        build_env.guest_os_version()?,
        build_env.build_options_bits(),
        fingerprint,
        seed,
    )
    .build()?;

    let (id_block, auth_info) = sev_snp_host_identity(
        measurement_ld,
        image_id,
        family_id,
        DEFAULT_SEV_SNP_POLICY,
        author_key.as_path(),
        id_key.as_path(),
    )?;

    info!(
        release_id = as_serde!(manifest.release().id),
        measurement = as_serde!(measurement),
        vcpu_type = as_serde!(vcpus_type),
        vcpus = as_serde!(vcpus),
        image_id = as_serde!(image_id.hex()),
        family_id = as_serde!(family_id.hex()),
        family_id_fp = as_serde!(bytes_to_hex(fingerprint)),
        family_id_seed = as_serde!(bytes_to_hex(seed)),
        public_key = as_serde!(bytes_to_hex(public_key_bytes));
        "Issued host identity (IdBlock)"
    );

    Ok((id_block, auth_info))
}

pub fn extract_host_identity_fingerprint(release_dir: &Path) -> Result<Vec<u8>> {
    let mut id_key = release_dir.to_path_buf();
    id_key.push(ID_KEY_FILE_NAME);
    if !id_key.exists() {
        return Err(unexpected_err(format!("expected id key to exist: {id_key:?}"), None));
    }

    fingerprint_id_key(id_key.as_path()).map_err(|e| conversion_err(e, None))
}

pub fn write_identity_files(
    dest_dir: &Path, guest_vcpu_type: GuestCpuType, guest_vcpus: u16, assets: &Vec<String>,
) -> Result<()> {
    let mut dest = dest_dir.to_path_buf();
    dest.push("id");

    if !dest.exists() {
        fs::create_dir_all(dest.as_path()).map_err(|e| {
            io_err(e, Some(format!("failed to create dir for identity files: {dest:?}")))
        })?;
    }

    match guest_vcpu_type {
        GuestCpuType::EPYCv4 => {
            if assets.len() < 2 {
                return Err(validation_err(
                    "failed to write AMD SEV-SNP identity files, assets len < 2", None,
                ));
            }
            let id_block = assets.get(0).expect_or_err("expected assets.0 to exist")?;
            let auth_info = assets.get(1).expect_or_err("expected assets.1 to exist")?;

            let mut auth_info_dest = dest.clone();
            auth_info_dest.push(AUTH_INFO_FILE_NAME);

            let mut id_block_dest = dest.clone();
            id_block_dest.push(ID_BLOCK_FILE_NAME);

            let mut state_dest = dest.clone();
            state_dest.push("state");

            fs::write(id_block_dest.as_path(), id_block).map_err(|e| io_err(e, None))?;
            fs::write(auth_info_dest.as_path(), auth_info).map_err(|e| io_err(e, None))?;
            fs::write(state_dest.as_path(), format!("{guest_vcpu_type}:{guest_vcpus}"))
                .map_err(|e| io_err(e, None))
        }
    }
}
