use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use clap::Args;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use nu_ansi_term::Color::{Blue, Green, LightCyan};
use pinata_sdk::{ApiError, JobStatus, PinByHash};
use sha2::digest::Output;
use sha2::Sha512;
use terminal_spinners::{SpinnerBuilder, SpinnerHandle, DOTS};

use crate::config::LitCliOsConfig;
use crate::guest::common::{MANIFEST_FILE, MANIFEST_PROOF_FILE};
use lit_attestation::attestation::{TryGenerate, DATA_KEY_MANIFEST_HASH, DATA_KEY_UNIX_TIME};
use lit_attestation::{Attestation, AttestationType};
use lit_blockchain::resolver::contract::ContractResolver;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_cli_core::utils::file::{create_file, read_dir_recursive};
use lit_cli_core::utils::system::require_root;
use lit_core::config::envs::LitEnv;
use lit_core::config::LitConfig;
use lit_core::utils::binary::{bytes_to_hex, remove_0x_prefix};
use lit_core::utils::hash::sha512_file;
use lit_core::utils::ipfs::ipfs_cid_of_file;
use lit_core::utils::pem::ec_pub_pem_to_bytes;
use lit_os_core::guest::env::build::GuestBuildEnv;
use lit_os_core::guest::types::GuestCpuType;
use lit_os_core::utils::sev_snp::sev_snp_measure;
use lit_os_prov_api_client::api::release::ProvApiClientRelease;
use lit_os_prov_api_client::client::ProvApiClient;
use lit_os_prov_core::release::common::manifest::{
    load_release_manifest_file, ASSET_KEY_AMD_OVMF, ASSET_KEY_BUILD_PEM, ASSET_KEY_GUEST_INITRD,
    ASSET_KEY_GUEST_KERNEL, ASSET_KEY_GUEST_KERNEL_CMDLINE, ASSET_KEY_RELEASE_ENV,
    RELEASE_FILE_INITIAL_PASSWORD,
};
use lit_os_prov_core::release::create::types::{CreateRelease, CreateReleaseRequest};

use crate::guest::template::{find_one_guest_template, GuestTemplateItem};

const BUILD_LIT_CFG_FILE: &str = "build-lit-config.toml";

const MEASUREMENT_VCPUS: [u16; 10] = [1, 4, 8, 16, 24, 32, 48, 64, 128, 256];

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestTemplateRelease {
    #[arg(value_name = "ID", value_enum)]
    pub(crate) id: String,
    /// Subnet ID
    #[arg(long, value_name = "SUBNET")]
    pub(crate) subnet_id: Option<String>,
    /// Push only (do not release)
    #[arg(long)]
    pub(crate) push_only: bool,
    /// Do not pin (useful for local testing)
    #[arg(long)]
    pub(crate) no_pinning: bool,
}

pub(crate) async fn handle_cmd_os_guest_template_release(
    cfg: LitConfig, opts: CliGlobalOpts, args: GuestTemplateRelease,
) -> bool {
    if !require_root() {
        return true;
    }

    if args.id.is_empty() {
        return false;
    }

    if let Some(item) = find_one_guest_template(&cfg, None, None, Some(&args.id), None) {
        let admin_key = cfg.admin_key().ok();
        let _ = do_release_template(&cfg, &opts, args, &item, admin_key).await;
    }

    true
}

struct IpfsPinItem {
    cid: String,
    name: String,
    path: String,
}

impl IpfsPinItem {
    fn new(cid: String, name: String, path: String) -> Self {
        Self { cid, name, path }
    }
}

