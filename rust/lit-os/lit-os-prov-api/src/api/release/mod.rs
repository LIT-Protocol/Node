use std::sync::Arc;

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::{Route, State};
use tracing::trace;

use lit_api_core::context::{with_context, Tracing};
use lit_api_core::error::handler::ApiError;
use lit_attestation::request::AttestedRequest;
use lit_attestation::verification::Policy;
use lit_blockchain::contracts::release::ReleaseType;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::ReloadableLitConfig;
use lit_os_prov_core::error::EC;
use lit_os_prov_core::release::common::load::load_release;
use lit_os_prov_core::release::create::types::{CreateReleaseRequest, CreateReleaseResponse};
use lit_os_prov_core::release::init::types::{InitReleaseRequest, InitReleaseResponse};
use lit_os_prov_core::release::issue::types::{IssueReleaseRequest, IssueReleaseResponse};
use lit_os_prov_core::release::query::types::{QueryReleaseRequest, QueryReleaseResponse};

use crate::error::attestation_err_code;
use crate::release::create::create_release;
use crate::release::init::init_release;
use crate::release::issue::issue_release;
use crate::release::query::query_releases;

#[post("/", format = "json", data = "<request>")]
pub(crate) async fn ep_create(
    cfg: &State<ReloadableLitConfig>, tracing: Tracing, resolver: &State<Arc<ContractResolver>>,
    request: Json<CreateReleaseRequest>,
) -> status::Custom<Value> {
    trace!("ep_create");
    let cfg = cfg.load_full();
    with_context(tracing, async move {
        if let Err(e) = request.body().verify() {
            return e.handle();
        }

        if let Err(e) = request.verify(&cfg, Some(resolver), Some(Policy::Admin)).await {
            return e.handle();
        }

        if let Err(e) = create_release(&cfg, resolver, &request).await {
            return e.handle();
        }

        status::Custom(
            Status::Ok,
            json!(CreateReleaseResponse::new(true, request.body().release_id().clone())),
        )
    })
    .await
}

#[post("/issue", format = "json", data = "<request>")]
pub(crate) async fn ep_issue(
    cfg: &State<ReloadableLitConfig>, tracing: Tracing, resolver: &State<Arc<ContractResolver>>,
    request: Json<IssueReleaseRequest>,
) -> status::Custom<Value> {
    trace!("ep_issue");
    let cfg = cfg.load_full();
    with_context(tracing, async move {
        if let Err(e) = request.body().verify() {
            return e.handle();
        }

        match load_release(&cfg, resolver, request.body().release_id(), true).await {
            Ok(release) => {
                let policy = match release.typ() {
                    ReleaseType::Node => Policy::AdminOrOperator,
                    ReleaseType::Prov => Policy::Admin,
                    ReleaseType::Build => Policy::Admin,
                    ReleaseType::Custom => Policy::Admin,
                };

                match request.verify(&cfg, Some(resolver), Some(policy)).await {
                    Ok(Some(public_key)) => {
                        match issue_release(&cfg, resolver, &request, public_key, &release).await {
                            Ok(artifacts) => status::Custom(
                                Status::Ok,
                                json!(IssueReleaseResponse::new(
                                        true,
                                        request.body().release_id().clone(),
                                        artifacts
                                    )),
                            ),
                            Err(e) => e.handle(),
                        }
                    }
                    Ok(None) => {
                        attestation_err_code(
                            "expected attestation result to have signing public key (required for issue)",
                            EC::ProvFault,
                            None
                        ).handle()
                    }
                    Err(e) => {
                        e.handle()
                    }
                }
            }
            Err(e) => e.handle(),
        }
    })
    .await
}

#[post("/init", format = "json", data = "<request>")]
pub(crate) async fn ep_init(
    cfg: &State<ReloadableLitConfig>, tracing: Tracing, resolver: &State<Arc<ContractResolver>>,
    request: Json<InitReleaseRequest>,
) -> status::Custom<Value> {
    trace!("ep_init");
    let cfg = cfg.load_full();
    with_context(tracing, async move {
        if let Err(e) = request.body().verify() {
            return e.handle();
        }

        match load_release(&cfg, resolver, request.body().release_id(), true).await {
            Ok(release) => {
                if let Err(e) = request.verify(&cfg, Some(resolver), Some(Policy::Init)).await {
                    return e.handle();
                }

                match init_release(&cfg, resolver, &request, &release).await {
                    Ok(passphrase) => status::Custom(
                        Status::Ok,
                        json!(InitReleaseResponse::new(
                            true,
                            request.body().release_id().clone(),
                            passphrase
                        )),
                    ),
                    Err(e) => e.handle(),
                }
            }
            Err(e) => e.handle(),
        }
    })
    .await
}

#[post("/query", format = "json", data = "<request>")]
pub(crate) async fn ep_query(
    cfg: &State<ReloadableLitConfig>, tracing: Tracing, resolver: &State<Arc<ContractResolver>>,
    request: Json<QueryReleaseRequest>,
) -> status::Custom<Value> {
    trace!("ep_query");
    let cfg = cfg.load_full();
    with_context(tracing, async move {
        if let Err(e) = request.verify() {
            return e.handle();
        }

        match query_releases(&cfg, resolver, &request).await {
            Ok(releases) => {
                status::Custom(Status::Ok, json!(QueryReleaseResponse::new(true, releases)))
            }
            Err(e) => e.handle(),
        }
    })
    .await
}

pub(crate) fn routes() -> Vec<Route> {
    routes![ep_create, ep_issue, ep_init, ep_query]
}
