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
    let re_sig = Regex::new(r#"sig:\s*(\\)*"(0x)?[a-fA-F0-9]+(\\)*""#).unwrap();
    let filtered_sig = re_sig.replace_all(&msg, r#"sig: "***filtered***""#);

    let re_access_token = Regex::new(r#"access_token:\s*(\\)*"(\w|\.|-)+(\\)*""#).unwrap();
    let filtered_access_token =
        re_access_token.replace_all(&filtered_sig, r#"access_token: "***filtered***""#);

    let re_inner_sig = Regex::new(r#""sig(\\)*":\s*(\\)*"(0x)?[a-fA-F0-9]+(\\)*""#).unwrap();
    re_inner_sig.replace_all(&filtered_access_token, r#""sig\":\"***filtered***\""#).to_string()
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

        let access_token =
            r#"{access_token: "1.fFAGRNJru1FTz70BzhT3Zg","expires_in":3920,"token_type":"Bearer"}"#;
        let access_token_no_space =
            r#"{access_token:"1.fFAGRNJru1FTz70BzhT3Zg","expires_in":3920,"token_type":"Bearer"}"#;

        let access_token_inner_sig = r#"sign_session_key, request: Json(JsonSignSessionKeyRequest { session_key: "lit:session:88f3b3d7cec50a0c02a5aa40b0a7e5bbaf27ebbfc28faa532163c5f2f762902b", auth_methods: [AuthMethod { auth_method_type: 1, access_token: "{\"sig\":\"0x78d58943441b2f94c8568d08aa3346e2d1d8ccd6733d039de298af16bb9fe9ca162ef8be7458ae62c2633d6b0cf839807ea9d10aaba31218061dc7472c9c0e5c1c\",\"derivedVia\":\"web3.eth.personal.sign\",\"signedMessage\":\"localhost wants you to sign in with your Ethereum account:\\n0xeF71c2604f17Ec6Fc13409DF24EfdC440D240d37\\n\\nTESTING TESTING 123\\n\\nURI: https://localhost/login\\nVersion: 1\\nChain ID: 1\\nNonce: E8NnQ74rcoslAlkdy\\nIssued At: 2023-10-03T14:32:02.839Z\\nExpiration Time: 2215-05-30T14:32:02.839Z\",\"address\":\"0xeF71c2604f17Ec6Fc13409DF24EfdC440D240d37\"}" }], pkp_public_key: Some("0x04af705894efee1734cb4286dde21047c24e49f73630778b81745d6df00c12bf6a94cafb64adfc613d6d4831c89fdb69c3b75c1f269e7342f06ffa2eaf77846e9b"), auth_sig: Some(Single(JsonAuthSig { sig: "0xed0469f4305d9cbe5739aaec1fb7bb13622d805830394828cc8616377ffca25e3025248167c3a2576328faa3b417ffd9d9cffebf4a648d9985059d6d5c0aa607", derived_via: "web3.eth.personal.sign", signed_message: "localhost wants you to sign in with your Ethereum account:\n0xeF71c2604f17Ec6Fc13409DF24EfdC440D240d37\n\nTESTING TESTING 123\n\nURI: https://localhost/login\nVersion: 1\nChain ID: 1\nNonce: E8NnQ74rcoslAlkdy\nIssued At: 2023-10-03T14:32:02.839Z\nExpiration Time: 2215-05-30T14:32:02.839Z", address: "0xeF71c2604f17Ec6Fc13409DF24EfdC440D240d37", algo: None, auth_material_type: WalletSig, chain: None })), siwe_message: "litprotocol.com wants you to sign in with your Ethereum account:\n0x04297320D992063F8b10c9F80F7419c811B25F05\n\nLit Protocol PKP session signature I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-litaction://*'.\n\nURI: lit:session:88f3b3d7cec50a0c02a5aa40b0a7e5bbaf27ebbfc28faa532163c5f2f762902b\nVersion: 1\nChain ID: 1\nNonce: yU2BM0BvHaoCccScf\nIssued At: 2023-10-23T18:22:04.873Z\nExpiration Time: 2023-10-30T18:22:04.870Z\nResources:\n- urn:recap:eyJhdHQiOnsibGl0LWxpdGFjdGlvbjovLyoiOnsiKi8qIjpbe31dfX0sInByZiI6W119" })"#;

        assert_eq!(
            filter_sensitive_info(auth_sig),
            r#"request: Json(JsonPKPSigningRequest { to_sign: [226, 112, 61, 209, 25, 152, 79, 112, 226, 74, 222, 91, 10, 13, 107, 155, 130, 154, 133, 94, 124, 30, 103, 110, 118, 106, 12, 43, 141, 43, 167, 192], pubkey: "0x04829459732377144667592900904dead0e44717664c1df61c9dd4a5b63e4161c95833ed7589c1b90676412109d32bdd2e3491dfe7c303c145625824d3a4e2bfd1", auth_sig: Single(JsonAuthSig { sig: "***filtered***", derived_via: "litSessionSignViaNacl", signed_message: "{\"sessionKey\":\"4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f\",\"resourceAbilityRequests\":[{\"resource\":{\"resource\":\"*\",\"resourcePrefix\":\"lit-litaction\"},\"ability\":\"pkp-signing\"}],\"capabilities\":[{\"sig\":\"***filtered***\",\"derivedVia\":\"web3.eth.personal.sign via Lit PKP\",\"signedMessage\":\"localhost:3000 wants you to sign in with your Ethereum account:\\n0xc17BD003ca0759f5914D2A9079d89b19B0CcD615\\n\\nLit Protocol PKP session signature I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-litaction://*'.\\n\\nURI: lit:session:4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f\\nVersion: 1\\nChain ID: 1\\nNonce: Ei85x7hogxywfUTyW\\nIssued At: 2023-10-21T19:22:48.623Z\\nExpiration Time: 2023-10-22T19:22:48.492Z\\nResources:\\n- urn:recap:eyJhdHQiOnsibGl0LWxpdGFjdGlvbjovLyoiOnsiKi8qIjpbe31dfX0sInByZiI6W119\",\"address\":\"0xc17BD003ca0759f5914D2A9079d89b19B0CcD615\"}],\"issuedAt\":\"2023-10-21T19:22:49.060Z\",\"expiration\":\"2023-10-21T19:27:49.060Z\",\"nodeAddress\":\"https://cayenne.litgateway.com:7370\"}", address: "4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f", algo: Some("ed25519"), auth_material_type: SessionSig, chain: None }), auth_methods: Some([]) })"#
        );
        assert_eq!(
            filter_sensitive_info(auth_sig_no_space),
            r#"request: Json(JsonPKPSigningRequest { to_sign: [226, 112, 61, 209, 25, 152, 79, 112, 226, 74, 222, 91, 10, 13, 107, 155, 130, 154, 133, 94, 124, 30, 103, 110, 118, 106, 12, 43, 141, 43, 167, 192], pubkey: "0x04829459732377144667592900904dead0e44717664c1df61c9dd4a5b63e4161c95833ed7589c1b90676412109d32bdd2e3491dfe7c303c145625824d3a4e2bfd1", auth_sig: Single(JsonAuthSig { sig: "***filtered***", derived_via: "litSessionSignViaNacl", signed_message: "{\"sessionKey\":\"4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f\",\"resourceAbilityRequests\":[{\"resource\":{\"resource\":\"*\",\"resourcePrefix\":\"lit-litaction\"},\"ability\":\"pkp-signing\"}],\"capabilities\":[{\"sig\":\"***filtered***\",\"derivedVia\":\"web3.eth.personal.sign via Lit PKP\",\"signedMessage\":\"localhost:3000 wants you to sign in with your Ethereum account:\\n0xc17BD003ca0759f5914D2A9079d89b19B0CcD615\\n\\nLit Protocol PKP session signature I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-litaction://*'.\\n\\nURI: lit:session:4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f\\nVersion: 1\\nChain ID: 1\\nNonce: Ei85x7hogxywfUTyW\\nIssued At: 2023-10-21T19:22:48.623Z\\nExpiration Time: 2023-10-22T19:22:48.492Z\\nResources:\\n- urn:recap:eyJhdHQiOnsibGl0LWxpdGFjdGlvbjovLyoiOnsiKi8qIjpbe31dfX0sInByZiI6W119\",\"address\":\"0xc17BD003ca0759f5914D2A9079d89b19B0CcD615\"}],\"issuedAt\":\"2023-10-21T19:22:49.060Z\",\"expiration\":\"2023-10-21T19:27:49.060Z\",\"nodeAddress\":\"https://cayenne.litgateway.com:7370\"}", address: "4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f", algo: Some("ed25519"), auth_material_type: SessionSig, chain: None }), auth_methods: Some([]) })"#
        );
        assert_eq!(
            filter_sensitive_info(auth_sig_0x),
            r#"request: Json(JsonPKPSigningRequest { to_sign: [226, 112, 61, 209, 25, 152, 79, 112, 226, 74, 222, 91, 10, 13, 107, 155, 130, 154, 133, 94, 124, 30, 103, 110, 118, 106, 12, 43, 141, 43, 167, 192], pubkey: "0x04829459732377144667592900904dead0e44717664c1df61c9dd4a5b63e4161c95833ed7589c1b90676412109d32bdd2e3491dfe7c303c145625824d3a4e2bfd1", auth_sig: Single(JsonAuthSig { sig: "***filtered***", derived_via: "litSessionSignViaNacl", signed_message: "{\"sessionKey\":\"4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f\",\"resourceAbilityRequests\":[{\"resource\":{\"resource\":\"*\",\"resourcePrefix\":\"lit-litaction\"},\"ability\":\"pkp-signing\"}],\"capabilities\":[{\"sig\":\"***filtered***\",\"derivedVia\":\"web3.eth.personal.sign via Lit PKP\",\"signedMessage\":\"localhost:3000 wants you to sign in with your Ethereum account:\\n0xc17BD003ca0759f5914D2A9079d89b19B0CcD615\\n\\nLit Protocol PKP session signature I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-litaction://*'.\\n\\nURI: lit:session:4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f\\nVersion: 1\\nChain ID: 1\\nNonce: Ei85x7hogxywfUTyW\\nIssued At: 2023-10-21T19:22:48.623Z\\nExpiration Time: 2023-10-22T19:22:48.492Z\\nResources:\\n- urn:recap:eyJhdHQiOnsibGl0LWxpdGFjdGlvbjovLyoiOnsiKi8qIjpbe31dfX0sInByZiI6W119\",\"address\":\"0xc17BD003ca0759f5914D2A9079d89b19B0CcD615\"}],\"issuedAt\":\"2023-10-21T19:22:49.060Z\",\"expiration\":\"2023-10-21T19:27:49.060Z\",\"nodeAddress\":\"https://cayenne.litgateway.com:7370\"}", address: "4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f", algo: Some("ed25519"), auth_material_type: SessionSig, chain: None }), auth_methods: Some([]) })"#
        );
        assert_eq!(
            filter_sensitive_info(auth_sig_0x_no_space),
            r#"request: Json(JsonPKPSigningRequest { to_sign: [226, 112, 61, 209, 25, 152, 79, 112, 226, 74, 222, 91, 10, 13, 107, 155, 130, 154, 133, 94, 124, 30, 103, 110, 118, 106, 12, 43, 141, 43, 167, 192], pubkey: "0x04829459732377144667592900904dead0e44717664c1df61c9dd4a5b63e4161c95833ed7589c1b90676412109d32bdd2e3491dfe7c303c145625824d3a4e2bfd1", auth_sig: Single(JsonAuthSig { sig: "***filtered***", derived_via: "litSessionSignViaNacl", signed_message: "{\"sessionKey\":\"4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f\",\"resourceAbilityRequests\":[{\"resource\":{\"resource\":\"*\",\"resourcePrefix\":\"lit-litaction\"},\"ability\":\"pkp-signing\"}],\"capabilities\":[{\"sig\":\"***filtered***\",\"derivedVia\":\"web3.eth.personal.sign via Lit PKP\",\"signedMessage\":\"localhost:3000 wants you to sign in with your Ethereum account:\\n0xc17BD003ca0759f5914D2A9079d89b19B0CcD615\\n\\nLit Protocol PKP session signature I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-litaction://*'.\\n\\nURI: lit:session:4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f\\nVersion: 1\\nChain ID: 1\\nNonce: Ei85x7hogxywfUTyW\\nIssued At: 2023-10-21T19:22:48.623Z\\nExpiration Time: 2023-10-22T19:22:48.492Z\\nResources:\\n- urn:recap:eyJhdHQiOnsibGl0LWxpdGFjdGlvbjovLyoiOnsiKi8qIjpbe31dfX0sInByZiI6W119\",\"address\":\"0xc17BD003ca0759f5914D2A9079d89b19B0CcD615\"}],\"issuedAt\":\"2023-10-21T19:22:49.060Z\",\"expiration\":\"2023-10-21T19:27:49.060Z\",\"nodeAddress\":\"https://cayenne.litgateway.com:7370\"}", address: "4c7988e9e4ce85d4ed4f0e9dd7cced4b334a662364a700b5968505a509d0789f", algo: Some("ed25519"), auth_material_type: SessionSig, chain: None }), auth_methods: Some([]) })"#
        );
        assert_eq!(
            filter_sensitive_info(access_token),
            r#"{access_token: "***filtered***","expires_in":3920,"token_type":"Bearer"}"#
        );
        assert_eq!(
            filter_sensitive_info(access_token_no_space),
            r#"{access_token: "***filtered***","expires_in":3920,"token_type":"Bearer"}"#
        );
        assert_eq!(
            filter_sensitive_info(access_token_inner_sig),
            r#"sign_session_key, request: Json(JsonSignSessionKeyRequest { session_key: "lit:session:88f3b3d7cec50a0c02a5aa40b0a7e5bbaf27ebbfc28faa532163c5f2f762902b", auth_methods: [AuthMethod { auth_method_type: 1, access_token: "{\"sig\":\"***filtered***\",\"derivedVia\":\"web3.eth.personal.sign\",\"signedMessage\":\"localhost wants you to sign in with your Ethereum account:\\n0xeF71c2604f17Ec6Fc13409DF24EfdC440D240d37\\n\\nTESTING TESTING 123\\n\\nURI: https://localhost/login\\nVersion: 1\\nChain ID: 1\\nNonce: E8NnQ74rcoslAlkdy\\nIssued At: 2023-10-03T14:32:02.839Z\\nExpiration Time: 2215-05-30T14:32:02.839Z\",\"address\":\"0xeF71c2604f17Ec6Fc13409DF24EfdC440D240d37\"}" }], pkp_public_key: Some("0x04af705894efee1734cb4286dde21047c24e49f73630778b81745d6df00c12bf6a94cafb64adfc613d6d4831c89fdb69c3b75c1f269e7342f06ffa2eaf77846e9b"), auth_sig: Some(Single(JsonAuthSig { sig: "***filtered***", derived_via: "web3.eth.personal.sign", signed_message: "localhost wants you to sign in with your Ethereum account:\n0xeF71c2604f17Ec6Fc13409DF24EfdC440D240d37\n\nTESTING TESTING 123\n\nURI: https://localhost/login\nVersion: 1\nChain ID: 1\nNonce: E8NnQ74rcoslAlkdy\nIssued At: 2023-10-03T14:32:02.839Z\nExpiration Time: 2215-05-30T14:32:02.839Z", address: "0xeF71c2604f17Ec6Fc13409DF24EfdC440D240d37", algo: None, auth_material_type: WalletSig, chain: None })), siwe_message: "litprotocol.com wants you to sign in with your Ethereum account:\n0x04297320D992063F8b10c9F80F7419c811B25F05\n\nLit Protocol PKP session signature I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-litaction://*'.\n\nURI: lit:session:88f3b3d7cec50a0c02a5aa40b0a7e5bbaf27ebbfc28faa532163c5f2f762902b\nVersion: 1\nChain ID: 1\nNonce: yU2BM0BvHaoCccScf\nIssued At: 2023-10-23T18:22:04.873Z\nExpiration Time: 2023-10-30T18:22:04.870Z\nResources:\n- urn:recap:eyJhdHQiOnsibGl0LWxpdGFjdGlvbjovLyoiOnsiKi8qIjpbe31dfX0sInByZiI6W119" })"#
        );
    }
}
