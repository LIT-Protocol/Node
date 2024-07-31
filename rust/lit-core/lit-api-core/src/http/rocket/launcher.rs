use std::collections::HashMap;
use std::fmt;
use std::result::Result as StdResult;

use futures::future::{join_all, BoxFuture};
use rocket::catcher::Handler;
use rocket::fairing::Fairing;
use rocket::http::uri::Origin;
use rocket::http::Status;
use rocket::response::{status, Redirect, Responder};
use rocket::serde::json::Value;
use rocket::{catcher, Build, Catcher, Error as RocketError, Ignite, Request, Rocket, Route};

use tokio::sync::mpsc;
use tokio::task_local;

use crate::config::{
    LitApiConfig, CFG_KEY_ADDRESS, CFG_KEY_IDENT, CFG_KEY_KEEP_ALIVE, CFG_KEY_PORT,
    CFG_KEY_REDIRECT_TO_HTTPS, CFG_KEY_REUSE_ADDRESS, CFG_KEY_REUSE_PORT, CFG_KEY_WORKERS,
};
use crate::error::handler::ApiError;
use crate::error::Result;
use crate::error::{validation_err_code, EC};
use crate::http::rocket::event::{Event, EventDataKey, EventManager};
use crate::http::tls::certs::{ep_certs, tls_config_load, CertManager};
use lit_core::config::{LitConfig, ReloadableLitConfig};

static ROCKET_CFG_INT_KEYS: [&str; 3] = [CFG_KEY_PORT, CFG_KEY_WORKERS, CFG_KEY_KEEP_ALIVE];
static ROCKET_CFG_STR_KEYS: [&str; 2] = [CFG_KEY_ADDRESS, CFG_KEY_IDENT];
static ROCKET_CFG_BOOL_KEYS: [&str; 2] = [CFG_KEY_REUSE_ADDRESS, CFG_KEY_REUSE_PORT];

static HTTPS_PORT: u16 = 443;

task_local! {
    pub static CONFIG: ReloadableLitConfig;
}

pub struct Launcher {
    cfg: ReloadableLitConfig,
    rocket: Option<Rocket<Build>>,
    ignited: Vec<Rocket<Ignite>>,
    cert_manager: CertManager,
    event_manager: EventManager,
}

impl Launcher {
    pub fn try_new(
        cfg: ReloadableLitConfig, grpc_cert_channel: Option<mpsc::Sender<bool>>,
    ) -> Result<Self> {
        let cert_manager = CertManager::new(cfg.clone());
        let is_https_enabled = cfg.load().https_enabled();
        if is_https_enabled {
            cert_manager.init()?;
        }

        let rocket = build_rocket(cfg.load().as_ref(), is_https_enabled, false)?;

        Ok(Self {
            cfg,
            rocket: Some(rocket),
            ignited: Vec::new(),
            cert_manager,
            event_manager: EventManager::new(grpc_cert_channel),
        })
    }

    // Rocket
    pub fn mount<'a, B, R>(mut self, base: B, routes: R) -> Self
    where
        B: TryInto<Origin<'a>> + Clone + fmt::Display,
        B::Error: fmt::Display,
        R: Into<Vec<Route>>,
    {
        let mut rocket = self.rocket.take().unwrap();
        rocket = rocket.mount(base, routes);
        self.rocket = Some(rocket);

        self
    }

    pub fn attach<F: Fairing>(mut self, fairing: F) -> Self {
        let mut rocket = self.rocket.take().unwrap();
        rocket = rocket.attach(fairing);
        self.rocket = Some(rocket);

        self
    }

    pub fn manage<T>(mut self, state: T) -> Self
    where
        T: Send + Sync + 'static,
    {
        let mut rocket = self.rocket.take().unwrap();
        rocket = rocket.manage(state);
        self.rocket = Some(rocket);

        self
    }

    pub fn register<'a, B, C>(mut self, base: B, catchers: C) -> Self
    where
        B: TryInto<Origin<'a>> + Clone + fmt::Display,
        B::Error: fmt::Display,
        C: Into<Vec<Catcher>>,
    {
        let mut rocket = self.rocket.take().unwrap();
        rocket = rocket.register(base, catchers);
        self.rocket = Some(rocket);

        self
    }

    pub async fn ignite(&mut self) -> StdResult<Shutdown, RocketError> {
        let mut http_rocket: Option<Rocket<Build>> = None;
        let mut rocket = self.rocket.take().unwrap();

        {
            let cfg = self.cfg.load();
            if cfg.https_enabled() {
                if cfg.http_enabled() {
                    http_rocket = Some(build_rocket(&cfg, false, true).unwrap());
                }

                if cfg.tls_auto() {
                    rocket = rocket.mount("/", ep_certs());

                    if cfg.http_enabled() {
                        let mut hr = http_rocket.take().unwrap();
                        hr = hr.mount("/", ep_certs());
                        http_rocket = Some(hr);
                    }

                    self.cert_manager.start(self.event_manager.clone());
                }
            }
        }

        self.ignited.clear();

        CONFIG
            .scope(self.cfg.clone(), async move {
                self.ignited.push(rocket.ignite().await?);

                if http_rocket.is_some() {
                    let http_rocket = http_rocket.take().unwrap();

                    self.ignited.push(http_rocket.ignite().await?);
                }

                Ok(Shutdown::from_ignited(&self.ignited))
            })
            .await
    }

