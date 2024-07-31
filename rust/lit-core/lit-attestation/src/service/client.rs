use std::sync::Arc;

use arc_swap::ArcSwap;
use async_trait::async_trait;
use once_cell::sync::Lazy;

use lit_api_core::client::hyper::default::Client;
use lit_api_core::client::hyper::default::UnixClientImpl;

use crate::config::attestation_service_socket_path;
use crate::error::Result;
use crate::service::types::*;

/// Wrapped in an ArcSwap to enable testing later (i.e. we can replace this with our own version).
pub(crate) static ATTESTATION_SERVICE_CLIENT: Lazy<ArcSwap<UnixClientImpl>> =
    Lazy::new(|| ArcSwap::from(Arc::new(UnixClientImpl::new(attestation_service_socket_path()))));

#[async_trait]
pub trait AttestationServiceClient {
    async fn prepare_attestation(
        &self, request: &AttestationIntentReq,
    ) -> Result<AttestationIntentResp>;
    async fn confirm_attestation(&self, request: &AttestationReq) -> Result<AttestationResp>;
    async fn kdf(&self, request: &KdfReq) -> Result<KdfResp>;
}

#[async_trait]
impl AttestationServiceClient for UnixClientImpl {
    async fn prepare_attestation(
        &self, request: &AttestationIntentReq,
    ) -> Result<AttestationIntentResp> {
        self.post("/attestation/intent", request).await
    }

    async fn confirm_attestation(&self, request: &AttestationReq) -> Result<AttestationResp> {
        self.post("/attestation/confirm", request).await
    }

    async fn kdf(&self, request: &KdfReq) -> Result<KdfResp> {
        self.post("/kdf", request).await
    }
}
