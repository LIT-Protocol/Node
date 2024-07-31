#[allow(unused_imports)]
use crate::cmd::os::guest::template::release::{do_release_template, GuestTemplateRelease};
use clap::{arg, Args, Subcommand};

use lit_cli_core::cmd::CliGlobalOpts;
use lit_cli_core::utils::command::child_wait_with_tee;
use lit_cli_core::utils::system::require_root;
use lit_cli_core::utils::uuid::unsafe_short_uuid;
use lit_core::config::envs::LitEnv;
use lit_core::config::LitConfig;
use lit_os_core::config::LitOsConfig;
use lit_os_core::guest::types::GuestType;

use lit_os_core::utils::validate::validate_host_name_part;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, fs};

use crate::config::LitCliOsConfig;
use crate::guest::template::find_one_guest_template;

#[derive(Clone, Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestTemplateCreate {
    #[command(subcommand)]
    command: GuestTemplateCreateCommands,
}

#[derive(Clone, Debug, Subcommand)]
#[command(arg_required_else_help = true)]
pub(crate) enum GuestTemplateCreateCommands {
    /// Create a node template
    #[command(arg_required_else_help = false)]
    Node(GuestTemplateCreateArgsNode),
    /// Create a provisioning template
    #[command(arg_required_else_help = false)]
    Prov(GuestTemplateCreateArgsProv),
    /// Create a build template
    #[command(arg_required_else_help = false)]
    Build(GuestTemplateCreateArgsBuild),
    /// Create a custom template
    #[command(arg_required_else_help = false)]
    Custom(GuestTemplateCreateArgsCustom),
}

#[derive(Clone, Debug, Args, Default)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestTemplateCreateArgsNode {
    #[command(flatten)]
    common: GuestTemplateCreateArgsCommon,
}

#[derive(Clone, Debug, Args, Default)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestTemplateCreateArgsProv {
    #[command(flatten)]
    pub(crate) common: GuestTemplateCreateArgsCommon,
}

#[derive(Clone, Debug, Args, Default)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestTemplateCreateArgsBuild {
    #[command(flatten)]
    common: GuestTemplateCreateArgsCommon,
}

#[derive(Clone, Debug, Args, Default)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestTemplateCreateArgsCustom {
    #[command(flatten)]
    common: GuestTemplateCreateArgsCommon,
    /// Guest instance kind
    #[arg(value_name = "KIND")]
    kind: String,
    /// Path to custom build specification directory
    #[arg(long, value_name = "CUSTOM_PATH")]
    custom_path: Option<String>,
}

#[derive(Clone, Debug, Args, Default)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestTemplateCreateArgsCommon {
    /// Build release environment
    #[arg(long, value_name = "ENV", value_enum)]
    pub(crate) build_release: Option<LitEnv>,
    /// Build directory
    #[arg(long, value_name = "DIR")]
    pub(crate) build_dir: Option<String>,
    /// Build size (i.e. 3G)
    #[arg(long, value_name = "SIZE")]
    pub(crate) build_size: Option<String>,
    /// Root size (i.e. 2G)
    #[arg(long, value_name = "SIZE")]
    pub(crate) root_size: Option<String>,
    /// Force read-only (for dev builds)
    #[arg(long)]
    pub(crate) ro: bool,
    /// No clean-up (preserve build directory)
    #[arg(long)]
    pub(crate) no_cleanup: bool,
    /// Release the template once it's built
    #[arg(long)]
    pub(crate) release: bool,
    /// Release Subnet ID
    #[arg(long, value_name = "SUBNET")]
    pub(crate) release_subnet_id: Option<String>,
    /// Do not pin release (useful for local testing)
    #[arg(long)]
    pub(crate) release_no_pinning: bool,
}

pub(crate) async fn handle_cmd_os_guest_template_create(
    cfg: LitConfig, opts: CliGlobalOpts, args: GuestTemplateCreate,
) -> bool {
    match args.command {
        GuestTemplateCreateCommands::Node(arg) => {
            do_os_guest_template_create(
                &cfg,
                &opts,
                GuestType::Node,
                arg.common.clone(),
                None,
                Some(arg.clone()),
                None,
            )
            .await
        }
        GuestTemplateCreateCommands::Prov(arg) => {
            do_os_guest_template_create(
                &cfg,
                &opts,
                GuestType::Prov,
                arg.common.clone(),
                Some(arg.clone()),
                None,
                None,
            )
            .await
        }
        GuestTemplateCreateCommands::Build(arg) => {
            do_os_guest_template_create(
                &cfg,
                &opts,
                GuestType::Build,
                arg.common.clone(),
                None,
                None,
                None,
            )
            .await
        }
        GuestTemplateCreateCommands::Custom(arg) => {
            do_os_guest_template_create(
                &cfg,
                &opts,
                GuestType::Custom,
                arg.common.clone(),
                None,
                None,
                Some(arg),
            )
            .await
        }
    }
}

