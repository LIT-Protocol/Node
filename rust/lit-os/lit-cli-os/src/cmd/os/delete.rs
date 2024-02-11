use clap::Args;

use lit_cli_core::cmd::CliGlobalOpts;
use lit_cli_core::utils::prompt::confirm;
use lit_cli_core::utils::system::require_root;
use lit_core::config::LitConfig;

#[cfg(feature = "guest-instance")]
use crate::cmd::os::guest::instance::delete::{
    handle_cmd_os_guest_instance_delete, GuestInstanceDelete,
};
#[cfg(feature = "guest-build")]
use crate::cmd::os::guest::template::delete::{
    handle_cmd_os_guest_template_delete, GuestTemplateDelete,
};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct DeleteAll {
    /// Accept all confirmations
    #[arg(long, short)]
    yes: bool,
}

pub(crate) async fn handle_cmd_os_delete_all(
    cfg: LitConfig, opts: CliGlobalOpts, args: DeleteAll,
) -> bool {
    if !require_root() {
        return true;
    }

    #[allow(unused_assignments, unused_mut)]
    let mut padding = false;

    #[cfg(feature = "guest-build")]
    {
        let mut yes = args.yes;
        if yes {
            println!("lit os guest templates:");
        } else {
            yes = confirm("Delete all guest templates?", false)
                .expect("failed to capture confirmation");
        }

        if yes {
            println!();

            handle_cmd_os_guest_template_delete(
                &cfg,
                &opts,
                GuestTemplateDelete { id: "all".into() },
            );
        }

        padding = true;
    }

    #[cfg(feature = "guest-instance")]
    {
        if padding {
            println!();
        }

        let mut yes = args.yes;
        if yes {
            println!("lit os guest instances:");
        } else {
            yes = confirm("Delete all guest instances?", false)
                .expect("failed to capture confirmation");
        }

        if yes {
            println!();

            handle_cmd_os_guest_instance_delete(
                &cfg,
                &opts,
                GuestInstanceDelete { id: "all".into() },
            );
        }
    }

    true
}
