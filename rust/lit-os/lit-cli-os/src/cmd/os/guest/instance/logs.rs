use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_cli_core::utils::system::journalctl_systemd_service;
use lit_core::config::LitConfig;

use crate::guest::instance::find_one_guest_instance;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestInstanceLogs {
    #[arg(value_name = "ID", value_enum)]
    pub(crate) id: String,
    /// Number of log lines to show
    #[arg(long, short)]
    pub(crate) num: Option<u16>,
    /// Follow the logß
    #[arg(long, short)]
    pub(crate) follow: bool,
}

pub(crate) fn handle_cmd_os_guest_instance_logs(
    cfg: &LitConfig, _opts: &CliGlobalOpts, args: GuestInstanceLogs,
) -> bool {
    if args.id.is_empty() {
        return false;
    }

    if let Some(item) = find_one_guest_instance(cfg, None, None, Some(&args.id)) {
        let instance_env = &item.instance_env;
        let instance_service = instance_env
            .instance_service
            .as_ref()
            .expect("missing INSTANCE_SERVICE key in instance.env");

        journalctl_systemd_service(instance_service, args.num, args.follow);
    }

    true
}
