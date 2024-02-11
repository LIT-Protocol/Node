use std::path::Path;

use cryptsetup_rs::{crypt_keyslot_info, open, Keyslot, Luks2CryptDevice};
use lit_os_core::error::{generic_err, Result};

pub fn cryptsetup_init_dev(path: &Path, label: &str) -> Result<impl Luks2CryptDevice> {
    let dev = open(path)
        .map_err(|e| generic_err(e, Some(format!("failed to init '{label}' luks device"))))?
        .luks2()
        .map_err(|e| generic_err(e, Some(format!("failed to init luks2 '{label}' luks device"))))?;

    Ok(dev)
}

pub fn cryptsetup_has_slot_active(
    dev: &mut impl Luks2CryptDevice, _label: &str, slot: u8,
) -> Result<bool> {
    let status = dev.keyslot_status(slot);

    Ok(status == crypt_keyslot_info::CRYPT_SLOT_ACTIVE
        || status == crypt_keyslot_info::CRYPT_SLOT_ACTIVE_LAST)
}

pub fn cryptsetup_add_keyslot(
    dev: &mut impl Luks2CryptDevice, label: &str, passphrase: &[u8], new_passphrase: &[u8],
) -> Result<Keyslot> {
    let slot = dev.add_keyslot(new_passphrase, Some(passphrase), None).map_err(|e| {
        generic_err(e, Some(format!("failed to add keyslot on '{label}' luks device")))
    })?;

    Ok(slot)
}

pub fn cryptsetup_destroy_slot(
    dev: &mut impl Luks2CryptDevice, label: &str, slot: u8,
) -> Result<()> {
    dev.destroy_keyslot(slot).map_err(|e| {
        generic_err(e, Some(format!("failed to destroy keyslot '{slot}' on '{label}' luks device")))
    })?;

    Ok(())
}
