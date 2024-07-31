use clap::{CommandFactory, Parser, Subcommand};
use std::env;

use lit_cli_core::cmd::{CliGlobalOpts, OutputType};
use lit_cli_core::utils::system::require_root;
#[cfg(feature = "os")]
use lit_cli_os::{handle_cmd_os, OsArgs};
use lit_core::config::LitConfigBuilder;

use crate::cmd::init::{handle_cmd_init, InitArgs};

pub mod cmd;

#[derive(Parser, Debug)]
#[command(name = "lit")]
#[command(about = "🔥 Lit CLI: Command line interface for Lit ", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Output type
    #[arg(short, long, value_name = "TYPE", value_enum, global = true)]
    output: Option<OutputType>,
    /// Display verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
    /// Display no output
    #[arg(short, long, global = true)]
    quiet: bool,
}

#[derive(Debug, Subcommand)]
#[allow(clippy::large_enum_variant)]
enum Commands {
    #[cfg(feature = "os")]
    /// OS commands
    Os(OsArgs),
    /// Init commands
    Init(InitArgs),
}

#[tokio::main]
async fn main() {
    let mut cmd = Cli::command();
    let args = Cli::parse();
    let opts = CliGlobalOpts::new(args.verbose, args.quiet, args.output);

    // Require root
    if !require_root() {
        return;
    };

    // Load config
    let cfg_builder = LitConfigBuilder::default();

    // Set marker for scripts
    env::set_var("LIT_CLI", "1");

    if !match args.command {
        #[cfg(feature = "os")]
        Commands::Os(args) => handle_cmd_os(cfg_builder, opts, args).await,
        Commands::Init(args) => handle_cmd_init(opts, args, &mut cmd),
    } {
        cmd.print_help().expect("print help failed");
    }
}
