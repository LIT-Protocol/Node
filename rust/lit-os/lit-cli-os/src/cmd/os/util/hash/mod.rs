use async_std::path::PathBuf;
use clap::Args;

use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;
use lit_core::utils::binary::bytes_to_hex;
use lit_core::utils::hash::sha512_file;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct UtilHash {
    /// The path of the file to hash
    #[arg(value_name = "PATH", value_enum)]
    path: String,
}

pub(crate) async fn handle_cmd_os_util_hash(
    _cfg: LitConfig, _opts: CliGlobalOpts, args: UtilHash,
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

    let hash = sha512_file(path.as_path().into()).expect("failed to hash path");

    println!("{}", bytes_to_hex(hash));

    true
}
