use std::fs;
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};

use async_std::path::PathBuf;
use nu_ansi_term::Color::{Blue, Green, LightCyan};
use terminal_spinners::{SpinnerBuilder, SpinnerHandle, DOTS};

use lit_attestation::attestation::TryGenerate;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::envs::LitEnv;
use lit_core::config::LitConfig;
use lit_core::utils::ipfs::ipfs_cat;
use lit_os_core::guest::env::build::GuestBuildEnv;
use lit_os_core::guest::env::release::GuestReleaseEnv;
use lit_os_core::guest::types::{GuestCpuType, GuestType};
use lit_os_prov_api_client::api::release::ProvApiClientRelease;
use lit_os_prov_api_client::client::ProvApiClient;
use lit_os_prov_core::release::common::keys::write_identity_files;
use lit_os_prov_core::release::common::load::load_release_assets;
use lit_os_prov_core::release::common::manifest::{
    ReleaseManifest, ASSET_KEY_AMD_OVMF, ASSET_KEY_BUILD_ENV, ASSET_KEY_BUILD_META,
    ASSET_KEY_GUEST_DISK, ASSET_KEY_GUEST_INITRD, ASSET_KEY_GUEST_KERNEL,
    ASSET_KEY_GUEST_KERNEL_CMDLINE, ASSET_KEY_RELEASE_ENV,
};
use lit_os_prov_core::release::common::types::Release;
use lit_os_prov_core::release::issue::types::{
    IssueRelease, IssueReleaseRequest, IssueReleaseResponse,
};
use lit_os_prov_core::release::query::types::QueryReleaseRequest;

use crate::guest::common::{MANIFEST_FILE, MANIFEST_PROOF_FILE};

const RELEASE_INSTALL_ROOT_ASSETS: [&str; 4] = [
    ASSET_KEY_GUEST_KERNEL, ASSET_KEY_GUEST_KERNEL_CMDLINE, ASSET_KEY_GUEST_INITRD,
    ASSET_KEY_GUEST_DISK,
];
const RELEASE_INSTALL_BUILD_ASSETS: [&str; 2] = [ASSET_KEY_BUILD_ENV, ASSET_KEY_BUILD_META];
const RELEASE_INSTALL_RELEASE_ASSETS: [&str; 1] = [ASSET_KEY_RELEASE_ENV];
const RELEASE_INSTALL_AMD_ASSETS: [&str; 1] = [ASSET_KEY_AMD_OVMF];

#[allow(dead_code)]
pub(crate) struct IssuedRelease {
    pub(crate) issue: IssueReleaseResponse,
    pub(crate) release: Release,
    pub(crate) manifest: ReleaseManifest,
    pub(crate) release_env: GuestReleaseEnv,
    pub(crate) build_env: GuestBuildEnv,
}

impl IssuedRelease {
    pub(crate) fn new(
        issue: IssueReleaseResponse, release: Release, manifest: ReleaseManifest,
        release_env: GuestReleaseEnv, build_env: GuestBuildEnv,
    ) -> Self {
        Self { issue, release, manifest, release_env, build_env }
    }
}

