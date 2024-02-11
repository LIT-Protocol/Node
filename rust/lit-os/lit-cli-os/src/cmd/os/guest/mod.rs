use clap::{Args, Subcommand};

use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;

#[cfg(feature = "guest-instance")]
use crate::cmd::os::guest::instance::{handle_cmd_os_guest_instance, GuestInstance};
#[cfg(feature = "guest-build")]
use crate::cmd::os::guest::template::{handle_cmd_os_guest_template, GuestTemplate};

#[cfg(feature = "guest-instance")]
pub(crate) mod instance;
#[cfg(feature = "guest-build")]
pub(crate) mod template;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct Guest {
    #[command(subcommand)]
    command: GuestCommands,
}

#[derive(Debug, Subcommand)]
#[command(arg_required_else_help = true)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum GuestCommands {
    #[cfg(feature = "guest-instance")]
    /// Guest instance commands
    #[command(arg_required_else_help = true)]
    Instance(GuestInstance),
    #[cfg(feature = "guest-build")]
    /// Guest template commands
    #[command(arg_required_else_help = true)]
    Template(GuestTemplate),
}

#[cfg(any(feature = "guest-instance", feature = "guest-build"))]
pub(crate) async fn handle_cmd_os_guest(cfg: LitConfig, opts: CliGlobalOpts, args: Guest) -> bool {
    match args.command {
        #[cfg(feature = "guest-instance")]
        GuestCommands::Instance(args) => handle_cmd_os_guest_instance(cfg, opts, args).await,
        #[cfg(feature = "guest-build")]
        GuestCommands::Template(args) => handle_cmd_os_guest_template(cfg, opts, args).await,
    }
}
