use crate::error::parser_err;
use crate::error::Result;
use bytesize::ByteSize;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{env, fs};

pub fn is_user_root() -> bool {
    match env::var("USER") {
        Err(e) => {
            eprintln!("Unable to determine active user! - {e:?}");

            false
        }
        Ok(name) => name == "root",
    }
}

pub fn require_root() -> bool {
    match env::var("USER") {
        Err(e) => {
            eprintln!("Unable to determine active user! - {e:?}");

            false
        }
        Ok(name) => {
            if name != "root" {
                println!("ERROR: You must be root to run this command.");
                return false;
            }

            true
        }
    }
}

pub fn journalctl_systemd_service(service: &String, num: Option<u16>, follow: bool) {
    let mut cmd = Command::new("journalctl");
    cmd.arg("-u").arg(service).stdout(Stdio::inherit()).stderr(Stdio::inherit());

    if follow {
        cmd.arg("-f");
    } else {
        cmd.arg("-n").arg(num.unwrap_or(30).to_string());
    }

    // Enable colors.
    cmd.arg("--output").arg("cat");

    cmd.status().expect("failed to run journalctl");
}

pub fn is_sys_param_eq(file: &str, val: &str) -> bool {
    let path = PathBuf::from(file);
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            return content.trim().eq_ignore_ascii_case(val);
        }
    }

    false
}

pub fn chmod(file: &Path, mode: &String) {
    let mut cmd = Command::new("chmod");
    cmd.arg(mode).arg(file.to_str().unwrap()).stdout(Stdio::inherit()).stderr(Stdio::inherit());

    cmd.status().expect("failed to run chmod");
}

pub fn parse_human_as_bytes<S: AsRef<str>>(hb: S) -> Result<u64> {
    hb.as_ref()
        .replace('G', "Gi")
        .replace('M', "Mi")
        .replace('T', "Ti")
        .parse::<ByteSize>()
        .map_err(|e| {
            parser_err(e, Some(format!("unable to parse bytes from human: {}", hb.as_ref())))
        })
        .map(|v| v.as_u64())
}
