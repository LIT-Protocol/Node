use async_std::path::PathBuf;
use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;
use lit_core::utils::ipfs::ipfs_cid_of_file;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct UtilIpfsCid {
    /// The path of the file
    #[arg(value_name = "PATH", value_enum)]
    path: String,
}

pub(crate) async fn handle_cmd_os_util_ipfs_cid(
    _cfg: LitConfig, _opts: CliGlobalOpts, args: UtilIpfsCid,
) -> bool {
    if args.path.is_empty() {
        return false;
    }

    let path = PathBuf::from(&args.path);
    if !path.exists().await {
        eprintln!("File does not exist: {}", &args.path);
        eprintln!();
        return false;
    }

    let cid = ipfs_cid_of_file(path.as_path()).await.expect("failed to calculate CID of file");
    println!("{cid}");

    true
}
