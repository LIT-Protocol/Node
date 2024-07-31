use async_std::path::PathBuf;
use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;
use lit_core::utils::ipfs::ipfs_cat;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct UtilIpfsGet {
    /// The CID to retrieve
    #[arg(value_name = "CID", value_enum)]
    cid: String,
    /// The path to save the file
    #[arg(value_name = "PATH", value_enum)]
    path: String,
}

pub(crate) async fn handle_cmd_os_util_ipfs_get(
    cfg: LitConfig, _opts: CliGlobalOpts, args: UtilIpfsGet,
) -> bool {
    if args.cid.is_empty() || args.path.is_empty() {
        return false;
    }

    let path = PathBuf::from(&args.path);
    if path.exists().await {
        eprintln!("File already exists: {}", &args.path);
        eprintln!();
        return false;
    }

    ipfs_cat(&cfg, &args.cid, Some(path.as_path()), None)
        .await
        .expect("failed to get file from IPFS");

    true
}
