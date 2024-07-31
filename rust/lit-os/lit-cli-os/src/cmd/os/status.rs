use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_cli_core::utils::system::require_root;
use lit_core::config::LitConfig;

use crate::host::status::print_host_status;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct Status {
    /// Show extended information
    #[arg(long, short)]
    extended: bool,
}

pub(crate) fn handle_cmd_os_status(cfg: LitConfig, _opts: CliGlobalOpts, arg: Status) -> bool {
    if !require_root() {
        return true;
    }

    print_host_status(cfg, arg.extended);

    true
}
