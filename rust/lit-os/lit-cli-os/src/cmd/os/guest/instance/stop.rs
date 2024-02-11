use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;

use crate::guest::instance::find_one_guest_instance;
use crate::guest::instance::helper::GuestInstanceHelper;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestInstanceStop {
    #[arg(value_name = "ID", value_enum)]
    id: String,
    /// Also disable the service.
    #[arg(long)]
    disable: bool,
}

pub(crate) fn handle_cmd_os_guest_instance_stop(
    cfg: LitConfig, _opts: CliGlobalOpts, args: GuestInstanceStop,
) -> bool {
    if args.id.is_empty() {
        return false;
    }

    if let Some(item) = find_one_guest_instance(&cfg, None, None, Some(&args.id)) {
        item.stop().expect("failed to stop service");

        if args.disable {
            item.disable().expect("failed to disable service");
        }
    }

    true
}
