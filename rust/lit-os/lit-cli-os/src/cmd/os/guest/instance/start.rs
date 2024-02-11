use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_cli_core::utils::system::journalctl_systemd_service;
use lit_core::config::LitConfig;

use crate::guest::instance::find_one_guest_instance;
use crate::guest::instance::helper::GuestInstanceHelper;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestInstanceStart {
    #[arg(value_name = "ID", value_enum)]
    id: String,
    /// Also enables the service.
    #[arg(long)]
    enable: bool,
    /// Follow logs.
    #[arg(long, short)]
    follow: bool,
}

pub(crate) fn handle_cmd_os_guest_instance_start(
    cfg: LitConfig, _opts: CliGlobalOpts, args: GuestInstanceStart,
) -> bool {
    if args.id.is_empty() {
        return false;
    }

    if let Some(item) = find_one_guest_instance(&cfg, None, None, Some(&args.id)) {
        let instance_service =
            item.service_name().expect("missing INSTANCE_SERVICE key in instance.env");

        if args.enable {
            item.enable(&cfg).expect("failed to enable service");
        }

        item.start().expect("failed to start service");

        if args.follow {
            journalctl_systemd_service(&instance_service, None, args.follow);
        }
    }

    true
}