#[rustfmt::skip]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn download_and_install_release(
    cfg: &LitConfig, resolver: &ContractResolver, opts: &CliGlobalOpts, instance_dir: &std::path::Path,
    subnet_id: &String, instance_env: LitEnv, instance_type: GuestType, instance_kind: Option<String>,
    guest_cpu_type: GuestCpuType, guest_vcpus: u16, download_admin_key: Option<String>
) -> IssuedRelease {
    let instance_dir = PathBuf::from(instance_dir);
    if instance_dir.exists().await {
        panic!("instance directory already exists! ({instance_dir:?})");
    }

    let client =
        ProvApiClient::new(cfg, Some(resolver)).await.expect("failed to construct ProvApiClient");

    // Request to query releases
    let req = QueryReleaseRequest::new(instance_env, guest_cpu_type)
        .with_type(instance_type, instance_kind.clone());

    let releases = client.query_releases(&req).await.expect("failed to query releases");

    let release_res = releases.releases.get(&instance_type);
    if release_res.is_none() {
        eprintln!(
            "No active releases found for; subnet: {}, env: {}, type: {}, kind: {:?}, cpu_type: {} (via: {})",
            subnet_id,
            &instance_env,
            &instance_type,
            &instance_kind,
            &guest_cpu_type,
            client.domain()
        );
        exit(1);
    }
    let release = release_res.unwrap();

    // Request to obtain IdBlock
    let download_admin_key = download_admin_key.unwrap_or_else(|| cfg.admin_key().expect("Invalid: admin key is required"));

    let unix_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("failed to get unix time");
    let noonce = unix_time.as_millis().to_le_bytes().to_vec();

    let issue_body = IssueRelease::new(release.id().clone(), guest_cpu_type, guest_vcpus);

    let request = IssueReleaseRequest::try_generate(cfg, (
        issue_body,
        Some(noonce),
        Some(download_admin_key),
    )).await
        .expect("failed to create attested request to obtain release issue (IdBlock)");

    let issued = client.issue_release(&request)
        .await.expect("failed to issue release");

    // Download the release
    if !opts.quiet() {
        println!(
            "🚀 Downloading release '{}' (type: {}, kind: {}, env: {}) from IPFS [{}]",
            Blue.paint(release.id()),
            Blue.paint(instance_type.to_string()),
            Blue.paint(instance_kind.clone().unwrap_or("N/A".to_string())),
            Blue.paint(instance_env.to_string()),
            cfg.ipfs_api()
        )
    }

    let (manifest, release_env) =
        load_release_assets(cfg, Some(resolver), release, false)
            .await.expect("failed to obtain release assets");

    let build_env = manifest.load_build_env(cfg).await.expect("failed to load build.env");

    // Create the instance dir.
    fs::create_dir_all(instance_dir.as_path())
        .unwrap_or_else(|_| panic!("failed to create dir: {:?}", &instance_dir));

    // Download assets
    download_assets(cfg, opts, &manifest, &instance_dir, None,
                    &RELEASE_INSTALL_ROOT_ASSETS, None).await;
    download_assets(cfg, opts, &manifest, &instance_dir, Some("build"),
                    &RELEASE_INSTALL_BUILD_ASSETS, None).await;
    download_assets(cfg, opts, &manifest, &instance_dir, Some("release"),
                    &RELEASE_INSTALL_RELEASE_ASSETS, None).await;
    download_assets(cfg, opts, &manifest, &instance_dir, Some("release"),
                    &[MANIFEST_FILE, MANIFEST_PROOF_FILE],
                    Some(&[release.manifest_cid().as_str(), manifest.proof().cid.as_str()]),
    ).await;

    match guest_cpu_type {
        GuestCpuType::EPYCv4 => {
            download_assets(
                cfg, opts, &manifest, &instance_dir, None, &RELEASE_INSTALL_AMD_ASSETS, None,
            )
                .await;
        }
    }

    // Write identity files
    write_identity_files(
        instance_dir.as_path().into(),
        guest_cpu_type,
        guest_vcpus,
        &issued.artifacts,
    )
        .expect("failed to write identity files");

    if !opts.quiet() {
        println!("🥳 Successfully installed release");
    }

    IssuedRelease::new(issued, release.clone(), manifest, release_env, build_env)
}

async fn download_assets(
    cfg: &LitConfig, opts: &CliGlobalOpts, manifest: &ReleaseManifest, instance_dir: &PathBuf,
    install_suffix: Option<&str>, assets: &[&str], from_cids: Option<&[&str]>,
) {
    let mut save_path = instance_dir.clone();
    if let Some(install_suffix) = install_suffix {
        save_path.push(install_suffix);
    }

    if let Some(from_cids) = from_cids {
        if from_cids.len() != assets.len() {
            panic!("download_assets: from_cids len != assets.len()");
        }
    }

    for i in 0..assets.len() {
        let asset_key = assets[i];

        let name = if let Some(install_suffix) = install_suffix {
            format!("{install_suffix}/{asset_key}")
        } else {
            asset_key.to_string()
        };

        let cid = if let Some(from_cids) = from_cids {
            from_cids[i].to_string()
        } else {
            let asset = manifest
                .get_asset(asset_key)
                .unwrap_or_else(|_| panic!("failed to get asset in manifest: {asset_key}"));

            asset.cid.clone()
        };

        let mut status_prefix: Option<String> = None;
        let mut handle: Option<SpinnerHandle> = None;
        if !opts.quiet() {
            let status_prefix_str =
                format!("📥 {}: {} ", Green.paint(&cid), LightCyan.paint(&name));
            handle = Some(
                SpinnerBuilder::new()
                    .spinner(&DOTS)
                    .prefix(status_prefix_str.clone())
                    .text("")
                    .start(),
            );
            status_prefix = Some(status_prefix_str);
        }

        let mut cur_path = save_path.clone();
        cur_path.push(asset_key);

        let cur_parent = cur_path.parent().unwrap();
        if !cur_parent.exists().await {
            fs::create_dir_all(cur_parent)
                .unwrap_or_else(|_| panic!("failed to create dir: {cur_parent:?}"));
        }

        ipfs_cat(cfg, cid, Some(cur_path.as_path()), None)
            .await
            .unwrap_or_else(|e| panic!("Error: {e} - failed to download asset: {name}"));

        if !opts.quiet() {
            handle.unwrap().stop();
            println!("{}💚️", status_prefix.unwrap())
        }
    }
}
