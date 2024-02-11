use std::fs;
use std::path::Path;

use serde_json::Value;
use sev_snp_utilities::{
    calc_launch_digest, BlockSigner, FamilyId, IdBlock, ImageId, LaunchDigest, SevMode, ToBase64,
};

use lit_core::utils::binary::bytes_to_hex;
use lit_core::utils::hash::sha512_file;

use crate::error::{io_err, sev_snp_err, validation_err, Error, Result};
use crate::guest::types::GuestCpuType;

pub const CMD_SEV_SNP_HOST_IDENTITY: &str = "sev-host-identity";

pub const DEFAULT_SEV_SNP_POLICY: u64 = 0x30000;

pub fn sev_snp_host_identity(
    measurement: LaunchDigest, image_id: ImageId, family_id: FamilyId, policy: u64,
    author_key_pem_path: &Path, id_key_pem_path: &Path,
) -> Result<(String, String)> {
    if !author_key_pem_path.exists() {
        return Err(io_err(
            format!("author key file does not exist: {author_key_pem_path:?}"),
            None,
        ));
    }
    if !id_key_pem_path.exists() {
        return Err(io_err(format!("id key file does not exist: {id_key_pem_path:?}"), None));
    }

    let id_block = IdBlock::default()
        .with_ld(measurement)
        .with_image_id(image_id)
        .with_family_id(family_id)
        .with_policy(policy);

    let id_auth_info = id_block
        .sign(id_key_pem_path, Some(author_key_pem_path))
        .map_err(|e| sev_snp_err(e, None))?;

    Ok((
        id_block.to_base64().map_err(|e| io_err(e, None))?,
        id_auth_info.to_base64().map_err(|e| io_err(e, None))?,
    ))
}

#[allow(clippy::too_many_arguments)]
pub fn sev_snp_measure(
    vcpus: usize, vcpu_type: GuestCpuType, ovmf_path: &Path, kernel_path: &Path,
    append_path: &Path, initrd_path: &Path,
) -> Result<String> {
    let append = fs::read_to_string(append_path).map_err(|e| {
        io_err(e, Some(format!("failed to read kernel append file '{:?}'", &append_path)))
    })?;

    Ok(bytes_to_hex(
        calc_launch_digest(
            SevMode::SevSnp,
            vcpus,
            vcpu_type.into(),
            ovmf_path,
            Some(kernel_path),
            Some(initrd_path),
            Some(append.as_str()),
        )
        .map_err(|e| {
            err_add_sev_snp_measure_fields(
                sev_snp_err(e, Some("failed to calculate AMD SEV-SNP launch digest".into())),
                vcpus,
                vcpu_type,
                ovmf_path,
                kernel_path,
                append_path,
                initrd_path,
            )
        })?,
    ))
}

#[allow(clippy::too_many_arguments)]
pub fn sev_snp_measure_cmp(
    vcpus: usize, vcpu_type: GuestCpuType, ovmf_path: &Path, kernel_path: &Path,
    append_path: &Path, initrd_path: &Path, expected_measurement: &str, label: &str,
) -> Result<()> {
    let calc_measurement =
        sev_snp_measure(vcpus, vcpu_type, ovmf_path, kernel_path, append_path, initrd_path)?;
    if !calc_measurement.eq(expected_measurement) {
        return Err(err_add_sev_snp_measure_fields(
            validation_err(
                format!(
                    "AMD SEV-SNP measurement invalid for '{label}' ('{expected_measurement}' vs '{calc_measurement}')"
                ),
                None,
            ),
            vcpus,
            vcpu_type,
            ovmf_path,
            kernel_path,
            append_path,
            initrd_path,
        ));
    }

    Ok(())
}

/// Dump as much information as possible for us to verify the cause.
pub(crate) fn err_add_sev_snp_measure_fields(
    err: Error, vcpus: usize, vcpu_type: GuestCpuType, ovmf_path: &Path, kernel_path: &Path,
    append_path: &Path, initrd_path: &Path,
) -> Error {
    err.add_field("vcpus", Value::from(vcpus))
        .add_field("vcpu_type", Value::from(vcpu_type.to_string()))
        .add_field("ovmf_hash", Value::from(maybe_sha512_file_to_hex(ovmf_path)))
        .add_field("kernel_hash", Value::from(maybe_sha512_file_to_hex(kernel_path)))
        .add_field("append_hash", Value::from(maybe_sha512_file_to_hex(append_path)))
        .add_field("initrd_hash", Value::from(maybe_sha512_file_to_hex(initrd_path)))
}

/// Just used to simplify the logging above.
pub(crate) fn maybe_sha512_file_to_hex(path: &Path) -> String {
    if let Ok(hash) = sha512_file(path) {
        bytes_to_hex(hash)
    } else {
        "None".to_string()
    }
}
