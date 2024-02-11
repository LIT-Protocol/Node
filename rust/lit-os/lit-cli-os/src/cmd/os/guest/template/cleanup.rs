use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;
use std::fs;
use std::path::PathBuf;

use crate::guest::template::{cleanup_guest_build, find_guest_dead_templates};

pub(crate) fn handle_cmd_os_guest_template_cleanup(cfg: &LitConfig, opts: &CliGlobalOpts) -> bool {
    let items: Option<Vec<PathBuf>> = find_guest_dead_templates(cfg);

    if let Some(items) = items {
        cleanup_dead_templates(cfg, items, opts);
    } else {
        eprintln!("No dirty guest templates found");
    }

    true
}

fn cleanup_dead_templates(cfg: &LitConfig, paths: Vec<PathBuf>, opts: &CliGlobalOpts) {
    for path in paths.iter() {
        if !opts.quiet() {
            println!("Cleaning up: {path:?}");
        }

        cleanup_guest_build(cfg, path.as_path(), opts.quiet());

        // Delete template
        fs::remove_dir_all(path).unwrap_or_else(|_| panic!("failed to delete: {path:?}"));
    }
}
