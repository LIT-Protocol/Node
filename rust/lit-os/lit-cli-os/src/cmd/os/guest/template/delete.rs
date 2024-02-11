use std::fs;
use std::path::Path;

use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_cli_core::utils::system::require_root;
use lit_core::config::LitConfig;

use crate::guest::template::{
    find_one_guest_template, print_guest_templates, walk_guest_templates,
};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestTemplateDelete {
    /// Instance Id or 'all' to delete all.
    #[arg(value_name = "ID", value_enum)]
    pub(crate) id: String,
}

pub(crate) fn handle_cmd_os_guest_template_delete(
    cfg: &LitConfig, opts: &CliGlobalOpts, args: GuestTemplateDelete,
) -> bool {
    if !require_root() {
        return true;
    }

    if args.id.is_empty() {
        return false;
    }

    if args.id.eq_ignore_ascii_case("all") {
        walk_guest_templates(cfg, {
            move |build_path, _res| {
                if !opts.quiet() {
                    println!("Deleting: {:?}", &build_path);
                }

                delete_template(build_path.as_path(), opts.quiet());

                if !opts.quiet() {
                    eprintln!();
                }

                false
            }
        });
    } else if let Some(item) = find_one_guest_template(cfg, None, None, Some(&args.id), None) {
        // Sanity checks.
        if !match &item.build_env.build_id {
            Some(build_id) => build_id.eq(&args.id),
            _ => false,
        } {
            print_guest_templates(cfg, vec![item]);

            eprintln!();
            eprintln!("Template found doesnt match id provided");

            return true;
        }

        delete_template(item.path.as_path(), opts.quiet())
    }

    true
}

fn delete_template<P: AsRef<Path>>(path: P, _quiet: bool) {
    if !path.as_ref().exists() {
        eprintln!("No path returned for template");

        return;
    }

    // Delete template
    fs::remove_dir_all(path.as_ref())
        .unwrap_or_else(|_| panic!("failed to delete: {:?}", path.as_ref()));
}
