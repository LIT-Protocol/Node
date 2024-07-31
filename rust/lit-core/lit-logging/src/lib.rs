use regex::Regex;
use std::backtrace::Backtrace;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::{fmt, io, panic};

use env_logger::fmt::{Color, Formatter, Style, StyledValue};
use env_logger::Env;
use log::{as_serde, Level};
#[cfg(feature = "tracing-support")]
use opentelemetry_otlp::WithExportConfig;
use serde_json::{Map, Value};
#[cfg(feature = "tracing-support")]
use tracing_subscriber::prelude::*;
#[cfg(feature = "tracing-support")]
use tracing_subscriber::Registry;
#[cfg(feature = "tracing-support")]
use tracing_subscriber::{EnvFilter, Layer};

use lit_core::config::LitConfig;
use lit_core::logging::plugin::Plugin;
#[cfg(feature = "tracing-support")]
use lit_core::logging::types::TracingLogRecord;
use lit_core::logging::types::{DefaultLogRecord, LogRecord};
use lit_core::utils::backtrace::{backtrace_to_vec, extract_panic_msg};

use crate::config::LitLoggingConfig;
use crate::error::{Error, Kind, Result, PKG_NAME};

pub mod config;
pub mod error;
pub mod plugin;
#[cfg(feature = "service")]
pub mod service;

pub struct Builder<'b> {
    cfg: &'b LitConfig,
    pkg: String,
    plugins: Vec<Box<dyn Plugin>>,
    fields: Option<Map<String, Value>>,
}

impl<'b> Builder<'b> {
    fn new(cfg: &'b LitConfig, pkg: &'static str) -> Builder<'b> {
        Self { cfg, pkg: pkg.to_string(), plugins: Vec::new(), fields: None }
    }

    pub fn plugin(mut self, plugin: impl Plugin + 'static) -> Builder<'b> {
        self.plugins.push(Box::new(plugin));
        self
    }

    /// Add some fields to the structured logs.
    pub fn add_field<K>(mut self, key: K, value: Value) -> Builder<'b>
    where
        K: Into<String>,
    {
        let mut fields = match self.fields.take() {
            Some(v) => v,
            None => Map::new(),
        };

        fields.insert(key.into(), value);
        self.fields = Some(fields);
        self
    }

    pub fn init(mut self) -> Result<()> {
        // Init plugins
        for plugin in self.plugins.iter_mut() {
            plugin.init(self.cfg)?;
        }

        let pkg = self.pkg;
        let plugins = Arc::new(self.plugins);
        let fields = Arc::new(self.fields);

        let cfg_log_level = self.cfg.logging_level()?;
        let is_log_timestamp = self.cfg.logging_timestamp();
        let is_prod = self.cfg.is_prod();

        // Init logger
        {
            let pkg = pkg.clone();
            let plugins = plugins.clone();
            let fields = fields.clone();

            let plugins = plugins.clone();
            env_logger::Builder::from_env(Env::default().default_filter_or(cfg_log_level.clone()))
                .format(move |buf, record| {
                    let pkg = pkg.clone();
                    let plugins = plugins.clone();
                    let fields = fields.clone();

                    let mut record = DefaultLogRecord::try_new(record, fields, plugins.clone())
                        .map_err(|e| e.into_io())?;

                    handle(buf, &mut record, pkg.as_ref(), plugins, Some(is_prod), is_log_timestamp)
                })
                .init();
        }

        // Init tracing subscriber
        #[cfg(feature = "tracing-support")]
        {
            use error::{config_err, unexpected_err};
            use std::str::FromStr;

            let env_filter = EnvFilter::try_from_default_env()
                .or_else(|_e| EnvFilter::from_str(cfg_log_level.as_str()))
                .map_err(|e| {
                    config_err(e, Some("failed to init tracing log level from env/config".into()))
                })?;

            let subscriber = Registry::default()
                .with(TracingLayer::new(pkg.clone(), plugins.clone(), fields))
                .with(env_filter);

            // Optionally enable Jaeger tracer
            if self.cfg.logging_jaeger() {
                let tracer = opentelemetry_otlp::new_pipeline()
                    .tracing()
                    .with_exporter(
                        opentelemetry_otlp::new_exporter()
                            .tonic()
                            .with_endpoint("http://localhost:4317"),
                    )
                    .with_trace_config(opentelemetry_sdk::trace::config().with_resource(
                        opentelemetry_sdk::Resource::new(vec![opentelemetry::KeyValue::new(
                            "service.name",
                            pkg.clone(),
                        )]),
                    ))
                    .install_batch(opentelemetry_sdk::runtime::Tokio)
                    .map_err(|e| unexpected_err(e, Some("failed to init Jaeger tracer".into())))?;

                tracing::subscriber::set_global_default(
                    subscriber.with(tracing_opentelemetry::layer().with_tracer(tracer)),
                )
            } else {
                tracing::subscriber::set_global_default(subscriber)
            }
            .map_err(|e| unexpected_err(e, Some("unable to set global subscriber".into())))?;
        }

        // Panic hook
        {
            let plugins = plugins.clone();
            panic::set_hook(Box::new(move |e| {
                let msg = extract_panic_msg(e);
                let backtrace = Backtrace::force_capture();
                let backtrace_vec = backtrace_to_vec(&backtrace);
                // let backtrace = format!("{backtrace}");

                // goes through the backtrace and removes lines with "/rustc/" and looks for the lines that have .rs files in them, to make finding the root cause easier.
                let mut filtered: Vec<String> = Vec::new();
                let mut prev_s: Option<String> = None;
                for s in &backtrace_vec {
                    if !s.contains("/rustc/")
                        && !s.contains("/.cargo/")
                        && !s.contains("/lit-core/lit-logging/")
                        && s.contains(".rs")
                    {
                        if let Some(prev) = prev_s {
                            filtered.push(prev);
                        }
                        filtered.push(s.clone());
                    }
                    prev_s = Some(s.clone());
                }
                let filtered = filtered.join("\n");

                let source: Option<String> = None;
                let err = Error::new(
                    Some(Kind::Unexpected),
                    PKG_NAME,
                    Some(msg.clone()),
                    None,
                    source,
                    None,
                );

                eprintln!("Unexpectedly panicked!: {}\nFull error: {}\nFiltered backtrace: \n{}\nFull backtrace:{}", msg, err, filtered, as_serde!(backtrace_vec));

                // Flush logs (wait until we can safely shutdown).
                for plugin in plugins.iter() {
                    let _ = plugin.flush().map_err(|e| {
                        // Do _NOT_ call anything from log*.
                        eprintln!("failed to call flush on plugin: {e:?}");
                    });
                }
            }));
        }

        Ok(())
    }
}

pub fn builder<'b>(cfg: &'b LitConfig, pkg: &'static str) -> Builder<'b> {
    Builder::new(cfg, pkg)
}

