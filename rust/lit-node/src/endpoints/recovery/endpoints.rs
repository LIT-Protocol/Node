use lit_api_core::error::ApiError;
#[allow(unused_imports)]
use lit_core::config::ReloadableLitConfig;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::State;
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::instrument;

use crate::endpoints::recovery::utils::{
    check_auth_sig_for_dec_share_upload, check_auth_sig_for_share_download,
};
use crate::endpoints::recovery::{
    do_delete_share_from_disk, do_share_download_from_rec_dkg, DECRYPTION_SHARE_CURVE_BLS,
    DECRYPTION_SHARE_CURVE_ECDSA,
};
use crate::error::{conversion_err, parser_err, unexpected_err};
use crate::models::{self};
use crate::tss::common::restore::RestoreState;
use crate::tss::common::tss_state::TssState;
use crate::utils::contract::get_backup_recovery_contract_with_signer;

#[instrument(name = "POST /web/recovery/set_dec_share", skip_all, ret)]
#[allow(dead_code)]
pub async fn recovery_set_dec_share(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RwLock<RestoreState>>>,
    request: Json<models::JsonRecoverySetDecShare>,
) -> status::Custom<Value> {
    let cfg = cfg.load_full();

    if let Err(e) =
        check_auth_sig_for_dec_share_upload(&cfg, restore_state, &request.auth_sig).await
    {
        return e.handle();
    }

    let staker_address = match super::get_staker_address(&cfg) {
        Ok(addr) => addr,
        Err(e) => return e.handle(),
    };

    let add_result = match request.share_data.curve.as_str() {
        DECRYPTION_SHARE_CURVE_BLS => {
            let decryption_share = match serde_json::from_str(&request.share_data.decryption_share)
            {
                Ok(share) => share,
                Err(e) => return conversion_err(e, None).handle(),
            };
            let mut restore_state = restore_state.write().await;
            restore_state.add_bls_decryption_share(
                request.auth_sig.address.clone(),
                request.share_data.verification_key.clone(),
                decryption_share,
                &staker_address,
            )
        }
        DECRYPTION_SHARE_CURVE_ECDSA => {
            let decryption_share = match serde_json::from_str(&request.share_data.decryption_share)
            {
                Ok(share) => share,
                Err(e) => return conversion_err(e, None).handle(),
            };
            let mut restore_state = restore_state.write().await;
            restore_state.add_ecdsa_decryption_share(
                request.auth_sig.address.clone(),
                request.share_data.verification_key.clone(),
                decryption_share,
            )
        }
        _ => {
            let err_msg = format!("Not a valid curve: {}", request.share_data.curve);
            let e = parser_err(Error::new(ErrorKind::Other, err_msg), None);
            return e.handle();
        }
    };

    if let Err(e) = add_result {
        return e.handle();
    }

    return status::Custom(
        Status::Ok,
        json!({
            "success": "true",
        }),
    );
}

#[instrument(name = "POST /web/recovery/get_dec_share", skip_all, ret)]
pub async fn recovery_get_dec_key_share(
    restore_state: &State<Arc<TssState>>,
    cfg: &State<ReloadableLitConfig>,
    request: Json<models::DownloadShareRequest>,
) -> status::Custom<Value> {
    let cfg = cfg.load_full();

    let recovery_contract = match get_backup_recovery_contract_with_signer(&cfg).await {
        Ok(recovery_contract) => recovery_contract,
        Err(e) => {
            return e.handle();
        }
    };
    let party_member = match recovery_contract.get_member_for_node_dkg().await {
        Ok(address) => address,
        Err(e) => {
            return unexpected_err(
                e,
                Some("Could not query backup party member for this peer, aborting".into()),
            )
            .handle();
        }
    };
    info!("found party member: {:?}", party_member);
    if let Err(e) = check_auth_sig_for_share_download(&cfg, &request.auth_sig, &party_member) {
        return e.handle();
    }
    info!("Party member authenticated, moving to share download");
    let shares = match do_share_download_from_rec_dkg(&cfg, &party_member, &recovery_contract).await
    {
        Ok(s) => s,
        Err(e) => {
            return unexpected_err(
                e,
                Some("Error while trying to resolve key share from disk".into()),
            )
            .handle();
        }
    };

    return status::Custom(Status::Ok, json!(shares));
}

#[instrument(name = "POST /web/recovery/delete_dec_share", skip_all, ret)]
pub async fn recovery_delete_dec_key_share(
    restore_state: &State<Arc<TssState>>,
    cfg: &State<ReloadableLitConfig>,
    request: Json<models::DownloadShareRequest>,
) -> status::Custom<Value> {
    let cfg = cfg.load_full();

    let recovery_contract = match get_backup_recovery_contract_with_signer(&cfg).await {
        Ok(recovery_contract) => recovery_contract,
        Err(e) => {
            return e.handle();
        }
    };
    let party_member = match recovery_contract.get_member_for_node_dkg().await {
        Ok(address) => address,
        Err(e) => {
            return unexpected_err(
                e,
                Some("Could not query backup party member for this peer, aborting".into()),
            )
            .handle();
        }
    };
    info!("found party member: {:?}", party_member);
    if let Err(e) = check_auth_sig_for_share_download(&cfg, &request.auth_sig, &party_member) {
        return e.handle();
    }

    let delete_res = match do_delete_share_from_disk(&cfg, &party_member, &recovery_contract).await
    {
        Ok(res) => res,
        Err(e) => {
            return unexpected_err(e, Some("Error while deleting share from disk".into())).handle();
        }
    };

    if !delete_res {
        return unexpected_err("Error while deleting share from disk", None).handle();
    }

    return status::Custom(Status::Ok, json!({ "shareDeleted": delete_res }));
}
