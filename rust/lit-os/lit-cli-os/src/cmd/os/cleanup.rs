#[cfg(feature = "guest-instance")]
use crate::cmd::os::guest::instance::cleanup::handle_cmd_os_guest_instances_cleanup;
#[cfg(feature = "guest-build")]
use crate::cmd::os::guest::template::cleanup::handle_cmd_os_guest_template_cleanup;
use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_cli_core::utils::system::require_root;
use lit_core::config::LitConfig;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct CleanUp {}

pub(crate) async fn handle_cmd_os_cleanup(
    cfg: LitConfig, opts: CliGlobalOpts, _arg: CleanUp,
) -> bool {
    if !require_root() {
        return true;
    }

    #[allow(unused_assignments, unused_mut)]
    let mut padding = false;

    #[cfg(feature = "guest-build")]
    {
        println!("lit os guest templates:");
        println!();

        handle_cmd_os_guest_template_cleanup(&cfg, &opts);

        padding = true;
    }

    #[cfg(feature = "guest-instance")]
    {
        if padding {
            println!();
        }

        println!("lit os guest instances:");
        println!();

        handle_cmd_os_guest_instances_cleanup(&cfg, &opts);
    }

    true
}