fn filter_sensitive_info(msg: &str) -> String {
    let re_auth_sig = Regex::new(r#"auth_sig: (Single|Multiple|Some)(?s).*?\}\)"#).unwrap();
    let filtered_sig = re_auth_sig.replace_all(&msg, r#"auth_sig: "***filtered***""#);

    let re_access_token = Regex::new(r#"access_token:(?s).*?\}\]"#).unwrap();
    let filtered_access_token =
        re_access_token.replace_all(&filtered_sig, r#"access_token: "***filtered***" }]"#);

    let re_js_params = Regex::new(r#"js_params: (Some)(?s).*?\}\)"#).unwrap();
    re_js_params.replace_all(&filtered_access_token, r#"js_params: "***filtered***""#).to_string()
}

#[allow(unused_variables)]
fn handle(
    buf: &mut Formatter, record: &mut dyn LogRecord, pkg: &str, plugins: Arc<Vec<Box<dyn Plugin>>>,
    is_prod: Option<bool>, is_log_timestamp: bool,
) -> io::Result<()> {
    // Handle other logging (i.e. JSON to log service).
    for plugin in plugins.iter() {
        plugin.handle(pkg, record).map_err(|e| e.into_io())?;
    }

    // Standard logging
    let target = record.target();
    let max_width = max_target_width(target);

    let mut style = buf.style();
    let level = colored_level(&mut style, record.level());

    let mut style = buf.style();
    let target = style.set_bold(true).value(Padded { value: target, width: max_width });

    if let Some(true) = is_prod {
        let display_args_str = format!("{}", record.display_args());
        write!(buf, " {} {} > {}", level, target, filter_sensitive_info(&display_args_str),)?;
    } else {
        #[cfg(feature = "chrono")]
        {
            if is_log_timestamp {
                let now = chrono::Utc::now();
                write!(buf, " {} {} {} > {}", now, level, target, record.display_args(),)?;
            } else {
                write!(buf, " {} {} > {}", level, target, record.display_args(),)?;
            }
        }

        #[cfg(not(feature = "chrono"))]
        {
            write!(buf, " {} {} > {}", level, target, record.display_args(),)?;
        }
    }

    if !record.fields().is_empty() {
        let mut fields_style = buf.style();
        fields_style.set_dimmed(true);

        write!(buf, "{}", fields_style.value(fields_to_json(record.fields())))?;
    }

    writeln!(buf)
}

pub struct Padded<T> {
    pub value: T,
    pub width: usize,
}

impl<T: fmt::Display> fmt::Display for Padded<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{: <width$}", self.value, width = self.width)
    }
}

static MAX_MODULE_WIDTH: AtomicUsize = AtomicUsize::new(0);

fn max_target_width(target: &str) -> usize {
    let max_width = MAX_MODULE_WIDTH.load(Ordering::Relaxed);
    if max_width < target.len() {
        MAX_MODULE_WIDTH.store(target.len(), Ordering::Relaxed);
        target.len()
    } else {
        max_width
    }
}

pub fn colored_level(style: &mut Style, level: Level) -> StyledValue<&'static str> {
    match level {
        Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
        Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
        Level::Info => style.set_color(Color::Green).value("INFO "),
        Level::Warn => style.set_color(Color::Yellow).value("WARN "),
        Level::Error => style.set_color(Color::Red).value("ERROR"),
    }
}

pub fn fields_to_json(fields: &Map<String, Value>) -> String {
    let mut s = String::new();

    if fields.is_empty() {
        return s;
    }

    s.push_str(" {");

    let len = fields.len();
    for (seq, (key, value)) in fields.iter().enumerate() {
        s.push_str(format!(" \"{key}\":{value}").as_str());
        if seq < (len - 1) {
            s.push(',');
        }
    }

    // Add space at the end as every item has a " " at the start.
    s.push_str(" }");

    s
}

#[cfg(feature = "tracing-support")]
pub struct TracingLayer {
    pkg: String,
    plugins: Arc<Vec<Box<dyn Plugin>>>,
    fields: Arc<Option<Map<String, Value>>>,
    writer: env_logger::fmt::writer::Writer,
}

#[cfg(feature = "tracing-support")]
impl TracingLayer {
    fn new(
        pkg: String, plugins: Arc<Vec<Box<dyn Plugin>>>, fields: Arc<Option<Map<String, Value>>>,
    ) -> Self {
        Self { pkg, plugins, fields, writer: env_logger::fmt::writer::Builder::default().build() }
    }
}

#[cfg(feature = "tracing-support")]
impl<S> Layer<S> for TracingLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self, event: &tracing::Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let plugins = self.plugins.clone();

        let mut record =
            match TracingLogRecord::try_new(event, self.fields.clone(), plugins.clone()) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("TracingLayer: failed to construct TracingLogRecord - {:?}", e);
                    return;
                }
            };

        let mut formatter = Formatter::new(&self.writer);

        if let Err(e) = handle(&mut formatter, &mut record, self.pkg.as_str(), plugins, None, false)
        {
            eprintln!("TracingLayer: failed to handle log - {:?}", e);
            return;
        }

        if let Err(e) = formatter.print(&self.writer) {
            eprintln!("TracingLayer: failed to call formatter.print() - {:?}", e);
        }
        formatter.clear();
    }
}

#[cfg(test)]
mod tests {
    use crate::filter_sensitive_info;

