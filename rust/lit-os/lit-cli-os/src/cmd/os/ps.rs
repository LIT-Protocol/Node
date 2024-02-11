#[cfg(feature = "guest-instance")]
use crate::cmd::os::guest::instance::ps::handle_cmd_os_guest_instance_ps;
use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_cli_core::utils::system::require_root;
use lit_core::config::LitConfig;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct Ps {}

pub(crate) async fn handle_cmd_os_ps(cfg: LitConfig, opts: CliGlobalOpts, _arg: Ps) -> bool {
    if !require_root() {
        return true;
    }

    #[cfg(feature = "guest-instance")]
    {
        println!("lit os guest instances:");
        println!();

        handle_cmd_os_guest_instance_ps(&cfg, &opts);
    }

    true
}
