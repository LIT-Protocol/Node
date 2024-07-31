use libcryptsetup_rs::consts::flags::{CryptActivate, CryptDeactivate};
use libcryptsetup_rs::{CryptDevice, CryptInit};
use lit_os_core::error::{generic_err, Result};
use std::path::Path;

pub fn cryptsetup_init_dev(path: &Path, label: &str) -> Result<CryptDevice> {
    log::info!("cryptsetup_init_dev:{}:{}", std::file!(), std::line!());
    let mut dev = CryptInit::init(path)
        .map_err(|e| generic_err(e, Some(format!("failed to init '{label}' as crypt device"))))?;

    dev.context_handle()
        .load(Some(libcryptsetup_rs::consts::vals::EncryptionFormat::Luks2), Some(&mut ()))
        .map_err(|e| {
            generic_err(
                format!("device '{label}' is not a luks2 device. Libcrypt returned {}", e),
                None,
            )
        })
        .and(Ok(dev))
}

pub fn cryptsetup_activate_dev(
    path: &Path, label: &str, name: &str, passphrase: &[u8], read_only: bool,
) -> Result<()> {
    log::info!("cryptsetup_activate_dev:{}:{}", std::file!(), std::line!());
    let mut dev: CryptDevice = cryptsetup_init_dev(path, label)?;

    let mut flags = libcryptsetup_rs::consts::flags::CryptActivate::empty();
    flags.set(CryptActivate::READONLY, read_only);

    // RAD: None in keyslot means any
    dev.activate_handle().activate_by_passphrase(Some(name), None, passphrase, flags).map_err(
        |e| {
            generic_err(
                format!("failed to activate '{}' luks device: status: {:?}", label, e.to_string()),
                None,
            )
        },
    )?;
    Ok(())
}

pub fn cryptsetup_deactivate_dev(path: &Path, label: &str, name: &str) -> Result<()> {
    log::info!("cryptsetup_deactivate_dev:{}:{}", std::file!(), std::line!());
    // RAD: We're discarding the good error here, may want to refactor
    let mut dev = cryptsetup_init_dev(path, label)?;
    dev.activate_handle().deactivate(name, CryptDeactivate::empty()).map_err(|e| {
        generic_err(
            format!(
                "failed to exec 'cryptsetup close' '{}' luks device: status: {:?}",
                label,
                e.to_string()
            ),
            None,
        )
    })
}

pub fn cryptsetup_has_slot_active(dev: &mut CryptDevice, label: &str, slot: u32) -> Result<bool> {
    log::info!("cryptsetup_has_slot_active:{}:{}", std::file!(), std::line!());
    let status = dev.keyslot_handle().status(slot).map_err(|e| {
        generic_err(
            e,
            Some(format!("failed to request status of keyslot '{slot}' on '{label}' luks device")),
        )
    })?;
    Ok(status == libcryptsetup_rs::consts::vals::KeyslotInfo::Active
        || status == libcryptsetup_rs::consts::vals::KeyslotInfo::ActiveLast)
}

pub fn cryptsetup_add_keyslot(
    dev: &mut CryptDevice, label: &str, passphrase: &[u8], new_passphrase: &[u8],
) -> Result<u32> {
    log::info!("cryptsetup_add_keyslot:{}:{}", std::file!(), std::line!());
    // RAD: I assume None here means "next slot" and the fn returns the used slot number, similar to cryptsetup_rs
    let slot =
        dev.keyslot_handle().add_by_passphrase(None, passphrase, new_passphrase).map_err(|e| {
            generic_err(e, Some(format!("failed to add keyslot on '{label}' luks device")))
        })?;
    Ok(slot)
}

// RAD: Why is this dead code
pub fn cryptsetup_destroy_slot(dev: &mut CryptDevice, label: &str, slot: u32) -> Result<()> {
    log::info!("cryptsetup_destroy_slot:{}:{}", std::file!(), std::line!());
    dev.keyslot_handle().destroy(slot).map_err(|e| {
        generic_err(e, Some(format!("failed to destroy keyslot '{slot}' on '{label}' luks device")))
    })
}
