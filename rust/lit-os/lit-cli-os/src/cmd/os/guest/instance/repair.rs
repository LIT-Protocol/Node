use crate::config::LitCliOsConfig;
use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;
use std::process::{Command, Stdio};

use crate::guest::instance::{
    find_one_guest_instance, walk_guest_instance_items, GuestInstanceItem,
};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestInstanceRepair {
    /// Instance Id or 'all' to repair all.
    #[arg(value_name = "ID", value_enum)]
    id: String,
}

/// A facade to repair that can be easily called from other commands.
pub(crate) fn repair_instance(cfg: &LitConfig, item: &GuestInstanceItem, opts: &CliGlobalOpts) {
    let repair_cmd = cfg
        .litos_guest_instance_repair_cmd()
        .expect("failed to determine the lit os install location");

    if !opts.quiet() {
        println!("Repairing: {:?}", &item.path);
    }

    do_repair_instance(&repair_cmd, item, opts.quiet())
}

pub(crate) fn handle_cmd_os_guest_instance_repair(
    cfg: LitConfig, opts: CliGlobalOpts, args: GuestInstanceRepair,
) -> bool {
    if args.id.is_empty() {
        return false;
    }

    let repair_cmd = cfg
        .litos_guest_instance_repair_cmd()
        .expect("failed to determine the lit os install location");

    if args.id.eq_ignore_ascii_case("all") {
        walk_guest_instance_items(&cfg, {
            move |item| {
                if !opts.quiet() {
                    println!("Repairing: {:?}", &item.path);
                }

                do_repair_instance(&repair_cmd, &item, opts.quiet());

                if !opts.quiet() {
                    eprintln!();
                }

                false
            }
        });
    } else if let Some(item) = find_one_guest_instance(&cfg, None, None, Some(&args.id)) {
        do_repair_instance(&repair_cmd, &item, opts.quiet())
    }

    true
}

fn do_repair_instance(repair_cmd: &String, item: &GuestInstanceItem, quiet: bool) {
    // Execute repair.
    let mut cmd = Command::new(repair_cmd);
    cmd.arg("-path").arg(&item.path);

    if quiet {
        cmd.stderr(Stdio::inherit()).stdout(Stdio::null());
    } else {
        cmd.stderr(Stdio::inherit()).stdout(Stdio::inherit());
    }

    match cmd.spawn() {
        Ok(mut child) => {
            let status = child.wait().expect("failed waiting for child process");

            if !status.success() {
                if !quiet {
                    eprintln!();
                }

                eprintln!("Failed to repair instance");
            }
        }
        Err(err) => {
            eprintln!("failed to spawn instance repair script: {cmd:?} - {err:?}");
        }
    }
}
