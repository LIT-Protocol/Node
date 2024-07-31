#[allow(unused_imports)]
use crate::error::{conversion_err, io_err, Result};
use std::path::Path;
#[allow(unused_imports)]
use std::process::{Command, Stdio};

pub const CMD_SHA512SUM: &str = "sha512sum";

#[allow(unused_variables)]
pub fn sys_sha512sum(path: &Path) -> Result<String> {
    #[cfg(target_os = "linux")]
    return sys_nix_sha512sum(path);
    #[cfg(not(target_os = "linux"))]
    unimplemented!("sys_sha512sum is only implemented for linux")
}

// This exists because when I tried to perform a rust based sha512 on the device it was different.
/// sha512sum /path/to/file (or device)
#[cfg(target_os = "linux")]
pub fn sys_nix_sha512sum(path: &Path) -> Result<String> {
    let out = Command::new(CMD_SHA512SUM)
        .arg(path.to_str().unwrap())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| io_err(e, Some(format!("failed to run '{CMD_SHA512SUM}'"))))?;

    if !out.status.success() {
        return Err(io_err(format!("failed to run '{CMD_SHA512SUM}'"), None));
    }

    let out = String::from_utf8(out.stdout).map_err(|e| {
        conversion_err(e, Some(format!("failed to convert output to string from: {CMD_SHA512SUM}")))
    })?;

    Ok(out
        .split(' ')
        .next()
        .ok_or_else(|| {
            conversion_err(format!("unable to get first string from: {CMD_SHA512SUM}"), None)
        })?
        .to_string())
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "linux")]
    use crate::utils::system::sys_nix_sha512sum;

    #[test]
    #[cfg(target_os = "linux")]
    fn sys_sha512sum_test() {
        let test_file = temp_file::with_contents(b"abcd1234");
        let out = sys_nix_sha512sum(test_file.path()).expect("failed to run sys_sha512sum");
        assert_eq!(out, "925f43c3cfb956bbe3c6aa8023ba7ad5cfa21d104186fffc69e768e55940d9653b1cd36fba614fba2e1844f4436da20f83750c6ec1db356da154691bdd71a9b1");
    }
}