#[rustfmt::skip]
pub(crate) async fn do_release_template(
    cfg: &LitConfig, opts: &CliGlobalOpts, args: GuestTemplateRelease,
    item: &GuestTemplateItem, admin_key: Option<String>,
) -> CreateRelease {
    let template_path = item.path.clone();

    let ipfs_client = cfg.ipfs_client();

    // Load password
    let mut password_path = template_path.clone();
    password_path.push(RELEASE_FILE_INITIAL_PASSWORD);
    let password_bytes = fs::read(password_path.as_path())
        .unwrap_or_else(|_| panic!("failed to read build password: {:?}", &password_path));

    // Load public key
    let mut public_key_path = template_path.clone();
    public_key_path.push(ASSET_KEY_BUILD_PEM);
    let public_key_str = String::from_utf8(fs::read(public_key_path.as_path()).unwrap_or_else(|_| panic!("failed to read {}: {:?}", ASSET_KEY_BUILD_PEM, &public_key_path)))
    .expect("failed to read build pem as string");

    let public_key_bytes =
        ec_pub_pem_to_bytes(public_key_str.as_str()).expect("failed to parse build pem as der");

    // Verify build env.
    let should_var_hash = match item.build_env.build_release.as_ref() {
        Some(rel) => !rel.eq("dev"),
        _ => true,
    };

    item.build_env.verify(true, should_var_hash).expect("failed to verify build.env");

    let build_id = item.build_env.build_id.as_ref().unwrap();
    let build_release = item.build_env.build_release.as_ref().unwrap();
    let build_release_env = item.build_env.env().expect("failed to convert build env to LitEnv");

    cfg.verify_env_available(&build_release_env).expect("env verification failed");

    let subnet_id = remove_0x_prefix(&args.subnet_id.unwrap_or_else(|| {
        if !cfg.env().eq_str(build_release) {
            panic!("--subnet-id is required when releasing to a different environment than the default ({})", cfg.env());
        }

        cfg.subnet_id()
            .expect("--subnet-id is required (or set a default in the config: subnet.id)")
    })).to_lowercase();

    let release_id = format!("{}{}", build_id, &subnet_id);

    let mut release_path = template_path.clone();
    release_path.push(format!("releases/{subnet_id}"));

    fs::create_dir_all(&release_path).expect("failed to create release dir");

    let mut files: Vec<(PathBuf, PathBuf)> = Vec::new();

    read_dir_recursive(&template_path, None, &mut files)
        .unwrap_or_else(|_| panic!("failed to read files in path: {:?}", &item.path));

    files.sort_by_key(|(f, _)| f.clone());

    if !opts.quiet() {
        println!(
            "🚀 Releasing template '{}' (type: {}, kind: {}, env: {}) to IPFS [{}]",
            Blue.paint(item.build_env.build_id.as_ref().unwrap()),
            Blue.paint(item.build_env.build_type.as_ref().unwrap()),
            Blue.paint(item.build_env.build_kind.clone().unwrap_or("N/A".to_string())),
            Blue.paint(item.build_env.build_release.as_ref().unwrap()),
            cfg.ipfs_api()
        )
    }

    let name_prefix = format!(
        "{}/{}/{}",
        item.build_env.build_type.as_ref().unwrap(),
        item.build_env.build_release.as_ref().unwrap(),
        item.build_env.build_id.as_ref().unwrap()
    );

    let rel_name_prefix = format!("{}/{}", name_prefix, &subnet_id);

    let mut pins: Vec<IpfsPinItem> = Vec::new();

    for (abs_path, rel_path) in files {
        if !abs_path.is_file() {
            continue;
        }
        if rel_path.starts_with("releases/") {
            continue;
        }
        if rel_path.ends_with(ASSET_KEY_GUEST_KERNEL_CMDLINE) {
            // We build a custom cmdline based on this file for a release.
            continue;
        }
        if rel_path.ends_with(RELEASE_FILE_INITIAL_PASSWORD) {
            // Don't upload the password.
            continue;
        }
        if rel_path.ends_with(BUILD_LIT_CFG_FILE) {
            // Don't upload the lit config.
            continue;
        }

        add_to_ipfs_and_build_pins(abs_path, rel_path, &name_prefix, &mut pins, &ipfs_client,
            opts.quiet()
        )
            .await
            .unwrap();
    }

    let unix_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("failed to get unix time");

    let rel_cid = generate_release_env(&release_id, &subnet_id, &release_path,
        &rel_name_prefix, unix_time, &mut pins, &ipfs_client, opts.quiet())
        .await;

    generate_release_guest_cmdline(&release_id, &subnet_id, &template_path, &release_path,
                                   &rel_cid, &rel_name_prefix, &mut pins,
                                   &ipfs_client, opts.quiet())
        .await;

    let manifest_cid = generate_manifest_with_proof(cfg, &item.build_env, &release_id,
                                                    &subnet_id, &template_path, &release_path,
                                                    &rel_name_prefix, unix_time,
                                                    &mut pins, &ipfs_client,
                                                    admin_key.clone(), opts.quiet())
        .await;

    if !args.no_pinning && !pins.is_empty() {
        request_remote_pinning(cfg, &mut pins, opts.quiet()).await;
    }

    // Build request to create
    let request = CreateRelease::new(release_id, manifest_cid, password_bytes, public_key_bytes);

    if !args.push_only {
        submit_release(cfg, request.clone(), &subnet_id, admin_key, build_release_env, opts.quiet()).await;
    }

    request
}

