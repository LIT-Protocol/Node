use crate::tasks::realtime_metrics::MetricsMessage;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{serde_json::json, Value};
use rocket::State;
use std::sync::Arc;
// use tracing::instrument;

#[get("/web/rt/metrics")]
// #[instrument(name = "GET /web/rt/metric", skip_all, ret)]
pub async fn metrics(
    metrics_tx: &State<Arc<flume::Sender<MetricsMessage>>>,
) -> status::Custom<Value> {
    let (tx, rx) = flume::bounded(1);
    let inquiry = metrics_tx.send(MetricsMessage::Poll(tx));
    if inquiry.is_err() {
        return status::Custom(
            Status::InternalServerError,
            json!({
                "success": "false",
                "error": "Failed to send inquiry to metrics task.",
            }),
        );
    }

    let result = match rx.recv_async().await {
        Ok(result) => result,
        Err(_) => {
            return status::Custom(
                Status::InternalServerError,
                json!({
                    "success": "false",
                    "error": "rx.recv_async().await is_err()",
                }),
            );
        }
    };

    status::Custom(Status::Ok, json!(result))
}
