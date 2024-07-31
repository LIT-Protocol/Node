#[cfg(feature = "server")]
use crate::context::{HeaderInjector, HEADER_KEY_X_CORRELATION_ID, TRACING};
use crate::error::{connect_err_code, http_client_err, timeout_err_code, Result, EC};

use lit_core::error::PublicError;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use {
    opentelemetry::propagation::TextMapPropagator,
    opentelemetry::sdk::propagation::TraceContextPropagator, std::collections::HashMap,
    tracing_opentelemetry::OpenTelemetrySpanExt,
};

#[allow(unused_mut)]
pub async fn handle_post<RQ, RS>(mut builder: RequestBuilder, req: &RQ) -> Result<RS>
where
    RQ: Serialize,
    RS: for<'a> Deserialize<'a>,
{
    #[cfg(feature = "server")]
    {
        // Inject the correlation ID into the header.
        let mut correlation_id: Option<String> = None;
        let _ = TRACING.try_with(|tracing| {
            let _ = correlation_id.insert(tracing.correlation_id().clone());
        });
        if let Some(correlation_id) = correlation_id {
            builder = builder.header(HEADER_KEY_X_CORRELATION_ID, correlation_id);
        }

        // Inject the tracing context into the header.
        // Get the OpenTelemetry `Context` via the current `tracing::Span`.
        let cx = tracing::Span::current().context();

        // Initialize the header injector.
        let mut additional_headers = HashMap::new();
        let mut header_injector = HeaderInjector::from(&mut additional_headers);

        // Inject the current context into the request header
        let propagator = TraceContextPropagator::new();
        propagator.inject_context(&cx, &mut header_injector);

        // Transfer header information from the carrier into the request builder
        for (key, value) in additional_headers {
            builder = builder.header(key, value);
        }
    }

    match builder.json(req).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                let resp: RS = resp.json().await.map_err(|e| {
                    http_client_err(e, Some("Error parsing response body as JSON".into()))
                })?;

                Ok(resp)
            } else {
                let err: PublicError = resp.json().await.map_err(|e| {
                    http_client_err(e, Some("Error parsing response body as JSON".into()))
                })?;

                Err(err.into())
            }
        }
        Err(e) => {
            if e.is_timeout() {
                Err(timeout_err_code(
                    e,
                    EC::CoreApiHttpClientTimeout,
                    Some("Request timed out".into()),
                ))
            } else if e.is_connect() {
                Err(connect_err_code(
                    e,
                    EC::CoreApiHttpClientConnectFailed,
                    Some("Error connecting to server".into()),
                ))
            } else {
                Err(http_client_err(e, Some("Error sending request".into())))
            }
        }
    }
}