async fn submit_release(
    cfg: &LitConfig, create_body: CreateRelease, subnet_id: &str, admin_key: Option<String>,
    env: LitEnv, quiet: bool,
) {
    let mut status_prefix: Option<String> = None;
    let mut handle: Option<SpinnerHandle> = None;
    if !quiet {
        let status_prefix_str =
            format!("🤜🏼 Submitting release: {} ", LightCyan.paint(create_body.release_id()));
        handle = Some(
            SpinnerBuilder::new().spinner(&DOTS).prefix(status_prefix_str.clone()).text("").start(),
        );
        status_prefix = Some(status_prefix_str);
    }

    // sleep for 30 secs so that IPFS has time to propagate the pinned files that we are about to "ipfs cat"
    async_std::task::sleep(Duration::from_secs(120)).await;

    let resolver = ContractResolver::new(subnet_id.to_owned(), env, None);
    let client =
        ProvApiClient::new(cfg, Some(&resolver)).await.expect("failed to create ProvApiClient");

    let unix_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("failed to get unix time");
    let noonce = unix_time.as_millis().to_le_bytes().to_vec();

    // Create attested request
    let request = CreateReleaseRequest::try_generate(cfg, (create_body, Some(noonce), admin_key))
        .await
        .expect("failed to create attested create release request");

    let response = client
        .create_release(&request)
        .await
        .expect("failed to call 'create_release' on ProvApiClient");

    if !response.success {
        panic!("failed to submit release ({}), success: {}", &response.release_id, response.success)
    }

    if !quiet {
        handle.unwrap().stop();
        println!("{}💚️", status_prefix.unwrap())
    }
}

async fn add_to_ipfs_and_build_pins(
    abs_path: PathBuf, rel_path: PathBuf, name_prefix: &String, pins: &mut Vec<IpfsPinItem>,
    ipfs_client: &IpfsClient, quiet: bool,
) -> Option<String> {
    if !abs_path.is_file() {
        return None;
    }

    let rel_path = rel_path.to_str().unwrap();

    let name = format!("{}/{}", &name_prefix, rel_path);

    let file =
        File::open(&abs_path).unwrap_or_else(|_| panic!("failed to open file: {:?}", &abs_path));

    let cid = ipfs_cid_of_file(&abs_path)
        .await
        .unwrap_or_else(|_| panic!("failed to calculate CID of file: {:?}", &abs_path));

    let mut status_prefix: Option<String> = None;
    let mut handle: Option<SpinnerHandle> = None;
    if !quiet {
        let status_prefix_str = format!("📤 {}: {} ", Green.paint(&cid), LightCyan.paint(&name));
        handle = Some(
            SpinnerBuilder::new().spinner(&DOTS).prefix(status_prefix_str.clone()).text("").start(),
        );
        status_prefix = Some(status_prefix_str);
    }

    let res = ipfs_client
        .add(file)
        .await
        .unwrap_or_else(|_| panic!("failed to add file ({:?}) to IPFS", &abs_path));
    if !res.hash.eq(&cid) {
        panic!(
            "Generated CID ({}) does not match added CID ({}) for file: {:?}",
            cid, res.hash, &abs_path
        );
    }

    pins.push(IpfsPinItem::new(cid.clone(), name, rel_path.to_string()));

    if !quiet {
        handle.unwrap().stop();
        println!("{}💚️", status_prefix.unwrap())
    }

    Some(cid)
}

