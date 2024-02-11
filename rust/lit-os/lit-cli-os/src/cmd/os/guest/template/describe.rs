use crate::guest::common::print_describe_str;
use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;

use crate::guest::template::{find_one_guest_template, print_guest_build_env};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestTemplateDescribe {
    #[arg(value_name = "ID", value_enum)]
    id: String,
}

pub(crate) fn handle_cmd_os_guest_template_describe(
    cfg: LitConfig, _opts: CliGlobalOpts, args: GuestTemplateDescribe,
) -> bool {
    if args.id.is_empty() {
        return false;
    }

    if let Some(item) = find_one_guest_template(&cfg, None, None, Some(&args.id), None) {
        print_describe_str("Build Path:", item.path.to_str());
        print_guest_build_env(&item.build_env, false);
    }

    true
}
