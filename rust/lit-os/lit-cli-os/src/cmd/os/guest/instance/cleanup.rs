use std::fs;
use std::path::PathBuf;

use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;

use crate::guest::instance::find_guest_dead_instances;

pub(crate) fn handle_cmd_os_guest_instances_cleanup(cfg: &LitConfig, opts: &CliGlobalOpts) -> bool {
    let items: Option<Vec<PathBuf>> = find_guest_dead_instances(cfg);

    if let Some(items) = items {
        cleanup_dead_instances(cfg, items, opts);
    } else {
        eprintln!("No dirty guest instances found");
    }

    true
}

fn cleanup_dead_instances(_cfg: &LitConfig, paths: Vec<PathBuf>, opts: &CliGlobalOpts) {
    for path in paths.iter() {
        if !opts.quiet() {
            println!("Cleaning up: {path:?}");
        }

        // Delete template
        fs::remove_dir_all(path).unwrap_or_else(|_| panic!("failed to delete: {path:?}"));
    }
}