#[allow(clippy::too_many_arguments)]
async fn generate_release_env(
    release_id: &String, subnet_id: &String, release_path: &Path, name_prefix: &String,
    unix_time: Duration, pins: &mut Vec<IpfsPinItem>, ipfs_client: &IpfsClient, quiet: bool,
) -> String {
    if !quiet {
        println!("📑 Generating release env '{}'", Blue.paint(ASSET_KEY_RELEASE_ENV));
    }

    let mut file_path = release_path.to_path_buf();
    file_path.push(ASSET_KEY_RELEASE_ENV);
    let mut file = create_file(&file_path);

    writeln!(file, "RELEASE_ID=\"{release_id}\"").unwrap();
    writeln!(file, "RELEASE_UNIX=\"{}\"", unix_time.as_secs()).unwrap();
    writeln!(file, "RELEASE_SUBNET_ID=\"{subnet_id}\"").unwrap();

    add_to_ipfs_and_build_pins(
        file_path,
        PathBuf::from(ASSET_KEY_RELEASE_ENV),
        name_prefix,
        pins,
        ipfs_client,
        quiet,
    )
    .await
    .unwrap()
}

#[allow(clippy::too_many_arguments)]
async fn generate_release_guest_cmdline(
    release_id: &String, subnet_id: &String, template_path: &Path, release_path: &Path,
    release_env_cid: &String, name_prefix: &String, pins: &mut Vec<IpfsPinItem>,
    ipfs_client: &IpfsClient, quiet: bool,
) -> String {
    if !quiet {
        println!("💻 Generating kernel cmdline '{}'", Blue.paint(ASSET_KEY_GUEST_KERNEL_CMDLINE));
    }

    let mut build_cmdline_path = template_path.to_path_buf();
    build_cmdline_path.push(ASSET_KEY_GUEST_KERNEL_CMDLINE);

    let build_cmdline = fs::read_to_string(&build_cmdline_path).unwrap_or_else(|_| {
        panic!("failed to read build guest cmdline: {ASSET_KEY_GUEST_KERNEL_CMDLINE}")
    });

    let mut file_path = release_path.to_path_buf();
    file_path.push(ASSET_KEY_GUEST_KERNEL_CMDLINE);
    let mut file = create_file(&file_path);

    writeln!(
        file,
        "{} litos.subnet_id={} litos.release_id={} litos.release_env={}",
        &build_cmdline, subnet_id, release_id, release_env_cid
    )
    .unwrap();

    add_to_ipfs_and_build_pins(
        file_path,
        PathBuf::from(ASSET_KEY_GUEST_KERNEL_CMDLINE),
        name_prefix,
        pins,
        ipfs_client,
        quiet,
    )
    .await
    .unwrap()
}

