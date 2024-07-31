use lit_os_core::error::{generic_err, Result};
use std::path::Path;
use std::process::{Command, Stdio};

// The function 'reencrypt' is not implemented in cryptsetup_rs.
// Additionally: the libcryptsetup_rs reencrypt didn't work.
#[allow(clippy::too_many_arguments)]
pub fn cryptsetup_reencrypt_dev(
    dev_path: &Path, label: &str, passphrase: &[u8], slot: u8, master_key: &[u8],
    key_size: Option<u16>, cipher: Option<&str>, hash: Option<&str>,
) -> Result<()> {
    let passphrase_file = temp_file::with_contents(passphrase);
    let master_key_file = temp_file::with_contents(master_key);

    let mut cmd = Command::new("cryptsetup");
    cmd.arg("reencrypt")
        .arg(dev_path)
        .arg("-q")
        .arg(format!("--key-file={}", passphrase_file.path().to_str().unwrap()))
        .arg(format!("--key-slot={slot}"))
        .arg(format!("--master-key-file={}", master_key_file.path().to_str().unwrap()));

    if let Some(key_size) = key_size {
        cmd.arg("--key-size").arg(key_size.to_string());
    }
    if let Some(cipher) = cipher {
        cmd.arg("--cipher").arg(cipher);
    }
    if let Some(hash) = hash {
        cmd.arg("--hash").arg(hash);
    }

    let mut child = cmd.stderr(Stdio::inherit()).stdout(Stdio::inherit()).spawn().map_err(|e| {
        generic_err(e, Some(format!("failed to 'cryptsetup reencrypt' '{label}' luks device")))
    })?;

    let res = child.wait().map_err(|e| {
        generic_err(
            e,
            Some(format!("failed while waiting for 'cryptsetup reencrypt' '{label}' luks device")),
        )
    })?;

    if !res.success() {
        return Err(generic_err(
            format!(
                "failed to exec 'cryptsetup reencrypt' '{}' luks device: status: {:?}",
                label,
                res.code()
            ),
            None,
        ));
    }

    Ok(())
}
