#[cfg(feature = "guest-instance")]
use crate::cmd::os::guest::instance::ls::handle_cmd_os_guest_instance_ls;
#[cfg(feature = "guest-build")]
use crate::cmd::os::guest::template::ls::handle_cmd_os_guest_template_ls;
use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_cli_core::utils::system::require_root;
use lit_core::config::LitConfig;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct Ls {}

pub(crate) async fn handle_cmd_os_ls(cfg: LitConfig, opts: CliGlobalOpts, _arg: Ls) -> bool {
    if !require_root() {
        return true;
    }

    #[allow(unused_assignments, unused_mut)]
    let mut padding = false;

    #[cfg(feature = "guest-build")]
    {
        println!("lit os guest templates:");
        println!();

        handle_cmd_os_guest_template_ls(&cfg, &opts);

        padding = true;
    }

    #[cfg(feature = "guest-instance")]
    {
        if padding {
            println!();
        }

        println!("lit os guest instances:");
        println!();

        handle_cmd_os_guest_instance_ls(&cfg, &opts);
    }

    true
}