#[allow(clippy::too_many_arguments)]
async fn generate_manifest_with_proof(
    cfg: &LitConfig, build_env: &GuestBuildEnv, release_id: &String, subnet_id: &String,
    template_path: &Path, release_path: &Path, name_prefix: &String, unix_time: Duration,
    pins: &mut Vec<IpfsPinItem>, ipfs_client: &IpfsClient, admin_key: Option<String>, quiet: bool,
) -> String {
    if !quiet {
        println!("📋 Generating manifest '{}'", Blue.paint(MANIFEST_FILE));
    }

    let mut file_path = release_path.to_path_buf();
    file_path.push(MANIFEST_FILE);
    let mut file = create_file(&file_path);

    writeln!(file, "[meta]").unwrap();
    writeln!(file, "build_id = \"{}\"", build_env.build_id.as_ref().unwrap()).unwrap();
    writeln!(file, "build_type = \"{}\"", build_env.build_type.as_ref().unwrap()).unwrap();
    writeln!(file, "build_release = \"{}\"", build_env.build_release.as_ref().unwrap()).unwrap();
    writeln!(file, "build_date = \"{}\"", build_env.build_date.as_ref().unwrap()).unwrap();
    writeln!(file, "build_unix = \"{}\"", build_env.build_unix.as_ref().unwrap()).unwrap();
    writeln!(file, "build_uname = \"{}\"", build_env.build_uname.as_ref().unwrap()).unwrap();

    writeln!(file).unwrap();
    writeln!(file, "[release]").unwrap();
    writeln!(file, "id = \"{release_id}\"").unwrap();
    writeln!(file, "subnet_id = \"{subnet_id}\"").unwrap();
    writeln!(file, "unix = \"{}\"", unix_time.as_secs()).unwrap();

    writeln!(file).unwrap();
    writeln!(file, "[assets]").unwrap();

    for pin in pins.iter() {
        writeln!(file, "\"{}\" = {{ cid = \"{}\" }}", &pin.path, &pin.cid).unwrap();
    }

    writeln!(file).unwrap();
    writeln!(file, "[measurements]").unwrap();

    let vcpu_type = build_env.build_cpu_type.as_ref().unwrap();
    let vcpu_type = GuestCpuType::from_str(vcpu_type).unwrap();
    for vcpus in MEASUREMENT_VCPUS {
        writeln!(
            file,
            "\"{}\" = {{ vcpu_type = \"{}\", vcpus = {} }}",
            sev_snp_measure_build(vcpus as usize, vcpu_type, template_path, release_path),
            vcpu_type,
            vcpus
        )
        .unwrap();
    }

    // Write break before proof so we only need to look for [proof] when hashing.
    writeln!(file).unwrap();
    file.flush().unwrap();

    let file_hash = sha512_file(file_path.as_path())
        .unwrap_or_else(|_| panic!("failed to hash file: {:?}", file_path.as_path()));
    let proof_cid = generate_manifest_proof(
        cfg, file_hash, release_path, name_prefix, unix_time, pins, ipfs_client, admin_key, quiet,
    )
    .await;

    writeln!(file, "[proof]").unwrap();
    writeln!(file, "cid = \"{proof_cid}\"").unwrap();
    file.flush().unwrap();

    // Verify file
    let manifest = load_release_manifest_file(file_path.as_path().into())
        .await
        .expect("failed to read manifest file");
    let manifest_hash = manifest.hash().expect("manifest file missing hash").to_vec();

    if !manifest_hash.eq(&file_hash.to_vec()) {
        panic!(
            "generated manifest file hash differs: {} vs {}",
            bytes_to_hex(&manifest_hash),
            bytes_to_hex(file_hash)
        );
    }

    add_to_ipfs_and_build_pins(
        file_path,
        PathBuf::from(MANIFEST_FILE),
        name_prefix,
        pins,
        ipfs_client,
        quiet,
    )
    .await
    .unwrap()
}

#[allow(clippy::too_many_arguments)]
async fn generate_manifest_proof(
    cfg: &LitConfig, hash: Output<Sha512>, release_path: &Path, name_prefix: &String,
    unix_time: Duration, pins: &mut Vec<IpfsPinItem>, ipfs_client: &IpfsClient,
    admin_key: Option<String>, quiet: bool,
) -> String {
    if !quiet {
        println!("🧪 Generating manifest proof '{}'", Blue.paint(MANIFEST_PROOF_FILE));
    }

    let mut file_path = release_path.to_path_buf();
    file_path.push(MANIFEST_PROOF_FILE);

    let noonce = unix_time.as_secs().to_le_bytes().to_vec();
    let attestation_type = match cfg.litos_guest().ok() {
        Some(true) => AttestationType::from_system()
            .expect("failed to determine attestation type for this guest"),
        _ => AttestationType::AdminSigned,
    };

    let mut proof = Attestation::new(attestation_type, noonce.clone())
        .await
        .expect("failed to request Attestation");
    proof
        .insert_data(DATA_KEY_MANIFEST_HASH, hash.to_vec())
        .insert_data(DATA_KEY_UNIX_TIME, noonce);

    // NB: DO _NOT_ ADD RELEASE_ID data (that implies the signer release_id, not the release id for this release).
    // TODO: Refactor:
    // TODO: Use build key (or attestation-service if available, i.e. we're running in a build guest).
    // TODO: ALSO CHANGE: lit-attestation/src/attestation.rs 394
    // - for AmdSevSnp we need to add all of the fields that attestation-service uses.
    match attestation_type {
        AttestationType::AdminSigned => proof
            .sign(admin_key.expect("missing required config: admin.key").as_str())
            .expect("failed to sign proof"),
        _ => unimplemented!("attestation type is not implemented yet: {}", attestation_type),
    }

    let proof = proof.generate().await.expect("failed to generate manifest proof");
    proof
        .to_file(file_path.as_path())
        .unwrap_or_else(|_| panic!("failed to save manifest proof to: {file_path:?}"));

    add_to_ipfs_and_build_pins(
        file_path,
        PathBuf::from(MANIFEST_PROOF_FILE),
        name_prefix,
        pins,
        ipfs_client,
        quiet,
    )
    .await
    .unwrap()
}

