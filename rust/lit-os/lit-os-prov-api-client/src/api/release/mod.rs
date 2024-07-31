use async_trait::async_trait;

use lit_api_core::client::reqwest::handler::handle_post;
use lit_os_prov_core::release::create::types::{CreateReleaseRequest, CreateReleaseResponse};
use lit_os_prov_core::release::init::types::{InitReleaseRequest, InitReleaseResponse};
use lit_os_prov_core::release::issue::types::{IssueReleaseRequest, IssueReleaseResponse};
use lit_os_prov_core::release::query::types::{QueryReleaseRequest, QueryReleaseResponse};

use crate::client::ProvApiClient;
use crate::error::Result;

#[async_trait]
pub trait ProvApiClientRelease {
    async fn create_release(&self, req: &CreateReleaseRequest) -> Result<CreateReleaseResponse>;
    async fn issue_release(&self, req: &IssueReleaseRequest) -> Result<IssueReleaseResponse>;
    async fn init_release(&self, req: &InitReleaseRequest) -> Result<InitReleaseResponse>;
    async fn query_releases(&self, req: &QueryReleaseRequest) -> Result<QueryReleaseResponse>;
}

#[async_trait]
impl ProvApiClientRelease for ProvApiClient {
    async fn create_release(&self, req: &CreateReleaseRequest) -> Result<CreateReleaseResponse> {
        handle_post(self.post("release"), req).await
    }

    async fn issue_release(&self, req: &IssueReleaseRequest) -> Result<IssueReleaseResponse> {
        handle_post(self.post("release/issue"), req).await
    }

    async fn init_release(&self, req: &InitReleaseRequest) -> Result<InitReleaseResponse> {
        handle_post(self.post("release/init"), req).await
    }

    async fn query_releases(&self, req: &QueryReleaseRequest) -> Result<QueryReleaseResponse> {
        handle_post(self.post("release/query"), req).await
    }
}
