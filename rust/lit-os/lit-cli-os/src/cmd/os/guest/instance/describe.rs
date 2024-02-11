use crate::guest::common::print_describe_str;
use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;

use crate::guest::instance::{
    find_one_guest_instance, print_guest_instance_env, print_guest_release_env,
};
use crate::guest::template::print_guest_build_env;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestInstanceDescribe {
    #[arg(value_name = "ID", value_enum)]
    id: String,
}

pub(crate) fn handle_cmd_os_guest_instance_describe(
    cfg: LitConfig, _opts: CliGlobalOpts, args: GuestInstanceDescribe,
) -> bool {
    if args.id.is_empty() {
        return false;
    }

    if let Some(item) = find_one_guest_instance(&cfg, None, None, Some(&args.id)) {
        print_describe_str("Instance Path:", item.path.to_str());
        print_guest_instance_env(&item.instance_env);
        if let Some(release_env) = item.release_env.as_ref() {
            print_guest_release_env(release_env)
        }
        print_guest_build_env(&item.build_env, true);
    }

    true
}