async fn request_remote_pinning(cfg: &LitConfig, pins: &mut Vec<IpfsPinItem>, quiet: bool) {
    let total_pins = pins.len();

    let mut cur = 0;
    // TODO: Hardcoded to pinata atm.
    let pinata_sdk = cfg.pinata_client().expect("failed to load PinataApi client");
    let remote = "pinata";
    let remotes = vec![remote];
    for pin in pins.iter() {
        cur += 1;

        let mut handle: Option<SpinnerHandle> = None;
        if !quiet {
            handle = Some(
                SpinnerBuilder::new()
                    .spinner(&DOTS)
                    .prefix(format!(
                        "📌 Pinning '{}' ({} of {}) on '{}' ",
                        Green.paint(&pin.cid),
                        cur,
                        total_pins,
                        LightCyan.paint(remote)
                    ))
                    .text("")
                    .start(),
            );
        }

        // Request pinning
        match pinata_sdk
            .pin_by_hash(
                PinByHash::new(pin.cid.as_str())
                    .set_metadata_with_name(pin.name.as_str(), HashMap::new()),
            )
            .await
        {
            Ok(res) => match res.status {
                JobStatus::Prechecking | JobStatus::Searching | JobStatus::Retrieving => {} // Ok
                _ => {
                    panic!(
                        "bad status '{:?}' returned when requesting pin of {} on {}",
                        &res.status, &pin.cid, remote
                    );
                }
            },
            Err(err) => {
                match err {
                    ApiError::GenericError(err) => {
                        // DJR: I tried to implement this with a request to check the pin status
                        //      and encountered rate limiting. Rate limiting doesn't apply to errors. LOL.
                        if err.contains("DUPLICATE_OBJECT") {
                            // Ignore.
                        } else {
                            panic!(
                                "error when requesting pin of {} on {}: {:?}",
                                &pin.cid, remote, err
                            );
                        }
                    }
                    _ => {
                        panic!(
                            "exception when requesting pin of {} on {}: {:?}",
                            &pin.cid, remote, err
                        );
                    }
                }
            }
        }

        if !quiet {
            handle.unwrap().stop();
        }
    }

    println!(
        "📌 Requested pinning for {} CIDs on {} 💚️",
        Green.paint(total_pins.to_string()),
        LightCyan.paint(remotes.join(", "))
    )
}

// Util

fn sev_snp_measure_build(
    vcpus: usize, vcpu_type: GuestCpuType, template_path: &Path, release_path: &Path,
) -> String {
    let mut ovmf = template_path.to_path_buf();
    ovmf.push(ASSET_KEY_AMD_OVMF);
    let mut kernel = template_path.to_path_buf();
    kernel.push(ASSET_KEY_GUEST_KERNEL);
    let mut append = release_path.to_path_buf();
    append.push(ASSET_KEY_GUEST_KERNEL_CMDLINE);
    let mut initrd = template_path.to_path_buf();
    initrd.push(ASSET_KEY_GUEST_INITRD);

    sev_snp_measure(
        vcpus,
        vcpu_type,
        ovmf.as_path(),
        kernel.as_path(),
        append.as_path(),
        initrd.as_path(),
    )
    .expect("failed to obtain sev snp measurement")
}
