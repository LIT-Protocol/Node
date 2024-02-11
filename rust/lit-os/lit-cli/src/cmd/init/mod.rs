use std::io;

use clap::{Args, Command, Subcommand};
use clap_complete::{generate, Generator, Shell};
use lit_cli_core::cmd::CliGlobalOpts;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct InitArgs {
    #[command(subcommand)]
    command: InitCommands,
}

#[derive(Debug, Subcommand)]
#[command(arg_required_else_help = true)]
pub enum InitCommands {
    /// generate shell completion file for given shell
    #[command(arg_required_else_help = true)]
    Generate {
        #[arg(value_name = "SHELL", value_enum)]
        shell: Shell,
    },
}

pub fn handle_cmd_init(_opts: CliGlobalOpts, args: InitArgs, cmd: &mut Command) -> bool {
    match args.command {
        InitCommands::Generate { shell } => {
            print_completions(shell, cmd);
        }
    }

    true
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
