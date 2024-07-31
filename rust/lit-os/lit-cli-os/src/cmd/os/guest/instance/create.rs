use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;
#[allow(unused_imports)]
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs};

use clap::{arg, Args, Subcommand};
use ipnet::Ipv4Net;

use lit_blockchain::config::{
    CFG_KEY_BLOCKCHAIN_WALLET, CFG_SUB_KEY_DEFAULT, CFG_SUB_KEY_PRIVATE_KEY,
};
use lit_blockchain::resolver::contract::ContractResolver;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_cli_core::utils::command::child_wait_with_tee;
use lit_cli_core::utils::system::{chmod, require_root};
use lit_cli_core::utils::uuid::unsafe_short_uuid;
use lit_core::config::envs::LitEnv;
use lit_core::config::{read_sensitive_stdin, LitConfig};
use lit_core::utils::binary::remove_0x_prefix;
use lit_core::utils::toml::SimpleToml;
use lit_os_core::config::LitOsConfig;
use lit_os_core::guest::oneshot::ONESHOT_FILE_CONFIG;
use lit_os_core::guest::types::{GuestCpuType, GuestType};
use lit_os_core::utils::validate::{validate_host_name_part, validate_image_size, validate_label};

use crate::cmd::os::guest::instance::repair::repair_instance;
use crate::config::LitCliOsConfig;
use crate::guest::instance::helper::{can_enable_instance, GuestInstanceHelper};
use crate::guest::instance::oneshot::create_oneshot_actions;
use crate::guest::instance::release::{download_and_install_release, IssuedRelease};
use crate::guest::instance::{find_one_guest_instance, GuestInstanceItem};
use crate::guest::template::{find_guest_templates, GuestTemplateItem};

static CFG_KEY_SECTION_GUEST_INSTANCE: &str = "guest.instance";
static CFG_KEY_SECTION_BLOCKCHAIN_WALLET_DEFAULT: &str =
    const_str::concat!(CFG_KEY_BLOCKCHAIN_WALLET, ".", CFG_SUB_KEY_DEFAULT);
static CFG_KEY_SECTION_PROV_API: &str = "api.prov";
static CFG_KEY_SECTION_NODE: &str = "node";
static CFG_KEY_SECTION_ZEROSSL: &str = "zerossl";

#[derive(Clone, Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestInstanceCreate {
    #[command(subcommand)]
    command: GuestInstanceCreateCommands,
}

#[derive(Clone, Debug, Subcommand)]
#[command(arg_required_else_help = true)]
pub(crate) enum GuestInstanceCreateCommands {
    #[cfg(feature = "guest-instance")]
    /// Create a node instance
    #[command(arg_required_else_help = true)]
    Node(GuestInstanceCreateArgsNode),
    #[cfg(feature = "guest-build")]
    /// Create a provisioning instance
    #[command(arg_required_else_help = true)]
    #[cfg(feature = "guest-build")]
    Prov(GuestInstanceCreateArgsProv),
    #[cfg(feature = "guest-build")]
    /// Create a build instance
    #[command(arg_required_else_help = true)]
    #[cfg(feature = "guest-build")]
    Build(GuestInstanceCreateArgsBuild),
    /// Create a custom instance
    #[command(arg_required_else_help = true)]
    #[cfg(feature = "guest-build")]
    Custom(GuestInstanceCreateArgsCustom),
}

#[derive(Clone, Debug, Args, Default)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestInstanceCreateArgsNode {
    #[command(flatten)]
    common: GuestInstanceCreateArgsCommon,
    #[command(flatten)]
    pub(crate) api: GuestInstanceCreateArgsApi,
    #[arg(hide(true), long, default_value = None, value_name = "PATH")]
    #[deprecated(since = "0.1.1", note = "This arg is no longer needed/used, please remove")]
    node_gpg_public_key: Option<String>,
    /// Staker address
    #[arg(long, value_name = "STR")]
    node_staker_address: Option<String>,
    /// Admin address
    #[arg(long, value_name = "STR")]
    node_admin_address: Option<String>,
    /// Enter restore state
    #[arg(long, value_name = "BOOL")]
    node_enter_restore_state: Option<bool>,
    /// BLS key blinder
    #[arg(long, value_name = "STR")]
    node_bls_key_blinder: Option<String>,
    /// ECDSA key blinder
    #[arg(long, value_name = "STR")]
    node_ecdsa_key_blinder: Option<String>,
}

