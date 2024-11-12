use std::env;

use lit_api_core::server::hyper::handler::types::{Request, Response};
use log::{as_serde, trace};

use lit_attestation::service::types::{AttestationReq, AttestationResp};
use lit_attestation::AttestationType;

use crate::context::ContextHelper;
use crate::data::cache::cache_key_session;
use crate::{
    data::cache::{CacheValue, CACHE},
    error::{self, generic_err_code, io_err_code, validation_err_code, Result},
};

pub(crate) const ENV_ATTESTATION_SERVICE_BYPASS_GENERATE: &str =
    "LIT_ATTESTATION_SERVICE_BYPASS_GENERATE";

pub(crate) async fn handle_req(req: Request) -> Result<Response> {
    let ctx = req.ctx().service_ctx()?;
    let req: AttestationReq = req.deserialize().await?;

    trace!(req = as_serde!(req); "attestation::handle_req");

    let attestation = req.attestation;
    let cache_key = cache_key_session(&req.session_id);

    match CACHE.get(&cache_key).await {
        Some(value) => {
            match value.as_attestation() {
                Some(existing_attestation) => {
                    // Check that release_id, build_id, subnet_id, instance_id and unix_time is still present
                    // and the same in the Attestation data field as when issued during the intent request.
                    if existing_attestation.release_id() != attestation.release_id()
                        || existing_attestation.build_id() != attestation.build_id()
                        || existing_attestation.subnet_id() != attestation.subnet_id()
                        || existing_attestation.instance_id() != attestation.instance_id()
                        || existing_attestation.unix_time() != attestation.unix_time()
                    {
                        return Err(
                            validation_err_code(
                                "RELEASE_ID, BUILD_ID, SUBNET_ID, INSTANCE_ID or UNIX_TIME data fields may not be modified".to_string(),
                                error::EC::AttestationServiceInconsistentAttestation,
                                None,
                            ).add_source_to_details()
                        );
                    }
                }
                None => {
                    return Err(validation_err_code(
                        "Session id is invalid or expired (value stored is not correct)"
                            .to_string(),
                        error::EC::AttestationServiceMissingIntent,
                        None,
                    )
                    .add_source_to_details());
                }
            }
        }
        None => {
            return Err(validation_err_code(
                "Session id is invalid or expired".to_string(),
                error::EC::AttestationServiceMissingIntent,
                None,
            )
            .add_source_to_details())
        }
    }

    // Check attestation type.
    if !AttestationType::AmdSevSnp.eq(attestation.typ()) {
        return Err(validation_err_code(
            "Attestation type not supported".to_string(),
            error::EC::AttestationServiceUnsupportedType,
            None,
        )
        .add_source_to_details());
    }

    // Use build private key to sign the Attestation object.
    trace!("Signing attestation object with build private key");
    let mut complete_attestation = attestation.clone();
    complete_attestation.sign(ctx.build_priv_key.as_str()).map_err(|e| {
        generic_err_code(
            e,
            error::EC::AttestationServiceFailureSigning,
            Some("Unable to sign attestation with private key".into()),
        )
    })?;

    if env::var(ENV_ATTESTATION_SERVICE_BYPASS_GENERATE).is_err() {
        // Generate a SEV-SNP report
        trace!("Generating SEV-SNP report");
        complete_attestation = complete_attestation
            .generate()
            .await
            .map_err(|e| io_err_code(e, error::EC::AttestationServiceFailureGenerating, None))?;
    }

    // Invalidate the session.
    CACHE.invalidate(&cache_key).await;

    Response::try_from(AttestationResp { attestation: complete_attestation })
}
/// Note that this test is only intended to be run with AMD SEV SNP firmware support.
#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::{env, sync::Arc, vec};

    use lit_api_core::server::hyper::handler::types::{Context, Request};
    use serial_test::serial;
    use uuid::Uuid;

    use lit_attestation::attestation::ENV_ATTESTATION_TYPE_OVERRIDE;
    use lit_attestation::config::ENV_ATTESTATION_SERVICE_SOCK_PATH;
    use lit_attestation::service::types::{AttestationReq, AttestationResp};
    use lit_attestation::{
        attestation::{
            DATA_KEY_BUILD_ID, DATA_KEY_INSTANCE_ID, DATA_KEY_RELEASE_ID, DATA_KEY_SUBNET_ID,
            DATA_KEY_UNIX_TIME,
        },
        Attestation,
    };
    use lit_core::config::LitConfig;
    use lit_core::config::ENV_LIT_CONFIG_FILE;
    use lit_os_core::config::ENV_BUILD_PRIV_KEY_PATH;

    use crate::context::{ServiceContext, CTX_KEY_SERVICE_CTX};
    use crate::data::cache::cache_key_session;
    use crate::handler::attestation::{handle_req, ENV_ATTESTATION_SERVICE_BYPASS_GENERATE};
    use crate::{config::LitAttestationServiceConfig, data::cache::CACHE, error};

    const RESOURCES_TEST_DIR: &str = "resources/test";

    #[serial]
    #[tokio::test]
    async fn test_handle_req_incorrect_session_id() {
        // Init
        setup_test_env();
        let cfg = Arc::new(<LitConfig as LitAttestationServiceConfig>::must_new());
        let attestation = Attestation::new(lit_attestation::AttestationType::AmdSevSnp, vec![0])
            .await
            .expect("failed to construct Attestation");
        let req_body = AttestationReq { attestation, session_id: Uuid::new_v4().to_string() };

        // Handle request
        let mut ctx = Context::new();
        ctx.insert(CTX_KEY_SERVICE_CTX, ServiceContext::from_lit_config(cfg.clone()).unwrap());

        let req = Request::try_from(&req_body, Arc::new(ctx)).expect("failed to extract Request");
        let res = handle_req(req).await;

        // Assertions
        assert!(res.is_err());

        let res_err = res.unwrap_err();

        println!("Res error: {res_err:?}");

        assert!(res_err.is_code(error::EC::AttestationServiceMissingIntent, true));
        assert!(res_err.is_kind(error::Kind::Validation, true));
    }

    #[serial]
    #[tokio::test]
    async fn test_handle_req_incorrect_data() {
        setup_test_env();
        let cfg = Arc::new(<LitConfig as LitAttestationServiceConfig>::must_new());

        // Run through test cases.
        for (cache_attestation, request_attestation, session_id) in
            get_test_handle_req_incorrect_data_test_cases().await
        {
            // Seed cache with attestation intent.
            CACHE.insert(cache_key_session(&session_id), Arc::new(cache_attestation)).await;

            // Init
            let req_body = AttestationReq { attestation: request_attestation, session_id };

            // Handle request
            let mut ctx = Context::new();
            ctx.insert(CTX_KEY_SERVICE_CTX, ServiceContext::from_lit_config(cfg.clone()).unwrap());

            let req =
                Request::try_from(&req_body, Arc::new(ctx)).expect("failed to extract Request");
            let res = handle_req(req).await;

            // Assertions
            assert!(res.is_err());

            let res_err = res.unwrap_err();

            println!("Res error: {res_err:?}");

            assert!(res_err.is_code(error::EC::AttestationServiceInconsistentAttestation, true));
            assert!(res_err.is_kind(error::Kind::Validation, true));
        }
    }

    async fn get_test_handle_req_incorrect_data_test_cases(
    ) -> Vec<(Attestation, Attestation, String)> {
        vec![
            get_test_handle_req_incorrect_data_single_test_case(DATA_KEY_RELEASE_ID).await,
            get_test_handle_req_incorrect_data_single_test_case(DATA_KEY_BUILD_ID).await,
            get_test_handle_req_incorrect_data_single_test_case(DATA_KEY_SUBNET_ID).await,
            get_test_handle_req_incorrect_data_single_test_case(DATA_KEY_INSTANCE_ID).await,
            get_test_handle_req_incorrect_data_single_test_case(DATA_KEY_UNIX_TIME).await,
        ]
    }

    async fn get_test_handle_req_incorrect_data_single_test_case(
        cache_key: &str,
    ) -> (Attestation, Attestation, String) {
        let mut cache_attestation =
            Attestation::new(lit_attestation::AttestationType::AmdSevSnp, vec![1, 2, 3])
                .await
                .expect("failed to construct Attestation");
        cache_attestation.insert_data(cache_key, "blah".as_bytes().to_vec());

        let mut request_attestation = cache_attestation.clone();
        request_attestation.insert_data(cache_key, "not blah".as_bytes().to_vec());

        (cache_attestation, request_attestation, Uuid::new_v4().to_string())
    }

    #[serial]
    #[tokio::test]
    async fn test_handle_req_incorrect_typ() {
        // Seed cache with attestation intent.
        setup_test_env();
        let cfg = Arc::new(<LitConfig as LitAttestationServiceConfig>::must_new());
        let session_id = Uuid::new_v4().simple().to_string();
        let cache_attestation =
            Attestation::new(lit_attestation::AttestationType::AdminSigned, vec![1, 2, 3])
                .await
                .expect("failed to construct Attestation");
        CACHE.insert(cache_key_session(&session_id), Arc::new(cache_attestation)).await;
        let request_attestation =
            Attestation::new(lit_attestation::AttestationType::AdminSigned, vec![1, 2, 3])
                .await
                .expect("failed to construct Attestation");

        // Init
        let req_body = AttestationReq { attestation: request_attestation, session_id };

        // Handle request
        let mut ctx = Context::new();
        ctx.insert(CTX_KEY_SERVICE_CTX, ServiceContext::from_lit_config(cfg.clone()).unwrap());

        let req = Request::try_from(&req_body, Arc::new(ctx)).expect("failed to extract Request");
        let res = handle_req(req).await;

        // Assertions
        assert!(res.is_err());

        let res_err = res.unwrap_err();

        println!("Res error: {res_err:?}");

        assert!(res_err.is_code(error::EC::AttestationServiceUnsupportedType, true));
        assert!(res_err.is_kind(error::Kind::Validation, true));
    }

    #[serial]
    #[tokio::test]
    async fn test_handle_req_success() {
        // Seed cache with attestation intent.
        setup_test_env();
        let cfg = Arc::new(<LitConfig as LitAttestationServiceConfig>::must_new());
        let session_id = Uuid::new_v4().simple().to_string();
        let cache_attestation =
            Attestation::new(lit_attestation::AttestationType::AmdSevSnp, vec![1, 2, 3])
                .await
                .expect("failed to construct Attestation");
        CACHE.insert(cache_key_session(&session_id), Arc::new(cache_attestation)).await;
        let request_attestation =
            Attestation::new(lit_attestation::AttestationType::AmdSevSnp, vec![1, 2, 3])
                .await
                .expect("failed to construct Attestation");

        // Init
        let req_body = AttestationReq { attestation: request_attestation, session_id };

        // Handle request
        let mut ctx = Context::new();
        ctx.insert(CTX_KEY_SERVICE_CTX, ServiceContext::from_lit_config(cfg.clone()).unwrap());

        let req = Request::try_from(&req_body, Arc::new(ctx)).expect("failed to extract Request");
        let res = handle_req(req).await;

        // Assertions
        if let Err(e) = res {
            panic!("Unexpected error: {e:?}");
        }

        assert!(res.is_ok());

        let handle_resp: AttestationResp =
            res.unwrap().deserialize().await.expect("failed to deserialize");
        let attestation = handle_resp.attestation;

        assert_eq!(attestation.signatures().len(), 1);
        //assert!(attestation.report_raw().is_ok());
        //assert_ne!(attestation.report_raw().unwrap().len(), 0);
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

        if !PathBuf::from("/dev/sev-guest").exists() {
            env::set_var(ENV_ATTESTATION_SERVICE_BYPASS_GENERATE, "1");
        }
    }
}
