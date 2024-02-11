use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;

use crate::guest::template::{find_guest_templates, print_guest_templates, GuestTemplateItem};

pub(crate) fn handle_cmd_os_guest_template_ls(cfg: &LitConfig, _opts: &CliGlobalOpts) -> bool {
    let items: Option<Vec<GuestTemplateItem>> = find_guest_templates(cfg, None, None, None, None);

    if let Some(items) = items {
        print_guest_templates(cfg, items);
    } else {
        eprintln!("No guest templates found");
    }

    true
}
