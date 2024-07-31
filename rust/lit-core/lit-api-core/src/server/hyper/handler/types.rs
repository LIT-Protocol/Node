use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use futures::future::BoxFuture;
use hyper::body::Bytes;
use hyper::{body, Body, Request as HyperRequest, Response as HyperResponse};
use serde::{Deserialize, Serialize};

use crate::error::{conversion_err, Result};

pub(crate) type Handler =
    Arc<dyn Send + Sync + Fn(Request) -> BoxFuture<'static, Result<Response>>>;
pub(crate) type HandlerFn = fn(Request) -> BoxFuture<'static, Result<Response>>;

#[derive(Debug)]
pub struct Request {
    req: HyperRequest<Body>,
    ctx: Arc<Context>,
}

impl Request {
    pub fn new(req: HyperRequest<Body>, ctx: Arc<Context>) -> Self {
        Self { req, ctx }
    }

    #[allow(dead_code)]
    pub fn try_from(v: impl Serialize, ctx: Arc<Context>) -> Result<Self> {
        let json = serde_json::to_vec(&v).map_err(|e| conversion_err(e, None))?;

        let bytes = Bytes::from(json);

        Ok(Self::new(HyperRequest::new(bytes.into()), ctx))
    }

    pub fn ctx(&self) -> Arc<Context> {
        self.ctx.clone()
    }

    pub async fn deserialize<T: for<'a> Deserialize<'a>>(self) -> Result<T> {
        let body_bytes = body::to_bytes(self.req.into_body())
            .await
            .map_err(|e| conversion_err(e, Some("Unable to obtain request body bytes".into())))?;

        serde_json::from_slice(&body_bytes).map_err(|e| {
            conversion_err(e, Some("Unable to deserialize request body as JSON".into()))
        })
    }
}

#[derive(Debug)]
pub struct Response {
    res: HyperResponse<Body>,
}

impl Response {
    pub fn new(res: HyperResponse<Body>) -> Self {
        Self { res }
    }

    pub fn try_from(v: impl Serialize) -> Result<Self> {
        let json = serde_json::to_vec(&v).map_err(|e| conversion_err(e, None))?;

        let bytes = Bytes::from(json);

        Ok(Self::new(HyperResponse::new(bytes.into())))
    }

    #[allow(dead_code)]
    pub async fn deserialize<T: for<'a> Deserialize<'a>>(self) -> Result<T> {
        let body_bytes = body::to_bytes(self.res.into_body())
            .await
            .map_err(|e| conversion_err(e, Some("Unable to obtain response body bytes".into())))?;

        serde_json::from_slice(&body_bytes).map_err(|e| {
            conversion_err(e, Some("Unable to deserialize response body as JSON".into()))
        })
    }
}

impl From<Response> for HyperResponse<Body> {
    fn from(val: Response) -> Self {
        val.res
    }
}

#[async_trait]
pub trait RequestHandler: Send + Sync {
    async fn handle_req(&self, req: Request) -> Result<Response>;
}

#[derive(Clone, Debug)]
pub struct Context {
    values: HashMap<String, Arc<dyn Any + Sync + Send>>,
}

impl Context {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    pub fn insert<K>(&mut self, key: K, value: impl Any + Send + Sync)
    where
        K: Into<String>,
    {
        self.insert_raw(key, Arc::new(value))
    }

    pub fn insert_raw<K>(&mut self, key: K, value: Arc<dyn Any + Send + Sync>)
    where
        K: Into<String>,
    {
        self.values.insert(key.into(), value);
    }

    pub fn get<K>(&self, key: K) -> Option<&Arc<dyn Any + Send + Sync>>
    where
        K: AsRef<str>,
    {
        self.values.get(key.as_ref())
    }

    pub fn get_bin<K>(&self, key: K) -> Option<&Vec<u8>>
    where
        K: AsRef<str>,
    {
        match self.values.get(key.as_ref()) {
            Some(val) => val.downcast_ref(),
            _ => None,
        }
    }

    pub fn get_bool<K>(&self, key: K) -> Option<&bool>
    where
        K: AsRef<str>,
    {
        match self.values.get(key.as_ref()) {
            Some(val) => val.downcast_ref(),
            _ => None,
        }
    }

    pub fn get_string<K>(&self, key: K) -> Option<&String>
    where
        K: AsRef<str>,
    {
        match self.values.get(key.as_ref()) {
            Some(val) => val.downcast_ref(),
            _ => None,
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