    pub async fn launch(&mut self) -> StdResult<(), RocketError> {
        if self.ignited.is_empty() {
            panic!("ignite must be called prior to launch");
        }

        let mut futures = Vec::new();
        while !self.ignited.is_empty() {
            futures.push(self.ignited.remove(0).launch());
        }

        join_all(futures).await;

        Ok(())
    }

    // Events
    pub fn on_event(
        mut self, event: Event,
        action: impl Send
            + Sync
            + Fn(&HashMap<EventDataKey, Vec<u8>>) -> BoxFuture<'static, Result<()>>
            + 'static,
    ) -> Self {
        self.event_manager.on_event(event, action);
        self
    }
}

#[derive(Clone)]
pub struct Shutdown {
    handles: Vec<rocket::Shutdown>,
}

impl Shutdown {
    pub(crate) fn from_ignited(ignited: &[Rocket<Ignite>]) -> Self {
        let mut handles = Vec::new();
        for ignited in ignited.iter() {
            handles.push(ignited.shutdown());
        }

        Self { handles }
    }

    pub fn shutdown(mut self) {
        while !self.handles.is_empty() {
            let handle = self.handles.remove(0);
            handle.notify();
        }
    }
}

fn build_rocket(cfg: &LitConfig, is_https: bool, is_aux_http: bool) -> Result<Rocket<Build>> {
    let mut figment = rocket::Config::figment();

    for key in ROCKET_CFG_INT_KEYS.iter() {
        if let Ok(val) = cfg.get_http_section_int(key, is_https) {
            figment = figment.merge((key, val));
        }
    }
    for key in ROCKET_CFG_STR_KEYS.iter() {
        if let Ok(val) = cfg.get_http_section_string(key, is_https) {
            figment = figment.merge((key, val));
        }
    }
    for key in ROCKET_CFG_BOOL_KEYS.iter() {
        if let Ok(val) = cfg.get_http_section_bool(key, is_https) {
            figment = figment.merge((key, val));
        }
    }

    if let Ok(val) = cfg.get_http_section_string("profile", is_https) {
        figment = figment.select(val);
    } else {
        figment = figment.select(cfg.env().to_string().to_lowercase());
    }

    if is_https {
        figment = figment.merge(("tls", tls_config_load(cfg)?));
    }

    let mut r = rocket::custom(figment);

    if !is_https
        && is_aux_http
        && matches!(cfg.get_http_section_bool(CFG_KEY_REDIRECT_TO_HTTPS, is_https), Ok(true))
    {
        r = r.register("/", vec![Catcher::new(404, RedirectToHttpsHandler::new(cfg))]);
    }

    r = r.register("/", catchers![guard_failure_catcher]);

    Ok(r)
}

#[derive(Clone)]
struct RedirectToHttpsHandler {
    https_host: Option<String>,
    https_port: Option<u16>,
}

impl RedirectToHttpsHandler {
    pub fn new(cfg: &LitConfig) -> Self {
        Self {
            https_host: cfg.api_domain().ok(),
            https_port: cfg.api_https_port_external().ok().map(|v| v as u16),
        }
    }
}

#[async_trait]
impl Handler for RedirectToHttpsHandler {
    async fn handle<'r>(&self, _status: Status, req: &'r Request<'_>) -> catcher::Result<'r> {
        let host = match self.https_host.as_ref() {
            Some(v) => v.clone(),
            None => req.host().map(|v| v.to_string()).unwrap_or_else(|| "127.0.0.1".to_string()),
        };
        let https_port = self.https_port.unwrap_or_else(|| HTTPS_PORT);
        let full_host =
            if https_port == HTTPS_PORT { host } else { format!("{host}:{https_port}") };

        let uri = format!("{}", req.uri().clone().into_normalized());
        let uri = uri.strip_prefix('/').unwrap_or(&uri);
        let url = format!("https://{full_host}/{uri}");
        let res = Redirect::to(url);
        res.respond_to(req)
    }
}

/// 418 (ImATeapot) is used here as a reserved status code to ensure we do not intercept other things.
#[catch(418)]
fn guard_failure_catcher(_status: Status, req: &Request<'_>) -> status::Custom<Value> {
    req.local_cache(|| {
        validation_err_code(
            "Bad Request - Unexpected guard failure (no upstream error in context)",
            EC::CoreApiUnexpected,
            None,
        )
    })
    .to_owned()
    .handle()
}