    #[test]
    fn test_sig_and_token_filtering() {
        let auth_sig = r#"request: Json(JsonPKPSigningRequest { to_sign: [226, 112, 61, 209, 25, 152, 79, 112, 226, 74, 222, 91, 10, 13, 107, 155, 130, 154, 133, 94, 124, 30, 103, 110, 118, 106, 12, 43, 141, 43, 167, 192], pubkey: "0x04829459732377144667592900904dead0e44717664c1df61c9dd4a5b63e4161c95833ed7589c1b90676412109d32bdd2e3491dfe7c303c145625824d3a4e2bfd1", auth_sig: Single(JsonAuthSig { sig: "ed0469f4305d9cbe5739aaec1fb7bb13622d805830394828cc8616377ffca25e3025248167c3a2576328faa3b417ffd9d9cffebf4a648d9985059d6d5c0aa607", derived_via: "litSessionSignViaNacl", signed_message: "{\"sessionKey\":\"4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f\",\"resourceAbilityRequests\":[{\"resource\":{\"resource\":\"*\",\"resourcePrefix\":\"lit-litaction\"},\"ability\":\"pkp-signing\"}],\"capabilities\":[{\"sig\":\"0xf92355a01d003c018b066c647b3c46890541092b841499d03c19cbac7718abd3478b5a66a967a4e09c0cbc2f7704c7431040c0c381785a53561bfc82c8c1e0621b\",\"derivedVia\":\"web3.eth.personal.sign via Lit PKP\",\"signedMessage\":\"localhost:3000 wants you to sign in with your Ethereum account:\\n0xc17BD003ca0759f5914D2A9079d89b19B0CcD615\\n\\nLit Protocol PKP session signature I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-litaction://*'.\\n\\nURI: lit:session:4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f\\nVersion: 1\\nChain ID: 1\\nNonce: Ei85x7hogxywfUTyW\\nIssued At: 2023-10-21T19:22:48.623Z\\nExpiration Time: 2023-10-22T19:22:48.492Z\\nResources:\\n- urn:recap:eyJhdHQiOnsibGl0LWxpdGFjdGlvbjovLyoiOnsiKi8qIjpbe31dfX0sInByZiI6W119\",\"address\":\"0xc17BD003ca0759f5914D2A9079d89b19B0CcD615\"}],\"issuedAt\":\"2023-10-21T19:22:49.060Z\",\"expiration\":\"2023-10-21T19:27:49.060Z\",\"nodeAddress\":\"https://cayenne.litgateway.com:7370\"}", address: "4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f", algo: Some("ed25519"), auth_material_type: SessionSig, chain: None }), auth_methods: Some([]) })"#;
        let auth_sig_no_space = r#"request: Json(JsonPKPSigningRequest { to_sign: [226, 112, 61, 209, 25, 152, 79, 112, 226, 74, 222, 91, 10, 13, 107, 155, 130, 154, 133, 94, 124, 30, 103, 110, 118, 106, 12, 43, 141, 43, 167, 192], pubkey: "0x04829459732377144667592900904dead0e44717664c1df61c9dd4a5b63e4161c95833ed7589c1b90676412109d32bdd2e3491dfe7c303c145625824d3a4e2bfd1", auth_sig: Single(JsonAuthSig { sig:"ed0469f4305d9cbe5739aaec1fb7bb13622d805830394828cc8616377ffca25e3025248167c3a2576328faa3b417ffd9d9cffebf4a648d9985059d6d5c0aa607", derived_via: "litSessionSignViaNacl", signed_message: "{\"sessionKey\":\"4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f\",\"resourceAbilityRequests\":[{\"resource\":{\"resource\":\"*\",\"resourcePrefix\":\"lit-litaction\"},\"ability\":\"pkp-signing\"}],\"capabilities\":[{\"sig\":\"0xf92355a01d003c018b066c647b3c46890541092b841499d03c19cbac7718abd3478b5a66a967a4e09c0cbc2f7704c7431040c0c381785a53561bfc82c8c1e0621b\",\"derivedVia\":\"web3.eth.personal.sign via Lit PKP\",\"signedMessage\":\"localhost:3000 wants you to sign in with your Ethereum account:\\n0xc17BD003ca0759f5914D2A9079d89b19B0CcD615\\n\\nLit Protocol PKP session signature I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-litaction://*'.\\n\\nURI: lit:session:4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f\\nVersion: 1\\nChain ID: 1\\nNonce: Ei85x7hogxywfUTyW\\nIssued At: 2023-10-21T19:22:48.623Z\\nExpiration Time: 2023-10-22T19:22:48.492Z\\nResources:\\n- urn:recap:eyJhdHQiOnsibGl0LWxpdGFjdGlvbjovLyoiOnsiKi8qIjpbe31dfX0sInByZiI6W119\",\"address\":\"0xc17BD003ca0759f5914D2A9079d89b19B0CcD615\"}],\"issuedAt\":\"2023-10-21T19:22:49.060Z\",\"expiration\":\"2023-10-21T19:27:49.060Z\",\"nodeAddress\":\"https://cayenne.litgateway.com:7370\"}", address: "4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f", algo: Some("ed25519"), auth_material_type: SessionSig, chain: None }), auth_methods: Some([]) })"#;
        let auth_sig_0x = r#"request: Json(JsonPKPSigningRequest { to_sign: [226, 112, 61, 209, 25, 152, 79, 112, 226, 74, 222, 91, 10, 13, 107, 155, 130, 154, 133, 94, 124, 30, 103, 110, 118, 106, 12, 43, 141, 43, 167, 192], pubkey: "0x04829459732377144667592900904dead0e44717664c1df61c9dd4a5b63e4161c95833ed7589c1b90676412109d32bdd2e3491dfe7c303c145625824d3a4e2bfd1", auth_sig: Single(JsonAuthSig { sig: \\\"0xed0469f4305d9cbe5739aaec1fb7bb13622d805830394828cc8616377ffca25e3025248167c3a2576328faa3b417ffd9d9cffebf4a648d9985059d6d5c0aa607\\", derived_via: "litSessionSignViaNacl", signed_message: "{\"sessionKey\":\"4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f\",\"resourceAbilityRequests\":[{\"resource\":{\"resource\":\"*\",\"resourcePrefix\":\"lit-litaction\"},\"ability\":\"pkp-signing\"}],\"capabilities\":[{\"sig\\\\":\\\"0xf92355a01d003c018b066c647b3c46890541092b841499d03c19cbac7718abd3478b5a66a967a4e09c0cbc2f7704c7431040c0c381785a53561bfc82c8c1e0621b\",\"derivedVia\":\"web3.eth.personal.sign via Lit PKP\",\"signedMessage\":\"localhost:3000 wants you to sign in with your Ethereum account:\\n0xc17BD003ca0759f5914D2A9079d89b19B0CcD615\\n\\nLit Protocol PKP session signature I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-litaction://*'.\\n\\nURI: lit:session:4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f\\nVersion: 1\\nChain ID: 1\\nNonce: Ei85x7hogxywfUTyW\\nIssued At: 2023-10-21T19:22:48.623Z\\nExpiration Time: 2023-10-22T19:22:48.492Z\\nResources:\\n- urn:recap:eyJhdHQiOnsibGl0LWxpdGFjdGlvbjovLyoiOnsiKi8qIjpbe31dfX0sInByZiI6W119\",\"address\":\"0xc17BD003ca0759f5914D2A9079d89b19B0CcD615\"}],\"issuedAt\":\"2023-10-21T19:22:49.060Z\",\"expiration\":\"2023-10-21T19:27:49.060Z\",\"nodeAddress\":\"https://cayenne.litgateway.com:7370\"}", address: "4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f", algo: Some("ed25519"), auth_material_type: SessionSig, chain: None }), auth_methods: Some([]) })"#;
        let auth_sig_0x_no_space = r#"request: Json(JsonPKPSigningRequest { to_sign: [226, 112, 61, 209, 25, 152, 79, 112, 226, 74, 222, 91, 10, 13, 107, 155, 130, 154, 133, 94, 124, 30, 103, 110, 118, 106, 12, 43, 141, 43, 167, 192], pubkey: "0x04829459732377144667592900904dead0e44717664c1df61c9dd4a5b63e4161c95833ed7589c1b90676412109d32bdd2e3491dfe7c303c145625824d3a4e2bfd1", auth_sig: Single(JsonAuthSig { sig:\"0xed0469f4305d9cbe5739aaec1fb7bb13622d805830394828cc8616377ffca25e3025248167c3a2576328faa3b417ffd9d9cffebf4a648d9985059d6d5c0aa607\\", derived_via: "litSessionSignViaNacl", signed_message: "{\"sessionKey\":\"4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f\",\"resourceAbilityRequests\":[{\"resource\":{\"resource\":\"*\",\"resourcePrefix\":\"lit-litaction\"},\"ability\":\"pkp-signing\"}],\"capabilities\":[{\"sig\\\":\\"0xf92355a01d003c018b066c647b3c46890541092b841499d03c19cbac7718abd3478b5a66a967a4e09c0cbc2f7704c7431040c0c381785a53561bfc82c8c1e0621b\\\",\"derivedVia\":\"web3.eth.personal.sign via Lit PKP\",\"signedMessage\":\"localhost:3000 wants you to sign in with your Ethereum account:\\n0xc17BD003ca0759f5914D2A9079d89b19B0CcD615\\n\\nLit Protocol PKP session signature I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-litaction://*'.\\n\\nURI: lit:session:4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f\\nVersion: 1\\nChain ID: 1\\nNonce: Ei85x7hogxywfUTyW\\nIssued At: 2023-10-21T19:22:48.623Z\\nExpiration Time: 2023-10-22T19:22:48.492Z\\nResources:\\n- urn:recap:eyJhdHQiOnsibGl0LWxpdGFjdGlvbjovLyoiOnsiKi8qIjpbe31dfX0sInByZiI6W119\",\"address\":\"0xc17BD003ca0759f5914D2A9079d89b19B0CcD615\"}],\"issuedAt\":\"2023-10-21T19:22:49.060Z\",\"expiration\":\"2023-10-21T19:27:49.060Z\",\"nodeAddress\":\"https://cayenne.litgateway.com:7370\"}", address: "4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f", algo: Some("ed25519"), auth_material_type: SessionSig, chain: None }), auth_methods: Some([]) })"#;
        let auth_sig_new_and_js_params = r#"request: Json(JsonExecutionRequest { code: None, ipfs_id: Some(\"QmQUSvCZY544S9epXznU2TY3Geqsjzyc7ALfsWjBcxkVNd\"), auth_sig: Single(JsonAuthSig { sig: \"0xed2b2f89e310783900cc1d8c2fd100f3d9b7237e2beb7c221ea1e4bbcd1faa772778d001dde20929a7d33d5980d5a7fa134fa63638dc69b36a58ef83aed066c91c\", derived_via: \"web3.eth.personal.sign\", signed_message: \"localhost wants you to sign in with your Ethereum account:\\n0x44F6a7cb0193c73019e3FAEFdce7a86f1081713d\\n\\nSign in to localhost\\n\\nURI: http://localhost/login\\nVersion: 1\\nChain ID: 1\\nNonce: 0xe24f4fe1e6dbfb20fad7795c2353726d98e26e76387b498ef44b96349f50004b\\nIssued At: 2024-02-19T23:01:26.716Z\\nExpiration Time: 2024-02-20T00:01:26.715Z\", address: \"0x44F6a7cb0193c73019e3FAEFdce7a86f1081713d\", algo: None, auth_material_type: WalletSig, chain: None }), js_params: Some(Object {\"JWTtoken\": String(\"eyJraWQiOiJ6cWR3SWxTODNsbXg2Q3k1VXN4RHkyenVsSDlDUm5TQlRjRjFyUnExWWI4PSIsImFsZyI6IlJTMjU2In0.eyJzdWIiOiI0N2QwMjk0Ni03ZjViLTQwYzUtOGYzOS0zZTY5MTIwMDFjZDEiLCJpc3MiOiJodHRwczpcL1wvY29nbml0by1pZHAudXMtZWFzdC0xLmFtYXpvbmF3cy5jb21cL3VzLWVhc3QtMV9neU55b2NyWjMiLCJjbGllbnRfaWQiOiI0bXNnc21wazI0MzQ2bDNpaDBlNGYyN3I3cSIsIm9yaWdpbl9qdGkiOiJhYTczNDVkMi0yOTc3LTRkM2YtODk5Yy1kMzUxM2VkMTMwYzUiLCJldmVudF9pZCI6ImQwNjI3ZGQ4LWFmNzYtNGUyMS1hOGM0LTI3NjY0NTgyNzY5NiIsInRva2VuX3VzZSI6ImFjY2VzcyIsInNjb3BlIjoiYXdzLmNvZ25pdG8uc2lnbmluLnVzZXIuYWRtaW4iLCJhdXRoX3RpbWUiOjE3MDgzODM2NjQsImV4cCI6MTcwODM4NzI2NCwiaWF0IjoxNzA4MzgzNjY0LCJqdGkiOiJkNjU5OWYxNS1kMzkxLTRiNDctYWU1NS04MWEzNTE4MDIwODgiLCJ1c2VybmFtZSI6ImRlbW8tdXNlci1leHRlcm5hbCJ9.mzVTZaJeawO5pafYeQmVXxf6ExrU4tsRp6xq_VJo8A4vBabXPOhYo4RvN_MIK__A8t3N0xQuiyNN4HhLgvUbilMxzU6wI5rFkzxH0K7T-XU_YiJd8bibhOM4Mi8keAPbi18PGtokEoQCD1nyiO9H9FNdGtJUSv3URmyPu1QATo5oh4ANwCS25f-bEqx_700rZKsgGiJgFO0oMbmt55udS605D4vSits8Xc4FlWUvCsAmtMr2wfZx-ZtBD19EbKe_K-VO7zkCgMIGDJe2Pvu-j8X2RB73qEiktS2QP32U1mxQz3vWZcTJ_uF7T0x2yyuWsBrfVGosg3NfV3BE_OOkyA\"), \"clientId\": String(\"demo-user-external\"), \"publicKey\": String(\"0x0429c21ae63797f20360012b467a9e3251c46d5c651f54d97a4147a08314b2c0405bec0fa2038c001ca5ca64e4367fb57212e1e8c69be265700ffe13b037b4f6c9\"), \"toSign\": Array [Number(5), Number(90), Number(51), Number(70), Number(56), Number(7), Number(247), Number(2), Number(18), Number(177), Number(24), Number(2), Number(115), Number(175), Number(137), Number(183), Number(88), Number(48), Number(226), Number(66), Number(77), Number(225), Number(243), Number(234), Number(4), Number(24), Number(122), Number(49), Number(10), Number(133), Number(194), Number(180)], \"sigName\": String(\"sig1\")}), auth_methods: Some([]) })"#;

        let access_token = r#"{access_token: "1.fFAGRNJru1FTz70BzhT3Zg" }],"expires_in":3920,"token_type":"Bearer"}"#;
        let access_token_no_space = r#"{access_token:"1.fFAGRNJru1FTz70BzhT3Zg"}],"expires_in":3920,"token_type":"Bearer"}"#;

        let access_token_authsig_backspace = r#"request: Json(JsonSignSessionKeyRequest { session_key: \"lit:session:c0d7301306dbfd23fba8fef800c959077c4f44ec9faa2d8deaa27a1ec22689e0\", auth_methods: [AuthMethod { auth_method_type: 1, access_token: \"{\\\"sig\\\":\\\"0x333701b41f9e7881379933b69458c43f1bf8409d96ed4adfcdec1a74f20730fe0fa579ac0c865f1020aba01dbbb286cf7f4b9aff85736867eba0ee7db76f55d41c\\\",\\\"derivedVia\\\":\\\"web3.eth.personal.sign\\\",\\\"signedMessage\\\":\\\"localhost wants you to sign in with your Ethereum account:\\\\n0xeF71c2604f17Ec6Fc13409DF24EfdC440D240d37\\\\n\\\\nThis is a test statement.  You can put anything you want here.\\\\n\\\\nURI: https://localhost/login\\\\nVersion: 1\\\\nChain ID: 1\\\\nNonce: 0x5c8f020b34053ac1639234ea2ebb9aad10694d64e01704836e76bf91f4a9ada6\\\\nIssued At: 2024-02-19T22:42:57.416Z\\\\nExpiration Time: 2024-02-19T23:42:57.408Z\\\",\\\"address\\\":\\\"0xeF71c2604f17Ec6Fc13409DF24EfdC440D240d37\\\"}\" }], pkp_public_key: Some(\"0x04146c6dc4d6f02681a82ba5bbaf04e062fbfd043806d25d7e7c04b2c544f17bf1dc71ec3da2f7d53fab7d33d196b99a70667375c1eca7c03cfcd1b4b435f22aa6\"), auth_sig: None, siwe_message: \"litprotocol.com wants you to sign in with your Ethereum account:\\n0xa4F249915082360f8DBf61Ee047F75943F23e0FF\\n\\nLit Protocol PKP session signature I further authorize the stated URI to perform the following actions on my behalf:\\n\\nURI: lit:session:c0d7301306dbfd23fba8fef800c959077c4f44ec9faa2d8deaa27a1ec22689e0\\nVersion: 1\\nChain ID: 1\\nNonce: 0x5c8f020b34053ac1639234ea2ebb9aad10694d64e01704836e76bf91f4a9ada6\\nIssued At: 2024-02-19T22:43:06.910Z\\nExpiration Time: 2024-02-19T23:43:06.861Z\\nResources:\\n- urn:recap:eyJhdHQiOnt9LCJwcmYiOltdfQ\" })"#;
        let access_token_lit_actions_backspace = r#"request: Json(JsonSignSessionKeyRequest { session_key: \"lit:session:aabae04e3de8908f7fd33e440c239fe64a447bb187a6b1922ffb3fb7e0da1172\", auth_methods: [AuthMethod { auth_method_type: 2, access_token: \"{\\\"signedMessage\\\":\\\"localhost:3000 wants you to sign in with your Ethereum account:\\\\n0x1b9Aceb609a62bae0c0a9682A9268138Faff4F5f\\\\n\\\\nGive this application access to some of your data on Ceramic\\\\n\\\\nURI: did:key:z6MkpxchWVryL81LVMTBspjjmZFiqNTZENrEvNPZ6H7Cfdue\\\\nVersion: 1\\\\nChain ID: 1\\\\nNonce: jLL7t4D4jb\\\\nIssued At: 2024-02-17T22:17:08.050Z\\\\nExpiration Time: 2024-03-13T22:17:08.050Z\\\\nResources:\\\\n- ceramic://*\\\",\\\"address\\\":\\\"0x1b9Aceb609a62bae0c0a9682A9268138Faff4F5f\\\",\\\"derivedVia\\\":\\\"web3.eth.personal.sign\\\",\\\"sig\\\":\\\"0x8d5bf2c553d577c6e62e154efc5a6bd66f71d1021720505d1723510b036d83d811c68a7afca0848f3d1e6e14e3da74c4aaa7a250a591ab79d3f181f2b7f61bb01b\\\"}\" }], pkp_public_key: Some(\"0x0416ee2babf901e35d45885a5f8b8ae98c4bba60c296f3f8366e5a8faf712699ba7a48b8b528bec153d9644df9ba2712a6d18875a8b0cddf65475ec18c201c07aa\"), auth_sig: None, siwe_message: \"litprotocol.com wants you to sign in with your Ethereum account:\\n0x90ed73Efe2f0f8a7d1850521d24e178B019ec0e8\\n\\nLit Protocol PKP session signature Some custom statement.\\n\\nURI: lit:session:aabae04e3de8908f7fd33e440c239fe64a447bb187a6b1922ffb3fb7e0da1172\\nVersion: 1\\nChain ID: 1\\nNonce: 0xd4167ed8885f5aa39fbbc4379b1adfde6ded1eb4dbc75e3ae301cff950b9459b\\nIssued At: 2024-02-18T14:00:38.111Z\\nExpiration Time: 2024-02-19T14:00:38.083Z\\nResources:\\n- urn:recap:eyJhdHQiOnt9LCJwcmYiOltdfQ\" })"#;
        let access_token_web_authin_backspace = r#"request: Json(JsonSignSessionKeyRequest { session_key: \"lit:session:bff7b795fcdea64910092584f32e1bb1d1591f4a47a6144ad987d9cd690d2a93\", auth_methods: [AuthMethod { auth_method_type: 3, access_token: \"{\\\"id\\\":\\\"ABlP0F8vVC-aYtbivbBvu1L1YYU\\\",\\\"rawId\\\":\\\"ABlP0E8vVC-aYtbivbBvu1L1YYU\\\",\\\"response\\\":{\\\"clientDataJSON\\\":\\\"eyJ0eXBlIjoid2ViYXV0aG4uZ2V0IiwiY2hhbGxlbmdlIjoiWS1rWkNBM0dXWFN6WU1JSnc5TWtYSVZsN3ZNUldEcFozUFlHSkp1QTdjMCIsIm9yaWdpbiI6Imh0dHBzOi8vbGl0LXJlbGF5LmFwaS5vYnZpb3VzLm1vbmV5In0\\\",\\\"authenticatorData\\\":\\\"AZ75OL2a0lZMTBfy3cqwgb9oAZsL7YO-xcPBv-0CTG8dAAAAAA\\\",\\\"signature\\\":\\\"MEYCIQDbI68GCD-Bcl-QY-VUurvKdpXjwMjGLN6-p9GPTUPGfwIhAIqg7dUtvkvGplMRmZxpDEX_Qx6kdeaniQivFZtUh3Yj\\\",\\\"userHandle\\\":\\\"N2RhNzc0ZDM5ZTk1N2M4OGYzMDQ5NTFlOWIxNmQ2MzY5OWM5YmRjM2RkZjUxNzY3NzM3NWQwNGJkMzNmNTU4Zg\\\"},\\\"type\\\":\\\"public-key\\\"}\" }], pkp_public_key: Some(\"0x04874a8fe4f75d63b1fc12063aab3b28a3862d92bdaf83c6e6dcacb7f44bca558593349dc18085386764550e3c525385631a164752c418630f4d6fe289a10b5441\"), auth_sig: None, siwe_message: \"lit-relay.api.obvious.money wants you to sign in with your Ethereum account:\\n0x278A736EFe085Dce2A8D1CD106486d8cB849b6E4\\n\\nLit Protocol PKP session signature\\n\\nURI: lit:session:bff7b795fcdea64910092584f32e1bb1d1591f4a47a6144ad987d9cd690d2a93\\nVersion: 1\\nChain ID: 1\\nNonce: 0x268bec59ee01b2d59f1e4f0504478987e43c8de5c09600c3cde36a49f4e7a4a7\\nIssued At: 2024-02-19T04:36:07.036Z\\nExpiration Time: 2024-02-26T04:36:06.954Z\\nResources:\\n- urn:recap:eyJhdHQiOnt9LCJwcmYiOltdfQ\" })"#;
        let access_token_stych_backspace = r#"request: Json(JsonSignSessionKeyRequest { session_key: \"lit:session:b3cb7de2071e9a5e5102069dd6884c8f505236b2c1342d4c2faf4cc60f6b85e5\", auth_methods: [AuthMethod { auth_method_type: 9, access_token: \"eyJhbGciOiJSUzI1NiIsJmtpZCI6Imp3ay1saXZlLTBjYTE4Mjk1LTlhNzktNDI1MS05MTA0LTRhZGM2ZTdhZDcyYSIsInR5cCI6IkpXVCJ9.eyJhdWQiOlsicHJvamVjdC1saXZlLTQzYzhmYjNlLTg5OWQtNGMxZC04ZTZkLTU3M2JkYzc4NzAxOSJdLCJleHAiOjE3MDgzMTkzODcsImh0dHBzOi8vc3R5dGNoLmNvbS9zZXNzaW9uIjp7ImlkIjoic2Vzc2lvbi1saXZlLTlhNmVlODQ4LWVjMzYtNDE3NS1iMWM4LTNkNmMwZjA5YjA4MSIsInN0YXJ0ZWRfYXQiOiIyMDI0LTAyLTE5VDA1OjA0OjQ3WiIsImxhc3RfYWNjZXNzZWRfYXQiOiIyMDI0LTAyLTE5VDA1OjA0OjQ3WiIsImV4cGlyZXNfYXQiOiIyMDI0LTAyLTE5VDA1OjE0OjQ3WiIsImF0dHJpYnV0ZXMiOnsidXNlcl9hZ2VudCI6IiIsImlwX2FkZHJlc3MiOiIifSwiYXV0aGVudGljYXRpb25fZmFjdG9ycyI6W3sidHlwZSI6Im90cCIsImRlbGl2ZXJ5X21ldGhvZCI6InNtcyIsImxhc3RfYXV0aGVudGljYXRlZF9hdCI6IjIwMjQtMDItMTlUMDU6MDQ6NDdaIiwicGhvbmVfbnVtYmVyX2ZhY3RvciI6eyJwaG9uZV9pZCI6InBob25lLW51bWJlci1saXZlLWIzYWZkMjAwLWMzMDAtNDBiMS04NTEyLWI1NWNhMjc2Nzg2YyIsInBob25lX251bWJlciI6Iis5MTczMDIxNTIxMjEifX1dfSwiaWF0IjoxNzA4MzE5MDg3LCJpc3MiOiJzdHl0Y2guY29tL3Byb2plY3QtbGl2ZS00M2M4ZmIzZS04OTlkLTRjMWQtOGU2ZC01NzNiZGM3ODcwMTkiLCJuYmYiOjE3MDgzMTkwODcsInN1YiI6InVzZXItbGl2ZS05YWZmODczZi03Yjc3LTRiZmMtYWY4ZC0zNDg3NmY1MDdjMDUifQ.kuUatgoY88oxBfX_InrN52ObH6CafgLqLV4Wez8MS2y3XpLl7FnkPCOxQk-FD3vtnK_pKDLULCbs9V2xno9Uq131vW6TNNMgdiBUH6yBt_HNP3E4CLko8NV6bl7tNCDc8AeM1-Lqkl_e0-_Uph7T2hJ3Z8SXDhsONu3i57q7dUFhWLkdQEBNxTQogD5ys6hu-WRb_289Us0u8WNDZ29lWtGHH1B08n-_BUudEeZJtn3bB19YUr-y7SnVqbtjqi3a2HiA0iUcALrW2O8Q5iONxQ3ZZCR22-75WG2wBP1vmBM67wRi2DlJsh8YoROvxgn9PmxyiG8u10aRPxO5y3TtAg\" }], pkp_public_key: Some(\"0x04835826250760349970e5f3a1893973ab94168f1cb8027793bfd9c25a439d5964aa977d5d156cd2d884e8b34d2662486d4bc9a40bad22338ae2224a6efbb6cadd\"), auth_sig: None, siwe_message: \"lit-relay.api.obvious.money wants you to sign in with your Ethereum account:\\n0x6856EA06418646F6aeF560ae3B497FBa1afaB7aF\\n\\nLit Protocol PKP session signature\\n\\nURI: lit:session:b3cb7de2071e9a5e5102069dd6884c8f505236b2c1342d4c2faf4cc60f6b85e5\\nVersion: 1\\nChain ID: 1\\nNonce: 0xc4ddc4f9b99a38a57c19b89f10b388b9b448bf46fcbe0eed3b9807f512a71500\\nIssued At: 2024-02-19T05:05:08.663Z\\nExpiration Time: 2024-02-26T05:05:08.661Z\\nResources:\\n- urn:recap:eyJhdHQiOnt9LCJwcmYiOltdfQ\" })"#;

        let access_token_inner_sig = r#"sign_session_key, request: Json(JsonSignSessionKeyRequest { session_key: "lit:session:88f3b3d7cec50a0c02a5aa40b0a7e5bbaf27ebbfc28faa532163c5f2f762902b", auth_methods: [AuthMethod { auth_method_type: 1, access_token: "{\"sig\":\"0x78d58943441b2f94c8568d08aa3346e2d1d8ccd6733d039de298af16bb9fe9ca162ef8be7458ae62c2633d6b0cf839807ea9d10aaba31218061dc7472c9c0e5c1c\",\"derivedVia\":\"web3.eth.personal.sign\",\"signedMessage\":\"localhost wants you to sign in with your Ethereum account:\\n0xeF71c2604f17Ec6Fc13409DF24EfdC440D240d37\\n\\nTESTING TESTING 123\\n\\nURI: https://localhost/login\\nVersion: 1\\nChain ID: 1\\nNonce: E8NnQ74rcoslAlkdy\\nIssued At: 2023-10-03T14:32:02.839Z\\nExpiration Time: 2215-05-30T14:32:02.839Z\",\"address\":\"0xeF71c2604f17Ec6Fc13409DF24EfdC440D240d37\"}" }], pkp_public_key: Some("0x04af705894efee1734cb4286dde21047c24e49f73630778b81745d6df00c12bf6a94cafb64adfc613d6d4831c89fdb69c3b75c1f269e7342f06ffa2eaf77846e9b"), auth_sig: Some(Single(JsonAuthSig { sig: "0xed0469f4305d9cbe5739aaec1fb7bb13622d805830394828cc8616377ffca25e3025248167c3a2576328faa3b417ffd9d9cffebf4a648d9985059d6d5c0aa607", derived_via: "web3.eth.personal.sign", signed_message: "localhost wants you to sign in with your Ethereum account:\n0xeF71c2604f17Ec6Fc13409DF24EfdC440D240d37\n\nTESTING TESTING 123\n\nURI: https://localhost/login\nVersion: 1\nChain ID: 1\nNonce: E8NnQ74rcoslAlkdy\nIssued At: 2023-10-03T14:32:02.839Z\nExpiration Time: 2215-05-30T14:32:02.839Z", address: "0xeF71c2604f17Ec6Fc13409DF24EfdC440D240d37", algo: None, auth_material_type: WalletSig, chain: None })), siwe_message: "litprotocol.com wants you to sign in with your Ethereum account:\n0x04297320D992063F8b10c9F80F7419c811B25F05\n\nLit Protocol PKP session signature I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-litaction://*'.\n\nURI: lit:session:88f3b3d7cec50a0c02a5aa40b0a7e5bbaf27ebbfc28faa532163c5f2f762902b\nVersion: 1\nChain ID: 1\nNonce: yU2BM0BvHaoCccScf\nIssued At: 2023-10-23T18:22:04.873Z\nExpiration Time: 2023-10-30T18:22:04.870Z\nResources:\n- urn:recap:eyJhdHQiOnsibGl0LWxpdGFjdGlvbjovLyoiOnsiKi8qIjpbe31dfX0sInByZiI6W119" })"#;

        assert_eq!(
            filter_sensitive_info(auth_sig),
            r#"request: Json(JsonPKPSigningRequest { to_sign: [226, 112, 61, 209, 25, 152, 79, 112, 226, 74, 222, 91, 10, 13, 107, 155, 130, 154, 133, 94, 124, 30, 103, 110, 118, 106, 12, 43, 141, 43, 167, 192], pubkey: "0x04829459732377144667592900904dead0e44717664c1df61c9dd4a5b63e4161c95833ed7589c1b90676412109d32bdd2e3491dfe7c303c145625824d3a4e2bfd1", auth_sig: "***filtered***", auth_methods: Some([]) })"#
        );
        assert_eq!(
            filter_sensitive_info(auth_sig_no_space),
            r#"request: Json(JsonPKPSigningRequest { to_sign: [226, 112, 61, 209, 25, 152, 79, 112, 226, 74, 222, 91, 10, 13, 107, 155, 130, 154, 133, 94, 124, 30, 103, 110, 118, 106, 12, 43, 141, 43, 167, 192], pubkey: "0x04829459732377144667592900904dead0e44717664c1df61c9dd4a5b63e4161c95833ed7589c1b90676412109d32bdd2e3491dfe7c303c145625824d3a4e2bfd1", auth_sig: "***filtered***", auth_methods: Some([]) })"#
        );
        assert_eq!(
            filter_sensitive_info(auth_sig_0x),
            r#"request: Json(JsonPKPSigningRequest { to_sign: [226, 112, 61, 209, 25, 152, 79, 112, 226, 74, 222, 91, 10, 13, 107, 155, 130, 154, 133, 94, 124, 30, 103, 110, 118, 106, 12, 43, 141, 43, 167, 192], pubkey: "0x04829459732377144667592900904dead0e44717664c1df61c9dd4a5b63e4161c95833ed7589c1b90676412109d32bdd2e3491dfe7c303c145625824d3a4e2bfd1", auth_sig: "***filtered***", auth_methods: Some([]) })"#
        );
        assert_eq!(
            filter_sensitive_info(auth_sig_0x_no_space),
            r#"request: Json(JsonPKPSigningRequest { to_sign: [226, 112, 61, 209, 25, 152, 79, 112, 226, 74, 222, 91, 10, 13, 107, 155, 130, 154, 133, 94, 124, 30, 103, 110, 118, 106, 12, 43, 141, 43, 167, 192], pubkey: "0x04829459732377144667592900904dead0e44717664c1df61c9dd4a5b63e4161c95833ed7589c1b90676412109d32bdd2e3491dfe7c303c145625824d3a4e2bfd1", auth_sig: "***filtered***", auth_methods: Some([]) })"#
        );
        assert_eq!(
            filter_sensitive_info(auth_sig_new_and_js_params),
            r#"request: Json(JsonExecutionRequest { code: None, ipfs_id: Some(\"QmQUSvCZY544S9epXznU2TY3Geqsjzyc7ALfsWjBcxkVNd\"), auth_sig: "***filtered***", js_params: "***filtered***", auth_methods: Some([]) })"#
        );
        assert_eq!(
            filter_sensitive_info(access_token),
            r#"{access_token: "***filtered***" }],"expires_in":3920,"token_type":"Bearer"}"#
        );
        assert_eq!(
            filter_sensitive_info(access_token_no_space),
            r#"{access_token: "***filtered***" }],"expires_in":3920,"token_type":"Bearer"}"#
        );
        assert_eq!(
            filter_sensitive_info(access_token_inner_sig),
            r#"sign_session_key, request: Json(JsonSignSessionKeyRequest { session_key: "lit:session:88f3b3d7cec50a0c02a5aa40b0a7e5bbaf27ebbfc28faa532163c5f2f762902b", auth_methods: [AuthMethod { auth_method_type: 1, access_token: "***filtered***" }], pkp_public_key: Some("0x04af705894efee1734cb4286dde21047c24e49f73630778b81745d6df00c12bf6a94cafb64adfc613d6d4831c89fdb69c3b75c1f269e7342f06ffa2eaf77846e9b"), auth_sig: "***filtered***"), siwe_message: "litprotocol.com wants you to sign in with your Ethereum account:\n0x04297320D992063F8b10c9F80F7419c811B25F05\n\nLit Protocol PKP session signature I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-litaction://*'.\n\nURI: lit:session:88f3b3d7cec50a0c02a5aa40b0a7e5bbaf27ebbfc28faa532163c5f2f762902b\nVersion: 1\nChain ID: 1\nNonce: yU2BM0BvHaoCccScf\nIssued At: 2023-10-23T18:22:04.873Z\nExpiration Time: 2023-10-30T18:22:04.870Z\nResources:\n- urn:recap:eyJhdHQiOnsibGl0LWxpdGFjdGlvbjovLyoiOnsiKi8qIjpbe31dfX0sInByZiI6W119" })"#
        );
        assert_eq!(
            filter_sensitive_info(access_token_authsig_backspace),
            r#"request: Json(JsonSignSessionKeyRequest { session_key: \"lit:session:c0d7301306dbfd23fba8fef800c959077c4f44ec9faa2d8deaa27a1ec22689e0\", auth_methods: [AuthMethod { auth_method_type: 1, access_token: "***filtered***" }], pkp_public_key: Some(\"0x04146c6dc4d6f02681a82ba5bbaf04e062fbfd043806d25d7e7c04b2c544f17bf1dc71ec3da2f7d53fab7d33d196b99a70667375c1eca7c03cfcd1b4b435f22aa6\"), auth_sig: None, siwe_message: \"litprotocol.com wants you to sign in with your Ethereum account:\\n0xa4F249915082360f8DBf61Ee047F75943F23e0FF\\n\\nLit Protocol PKP session signature I further authorize the stated URI to perform the following actions on my behalf:\\n\\nURI: lit:session:c0d7301306dbfd23fba8fef800c959077c4f44ec9faa2d8deaa27a1ec22689e0\\nVersion: 1\\nChain ID: 1\\nNonce: 0x5c8f020b34053ac1639234ea2ebb9aad10694d64e01704836e76bf91f4a9ada6\\nIssued At: 2024-02-19T22:43:06.910Z\\nExpiration Time: 2024-02-19T23:43:06.861Z\\nResources:\\n- urn:recap:eyJhdHQiOnt9LCJwcmYiOltdfQ\" })"#
        );
        assert_eq!(
            filter_sensitive_info(access_token_lit_actions_backspace),
            r#"request: Json(JsonSignSessionKeyRequest { session_key: \"lit:session:aabae04e3de8908f7fd33e440c239fe64a447bb187a6b1922ffb3fb7e0da1172\", auth_methods: [AuthMethod { auth_method_type: 2, access_token: "***filtered***" }], pkp_public_key: Some(\"0x0416ee2babf901e35d45885a5f8b8ae98c4bba60c296f3f8366e5a8faf712699ba7a48b8b528bec153d9644df9ba2712a6d18875a8b0cddf65475ec18c201c07aa\"), auth_sig: None, siwe_message: \"litprotocol.com wants you to sign in with your Ethereum account:\\n0x90ed73Efe2f0f8a7d1850521d24e178B019ec0e8\\n\\nLit Protocol PKP session signature Some custom statement.\\n\\nURI: lit:session:aabae04e3de8908f7fd33e440c239fe64a447bb187a6b1922ffb3fb7e0da1172\\nVersion: 1\\nChain ID: 1\\nNonce: 0xd4167ed8885f5aa39fbbc4379b1adfde6ded1eb4dbc75e3ae301cff950b9459b\\nIssued At: 2024-02-18T14:00:38.111Z\\nExpiration Time: 2024-02-19T14:00:38.083Z\\nResources:\\n- urn:recap:eyJhdHQiOnt9LCJwcmYiOltdfQ\" })"#
        );
        assert_eq!(
            filter_sensitive_info(access_token_web_authin_backspace),
            r#"request: Json(JsonSignSessionKeyRequest { session_key: \"lit:session:bff7b795fcdea64910092584f32e1bb1d1591f4a47a6144ad987d9cd690d2a93\", auth_methods: [AuthMethod { auth_method_type: 3, access_token: "***filtered***" }], pkp_public_key: Some(\"0x04874a8fe4f75d63b1fc12063aab3b28a3862d92bdaf83c6e6dcacb7f44bca558593349dc18085386764550e3c525385631a164752c418630f4d6fe289a10b5441\"), auth_sig: None, siwe_message: \"lit-relay.api.obvious.money wants you to sign in with your Ethereum account:\\n0x278A736EFe085Dce2A8D1CD106486d8cB849b6E4\\n\\nLit Protocol PKP session signature\\n\\nURI: lit:session:bff7b795fcdea64910092584f32e1bb1d1591f4a47a6144ad987d9cd690d2a93\\nVersion: 1\\nChain ID: 1\\nNonce: 0x268bec59ee01b2d59f1e4f0504478987e43c8de5c09600c3cde36a49f4e7a4a7\\nIssued At: 2024-02-19T04:36:07.036Z\\nExpiration Time: 2024-02-26T04:36:06.954Z\\nResources:\\n- urn:recap:eyJhdHQiOnt9LCJwcmYiOltdfQ\" })"#
        );
        assert_eq!(
            filter_sensitive_info(access_token_stych_backspace),
            r#"request: Json(JsonSignSessionKeyRequest { session_key: \"lit:session:b3cb7de2071e9a5e5102069dd6884c8f505236b2c1342d4c2faf4cc60f6b85e5\", auth_methods: [AuthMethod { auth_method_type: 9, access_token: "***filtered***" }], pkp_public_key: Some(\"0x04835826250760349970e5f3a1893973ab94168f1cb8027793bfd9c25a439d5964aa977d5d156cd2d884e8b34d2662486d4bc9a40bad22338ae2224a6efbb6cadd\"), auth_sig: None, siwe_message: \"lit-relay.api.obvious.money wants you to sign in with your Ethereum account:\\n0x6856EA06418646F6aeF560ae3B497FBa1afaB7aF\\n\\nLit Protocol PKP session signature\\n\\nURI: lit:session:b3cb7de2071e9a5e5102069dd6884c8f505236b2c1342d4c2faf4cc60f6b85e5\\nVersion: 1\\nChain ID: 1\\nNonce: 0xc4ddc4f9b99a38a57c19b89f10b388b9b448bf46fcbe0eed3b9807f512a71500\\nIssued At: 2024-02-19T05:05:08.663Z\\nExpiration Time: 2024-02-26T05:05:08.661Z\\nResources:\\n- urn:recap:eyJhdHQiOnt9LCJwcmYiOltdfQ\" })"#
        );
    }
}
