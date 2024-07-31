use lit_os_prov_core::release::common::manifest::RELEASE_FILE_INITIAL_PASSWORD;
use std::fs;
use std::path::Path;

#[allow(dead_code)]
pub(crate) fn write_password(instance_ci_dir: &Path, password: &Vec<u8>) {
    let mut instance_ci_pass_path = instance_ci_dir.to_path_buf();
    instance_ci_pass_path.push(RELEASE_FILE_INITIAL_PASSWORD);

    fs::write(instance_ci_pass_path.as_path(), password)
        .expect("failed to write password to cloud-init dir");
}
