#[cfg(feature = "testing")]
use crate::endpoints::admin::utils::get_test_recovery_party;
use crate::endpoints::admin::utils::{
    check_admin_auth_sig, encrypt_and_tar_backup_keys, purge_precomputes, untar_keys_stream,
};
use crate::error::{parser_err, validation_err_code, EC};
use crate::models;
#[cfg(not(feature = "testing"))]
use crate::tss::common::backup::get_recovery_party;
use crate::tss::common::restore::{
    get_blinders, report_progress, NodeRecoveryStatus, RestoreState,
};

use crate::config::LitNodeConfig;
use chrono::{DateTime, Utc};
use lit_api_core::error::ApiError;
use lit_api_core::http::rocket::helper::stream::ChildStream;
use lit_blockchain::resolver::rpc::config::{RpcConfig, RPC_CONFIG_PROTECTED_CHAINS};
use lit_blockchain::resolver::rpc::{RpcResolver, RPC_RESOLVER};
use lit_core::config::{ReloadableLitConfig, CFG_ADMIN_OVERRIDE_NAME};
use rocket::data::ByteUnit;
use rocket::http::{ContentType, Status};
use rocket::response::status;
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::{Data, State};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::instrument;

use super::guards::AdminAuthSig;

#[instrument(name = "POST /web/admin/set", skip_all, ret)]
pub async fn admin_set(
    remote_addr: SocketAddr,
    reloadable_cfg: &State<ReloadableLitConfig>,
    request: Json<models::JsonAdminSetRequest>,
) -> status::Custom<Value> {
    let cfg = reloadable_cfg.load_full();

    if let Err(e) = check_admin_auth_sig(&cfg, &request.auth_sig) {
        return e.handle();
    }

    // validate the config
    if let Err(e) = cfg.verify_user_editable(&request.new_config) {
        return parser_err(
            e,
            Some(
                "Invalid config.  You may have a non-user editable config key in there somewhere."
                    .into(),
            ),
        )
        .add_msg_to_details()
        .handle();
    }

    // write the config to the config file
    if let Err(e) = cfg.save_local_config(CFG_ADMIN_OVERRIDE_NAME, &request.new_config) {
        return e.handle();
    }

    if let Err(e) = reloadable_cfg.reload() {
        return e.handle();
    }

    // ok now do the same for the rpc config
    if let Err(e) = request.rpc_config.verify() {
        return e.handle();
    }

    // Prevent any changes to the RPC entries that are not allowed
    let existing_rpc_resolver = RPC_RESOLVER.load();
    let new_rpc_config = request.rpc_config.chains();
    for chain in RPC_CONFIG_PROTECTED_CHAINS {
        // Get RPC entries from request
        let new_rpc_entries = match new_rpc_config.get(chain) {
            Some(new_rpc_entries) => new_rpc_entries,
            None => {
                return validation_err_code(
                    "Missing RPC config entry for mandatory chain",
                    EC::NodeRpcConfigForbidden,
                    None,
                )
                .add_msg_to_details()
                .handle();
            }
        };

        // Get RPC entries from existing config
        let existing_rpc_entries = match existing_rpc_resolver.resolve(chain) {
            Ok(existing_rpc_entries) => existing_rpc_entries,
            Err(e) => {
                return validation_err_code(
                    e,
                    EC::NodeRpcConfigForbidden,
                    Some("Cannot resolve chain in existing RPC config".into()),
                )
                .add_msg_to_details()
                .handle();
            }
        };

        // Compare - if they are different, then the user is trying to change a protected value. Reject if so.
        if *existing_rpc_entries != *new_rpc_entries {
            return validation_err_code(
                "Unauthorized to edit protected chain values",
                EC::NodeRpcConfigForbidden,
                None,
            )
            .handle();
        }
    }

    if let Err(e) = request.rpc_config.write_file_local() {
        return e.handle();
    }

    if let Err(e) = RpcResolver::reload() {
        return e.handle();
    }

    return status::Custom(
        Status::Ok,
        json!({
            "success": "true",
        }),
    );
}

