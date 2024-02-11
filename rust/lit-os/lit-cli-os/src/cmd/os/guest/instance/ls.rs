use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;

use crate::guest::instance::{find_guest_instances, print_guest_instances, GuestInstanceItem};

pub(crate) fn handle_cmd_os_guest_instance_ls(cfg: &LitConfig, opts: &CliGlobalOpts) -> bool {
    let items: Option<Vec<GuestInstanceItem>> = find_guest_instances(cfg, None, None, None);

    if let Some(items) = items {
        print_guest_instances(cfg, items, opts.output());
    } else {
        eprintln!("No guest instances found");
    }

    true
}
