use crate::error::{io_err, Result};
use std::path::Path;
use std::process::{Command, Stdio};

pub const CMD_OPENSSL: &str = "openssl";

// TODO: Rewrite to use openssl lib?
/// openssl genpkey -algorithm ec -pkeyopt ec_paramgen_curve:"P-384" -out ./author-key.pem
pub fn openssl_genpkey(out: &Path, algorithm: Option<&str>, pkeyopt: Option<&str>) -> Result<()> {
    let out = Command::new(CMD_OPENSSL)
        .arg("genpkey")
        .arg("-algorithm")
        .arg(algorithm.unwrap_or("ec"))
        .arg("-pkeyopt")
        .arg(pkeyopt.unwrap_or("ec_paramgen_curve:P-384"))
        .arg("-out")
        .arg(out.as_os_str())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| crate::error::io_err(e, Some(format!("failed to run '{CMD_OPENSSL}'"))))?;

    if !out.status.success() {
        return Err(io_err(format!("failed to run '{CMD_OPENSSL}'"), None));
    }

    Ok(())
}