#[derive(Clone, Debug, Args, Default)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestInstanceCreateArgsProv {
    #[command(flatten)]
    pub(crate) common: GuestInstanceCreateArgsCommon,
    #[command(flatten)]
    pub(crate) api: GuestInstanceCreateArgsApi,
    /// Bootstrap provisioning from local build.
    #[cfg(feature = "guest-build")]
    #[arg(long)]
    pub(crate) bootstrap: bool,
    /// Do not pin during bootstrap (useful for local testing)
    #[cfg(feature = "guest-build")]
    #[arg(long)]
    pub(crate) no_pinning: bool,
}

#[derive(Clone, Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[cfg(feature = "guest-build")]
pub(crate) struct GuestInstanceCreateArgsBuild {
    #[command(flatten)]
    common: GuestInstanceCreateArgsCommon,
}

#[derive(Clone, Debug, Args, Default)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestInstanceCreateArgsCustom {
    #[command(flatten)]
    common: GuestInstanceCreateArgsCommon,
    /// instance kind
    #[arg(value_name = "KIND")]
    kind: String,
    /// custom path source
    #[arg(long, value_name = "PATH")]
    custom_path: Option<String>,
    /// custom parameters
    #[arg(long, short, value_name = "STR", value_enum)]
    param: Vec<String>,
}

#[derive(Clone, Debug, Args, Default)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestInstanceCreateArgsCommon {
    /// Provide a name suffix for this instance to help identify it.
    #[arg(long, value_name = "STR", value_enum)]
    pub(crate) name: Option<String>,
    /// Provide a label for this instance.
    #[arg(long, short, value_name = "STR", value_enum)]
    pub(crate) label: Vec<String>,
    /// Environment.
    #[arg(long, value_name = "ENV", value_enum)]
    pub(crate) env: Option<LitEnv>,
    /// Subnet ID to use.
    #[arg(long, value_name = "SUBNET")]
    pub(crate) subnet_id: Option<String>,
    /// Template ID to use.
    #[arg(long, value_name = "TEMPLATE_ID")]
    pub(crate) template_id: Option<String>,
    /// Instance ID to use as a base.
    #[arg(long, value_name = "INSTANCE_ID")]
    pub(crate) from_id: Option<String>,
    /// Number of VCPUs to allocate.
    #[arg(long, value_name = "NUM")]
    pub(crate) vcpus: Option<i64>,
    /// Memory to allocate (i.e. 32G).
    #[arg(long, value_name = "STR")]
    pub(crate) mem: Option<String>,
    // TODO: Validate img_size
    /// Extra storage to allocate to image (i.e. 50G).
    #[arg(long, value_name = "STR")]
    pub(crate) img_size: Option<String>,
    /// Do not resize the guest image.
    #[arg(long)]
    pub(crate) no_resize: bool,
    /// IPv4 interface IP address.
    #[arg(long, value_name = "STR")]
    pub(crate) net4_ip: Option<String>,
    /// IPv4 gateway IP address.
    #[arg(long, value_name = "STR")]
    pub(crate) net4_gw: Option<String>,
    /// IPv6 interface IP address.
    #[arg(long, value_name = "STR")]
    pub(crate) net6_ip: Option<String>,
    /// IPv6 gateway IP address.
    #[arg(long, value_name = "STR")]
    pub(crate) net6_gw: Option<String>,
    /// IPv6 use dhcp.
    #[arg(long)]
    pub(crate) net6_dhcp: bool,
    /// Do not automatically start guest.
    #[arg(long)]
    pub(crate) no_start: bool,
    /// Follow logs (when automatically starting).
    #[arg(long, short)]
    pub(crate) follow: bool,
    /// Automatically generate a self-signed identity.
    #[cfg(feature = "guest-build")]
    #[arg(long)]
    pub(crate) self_signed: bool,
    /// Safe boot (no network, for self-signed only).
    #[cfg(feature = "guest-build")]
    #[arg(long)]
    pub(crate) safe_boot: bool,
    /// Always prompt for the admin key to download the release.
    #[arg(long)]
    pub(crate) prompt_admin_key: bool,
}

