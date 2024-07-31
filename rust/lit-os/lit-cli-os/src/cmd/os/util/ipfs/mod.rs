use clap::{Args, Subcommand};

use crate::cmd::os::util::ipfs::cid::{handle_cmd_os_util_ipfs_cid, UtilIpfsCid};
use crate::cmd::os::util::ipfs::get::{handle_cmd_os_util_ipfs_get, UtilIpfsGet};
use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;

pub(crate) mod cid;
pub(crate) mod get;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct UtilIpfs {
    #[command(subcommand)]
    command: UtilIPFSCommands,
}

#[derive(Debug, Subcommand)]
#[command(arg_required_else_help = true)]
pub(crate) enum UtilIPFSCommands {
    /// Calculate a CID for a given file
    #[command(arg_required_else_help = true)]
    Cid(UtilIpfsCid),
    /// Retrieve a file from IPFS
    #[command(arg_required_else_help = true)]
    Get(UtilIpfsGet),
}

pub(crate) async fn handle_cmd_os_util_ipfs(
    cfg: LitConfig, opts: CliGlobalOpts, args: UtilIpfs,
) -> bool {
    match args.command {
        UtilIPFSCommands::Cid(args) => handle_cmd_os_util_ipfs_cid(cfg, opts, args).await,
        UtilIPFSCommands::Get(args) => handle_cmd_os_util_ipfs_get(cfg, opts, args).await,
    }
}
