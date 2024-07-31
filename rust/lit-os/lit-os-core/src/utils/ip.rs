use crate::error::{io_err, Result};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{fmt, fs};

pub const CMD_IP: &str = "ip";
pub const CMD_DHCLIENT: &str = "dhclient";

#[derive(Debug, Clone, PartialEq)]
pub enum IpKind {
    V4,
    V6,
}

impl fmt::Display for IpKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IpKind::V4 => write!(f, "IPv4"),
            IpKind::V6 => write!(f, "IPv6"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub enum LinkState {
    Up,
    Down,
}

impl fmt::Display for LinkState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LinkState::Up => write!(f, "up"),
            LinkState::Down => write!(f, "down"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub enum IpAction {
    Add,
    Del,
}

impl fmt::Display for IpAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IpAction::Add => write!(f, "add"),
            IpAction::Del => write!(f, "del"),
        }
    }
}

/// ip link set dev ${dev} up
pub fn ip_link(dev: &str, state: LinkState) -> Result<()> {
    let out = Command::new(CMD_IP)
        .arg("link")
        .arg("set")
        .arg("dev")
        .arg(dev)
        .arg(format!("{state}"))
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| io_err(e, Some(format!("failed to run '{CMD_IP}'"))))?;

    if !out.status.success() {
        return Err(io_err(format!("failed to run '{CMD_IP}'"), None));
    }

    Ok(())
}

/// ip addr add 10.1.1.2/8 dev ens33
pub fn ip_addr(dev: &str, action: IpAction, ip: &str) -> Result<()> {
    let out = Command::new(CMD_IP)
        .arg("addr")
        .arg(format!("{action}"))
        .arg(ip)
        .arg("dev")
        .arg(dev)
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| io_err(e, Some(format!("failed to run '{CMD_IP}'"))))?;

    if !out.status.success() {
        return Err(io_err(format!("failed to run '{CMD_IP}'"), None));
    }

    Ok(())
}

/// ip route add default via 192.168.1.1
pub fn ip_route(action: IpAction, net: &str, ip: &str) -> Result<()> {
    let out = Command::new(CMD_IP)
        .arg("route")
        .arg(format!("{action}"))
        .arg(net)
        .arg("via")
        .arg(ip)
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| crate::error::io_err(e, Some(format!("failed to run '{CMD_IP}'"))))?;

    if !out.status.success() {
        return Err(io_err(format!("failed to run '{CMD_IP}'"), None));
    }

    Ok(())
}

/// dhclient -4 ${dev}
pub fn dhclient_up(dev: &str, kind: IpKind, verbose: bool) -> Result<()> {
    // Ensure this path exists.
    let var_path = PathBuf::from("/var/lib/dhcp");
    if !var_path.exists() {
        fs::create_dir_all(&var_path).map_err(|e| io_err(e, None))?;
    }

    let kind_arg = match kind {
        IpKind::V4 => "-4",
        IpKind::V6 => "-6",
    };

    let mut cmd = Command::new(CMD_DHCLIENT);
    cmd.arg(kind_arg).arg(dev);

    if verbose {
        cmd.arg("-v");
    }

    let out =
        cmd.stdout(Stdio::null()).stderr(Stdio::inherit()).output().map_err(|e| {
            crate::error::io_err(e, Some(format!("failed to run '{CMD_DHCLIENT}'")))
        })?;

    if !out.status.success() {
        return Err(io_err(format!("failed to run '{CMD_DHCLIENT}'"), None));
    }

    Ok(())
}