#[derive(Clone, Debug, Args, Default)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestInstanceCreateArgsApi {
    /// API domain name.
    #[arg(long, value_name = "STR", value_enum)]
    api_domain: Option<String>,
    /// Do not prefix API domain with env for test/dev.
    #[arg(long)]
    api_domain_no_prefix: bool,
    /// ZeroSSL API Key.
    #[arg(long, value_name = "STR", value_enum)]
    zerossl_api_key: Option<String>,
}

pub(crate) async fn handle_cmd_os_guest_instance_create(
    cfg: LitConfig, opts: CliGlobalOpts, args: GuestInstanceCreate,
) -> bool {
    match args.command {
        GuestInstanceCreateCommands::Node(arg) => {
            do_os_guest_instance_create(
                &cfg,
                &opts,
                GuestType::Node,
                arg.common.clone(),
                None,
                Some(arg.clone()),
                Some(arg.api),
                None,
            )
            .await
            .0
        }
        #[cfg(feature = "guest-build")]
        GuestInstanceCreateCommands::Prov(arg) => {
            do_os_guest_instance_create(
                &cfg,
                &opts,
                GuestType::Prov,
                arg.common.clone(),
                Some(arg.clone()),
                None,
                Some(arg.api),
                None,
            )
            .await
            .0
        }
        #[cfg(feature = "guest-build")]
        GuestInstanceCreateCommands::Build(arg) => {
            do_os_guest_instance_create(
                &cfg,
                &opts,
                GuestType::Build,
                arg.common,
                None,
                None,
                None,
                None,
            )
            .await
            .0
        }
        #[cfg(feature = "guest-build")]
        GuestInstanceCreateCommands::Custom(arg) => {
            do_os_guest_instance_create(
                &cfg,
                &opts,
                GuestType::Custom,
                arg.common.clone(),
                None,
                None,
                None,
                Some(arg.clone()),
            )
            .await
            .0
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn do_os_guest_instance_create(
    cfg: &LitConfig, opts: &CliGlobalOpts, instance_type: GuestType,
    common_args: GuestInstanceCreateArgsCommon, prov_args: Option<GuestInstanceCreateArgsProv>,
    node_args: Option<GuestInstanceCreateArgsNode>, api_args: Option<GuestInstanceCreateArgsApi>,
    custom_args: Option<GuestInstanceCreateArgsCustom>,
) -> (bool, Option<String>) {
    if !require_root() {
        return (true, None);
    }

    let mut from_item: Option<GuestInstanceItem> = None;
    if let Some(instance_id) = common_args.from_id.as_ref() {
        match find_one_guest_instance(cfg, None, None, Some(instance_id)) {
            Some(item) => {
                // Verify from instance item.
                #[allow(clippy::expect_fun_call)]
                item.instance_env.verify().expect(
                    format!(
                        "Instance item ({}) for --from-id is invalid (instance.env)",
                        instance_id
                    )
                    .as_str(),
                );
                #[allow(clippy::expect_fun_call)]
                item.build_env.verify(false, false).expect(
                    format!("Instance item ({}) for --from-id is invalid (build.env)", instance_id)
                        .as_str(),
                );

                let _ = from_item.insert(item);
            }
            None => {
                eprintln!("Invalid: --from-id ({}) is invalid (instance not found)", instance_id);
                eprintln!();

                return (false, None);
            }
        }
    }

    let subnet_id: Option<String>;
    let instance_env: LitEnv;
    let guest_vcpus: u16;
    let guest_mem: String;
    let guest_img_size: String;
    let name: Option<String>;
    let label: Vec<String>;
    if let Some(from_item) = from_item.as_ref() {
        // subnet_id
        subnet_id = if common_args.subnet_id.is_some() {
            common_args.subnet_id.clone()
        } else {
            from_item.instance_env.subnet_id.clone()
        };

        // env
        instance_env = if let Some(v) = common_args.env.as_ref() {
            *v
        } else {
            from_item.build_env.env().unwrap()
        };

        // guest options
        guest_vcpus =
            common_args.vcpus.or(from_item.instance_env.instance_vcpus_i64()).unwrap() as u16;
        guest_mem =
            common_args.mem.clone().or(from_item.instance_env.instance_mem.clone()).unwrap();
        guest_img_size = common_args
            .img_size
            .clone()
            .or(from_item.instance_env.instance_img_size.clone())
            .unwrap();
        name = if common_args.name.is_some() {
            common_args.name.clone()
        } else {
            from_item.instance_env.instance_name_suffix.clone()
        };
        label = if !common_args.label.is_empty() {
            common_args.label.clone()
        } else {
            from_item.instance_env.labels()
        };
    } else {
        // subnet_id
        subnet_id = common_args.subnet_id.clone().or(cfg.subnet_id().ok());
        if subnet_id.is_none() {
            eprintln!("Invalid: --subnet-id must be provided (or 'subnet.id' defined in cfg)");
            eprintln!();

            return (false, None);
        }

        // env
        instance_env = common_args.env.unwrap_or(*cfg.env());

        // guest options
        guest_vcpus = common_args.vcpus.or(cfg.litos_guest_default_vcpus().ok()).unwrap() as u16;
        guest_mem = common_args.mem.clone().or(cfg.litos_guest_default_mem().ok()).unwrap();
        guest_img_size =
            common_args.img_size.clone().or(cfg.litos_guest_default_img_size().ok()).unwrap();
        name = common_args.name.clone();
        label = common_args.label.clone();
    }

    let subnet_id = remove_0x_prefix(subnet_id.unwrap().to_lowercase());

    cfg.verify_env_available(&instance_env).expect("env verification failed");

    let resolver = ContractResolver::new(subnet_id.clone(), instance_env, None);

    let instances_dir_str =
        cfg.litos_var_guest_instances_dir().expect("failed to determine instances dir");
    let instance_id = unsafe_short_uuid().to_lowercase();
    let instance_dir_str =
        format!("{}/{}/{}/{}", instances_dir_str, instance_env, instance_type, instance_id);
    let instance_dir = Path::new(&instance_dir_str);
    let mut instance_ci_dir = PathBuf::from(instance_dir);
    instance_ci_dir.push("cloud-init");
    let mut instance_etc_file = PathBuf::from(&instance_ci_dir);
    instance_etc_file.push("config.toml");

    // Special opts
    let (self_signed, safe_boot, prov_bootstrap) =
        extract_special_opts(cfg, &common_args, prov_args.as_ref(), api_args.as_ref());

    // Validate
    if self_signed && instance_env != LitEnv::Dev {
        eprintln!("Invalid: --self-signed can only be used with env: {}", LitEnv::Dev);
        eprintln!();

        return (false, None);
    }
    if safe_boot && !self_signed {
        eprintln!("Invalid: --safe-boot can only be used with --self-signed");
        eprintln!();

        return (false, None);
    }

    if validate_image_size(&guest_img_size).is_err() {
        eprintln!("Invalid: --img-size is invalid (must be of format [0-9]+G).");
        eprintln!();

        return (false, None);
    }

    if (common_args.net4_ip.is_some() || common_args.net4_gw.is_some())
        && (common_args.net4_ip.is_none() || common_args.net4_gw.is_none())
    {
        eprintln!("Invalid: --net4-ip and --net4-gw must be provided as a pair.");
        eprintln!();

        return (false, None);
    }

    if common_args.net6_dhcp {
        if common_args.net6_ip.is_some() || common_args.net6_gw.is_some() {
            eprintln!("Invalid: --net6-dhcp may not be provided with --net6-ip or --net6-gw.");
            eprintln!();

            return (false, None);
        }
    } else if (common_args.net6_ip.is_some() || common_args.net6_gw.is_some())
        && (common_args.net6_ip.is_none() || common_args.net6_gw.is_none())
    {
        eprintln!("Invalid: --net6-ip and --net6-gw must be provided as a pair.");
        eprintln!();

        return (false, None);
    }

    if let Some(name) = name.as_ref() {
        if let Err(e) = validate_host_name_part(name, Some(15)) {
            eprintln!("Invalid: --name is invalid ({})", e);
            eprintln!();

            return (false, None);
        }
    }

    if !label.is_empty() {
        for label in label.iter() {
            if let Err(e) = validate_label(label) {
                eprintln!("Invalid: --label is invalid ({})", e);
                eprintln!();

                return (false, None);
            }
        }
    }

    // Prepare donor instance (stop, disable, repair)
    // - This may change, but we disable to prevent the memory usage being counted.
    if let Some(from_item) = from_item.as_ref() {
        if !opts.quiet() {
            println!("Preparing instance for copy: {:?}", &from_item.path);
        }

        // NB: repair must be performed first (otherwise it re-enables the service).
        repair_instance(cfg, from_item, opts);

        #[allow(clippy::expect_fun_call)]
        from_item.stop().expect(
            format!(
                "failed to stop service for --from-id instance ({})",
                from_item.instance_env.instance_id.as_ref().unwrap()
            )
            .as_str(),
        );

        #[allow(clippy::expect_fun_call)]
        from_item.disable().expect(
            format!(
                "failed to disable service for --from-id instance ({})",
                from_item.instance_env.instance_id.as_ref().unwrap()
            )
            .as_str(),
        );
    }

    // Verify can enable
    can_enable_instance(cfg, &guest_mem).expect("unable to create/enable new instance");

    // Create config and verify arguments
    let (cfg_res, custom_cfg) = create_lit_custom_cfg(
        cfg,
        opts,
        instance_type,
        instance_env,
        &common_args,
        prov_args.as_ref(),
        node_args.as_ref(),
        api_args.as_ref(),
        custom_args.as_ref(),
        &instance_id,
        from_item.as_ref(),
    );
    if let Some(res) = cfg_res {
        return (res, None);
    }

    // Find template or release
    let mut release: Option<IssuedRelease> = None;
    let mut template: Option<GuestTemplateItem> = None;
    let mut admin_key: Option<String> = None;
    if prov_bootstrap || self_signed {
        if let Some(items) = find_guest_templates(
            cfg,
            Some(&instance_type.to_string()),
            Some(&instance_env.to_string()),
            common_args.template_id.as_deref(),
            custom_args.as_ref().map(|a| a.kind.clone()).as_deref(),
        ) {
            // Get the first (latest) matching template.
            template = Some(items.get(0).expect("expected to have template items").clone());
        } else {
            eprintln!(
                "No template id's found matching; type: {:?}, env: {:?}, id: {:?}",
                instance_type.to_string(),
                instance_env.to_string(),
                common_args.template_id.as_ref()
            );
            return (true, None);
        }

        // Admin keys
        if !self_signed {
            match cfg.admin_key() {
                Ok(key) => {
                    let _ = admin_key.insert(key);
                }
                Err(_e) => {
                    eprintln!("Invalid: admin key is required");
                    eprintln!();

                    return (false, None);
                }
            }
        }
    } else {
        let mut download_admin_key: Option<String> = None;
        if !common_args.prompt_admin_key && matches!(instance_type, GuestType::Node) {
            if let Some(custom_cfg) = custom_cfg.as_ref() {
                if let Some(private_key) = custom_cfg
                    .get(CFG_KEY_SECTION_BLOCKCHAIN_WALLET_DEFAULT, CFG_SUB_KEY_PRIVATE_KEY)
                {
                    let _ = download_admin_key.insert(private_key.clone());
                }
            }
        }

        release = Some(
            download_and_install_release(
                cfg,
                &resolver,
                opts,
                instance_dir,
                &subnet_id,
                instance_env,
                instance_type,
                custom_args.as_ref().map(|a| a.kind.clone()),
                GuestCpuType::EPYCv4, // Hardcoded for now.
                guest_vcpus,
                download_admin_key,
            )
            .await,
        );
    }

    if release.is_none() {
        // Ensure doesn't exist.
        if instance_dir.exists() {
            panic!("instance directory already exists! ({instance_dir:?})");
        }

        // Create the instance dir.
        fs::create_dir_all(instance_dir)
            .unwrap_or_else(|_| panic!("failed to create dir: {instance_dir:?}"));
    }

    env::set_current_dir(instance_dir)
        .unwrap_or_else(|_| panic!("failed to change to dir: {instance_dir:?}"));

    // Create cloud-init dir (may need to write files during oneshot)
    fs::create_dir_all(&instance_ci_dir)
        .unwrap_or_else(|_| panic!("failed to create dir: {:?}", &instance_ci_dir));

    // Create oneshot config
    let oneshot_cfg = create_oneshot_actions(
        cfg,
        opts,
        &subnet_id,
        instance_type,
        template.as_ref(),
        &instance_ci_dir,
        prov_args.as_ref(),
        guest_vcpus,
        admin_key,
    )
    .await;

    if let Some(custom_cfg) = custom_cfg {
        // Write custom config
        if !custom_cfg.is_empty() {
            custom_cfg.write_file(instance_etc_file.as_path()).unwrap_or_else(|_| {
                panic!("failed to write custom config file: {instance_etc_file:?}")
            });

            chmod(instance_etc_file.as_path(), &"600".to_string());
        }
    }

    // Write oneshot
    if !oneshot_cfg.is_empty() {
        let mut instance_oneshot_dir = PathBuf::from(instance_dir);
        instance_oneshot_dir.push("oneshot.input");
        let mut instance_oneshot_conf_file = PathBuf::from(&instance_oneshot_dir);
        instance_oneshot_conf_file.push(ONESHOT_FILE_CONFIG);

        fs::create_dir_all(&instance_oneshot_dir)
            .unwrap_or_else(|_| panic!("failed to create dir: {:?}", &instance_oneshot_dir));
        oneshot_cfg.write_file(instance_oneshot_conf_file.as_path()).unwrap_or_else(|_| {
            panic!("failed to write oneshot config file: {instance_oneshot_conf_file:?}")
        });

        chmod(instance_oneshot_conf_file.as_path(), &"600".to_string());
    }

    // Construct create command
    let create_cmd = cfg
        .litos_guest_instance_create_cmd()
        .expect("failed to determine the lit os install location");

    let mut cmd = Command::new(create_cmd);
    cmd.arg("-id").arg(&instance_id).arg("-path").arg(&instance_dir_str);

    if let Some(_release) = release.as_ref() {
        cmd.arg("-release");
    } else if let Some(template) = template.as_ref() {
        cmd.arg("-template").arg(&template.path);
    } else {
        panic!("Unexpected: neither release nor template is present!");
    }

    if let Some(name) = name {
        cmd.arg("-name").arg(name);
    }

    if !label.is_empty() {
        cmd.arg("-labels").arg(label.join(" "));
    }

    cmd.arg("-subnet-id")
        .arg(&subnet_id)
        .arg("-vcpus")
        .arg(&guest_vcpus.to_string())
        .arg("-mem")
        .arg(&guest_mem)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(from_item) = from_item.as_ref() {
        cmd.arg("-from-path").arg(from_item.path.clone());
    }

    if common_args.no_resize {
        cmd.arg("-no-img-resize");
    } else {
        cmd.arg("-img-size").arg(&guest_img_size);
    }

    if let Some(net_ip) = common_args.net4_ip {
        cmd.arg("-net4-ip").arg(net_ip).arg("-net4-gw").arg(common_args.net4_gw.unwrap());
    }
    if common_args.net6_dhcp {
        cmd.arg("-net6-dhcp");
    } else if let Some(net_ip) = common_args.net6_ip {
        cmd.arg("-net6-ip").arg(net_ip).arg("-net6-gw").arg(common_args.net6_gw.unwrap());
    }
    #[cfg(feature = "guest-build")]
    if self_signed {
        cmd.arg("-self-signed");

        if safe_boot {
            cmd.arg("-safe-boot");
        }
    }
    if !oneshot_cfg.is_empty() {
        cmd.arg("-oneshot");
        if prov_bootstrap {
            // Needed to read the id block e.t.c. from oneshot.
            cmd.arg("-bootstrap");
        }
    }
    if !common_args.no_start {
        cmd.arg("-start");
        if common_args.follow {
            cmd.arg("-f");
        }
    }

    // Execute create.
    match cmd.spawn() {
        Ok(mut child) => {
            if let Some(status) =
                child_wait_with_tee(&mut child, "create.out", "create.err", opts.quiet())
            {
                if !status.success() {
                    if !opts.quiet() {
                        eprintln!();
                    }
                    eprintln!("Failed to create instance (see output: {})", &instance_dir_str);
                }
            }
        }
        Err(err) => {
            eprintln!("failed to spawn instance create script: {cmd:?} - {err:?}");

            if instance_dir.exists() {
                fs::remove_dir_all(instance_dir).expect("failed to clean up deleting instance dir")
            }
        }
    }

    (true, Some(instance_id))
}

#[allow(clippy::too_many_arguments)]
fn create_lit_custom_cfg<S: AsRef<str> + Into<String>>(
    cfg: &LitConfig, _opts: &CliGlobalOpts, instance_type: GuestType, instance_env: LitEnv,
    common_args: &GuestInstanceCreateArgsCommon, _prov_args: Option<&GuestInstanceCreateArgsProv>,
    node_args: Option<&GuestInstanceCreateArgsNode>, api_args: Option<&GuestInstanceCreateArgsApi>,
    custom_args: Option<&GuestInstanceCreateArgsCustom>, instance_id: S,
    from_item: Option<&GuestInstanceItem>,
) -> (Option<bool>, Option<SimpleToml>) {
    if let Some(from) = from_item {
        let mut custom_cfg = from.load_config().unwrap_or_else(|e| {
            panic!("failed to load config.toml from donor guest ({:?}): {e}", from.path.as_path())
        });

        custom_cfg.insert(CFG_KEY_SECTION_GUEST_INSTANCE.into(), "id".into(), instance_id.into());

        (None, Some(custom_cfg))
    } else {
        let mut custom_cfg = SimpleToml::new();

        custom_cfg.insert(CFG_KEY_SECTION_GUEST_INSTANCE.into(), "id".into(), instance_id.into());

        if GuestType::Custom.eq(&instance_type) {
            let custom_args = custom_args.as_ref().unwrap();

            custom_cfg
                .apply_params(custom_args.param.as_ref())
                .expect("failed to apply custom params");
        } else {
            // Wallet
            let wallet_key_cfg_key =
                format!("{}.wallet.private_key", instance_type.to_string().to_lowercase());
            let mut wallet_key = cfg.get_string(&wallet_key_cfg_key).ok();
            if wallet_key.is_none() {
                wallet_key = read_sensitive_stdin("private key to node wallet", Some(64), true).ok()
            }
            if wallet_key.is_none() {
                eprintln!(
                    "Invalid: private key to node wallet must be provided (or '{wallet_key_cfg_key}' defined in cfg)"
                );
                eprintln!();

                return (Some(false), None);
            }
            let wallet_key = wallet_key.unwrap();

            custom_cfg.insert(
                CFG_KEY_SECTION_BLOCKCHAIN_WALLET_DEFAULT.into(),
                CFG_SUB_KEY_PRIVATE_KEY.into(),
                wallet_key,
            );

            // Type specific
            if [GuestType::Prov, GuestType::Node].contains(&instance_type) {
                let api_args = api_args.as_ref().unwrap();

                let api_section = match instance_type {
                    GuestType::Prov => CFG_KEY_SECTION_PROV_API.to_string(),
                    GuestType::Node => CFG_KEY_SECTION_NODE.to_string(),
                    _ => unreachable!("expected match here"),
                };

                if let Some(domain) = api_args
                    .api_domain
                    .clone()
                    .or_else(|| match instance_type {
                        GuestType::Prov => cfg.litos_guest_default_prov_api_domain().ok(),
                        GuestType::Node => common_args.net4_ip.as_ref().map(|ip| {
                            Ipv4Net::from_str(ip)
                                .expect("--net4-ip argument is invalid")
                                .addr()
                                .to_string()
                        }),
                        _ => unreachable!("expected match here"),
                    })
                    .map(|v| {
                        if api_args.api_domain_no_prefix
                            || instance_type == GuestType::Node
                            || instance_env == LitEnv::Prod
                        {
                            v
                        } else {
                            format!("{instance_env}.{v}")
                        }
                    })
                    .map(|v| v.to_lowercase())
                {
                    custom_cfg.insert(api_section, "domain".into(), domain);
                } else {
                    eprintln!("Invalid: --api-domain required for instance type {instance_type}");
                    eprintln!();

                    return (Some(false), None);
                }

                if let Some(zerossl_api_key) = api_args
                    .zerossl_api_key
                    .clone()
                    .or(cfg.litos_guest_default_zerossl_api_key().ok())
                {
                    custom_cfg.insert(
                        CFG_KEY_SECTION_ZEROSSL.into(),
                        "api_key".into(),
                        zerossl_api_key,
                    );
                } else {
                    eprintln!(
                        "Invalid: --zerossl-api-key required for instance type {instance_type}"
                    );
                    eprintln!();

                    return (Some(false), None);
                }
            }

            if GuestType::Node.eq(&instance_type) {
                let node_args = node_args.as_ref().unwrap();
                if node_args.node_staker_address.is_none() {
                    eprintln!(
                        "Invalid: --node-staker-address required for instance type {instance_type}"
                    );
                    eprintln!();

                    return (Some(false), None);
                }
                if node_args.node_admin_address.is_none() {
                    eprintln!(
                        "Invalid: --node-admin-address required for instance type {instance_type}"
                    );
                    eprintln!();

                    return (Some(false), None);
                }

                custom_cfg
                    .insert(
                        CFG_KEY_SECTION_NODE.into(),
                        "staker_address".into(),
                        node_args.node_staker_address.clone().unwrap(),
                    )
                    .insert(
                        CFG_KEY_SECTION_NODE.into(),
                        "admin_address".into(),
                        node_args.node_admin_address.clone().unwrap(),
                    );

                if let Some(b) = node_args.node_enter_restore_state {
                    custom_cfg.insert(
                        CFG_KEY_SECTION_NODE.into(),
                        "enter_restore_state".into(),
                        b.to_string(),
                    );
                }

                if let Some(b) = &node_args.node_bls_key_blinder {
                    custom_cfg.insert(
                        CFG_KEY_SECTION_NODE.into(),
                        "bls_key_blinder".into(),
                        b.clone(),
                    );
                }

                if let Some(b) = &node_args.node_ecdsa_key_blinder {
                    custom_cfg.insert(
                        CFG_KEY_SECTION_NODE.into(),
                        "ecdsa_key_blinder".into(),
                        b.clone(),
                    );
                }

                let coms_keys_sender_privkey =
                    read_sensitive_stdin("coms sender private key", Some(64), true)
                        .expect("coms sender private key is required");
                let coms_keys_receiver_privkey =
                    read_sensitive_stdin("coms receiver private key", Some(64), true)
                        .expect("coms receiver private key is required");

                custom_cfg
                    .insert(
                        CFG_KEY_SECTION_NODE.into(),
                        "coms_keys_sender_privkey".into(),
                        coms_keys_sender_privkey,
                    )
                    .insert(
                        CFG_KEY_SECTION_NODE.into(),
                        "coms_keys_receiver_privkey".into(),
                        coms_keys_receiver_privkey,
                    );
            }
        }

        (None, Some(custom_cfg))
    }
}

#[allow(unused_variables)]
fn extract_special_opts(
    _cfg: &LitConfig, common_args: &GuestInstanceCreateArgsCommon,
    prov_args: Option<&GuestInstanceCreateArgsProv>,
    _api_args: Option<&GuestInstanceCreateArgsApi>,
) -> (bool, bool, bool) {
    #[cfg(feature = "guest-build")]
    let prov_bootstrap = match prov_args.as_ref() {
        Some(arg) => arg.bootstrap,
        _ => false,
    };
    #[cfg(not(feature = "guest-build"))]
    let prov_bootstrap = false;

    #[cfg(feature = "guest-build")]
    let self_signed = common_args.self_signed;
    #[cfg(not(feature = "guest-build"))]
    let self_signed = false;

    #[cfg(feature = "guest-build")]
    let safe_boot = common_args.safe_boot;
    #[cfg(not(feature = "guest-build"))]
    let safe_boot = false;

    (self_signed, safe_boot, prov_bootstrap)
}
