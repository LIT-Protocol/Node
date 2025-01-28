use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;

use crate::guest::instance::find_one_guest_instance;
use crate::guest::instance::helper::GuestInstanceHelper;
use crate::guest::instance::helper::GuestInstanceItemHelper;
use lit_os_core::guest::types::GuestType;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestInstanceRestart {
    #[arg(value_name = "ID", value_enum)]
    id: String,
    /// force restart even if the node is active on its network
    #[arg(long, short)]
    force: bool,
}

pub(crate) async fn handle_cmd_os_guest_instance_restart(
    cfg: LitConfig, _opts: CliGlobalOpts, args: GuestInstanceRestart,
) -> bool {
    if args.id.is_empty() {
        return false;
    }

    if let Some(item) = find_one_guest_instance(&cfg, None, None, Some(&args.id)) {
        let guest_type = item.build_env.guest_type().expect("build.env missing GuestType");
        if guest_type == GuestType::Node
            && !args.force
            && item.is_active_on_network(&cfg).await.expect("node type should never fail")
        {
            panic!("NOT restarting, node is active on subnet {:?}. Leave first, or AT RISK OF SLASHING set --force to override",item.instance_env.subnet_id.unwrap());
        }
        item.restart().expect("failed to restart service");
    }

    true
}
