use lit_api_core::context::{
    Tracer, Tracing, HEADER_KEY_X_CORRELATION_ID, HEADER_KEY_X_REQUEST_ID,
};
use tonic::Request;

pub trait FromGrpcRequest<T> {
    fn from_grpc_request(req: T) -> Self;
}

// The resulting tracer adds fields for identifying the connecting Lit node and the request's correlation ID. For example:
// DEBUG lit_actions::runtime  > Deno isolate init with snapshots. { "node":"127.0.0.1:7470", "correlation_id":"368ed56d88caf" }
impl<T> FromGrpcRequest<&Request<T>> for Tracing {
    fn from_grpc_request(req: &Request<T>) -> Self {
        let correlation_id = req
            .metadata()
            .get(HEADER_KEY_X_REQUEST_ID)
            .and_then(|v| v.to_str().ok())
            .or_else(|| {
                req.metadata()
                    .get(HEADER_KEY_X_CORRELATION_ID)
                    .and_then(|v| v.to_str().ok())
            })
            .unwrap_or_default()
            .to_string();

        let mut tracer = Tracing::new(correlation_id.clone());

        if let Some(host) = req.metadata().get("x-host").and_then(|v| v.to_str().ok()) {
            tracer.add_field(
                "node".to_string(),
                serde_json::Value::String(host.to_string()),
            );
        }

        tracer
    }
}
