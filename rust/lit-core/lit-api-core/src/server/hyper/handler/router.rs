use arc_swap::ArcSwap;
use core::result::Result as StdResult;
use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use hyper::http::HeaderValue;
use hyper::{Body, HeaderMap, Method, Request as HyperRequest, Response as HyperResponse};
use log::{as_error, debug};
use uuid::Uuid;

use crate::context::{with_context, Tracing, HEADER_KEY_X_CORRELATION_ID, HEADER_KEY_X_REQUEST_ID};

use crate::error::{validation_err_code, EC};
use crate::server::hyper::handler::error::ApiError;
use crate::server::hyper::handler::types::{Context, Handler, HandlerFn, Request};

#[derive(Hash, PartialEq, Eq)]
pub(crate) struct Route {
    method: Method,
    path: String,
}

impl Route {
    pub(crate) fn new(method: Method, path: String) -> Self {
        Self { method, path }
    }
}

pub struct Router {
    routes: HashMap<Route, Handler>,
    ctx: ArcSwap<Context>,
}

#[allow(dead_code)]
impl Router {
    pub fn new() -> Self {
        Self { routes: HashMap::new(), ctx: ArcSwap::from(Arc::new(Context::new())) }
    }

    pub fn attach<K>(self, key: K, value: Arc<dyn Any + Send + Sync>) -> Self
    where
        K: Into<String>,
    {
        let mut ctx = self.ctx.load_full().as_ref().clone();
        ctx.insert_raw(key.into(), value);
        self.ctx.store(Arc::new(ctx));
        self
    }

    pub fn get<P>(mut self, path: P, handler: HandlerFn) -> Self
    where
        P: Into<String>,
    {
        self.routes.insert(Route::new(Method::GET, path.into()), Arc::new(handler));
        self
    }

    pub fn post<P>(mut self, path: P, handler: HandlerFn) -> Self
    where
        P: Into<String>,
    {
        self.routes.insert(Route::new(Method::POST, path.into()), Arc::new(handler));
        self
    }

    pub fn put<P>(mut self, path: P, handler: HandlerFn) -> Self
    where
        P: Into<String>,
    {
        self.routes.insert(Route::new(Method::PUT, path.into()), Arc::new(handler));
        self
    }

    pub fn patch<P>(mut self, path: P, handler: HandlerFn) -> Self
    where
        P: Into<String>,
    {
        self.routes.insert(Route::new(Method::PATCH, path.into()), Arc::new(handler));
        self
    }

    pub fn delete<P>(mut self, path: P, handler: HandlerFn) -> Self
    where
        P: Into<String>,
    {
        self.routes.insert(Route::new(Method::DELETE, path.into()), Arc::new(handler));
        self
    }

    pub fn find(&self, method: Method, path: String) -> Option<Handler> {
        self.routes.get(&Route::new(method, path)).cloned()
    }

    pub async fn route(
        &self, req: HyperRequest<Body>,
    ) -> StdResult<HyperResponse<Body>, hyper::Error> {
        let req_method = req.method().clone();
        let req_uri = req.uri().to_string();

        trace!("Routing request: {:?}:{:?}", &req_method, &req_uri);

        let tracing = get_tracing_from_request_header(req.headers().clone());

        with_context(tracing, async move {
            match self.find(req_method.clone(), req_uri.clone()) {
                Some(route) => match route(Request::new(req, self.ctx.load_full())).await {
                    Ok(res) => Ok(res.into()),
                    Err(e) => Ok(e.handle()),
                },
                None => Ok(validation_err_code(
                    format!("Route not found: {:?}:{:?}", &req_method, &req_uri),
                    EC::CoreApiRouteNotFound,
                    None,
                )
                .handle()),
            }
        })
        .await
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

// Get Tracing from request headers.
fn get_tracing_from_request_header(headers: HeaderMap<HeaderValue>) -> Tracing {
    if let Some(correlation_id) = extract_correlation_id(headers) {
        Tracing::new(correlation_id)
    } else {
        Tracing::new(Uuid::new_v4().simple().to_string())
    }
}

fn extract_correlation_id(headers: HeaderMap<HeaderValue>) -> Option<String> {
    if let Some(v) =
        headers.get(HEADER_KEY_X_CORRELATION_ID).or_else(|| headers.get(HEADER_KEY_X_REQUEST_ID))
    {
        get_header_string(v)
    } else {
        None
    }
}

fn get_header_string(val: &HeaderValue) -> Option<String> {
    match val.to_str() {
        Ok(val) => Some(val.into()),
        Err(e) => {
            debug!(error = as_error!(e); "failed to parse header as string");

            None
        }
    }
}