#[instrument(name = "POST /web/admin/get", skip_all, ret)]
pub async fn admin_get(
    cfg: &State<ReloadableLitConfig>,
    auth: Json<models::AdminAuth>,
) -> status::Custom<Value> {
    let cfg = cfg.load_full();

    if let Err(e) = check_admin_auth_sig(&cfg, &auth.auth_sig) {
        return e.handle();
    }

    let exported = match cfg.export_user_editable() {
        Ok(exported) => exported,
        Err(e) => {
            return parser_err(e, Some("Error exporting config".into()))
                .add_msg_to_details()
                .handle();
        }
    };

    // get rpc config, too
    let rpc_config = match RpcConfig::load() {
        Ok(rpc_config) => rpc_config,
        Err(e) => {
            return parser_err(e, Some("Error loading rpc config".into()))
                .add_msg_to_details()
                .handle();
        }
    };
    let chains = rpc_config.chains();

    return status::Custom(
        Status::Ok,
        json!({
            "success": "true",
            "config": exported,
            "chains": chains,
        }),
    );
}

#[instrument(name = "POST /web/admin/get_blinders", skip_all, ret)]
pub async fn admin_get_blinders(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RwLock<RestoreState>>>,
    auth: Json<models::AdminAuth>,
) -> status::Custom<Value> {
    let cfg = cfg.load_full();

    if let Err(e) = check_admin_auth_sig(&cfg, &auth.auth_sig) {
        return e.handle();
    }

    let (bls_blinder, k256_blinder) = get_blinders(restore_state.inner()).await;

    return status::Custom(
        Status::Ok,
        json!({
            "success": "true",
            "bls_blinder": bls_blinder,
            "k256_blinder": k256_blinder,
        }),
    );
}

#[instrument(name = "GET /web/admin/get_key_backup", skip_all, ret)]
pub async fn admin_get_key_backup(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RwLock<RestoreState>>>,
    auth: models::AdminAuth,
    node_set_hash: Option<String>,
) -> Result<ChildStream, status::Custom<Value>> {
    let cfg = cfg.load_full();

    if let Err(e) = check_admin_auth_sig(&cfg, &auth.auth_sig) {
        return Err(e.handle());
    }
    trace!("Auth sig check passed");

    let now: DateTime<Utc> = Utc::now();

    let (bls_blinder, k256_blinder) = get_blinders(restore_state.inner()).await;
    trace!("Got blinders");
    let recovery_party;
    #[cfg(feature = "testing")]
    {
        recovery_party = get_test_recovery_party();
    }
    #[cfg(not(feature = "testing"))]
    {
        recovery_party = match get_recovery_party(&cfg).await {
            Ok(recovery_party) => recovery_party,
            Err(e) => return Err(e.handle()),
        };
    }
    trace!("Got recovery party");

    // Zip up and encrypt.
    match encrypt_and_tar_backup_keys(
        &cfg,
        &bls_blinder,
        &k256_blinder,
        &recovery_party,
        node_set_hash,
    )
    .await
    {
        Ok(child) => Ok(ChildStream((
            child,
            ContentType::Binary,
            format!(
                "{}_lit_backup_encrypted_keys.tar.gz",
                now.format("%Y-%m-%d")
            ),
        ))),
        Err(e) => Err(e.handle()),
    }
}

#[instrument(name = "POST /web/admin/set_key_backup", skip_all, ret)]
pub async fn admin_set_key_backup(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RwLock<RestoreState>>>,
    admin_auth_sig: AdminAuthSig,
    data: Data<'_>,
) -> status::Custom<Value> {
    trace!("admin_set_key_backup() called");
    let cfg = cfg.load_full();

    if let Err(e) = check_admin_auth_sig(&cfg, &admin_auth_sig.auth_sig) {
        return e.handle();
    }

    trace!("admin_set_key_backup() - decrypting and untaring file");

    // Unzip the file, which should replace the BLS and ECDSA key material.
    let stream = data.open(ByteUnit::Gigabyte(u64::MAX));
    if let Err(e) = untar_keys_stream(&cfg, restore_state.as_ref(), stream).await {
        return e.handle();
    }

    report_progress(cfg.as_ref(), NodeRecoveryStatus::BackupsAreLoaded).await;

    trace!("admin_set_key_backup() - removing precompute files");
    if let Err(e) = purge_precomputes(&cfg).await {
        return e.handle();
    }

    return status::Custom(
        Status::Ok,
        json!({
            "success": "true",
        }),
    );
}
