use std::{fs, process::Command};
use tracing::info;

pub fn get_crate_version() -> String {
    // First get the current crate version. Do this by running the command `cargo pkgid` and parsing
    // the characters at the end after the `@`.
    let cmd = Command::new("cargo")
        .args(["pkgid"])
        .output()
        .expect("Failed to get current crate version");
    assert!(cmd.status.success());
    let current_crate_version = String::from_utf8(cmd.stdout)
        .unwrap()
        .split('@')
        .last()
        .unwrap()
        .trim()
        .to_string();
    info!("Current crate version: {:?}", current_crate_version);
    current_crate_version
}

pub fn update_node_crate_version(new_crate_version: String) -> CrateVersionHandle {
    let current_crate_version = get_crate_version();

    // Update the crate version.
    let cargo_toml = fs::read_to_string("./Cargo.toml").expect("Failed to read Cargo.toml");
    let mut doc = cargo_toml
        .parse::<toml_edit::Document>()
        .expect("Failed to parse Cargo.toml");
    doc["package"]["version"] = toml_edit::value(new_crate_version.clone());
    fs::write("./Cargo.toml", doc.to_string()).expect("Failed to write Cargo.toml");
    info!("Updated node crate version to {}", new_crate_version);

    CrateVersionHandle(current_crate_version)
}

/// A simple struct that stores a version, and updates the crate to this version when this
/// struct is dropped.
pub struct CrateVersionHandle(pub String);

impl Drop for CrateVersionHandle {
    fn drop(&mut self) {
        info!("Reverting crate version back to {:?}", self.0);

        // Update the crate version.
        let cargo_toml = fs::read_to_string("./Cargo.toml").expect("Failed to read Cargo.toml");
        let mut doc = cargo_toml
            .parse::<toml_edit::Document>()
            .expect("Failed to parse Cargo.toml");
        doc["package"]["version"] = toml_edit::value(self.0.clone());
        fs::write("./Cargo.toml", doc.to_string()).expect("Failed to write Cargo.toml");
        info!("Updated node crate version to {}", self.0);
    }
}
