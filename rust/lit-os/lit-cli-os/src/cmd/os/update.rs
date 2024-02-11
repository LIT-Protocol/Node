use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_cli_core::utils::system::require_root;
use lit_core::config::LitConfig;

use crate::host::update::host_update;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct Update {}

pub(crate) fn handle_cmd_os_update(cfg: LitConfig, _opts: CliGlobalOpts, _arg: Update) -> bool {
    if !require_root() {
        return true;
    }

    host_update(cfg);

    true
}
