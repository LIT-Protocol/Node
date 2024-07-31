#[allow(unused_imports)]
use lit_blockchain::resolver::contract::ContractResolver;
#[allow(unused_imports)]
use lit_os_core::config::LitOsGuestConfig;
#[allow(unused_imports)]
use lit_os_core::error::{config_err, serializer_err, validation_err, Result};
#[allow(unused_imports)]
use lit_os_core::error::{conversion_err, unexpected_err};
#[allow(unused_imports)]
use lit_os_core::guest::oneshot::config::{
    ActionEntry, ACTION_SETTINGS_KEY_GUEST_VCPUS, ACTION_SETTINGS_KEY_REQUEST,
    ACTION_TYPE_BOOTSTRAP,
};
use lit_os_core::guest::oneshot::context::OneShotContext;
#[allow(unused_imports)]
use lit_os_core::guest::types::GuestType;
#[cfg(feature = "type-prov")]
use lit_os_prov_core::bootstrap::bootstrap as prov_core_bootstrap;
#[cfg(feature = "type-prov")]
use lit_os_prov_core::release::common::keys::write_identity_files;
#[cfg(feature = "type-prov")]
use lit_os_prov_core::release::create::types::CreateReleaseRequest;
#[allow(unused_imports)]
use log::info;
#[cfg(feature = "type-prov")]
use posix_acl::{ACL_EXECUTE, ACL_READ, ACL_WRITE};
#[allow(unused_imports)]
use std::path::PathBuf;

use crate::init::context::InitContext;
use crate::init::stage::oneshot::action::Outcome;
#[cfg(feature = "type-prov")]
use crate::init::stage::sync::{AclType, SyncItemAcl};

#[allow(unused_variables)]
#[allow(dead_code)]
pub(crate) async fn run(
    ctx: &mut InitContext, _oneshot_ctx: &OneShotContext, entry: &ActionEntry,
) -> Result<Outcome> {
    let guest_type = ctx.build_env().guest_type()?;

    match guest_type {
        #[cfg(feature = "type-prov")]
        GuestType::Prov => bootstrap_prov(ctx, _oneshot_ctx, entry).await,
        _ => Err(validation_err(
            format!("oneshot bootstrap not supported for guest type: {guest_type}"),
            None,
        )),
    }
}

#[cfg(feature = "type-prov")]
async fn bootstrap_prov(
    ctx: &mut InitContext, _oneshot_ctx: &OneShotContext, entry: &ActionEntry,
) -> Result<Outcome> {
    info!("Bootstrapping instance and subnet");

    let request =
        entry.settings().get(&ACTION_SETTINGS_KEY_REQUEST.to_string()).ok_or_else(|| {
            config_err(
                format!("bootstrap action requires setting: {ACTION_SETTINGS_KEY_REQUEST}"),
                None,
            )
        })?;
    let request: CreateReleaseRequest = serde_json::from_slice(&request[..]).map_err(|e| {
        serializer_err(
            e,
            Some("failed to deserialize CreateReleaseRequest from oneshot settings".into()),
        )
    })?;

    let guest_vcpus =
        entry.settings().get(&ACTION_SETTINGS_KEY_GUEST_VCPUS.to_string()).ok_or_else(|| {
            config_err(
                format!("bootstrap action requires setting: {ACTION_SETTINGS_KEY_GUEST_VCPUS}"),
                None,
            )
        })?;
    let guest_vcpus = u16::from_le_bytes(guest_vcpus.to_vec().try_into().map_err(|_e| {
        conversion_err(
            format!(
                "unable to convert '{ACTION_SETTINGS_KEY_GUEST_VCPUS}': '{guest_vcpus:?}' to u16"
            ),
            None,
        )
    })?);

    let resolver = ContractResolver::try_from(ctx.cfg())?;

    let mut updates: Option<Vec<PathBuf>> = Some(Vec::new());

    let (id_block, auth_info) =
        prov_core_bootstrap(ctx.cfg(), &resolver, &request, guest_vcpus, updates.as_mut())
            .await
            .map_err(|e| unexpected_err(e, Some("failed to bootstrap prov".into())))?;

    // Copy the id blocks to the oneshot volume
    let mut dest = ctx.cfg().litos_oneshot_mnt();
    dest.push(ACTION_TYPE_BOOTSTRAP);

    // Write identity files
    write_identity_files(
        dest.as_path(),
        ctx.build_env().guest_cpu_type()?,
        guest_vcpus,
        &vec![id_block, auth_info],
    )?;

    // Push updates (if any).
    if let Some(updates) = updates {
        for update in updates {
            ctx.push_synced_acl(
                update,
                None,
                vec![SyncItemAcl::new(
                    AclType::Group,
                    "lit-prov",
                    ACL_READ | ACL_WRITE | ACL_EXECUTE,
                )],
            );
        }
    }

    info!("Successfully bootstrapped prov");

    Ok(Outcome::Continue)
}
