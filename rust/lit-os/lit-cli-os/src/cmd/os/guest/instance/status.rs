use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;
use std::process::{Command, Stdio};

use crate::guest::instance::find_one_guest_instance;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestInstanceStatus {
    #[arg(value_name = "ID", value_enum)]
    id: String,
}

pub(crate) fn handle_cmd_os_guest_instance_status(
    cfg: LitConfig, _opts: CliGlobalOpts, args: GuestInstanceStatus,
) -> bool {
    if args.id.is_empty() {
        return false;
    }

    if let Some(item) = find_one_guest_instance(&cfg, None, None, Some(&args.id)) {
        let instance_env = &item.instance_env;
        let instance_service = instance_env
            .instance_service
            .as_ref()
            .expect("missing INSTANCE_SERVICE key in instance.env");

        let mut cmd = Command::new("systemctl");
        cmd.arg("status").arg(instance_service).stdout(Stdio::inherit()).stderr(Stdio::inherit());

        cmd.status().expect("failed to run systemctl");
    }

    true
}
