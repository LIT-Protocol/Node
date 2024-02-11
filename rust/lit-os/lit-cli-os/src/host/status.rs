use std::process::{Command, Stdio};

use crate::config::LitCliOsConfig;
use lit_cli_core::utils::system::is_sys_param_eq;
use lit_core::config::LitConfig;
use nu_ansi_term::Color::{Green, Red};
use once_cell::sync::Lazy;

static COLORED_YES: Lazy<String> = Lazy::new(|| Green.paint("Yes").to_string());
static COLORED_NO: Lazy<String> = Lazy::new(|| Red.paint("No").to_string());

pub fn print_host_status(cfg: LitConfig, extended: bool) {
    fn print_string(key: &str, val: &str) {
        println!("{key:<18} {val}");
    }

    let cpu = aleo_std_cpu::get_cpu();

    print_string(
        "CPU Arch:",
        match cpu {
            aleo_std_cpu::Cpu::AMD => "Authentic AMD",
            aleo_std_cpu::Cpu::Intel => "Genuine Intel",
            _ => "Unknown",
        },
    );

    match cpu {
        aleo_std_cpu::Cpu::AMD => {
            print_string("CPU Supported:", COLORED_YES.as_str());

            let sev_status = if is_sys_param_eq("/sys/module/kvm_amd/parameters/sev", "Y") {
                COLORED_YES.as_str()
            } else {
                COLORED_NO.as_str()
            };
            let sev_es_status = if is_sys_param_eq("/sys/module/kvm_amd/parameters/sev_es", "Y") {
                COLORED_YES.as_str()
            } else {
                COLORED_NO.as_str()
            };
            let sev_snp_status = if is_sys_param_eq("/sys/module/kvm_amd/parameters/sev_snp", "Y") {
                COLORED_YES.as_str()
            } else {
                COLORED_NO.as_str()
            };

            print_string("AMD SEV:", sev_status);
            print_string("AMD SEV-ES:", sev_es_status);
            print_string("AMD SEV-SNP:", sev_snp_status);
        }
        _ => {
            print_string("CPU Supported:", COLORED_NO.as_str());
        }
    }

    if extended {
        println!();
        println!("Extended:");
        println!();

        show_host_check(cfg);
    }
}

fn show_host_check(cfg: LitConfig) {
    let check_cmd =
        cfg.litos_host_check_cmd().expect("failed to determine the lit os install location");

    let mut cmd = Command::new(check_cmd);
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

    cmd.status().expect("failed to run host check command");
}
