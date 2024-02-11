use clap::{Args, Subcommand};

use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;

use crate::cmd::os::util::hash::{handle_cmd_os_util_hash, UtilHash};
use crate::cmd::os::util::ipfs::{handle_cmd_os_util_ipfs, UtilIpfs};

pub(crate) mod hash;
pub(crate) mod ipfs;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct Util {
    #[command(subcommand)]
    command: UtilCommands,
}

#[derive(Debug, Subcommand)]
#[command(arg_required_else_help = true)]
pub(crate) enum UtilCommands {
    /// IPFS utility commands
    #[command(arg_required_else_help = true)]
    Ipfs(UtilIpfs),
    /// Hash utility commands
    #[command(arg_required_else_help = true)]
    Hash(UtilHash),
}

pub(crate) async fn handle_cmd_os_util(cfg: LitConfig, opts: CliGlobalOpts, args: Util) -> bool {
    match args.command {
        UtilCommands::Ipfs(args) => handle_cmd_os_util_ipfs(cfg, opts, args).await,
        UtilCommands::Hash(args) => handle_cmd_os_util_hash(cfg, opts, args).await,
    }
}