pub(crate) async fn do_os_guest_template_create(
    cfg: &LitConfig, opts: &CliGlobalOpts, build_type: GuestType,
    common_args: GuestTemplateCreateArgsCommon, _prov_args: Option<GuestTemplateCreateArgsProv>,
    _node_args: Option<GuestTemplateCreateArgsNode>,
    custom_args: Option<GuestTemplateCreateArgsCustom>,
) -> bool {
    if !require_root() {
        return true;
    }

    let build_release = common_args.build_release.unwrap_or(*cfg.env());
    let build_id = unsafe_short_uuid().to_lowercase();
    let build_dir_str = match common_args.build_dir {
        Some(dir) => dir,
        None => {
            format!(
                "{}/{}/{}/{}",
                cfg.litos_var_guest_templates_dir().expect("failed to determine templates dir"),
                build_release,
                build_type,
                build_id
            )
        }
    };
    let build_dir = Path::new(&build_dir_str);
    let build_size = match common_args.build_size {
        Some(size) => size,
        None => match build_release {
            // Dev needs more space due to debug rust build.
            LitEnv::Dev => "15G".to_string(),
            _ => "8G".to_string(),
        },
    };
    let root_size = match common_args.root_size {
        Some(size) => size,
        None => match build_release {
            // Dev needs more space due to debug rust build.
            LitEnv::Dev => "7G".to_string(),
            _ => "5G".to_string(),
        },
    };

    let mut build_kind: Option<String> = None;
    let mut custom_path: Option<String> = None;
    if matches!(build_type, GuestType::Custom) {
        let custom_args = custom_args.as_ref().unwrap();

        if let Err(e) = validate_host_name_part(custom_args.kind.as_str(), Some(15)) {
            eprintln!("Invalid: --kind is invalid ({})", e);
            eprintln!();

            return false;
        }

        let _ = build_kind.insert(custom_args.kind.clone());

        if let Some(val) = custom_args.custom_path.as_ref() {
            let _ = custom_path.insert(val.clone());
        }
    }

    let mut admin_key: Option<String> = None;
    if common_args.release {
        admin_key = cfg.admin_key().ok();
    }

    cfg.verify_env_available(&build_release).expect("env verification failed");

    let create_cmd = cfg
        .litos_guest_template_build_cmd()
        .expect("failed to determine the lit os install location");

    // Ensure doesn't exist.
    if build_dir.exists() {
        panic!("build directory already exists! ({build_dir:?})");
    }

    // Create the build dir.
    fs::create_dir_all(build_dir).unwrap_or_else(|_| panic!("failed to create dir: {build_dir:?}"));

    env::set_current_dir(build_dir)
        .unwrap_or_else(|_| panic!("failed to change to dir: {build_dir:?}"));

    // Execute the build.
    let mut cmd = Command::new(create_cmd);
    cmd.arg("-id")
        .arg(&build_id)
        .arg("-path")
        .arg(&build_dir_str)
        .arg("-type")
        .arg(build_type.to_string())
        .arg("-release")
        .arg(build_release.to_string())
        .arg("-image-size")
        .arg(build_size)
        .arg("-root-size")
        .arg(root_size)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if LitEnv::Dev == build_release && common_args.ro {
        cmd.arg("-ro");
    }

    if common_args.no_cleanup {
        cmd.arg("-no-cleanup");
    }

    if opts.verbose() {
        cmd.arg("-verbose");
    }

    if let Some(build_kind) = build_kind {
        cmd.arg("-kind").arg(build_kind);
    }
    if let Some(custom_path) = custom_path {
        cmd.arg("-custom-path").arg(custom_path);
    }

    match cmd.spawn() {
        Ok(mut child) => {
            if let Some(status) =
                child_wait_with_tee(&mut child, "build.out", "build.err", opts.quiet())
            {
                if !status.success() {
                    if !opts.quiet() {
                        eprintln!();
                    }
                    eprintln!("Failed to create template (see output: {})", &build_dir_str);
                    return true;
                }
            }
        }
        Err(err) => {
            eprintln!("failed to spawn template create script: {cmd:?} - {err:?}");

            if build_dir.exists() {
                fs::remove_dir_all(build_dir).expect("failed to clean up deleting build dir")
            }

            return true;
        }
    }

    if common_args.release {
        if let Some(item) = find_one_guest_template(cfg, None, None, Some(&build_id), None) {
            let args = GuestTemplateRelease {
                id: build_id,
                subnet_id: common_args.release_subnet_id.clone(),
                no_pinning: common_args.release_no_pinning,
                push_only: false,
            };

            if !opts.quiet() {
                println!();
            }

            let _ = do_release_template(cfg, opts, args, &item, admin_key.clone()).await;
        } else {
            eprintln!("failed to find template id '{}' after build", &build_id);
        }
    }

    true
}
