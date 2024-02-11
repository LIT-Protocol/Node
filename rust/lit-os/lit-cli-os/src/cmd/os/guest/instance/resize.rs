use crate::config::LitCliOsConfig;
use clap::Args;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_cli_core::utils::system::journalctl_systemd_service;
use lit_core::config::LitConfig;
use std::process::{Command, Stdio};

use crate::guest::instance::find_one_guest_instance;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestInstanceResize {
    #[arg(value_name = "ID", value_enum)]
    id: String,
    /// Number of log lines to show
    #[arg(long, short)]
    num: Option<u8>,
    // TODO: Validate add_img_size
    /// Extra storage to allocate to image (i.e. 50G).
    #[arg(value_name = "SIZE", value_enum)]
    add_img_size: String,
    /// Automatically stop the guest prior to resize.
    #[arg(long)]
    restart: bool,
    /// Follow logs when service starts.
    #[arg(long, short)]
    follow: bool,
}

pub(crate) fn handle_cmd_os_guest_instance_resize(
    cfg: LitConfig, opts: CliGlobalOpts, args: GuestInstanceResize,
) -> bool {
    if args.id.is_empty() {
        return false;
    }

    if let Some(item) = find_one_guest_instance(&cfg, None, None, Some(&args.id)) {
        let instance_env = &item.instance_env;
        let instance_service = instance_env
            .instance_service
            .as_ref()
            .expect("missing INSTANCE_SERVICE key in instance.env");

        if systemctl::is_active(instance_service.as_str())
            .unwrap_or_else(|_| panic!("failed to run systemctl is-active on {instance_service}"))
        {
            if args.restart {
                systemctl::stop(instance_service.as_str()).unwrap_or_else(|_| {
                    panic!("failed to run systemctl stop on {instance_service}")
                });
            } else {
                eprintln!("Invalid: --restart is required if the guest is still running.");
                return false;
            }
        }

        // Execute resize.
        let resize_cmd = cfg
            .litos_guest_instance_resize_cmd()
            .expect("failed to determine the lit os install location");

        let mut cmd = Command::new(resize_cmd);
        cmd.arg("-path").arg(&item.path).arg("-add-img-size").arg(&args.add_img_size);

        if opts.quiet() {
            cmd.stderr(Stdio::inherit()).stdout(Stdio::null());
        } else {
            cmd.stderr(Stdio::inherit()).stdout(Stdio::inherit());
        }

        match cmd.spawn() {
            Ok(mut child) => {
                let status = child.wait().expect("failed waiting for child process");

                if !status.success() {
                    if !opts.quiet() {
                        eprintln!();
                    }

                    eprintln!("Failed to resize instance");
                }
            }
            Err(err) => {
                eprintln!("failed to spawn instance resize script: {cmd:?} - {err:?}");
            }
        }

        systemctl::start(instance_service.as_str())
            .unwrap_or_else(|_| panic!("failed to run systemctl start on {instance_service}"));

        if args.follow {
            journalctl_systemd_service(instance_service, None, args.follow);
        }
    }

    true
}
