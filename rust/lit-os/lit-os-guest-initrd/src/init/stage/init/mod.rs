use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::error::Unexpected;
use lit_os_core::error::unexpected_err;
#[allow(unused_imports)]
use lit_os_core::error::{generic_err, Result};
use lit_os_prov_api_client::api::release::ProvApiClientRelease;
use lit_os_prov_api_client::client::ProvApiClient;
use lit_os_prov_core::release::init::types::{InitRelease, InitReleaseRequest};
#[allow(unused_imports)]
use log::info;
use std::path::PathBuf;

use crate::init::context::{
    InitContext, CTX_KEY_PASSPHRASE_INIT, CTX_KEY_ROOT_PASSPHRASE_BOOT, CTX_KEY_VAR_PASSPHRASE_BOOT,
};
use crate::init::stage::Outcome;
#[cfg(not(target_os = "macos"))]
use crate::utils::cryptsetup_sys::cryptsetup_reencrypt_dev;
#[cfg(not(target_os = "macos"))]
use crate::utils::libcryptsetup::{
    cryptsetup_add_keyslot, cryptsetup_has_slot_active, cryptsetup_init_dev,
};
#[cfg(not(target_os = "macos"))]
use libcryptsetup_rs::CryptDevice;

#[allow(unused_variables)]
pub(crate) async fn run(ctx: &mut InitContext) -> Result<Outcome> {
    // Load LUKS devices
    let (root_dev_path, root_dev_label, var_dev_path, var_dev_label) =
        volume_paths_and_labels(ctx)?;

    #[cfg(not(target_os = "macos"))]
    let mut root_dev = cryptsetup_init_dev(root_dev_path.as_path(), root_dev_label.as_str())?;

    #[cfg(not(target_os = "macos"))]
    let mut var_dev = cryptsetup_init_dev(var_dev_path.as_path(), var_dev_label.as_str())?;

    // Load / generate keys
    let init_passphrase = load_init_passphrases(ctx).await?;
    let (root_passphrase, var_passphrase) =
        load_boot_passphrases(ctx, &root_dev_label, &var_dev_label).await?;

    #[cfg(not(target_os = "macos"))]
    if cryptsetup_has_slot_active(&mut root_dev, root_dev_label.as_str(), 0)? {
        // First boot
        ctx.set_is_first_boot(true);

        info!("First boot, initializing LUKS volumes");

        init_luks_volumes(
            ctx,
            &mut root_dev,
            &root_dev_label,
            &root_passphrase[..],
            &mut var_dev,
            &var_dev_label,
            &var_passphrase[..],
            &init_passphrase[..],
        )
        .await?;
    }

    Ok(Outcome::Continue)
}

#[allow(dead_code)]
async fn load_init_passphrases(ctx: &mut InitContext) -> Result<Vec<u8>> {
    if let Some(passphrase) = ctx.get_bin(CTX_KEY_PASSPHRASE_INIT) {
        // Provided by cloud-init files.
        Ok(passphrase.clone())
    } else {
        if !ctx.is_release() {
            return Err(unexpected_err(
                "Missing passphrase! Expected the guest to be a release!", None,
            ));
        }

        let resolver = ContractResolver::try_from(ctx.cfg())?;
        let client = ProvApiClient::new(ctx.cfg(), Some(&resolver)).await?;

        let release_env = ctx.release_env().expect_or_err("expected release.env to be present!")?;
        let release_id = release_env
            .release_id
            .as_ref()
            .expect_or_err("expected release.env to contain a release id")?;

        let req = InitRelease::new(release_id.clone());
        let auth = ctx.request_attestation(None, Some(req.sha512().to_vec())).await?;

        let res = client.init_release(&InitReleaseRequest::new(auth, req)).await?;

        Ok(res.passphrase.to_vec())
    }
}

pub(crate) fn volume_paths_and_labels(
    ctx: &mut InitContext,
) -> Result<(PathBuf, String, PathBuf, String)> {
    let root_dev_path = ctx
        .build_env()
        .build_luks_root_dev()
        .expect("expected build_luks_root_dev to have already been verified");
    let root_dev_label = format!("{}:{:?}", "root", root_dev_path.as_path());

    let var_dev_path = ctx
        .build_env()
        .build_luks_var_dev()
        .expect("expected build_luks_var_dev to have already been verified");
    let var_dev_label = format!("{}:{:?}", "var", var_dev_path.as_path());

    Ok((root_dev_path, root_dev_label, var_dev_path, var_dev_label))
}

