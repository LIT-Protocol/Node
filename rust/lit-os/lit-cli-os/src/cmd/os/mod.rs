use clap::{Args, Subcommand};

use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::{LitConfig, LitConfigBuilder};

#[cfg(any(feature = "guest-instance", feature = "guest-build"))]
use crate::cmd::os::cleanup::{handle_cmd_os_cleanup, CleanUp};
#[cfg(any(feature = "guest-instance", feature = "guest-build"))]
use crate::cmd::os::delete::{handle_cmd_os_delete_all, DeleteAll};
#[cfg(any(feature = "guest-instance", feature = "guest-build"))]
use crate::cmd::os::guest::{handle_cmd_os_guest, Guest};
#[cfg(any(feature = "guest-instance", feature = "guest-build"))]
use crate::cmd::os::ls::{handle_cmd_os_ls, Ls};
#[cfg(any(feature = "guest-instance", feature = "guest-build"))]
use crate::cmd::os::ps::{handle_cmd_os_ps, Ps};
#[cfg(feature = "host-core")]
use crate::cmd::os::status::{handle_cmd_os_status, Status};
#[cfg(feature = "host-core")]
use crate::cmd::os::update::{handle_cmd_os_update, Update};
use crate::cmd::os::util::{handle_cmd_os_util, Util};
use crate::config::LitCliOsConfig;

#[cfg(any(feature = "guest-instance", feature = "guest-build"))]
mod cleanup;
#[cfg(any(feature = "guest-instance", feature = "guest-build"))]
mod delete;
#[cfg(any(feature = "guest-instance", feature = "guest-build"))]
pub(crate) mod guest;
#[cfg(any(feature = "guest-instance", feature = "guest-build"))]
mod ls;
#[cfg(any(feature = "guest-instance", feature = "guest-build"))]
mod ps;
#[cfg(feature = "host-core")]
mod status;
#[cfg(feature = "host-core")]
mod update;
mod util;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct OsArgs {
    #[command(subcommand)]
    command: OsCommands,
}

#[derive(Debug, Subcommand)]
#[allow(clippy::large_enum_variant)]
enum OsCommands {
    #[cfg(feature = "host-core")]
    /// Show host status
    Status(Status),
    #[cfg(feature = "host-core")]
    /// Update installation
    Update(Update),
    #[cfg(any(feature = "guest-instance", feature = "guest-build"))]
    /// Guest commands
    #[command(arg_required_else_help = true)]
    Guest(Guest),
    #[cfg(any(feature = "guest-instance", feature = "guest-build"))]
    /// List all resources
    Ls(Ls),
    #[cfg(any(feature = "guest-instance", feature = "guest-build"))]
    /// List all processes
    Ps(Ps),
    #[cfg(any(feature = "guest-instance", feature = "guest-build"))]
    /// Clean up all stale files
    CleanUp(CleanUp),
    #[cfg(any(feature = "guest-instance", feature = "guest-build"))]
    /// Delete all resources
    DeleteAll(DeleteAll),
    /// Utility commands
    #[command(arg_required_else_help = true)]
    Util(Util),
}

pub async fn handle_cmd_os(
    cfg_builder: LitConfigBuilder, opts: CliGlobalOpts, args: OsArgs,
) -> bool {
    let cfg = LitConfig::from_builder(cfg_builder).expect("failed to load lit config");

    match args.command {
        #[cfg(feature = "host-core")]
        OsCommands::Status(arg) => handle_cmd_os_status(cfg, opts, arg),
        #[cfg(feature = "host-core")]
        OsCommands::Update(arg) => handle_cmd_os_update(cfg, opts, arg),
        #[cfg(any(feature = "guest-instance", feature = "guest-build"))]
        OsCommands::Guest(args) => handle_cmd_os_guest(cfg, opts, args).await,
        #[cfg(any(feature = "guest-instance", feature = "guest-build"))]
        OsCommands::Ls(args) => handle_cmd_os_ls(cfg, opts, args).await,
        #[cfg(any(feature = "guest-instance", feature = "guest-build"))]
        OsCommands::Ps(args) => handle_cmd_os_ps(cfg, opts, args).await,
        #[cfg(any(feature = "guest-instance", feature = "guest-build"))]
        OsCommands::CleanUp(args) => handle_cmd_os_cleanup(cfg, opts, args).await,
        #[cfg(any(feature = "guest-instance", feature = "guest-build"))]
        OsCommands::DeleteAll(args) => handle_cmd_os_delete_all(cfg, opts, args).await,
        OsCommands::Util(args) => handle_cmd_os_util(cfg, opts, args).await,
    }
}
