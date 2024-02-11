use lit_api_core::context::{with_context, Tracing};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Route;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Status {
    pub ok: bool,
}

#[get("/status")]
pub(crate) async fn ep_status(tracing: Tracing) -> Json<Status> {
    with_context(tracing, async move {
        // TODO:
        Json(Status { ok: true })
    })
    .await
}

pub(crate) fn routes() -> Vec<Route> {
    routes![ep_status]
}
