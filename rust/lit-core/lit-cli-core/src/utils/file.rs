use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, io};

/// Returns a Vec of absolute and relative paths for the files in a given directory.
pub fn read_dir_recursive(
    path: &PathBuf, parent: Option<PathBuf>, files: &mut Vec<(PathBuf, PathBuf)>,
) -> io::Result<()> {
    let cur = path.file_name().unwrap().to_str().unwrap();

    if path.is_dir() {
        let mut new_parent;
        if let Some(parent) = parent {
            new_parent = parent;
            new_parent.push(cur);
        } else {
            new_parent = PathBuf::new();
        }

        for entry in fs::read_dir(path)? {
            read_dir_recursive(&entry.unwrap().path(), Some(new_parent.clone()), files)?;
        }
    } else {
        let full_path = path.to_owned();
        let rel_path = if let Some(parent) = parent {
            let mut parent = parent;
            parent.push(cur);

            parent
        } else {
            PathBuf::from_str(cur).unwrap()
        };

        files.push((full_path, rel_path));
    }

    Ok(())
}

pub fn create_file(file_path: &PathBuf) -> File {
    if file_path.exists() {
        fs::remove_file(file_path)
            .unwrap_or_else(|_| panic!("failed to remove existing file: {file_path:?}"));
    }

    OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(file_path)
        .unwrap_or_else(|_| panic!("failed to create file: {file_path:?}"))
}
