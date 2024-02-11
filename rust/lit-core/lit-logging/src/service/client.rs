use std::sync::Arc;

use arc_swap::ArcSwap;
use async_trait::async_trait;
use once_cell::sync::Lazy;

use lit_api_core::client::hyper::default::{Client, UnixClientImpl};

use crate::config::logging_service_socket_path;
use crate::error::Result;
use crate::service::types::{SubmitReq, SubmitResp};

/// Wrapped in an ArcSwap to enable testing later (i.e. we can replace this with our own version).
pub(crate) static LOGGING_SERVICE_CLIENT: Lazy<ArcSwap<UnixClientImpl>> =
    Lazy::new(|| ArcSwap::from(Arc::new(UnixClientImpl::new(logging_service_socket_path()))));

#[async_trait]
pub trait LoggingServiceClient {
    async fn submit(&self, request: &SubmitReq) -> Result<SubmitResp>;
}

#[async_trait]
impl LoggingServiceClient for UnixClientImpl {
    async fn submit(&self, request: &SubmitReq) -> Result<SubmitResp> {
        self.post("/submit", request).await
    }
}
