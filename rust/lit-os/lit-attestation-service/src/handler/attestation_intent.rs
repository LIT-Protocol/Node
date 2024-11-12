use std::sync::Arc;
use std::{
    time::{SystemTime, UNIX_EPOCH},
    vec,
};

use lit_api_core::server::hyper::handler::types::{Request, Response};
use log::{as_serde, trace};
use serde_bytes_base64::Bytes;

use lit_attestation::attestation::{DATA_KEY_BUILD_ID, DATA_KEY_SUBNET_ID};
use lit_attestation::service::types::{AttestationIntentReq, AttestationIntentResp};
use lit_attestation::{
    attestation::{DATA_KEY_INSTANCE_ID, DATA_KEY_RELEASE_ID, DATA_KEY_UNIX_TIME},
    Attestation, AttestationType,
};

use crate::context::ContextHelper;
use crate::data::cache::cache_key_session;
use crate::error::{unexpected_err, Result};
use crate::{
    data::cache::CACHE,
    error::{self, validation_err_code},
};

pub(crate) async fn handle_req(req: Request) -> Result<Response> {
    let ctx = req.ctx().service_ctx()?;
    let req: AttestationIntentReq = req.deserialize().await?;

    trace!(req = as_serde!(req); "attestation_intent::handle_req");

    // Create initial Attestation object.
    let attestation_type = AttestationType::from_system().ok_or_else(|| {
        validation_err_code(
            "System does not support attestation".to_string(),
            error::EC::AttestationServiceUnsupportedFunction,
            None,
        )
        .add_source_to_details()
    })?;

    // Generate Attestation object template.
    let mut attestation =
        Attestation::new(attestation_type, req.noonce.unwrap_or(Bytes::from(vec![])).val).await?;

    // Insert release_id, instance_id, unix_time.
    let unix_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| unexpected_err(e, Some("Unable to get system time".into())))?;

    attestation.insert_data(DATA_KEY_UNIX_TIME, unix_time.as_secs().to_le_bytes().to_vec());
    attestation.insert_data(DATA_KEY_INSTANCE_ID, ctx.instance_id.as_bytes().to_vec());

    if let Some(release_id) = ctx.release_id.clone() {
        attestation.insert_data(DATA_KEY_RELEASE_ID, release_id.as_bytes().to_vec());
    } else {
        attestation.insert_data(DATA_KEY_BUILD_ID, ctx.build_id.as_bytes().to_vec());

        if let Some(subnet_id) = ctx.subnet_id.clone() {
            attestation.insert_data(DATA_KEY_SUBNET_ID, subnet_id.as_bytes().to_vec());
        }
    }

    // Persist attestation intent.
    let session_id = uuid::Uuid::new_v4().simple().to_string();
    CACHE.insert(cache_key_session(&session_id), Arc::new(attestation.clone())).await;

    // Return response.
    Response::try_from(AttestationIntentResp { attestation, session_id })
}

/// Note that this test is only intended to be run with AMD SEV SNP firmware support.
#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::{env, sync::Arc};

    use lit_api_core::server::hyper::handler::types::{Context, Request};
    use serde_bytes_base64::Bytes;

    use lit_attestation::attestation::{
        DATA_KEY_BUILD_ID, DATA_KEY_INSTANCE_ID, DATA_KEY_RELEASE_ID, DATA_KEY_SUBNET_ID,
        DATA_KEY_UNIX_TIME, ENV_ATTESTATION_TYPE_OVERRIDE,
    };
    use lit_attestation::config::ENV_ATTESTATION_SERVICE_SOCK_PATH;
    use lit_attestation::service::types::{AttestationIntentReq, AttestationIntentResp};
    use lit_core::config::LitConfig;
    use lit_core::config::ENV_LIT_CONFIG_FILE;
    use lit_os_core::config::ENV_BUILD_PRIV_KEY_PATH;

    use crate::context::{ServiceContext, CTX_KEY_SERVICE_CTX};
    use crate::data::cache::cache_key_session;
    use crate::handler::attestation_intent::handle_req;
    use crate::{config::LitAttestationServiceConfig, data::cache::CACHE};

    const RESOURCES_TEST_DIR: &str = "resources/test";

    #[tokio::test]
    async fn test_handle_req_success() {
        // Init
        setup_test_env();
        let cfg = Arc::new(<LitConfig as LitAttestationServiceConfig>::must_new());
        let noonce = vec![2, 1, 3];
        let req_body = AttestationIntentReq { noonce: Some(Bytes::from(noonce.clone())) };

        // Handle request
        let mut ctx = Context::new();
        ctx.insert(CTX_KEY_SERVICE_CTX, ServiceContext::from_lit_config(cfg.clone()).unwrap());

        let req = Request::try_from(&req_body, Arc::new(ctx)).expect("failed to extract Request");
        let res = handle_req(req).await;

        // Assertions
        if let Err(e) = res {
            panic!("Res is error: {e:?}")
        }

        let attestation_intent_resp: AttestationIntentResp =
            res.unwrap().deserialize().await.expect("failed to deserialize");

        assert_eq!(
            *attestation_intent_resp.attestation.typ(),
            lit_attestation::AttestationType::AmdSevSnp
        );
        assert_eq!(*attestation_intent_resp.attestation.noonce(), noonce.clone());
        assert!(attestation_intent_resp.attestation.data().contains_key(DATA_KEY_UNIX_TIME));
        assert!(attestation_intent_resp.attestation.data().contains_key(DATA_KEY_INSTANCE_ID));

        let data_contains_release_or_both_build_and_subnet_ids =
            attestation_intent_resp.attestation.data().contains_key(DATA_KEY_RELEASE_ID)
                || (attestation_intent_resp.attestation.data().contains_key(DATA_KEY_BUILD_ID)
                    && attestation_intent_resp.attestation.data().contains_key(DATA_KEY_SUBNET_ID));
        assert!(data_contains_release_or_both_build_and_subnet_ids);
        assert_eq!(attestation_intent_resp.attestation.signatures().len(), 0);
        assert!(attestation_intent_resp.attestation.report_raw().is_err());

        assert!(CACHE.get(&cache_key_session(&attestation_intent_resp.session_id)).await.is_some());
    }

    // Util
    fn get_test_path(path: &str) -> PathBuf {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push(RESOURCES_TEST_DIR);
        test_path.push(path);
        test_path
    }

    fn setup_test_env() {
        env::set_var(ENV_LIT_CONFIG_FILE, get_test_path("config/config.toml"));
        env::set_var(ENV_BUILD_PRIV_KEY_PATH, get_test_path("config/build.pem"));
        env::set_var(ENV_ATTESTATION_SERVICE_SOCK_PATH, "/tmp/lit-attestation-service.sock");
        env::set_var(ENV_ATTESTATION_TYPE_OVERRIDE, "AMD_SEV_SNP");
    }
}
