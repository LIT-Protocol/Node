use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{json, Value};
use rocket::{Route, State};

use lit_api_core::context::{with_context, Tracing};
use lit_api_core::error::ApiError;
use lit_attestation::{Attestation, TryGenerate};
use lit_blockchain::config::LitBlockchainConfig;
use lit_core::config::ReloadableLitConfig;

#[get("/attest/<noonce>")]
pub(crate) async fn ep_attest(
    cfg: &State<ReloadableLitConfig>, tracing: Tracing, noonce: &str,
) -> status::Custom<Value> {
    let cfg = cfg.load_full();
    with_context(tracing, async move {
        match Attestation::try_generate(
            cfg.as_ref(),
            (Some(noonce.as_bytes().to_vec()), None, cfg.blockchain_wallet_private_key(None).ok()),
        )
        .await
        {
            Ok(res) => status::Custom(Status::Ok, json!(res)),
            Err(e) => e.handle(),
        }
    })
    .await
}

pub(crate) fn routes() -> Vec<Route> {
    routes![ep_attest]
}
