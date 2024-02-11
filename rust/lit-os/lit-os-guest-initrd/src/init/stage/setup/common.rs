use lit_os_core::error::{generic_err, io_err, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub(crate) fn verify_allowed_in_mount(
    mnt: &Path, allowed_files: &[&str], label: &str,
) -> Result<()> {
    // Map allowed
    let mut allowed: HashMap<String, bool> = HashMap::new();
    for a in allowed_files {
        allowed.insert(a.to_string(), true);
    }

    // Verify all files in the directory
    let paths = fs::read_dir(mnt)
        .map_err(|e| io_err(e, Some(format!("failed to read dir: {:?}", &mnt))))?;

    for path in paths {
        let path =
            path.map_err(|e| io_err(e, Some(format!("failed to read dir entry: {:?}", &mnt))))?;
        let file_name = path
            .path()
            .file_name()
            .ok_or_else(|| {
                generic_err(format!("failed to get file_name of entry: {:?}", path.path()), None)
            })?
            .to_str()
            .ok_or_else(|| {
                generic_err(
                    format!("failed to convert filename from OsStr: {:?}", path.path()),
                    None,
                )
            })?
            .to_string();

        if !allowed.contains_key(&file_name) {
            return Err(io_err(format!("'{label}' file: {file_name} is forbidden"), None));
        }
    }

    Ok(())
}