pub(crate) async fn prepare_boot_passphrases(ctx: &mut InitContext) -> Result<()> {
    let (_root_dev_path, root_dev_label, _var_dev_path, var_dev_label) =
        volume_paths_and_labels(ctx)?;

    let _ = load_boot_passphrases(ctx, &root_dev_label, &var_dev_label).await?;

    Ok(())
}

#[allow(dead_code)]
async fn load_boot_passphrases(
    ctx: &mut InitContext, root_dev_label: &String, var_dev_label: &String,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let root_passphrase =
        ctx.passphrase_of_length(format!("{root_dev_label}:boot-passphrase").as_str(), 64).await?;

    ctx.insert(CTX_KEY_ROOT_PASSPHRASE_BOOT.to_string(), Box::new(root_passphrase.clone()));

    let var_passphrase =
        ctx.passphrase_of_length(format!("{var_dev_label}:boot-passphrase").as_str(), 64).await?;

    ctx.insert(CTX_KEY_VAR_PASSPHRASE_BOOT.to_string(), Box::new(var_passphrase.clone()));

    Ok((root_passphrase, var_passphrase))
}

#[cfg(not(target_os = "macos"))]
#[allow(clippy::too_many_arguments)]
async fn init_luks_volumes(
    ctx: &mut InitContext, root_dev: &mut CryptDevice, root_dev_label: &String,
    root_passphrase: &[u8], var_dev: &mut CryptDevice, var_dev_label: &String,
    var_passphrase: &[u8], init_passphrase: &[u8],
) -> Result<()> {
    init_luks_volume(ctx, root_dev, root_dev_label, init_passphrase, root_passphrase).await?;
    init_luks_volume(ctx, var_dev, var_dev_label, init_passphrase, var_passphrase).await?;

    Ok(())
}

// Init the luks volume:
// 1. Add the new passphrase
// 2. DO NOT delete the old keyslot (force the new keyslot to be 2 during reencrypt).
// 3. Call reencrypt, which will wipe the first two key slots and store the new key in slot 2.
// 4. We rely on slot 0 being empty to determine if init has been done.
#[cfg(not(target_os = "macos"))]
async fn init_luks_volume(
    ctx: &mut InitContext, dev: &mut CryptDevice, label: &String, init_passphrase: &[u8],
    boot_passphrase: &[u8],
) -> Result<()> {
    info!("Initializing volume '{}'", label);

    // Generate a 64 byte master_key
    let master_key = ctx.random_key_of_length(format!("{label}:master-key").as_str(), 64).await?;

    let mut new_slot = cryptsetup_add_keyslot(dev, label, init_passphrase, boot_passphrase)?;
    if new_slot != 1 {
        return Err(generic_err(
            format!(
                "luks init failed for '{}', expected new keyslot to be '{}' (is: '{}')",
                label, 1, new_slot
            ),
            None,
        ));
    }

    cryptsetup_reencrypt_dev(
        dev.status_handle()
            .get_device_path()
            .map_err(|e| generic_err(e, Some("Couldn't get luks device path".into())))?,
        label,
        boot_passphrase,
        1,
        &master_key[..],
        Some(512),
        Some("aes-xts-plain64"),
        Some("sha512"),
    )?;

    // New keyslot will be the next one (usually slot 2).
    new_slot += 1;

    // Reload device
    let mut dev = cryptsetup_init_dev(
        dev.status_handle()
            .get_device_path()
            .map_err(|e| generic_err(e, Some("Couldn't get luks device path".into())))?,
        label,
    )?;
    if cryptsetup_has_slot_active(&mut dev, label, 0)? {
        return Err(generic_err(
            format!("luks init failed for '{label}', slot '0' is still defined"),
            None,
        ));
    }
    if !cryptsetup_has_slot_active(&mut dev, label, new_slot)? {
        return Err(generic_err(
            format!("luks init failed for '{label}', slot '{new_slot}' is not defined"),
            None,
        ));
    }

    Ok(())
}
