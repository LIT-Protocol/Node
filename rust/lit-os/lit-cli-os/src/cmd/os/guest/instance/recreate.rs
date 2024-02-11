use std::str::FromStr;

use clap::Args;

use crate::cmd::os::guest::instance::create::{
    do_os_guest_instance_create, GuestInstanceCreateArgsApi, GuestInstanceCreateArgsCommon,
    GuestInstanceCreateArgsCustom, GuestInstanceCreateArgsNode, GuestInstanceCreateArgsProv,
};
use crate::cmd::os::guest::instance::delete::delete_instance;
use crate::cmd::os::guest::instance::logs::{handle_cmd_os_guest_instance_logs, GuestInstanceLogs};
use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::envs::LitEnv;
use lit_core::config::LitConfig;
use lit_os_core::guest::types::GuestType;

use crate::guest::instance::helper::GuestInstanceHelper;
use crate::guest::instance::{find_latest_guest_instance, find_one_guest_instance};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestInstanceRecreate {
    /// Instance Id or 'node', 'prov' etc for latest by type.
    #[arg(value_name = "ID", value_enum)]
    id: String,
    /// Provide a name suffix for this instance to help identify it.
    #[arg(long, value_name = "STR", value_enum)]
    name: Option<String>,
    /// Provide a label for this instance.
    #[arg(long, short, value_name = "STR", value_enum)]
    label: Vec<String>,
    /// Environment.
    #[arg(long, value_name = "ENV", value_enum)]
    env: Option<LitEnv>,
    /// Subnet ID to use.
    #[arg(long, value_name = "SUBNET")]
    subnet_id: Option<String>,
    /// Template ID to use.
    #[arg(long, value_name = "TEMPLATE_ID")]
    template_id: Option<String>,
    /// Number of VCPUs to allocate.
    #[arg(long, value_name = "NUM")]
    vcpus: Option<i64>,
    /// Memory to allocate (i.e. 32G).
    #[arg(long, value_name = "STR")]
    mem: Option<String>,
    // TODO: Validate img_size
    /// Extra storage to allocate to image (i.e. 50G).
    #[arg(long, value_name = "STR")]
    img_size: Option<String>,
    /// Do not resize the guest image.
    #[arg(long)]
    no_resize: bool,
    /// IPv4 interface IP address.
    #[arg(long, value_name = "STR")]
    net4_ip: Option<String>,
    /// IPv4 gateway IP address.
    #[arg(long, value_name = "STR")]
    net4_gw: Option<String>,
    /// IPv6 interface IP address.
    #[arg(long, value_name = "STR")]
    net6_ip: Option<String>,
    /// IPv6 gateway IP address.
    #[arg(long, value_name = "STR")]
    net6_gw: Option<String>,
    /// IPv6 use dhcp.
    #[arg(long)]
    net6_dhcp: bool,
    /// Do not automatically start guest.
    #[arg(long)]
    no_start: bool,
    /// Follow logs (when automatically starting).
    #[arg(long, short)]
    follow: bool,
    /// Automatically generate a self-signed identity.
    #[cfg(feature = "guest-build")]
    #[arg(long)]
    self_signed: bool,
    /// Safe boot (no network, for self-signed only).
    #[cfg(feature = "guest-build")]
    #[arg(long)]
    safe_boot: bool,
    /// Always prompt for the admin key to download the release.
    #[arg(long)]
    pub(crate) prompt_admin_key: bool,
    /// Do not automatically delete the old instance.
    #[arg(long, short)]
    preserve: bool,
}

pub(crate) async fn handle_cmd_os_guest_instance_recreate(
    cfg: LitConfig, opts: CliGlobalOpts, args: GuestInstanceRecreate,
) -> bool {
    if args.id.is_empty() {
        return false;
    }

    let guest_type = GuestType::from_str(&args.id).ok();
    let item = if guest_type.is_some() {
        find_latest_guest_instance(&cfg, Some(&args.id), None)
    } else {
        find_one_guest_instance(&cfg, None, None, Some(&args.id))
    };

    if let Some(item) = item {
        let guest_type = item.build_env.guest_type().expect("build.env missing GuestType");
        let common_args = GuestInstanceCreateArgsCommon {
            name: args.name.clone(),
            label: args.label.clone(),
            env: args.env,
            subnet_id: args.subnet_id.clone(),
            template_id: args.template_id.clone(),
            from_id: item.instance_env.instance_id.clone(), // The magic
            vcpus: args.vcpus,
            mem: args.mem.clone(),
            img_size: args.img_size.clone(),
            no_resize: args.no_resize,
            net4_ip: args.net4_ip.clone(),
            net4_gw: args.net4_gw.clone(),
            net6_ip: args.net6_ip.clone(),
            net6_gw: args.net6_gw.clone(),
            net6_dhcp: args.net6_dhcp,
            no_start: args.no_start,
            follow: false, // Handled below.
            #[cfg(feature = "guest-build")]
            self_signed: args.self_signed,
            #[cfg(feature = "guest-build")]
            safe_boot: args.safe_boot,
            prompt_admin_key: args.prompt_admin_key,
        };

        let (created, instance_id) = do_os_guest_instance_create(
            &cfg,
            &opts,
            guest_type,
            common_args,
            Some(GuestInstanceCreateArgsProv::default()),
            Some(GuestInstanceCreateArgsNode::default()),
            Some(GuestInstanceCreateArgsApi::default()),
            Some(GuestInstanceCreateArgsCustom::default()),
        )
        .await;

        if !created || instance_id.is_none() {
            return true; // The fault wasn't with our arguments.
        }
        let instance_id = instance_id.unwrap();

        // Delete (or at least disable)
        if args.preserve {
            // Stop / disable (should have already happened, but to be sure).
            item.stop().expect("failed to stop old instance");
            item.disable().expect("failed to disable old instance");
        } else {
            // Delete instance
            delete_instance(&cfg, &item, &opts);
        }

        // Tail logs?
        if !args.no_start
            && args.follow
            && !handle_cmd_os_guest_instance_logs(
                &cfg,
                &opts,
                GuestInstanceLogs { id: instance_id, num: None, follow: true },
            )
        {
            return true; // The fault wasn't with our arguments.
        }
    }

    true
}
