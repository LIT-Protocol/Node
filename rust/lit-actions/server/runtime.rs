use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Once;
use std::thread;
use std::time::Duration;

use anyhow::{anyhow, bail, Context, Result};
use deno_core::{v8, JsRuntime, NoopModuleLoader};
use deno_runtime::{
    deno_fs::RealFs,
    deno_permissions::{Permissions, PermissionsContainer, PermissionsOptions},
    fmt_errors::format_js_error,
    worker::{MainWorker, WorkerOptions},
    BootstrapOptions, WorkerLogLevel,
};
use indoc::formatdoc;
use lit_actions_grpc::proto::{ExecuteJsRequest, ExecuteJsResponse};
use tokio::sync::{mpsc, oneshot};
use tonic::Status;
use tracing::{debug, info_span, instrument, warn};

// Same default limits as in lit-node's action client
const DEFAULT_TIMEOUT_MS: u64 = 30000; // 30s
const DEFAULT_MEMORY_LIMIT_MB: usize = 256; // 256MB

const EXECUTION_TERMINATED_ERROR: &str = "Uncaught Error: execution terminated";

#[derive(Debug, Copy, Clone, PartialEq)]
enum ExecutionResult {
    Complete,
    Timeout,
    OutOfMemory,
}

static RUNTIME_SNAPSHOT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/RUNTIME_SNAPSHOT.bin"));

fn deno_isolate_init() -> Option<&'static [u8]> {
    debug!("Deno isolate init with snapshots.");
    Some(RUNTIME_SNAPSHOT)
}

fn get_error_class_name(e: &anyhow::Error) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or_else(|| {
        #[allow(unexpected_cfgs)]
        if cfg!(debug) {
            warn!(
                "Error '{}' contains boxed error of unknown type:{}",
                e,
                e.chain().fold(String::new(), |mut output, e| {
                    let _ = write!(output, "\n  {e:?}");
                    output
                })
            );
        }
        "Error"
    })
}

// using the worker built into deno
#[instrument(skip_all, err)]
fn build_main_worker_and_inject_sdk(
    globals_to_inject: &Option<serde_json::Value>,
    auth_context: &Option<serde_json::Value>,
    http_headers: BTreeMap<String, String>,
    memory_limit_mb: Option<usize>,
) -> Result<MainWorker> {
    // Deny everything except for network access, e.g. via fetch()
    let perms = Permissions::from_options(&PermissionsOptions {
        allow_net: Some(vec![]),
        ..Default::default()
    })?;

    let options = WorkerOptions {
        bootstrap: BootstrapOptions {
            cpu_count: 1,
            log_level: WorkerLogLevel::Info,
            no_color: true,
            user_agent: "lit_protocol_node".to_string(),
            ..Default::default()
        },
        extensions: vec![lit_actions_ext::lit_actions::init_ops()],
        startup_snapshot: deno_isolate_init(),
        skip_op_registration: false,
        create_params: memory_limit_mb
            .map(|limit| v8::CreateParams::default().heap_limits(0, limit * 1024 * 1024)),
        unsafely_ignore_certificate_errors: None,
        root_cert_store_provider: None,
        seed: None,
        fs: Arc::new(RealFs),
        module_loader: Rc::new(NoopModuleLoader), // avoid loading any modules
        node_resolver: None,
        npm_resolver: None,
        create_web_worker_cb: Arc::new(|_| unimplemented!("web workers are not supported")),
        format_js_error_fn: Some(Arc::new(format_js_error)),
        source_map_getter: None,
        maybe_inspector_server: None,
        should_break_on_first_statement: false,
        should_wait_for_inspector_session: false,
        strace_ops: None,
        get_error_class_fn: Some(&get_error_class_name),
        cache_storage_dir: None,
        origin_storage_dir: None,
        blob_store: Default::default(),
        broadcast_channel: Default::default(),
        shared_array_buffer_store: None,
        compiled_wasm_module_store: None,
        stdio: Default::default(),
        feature_checker: Default::default(),
        v8_code_cache: None,
    };

    let main_module =
        deno_core::resolve_url_or_path("./$lit$actions.js", Path::new(env!("CARGO_MANIFEST_DIR")))?;
    let mut worker =
        MainWorker::bootstrap_from_options(main_module, PermissionsContainer::new(perms), options);

    {
        let _span = info_span!("LitNamespace.js").entered();

        debug!("Populating LitHeaders: {http_headers:?}");

        // NB: globalThis.LitActions is already part of the V8 snapshot
        let mut code = formatdoc! {r#"
            "use strict";
            (function (actions, auth, headers) {{
                const {{ freeze }} = Object;
                const readOnly = value => ({{ value, enumerable: true, writable: false, configurable: false }});

                Object.defineProperties(globalThis, {{
                    LitActions: readOnly(freeze(actions)),
                    LitAuth: readOnly(freeze(auth)),
                    LitHeaders: readOnly(headers),
                }});

                Object.defineProperty(globalThis, "Lit", readOnly(freeze({{
                    Actions: LitActions,
                    Auth: LitAuth,
                    Headers: LitHeaders,
                }})));
            }})(LitActions, {auth}, new Headers({headers}));
            "#,
            auth = if let Some(ctx) = auth_context {
                serde_json::to_string(ctx).context("Could not serialize auth_context")?
            } else {
                "{}".to_string()
            },
            headers = serde_json::to_string(&http_headers).context("Could not serialize HTTP headers")?
        };

        // Remove LitTest from non-debug builds
        if !cfg!(debug_assertions) {
            code.push_str("delete globalThis.LitTest;\n");
        }

        worker
            .execute_script("LitNamespace.js", code.into())
            .context("Error populating Lit namespace")?;
    }

    {
        let _span = info_span!("DenoNamespace.js").entered();

        let code = formatdoc! {r#"
            "use strict";
            delete Deno.build;
            delete Deno.permissions;
            delete Deno.version;
        "#};

        worker
            .execute_script("DenoNamespace.js", code.into())
            .context("Error patching Deno namespace")?;
    }

    if let Some(params) = globals_to_inject {
        let _span = info_span!("Params.js").entered();

        debug!("Injecting params as globals: {params:?}");

        let _ = params
            .as_object()
            .context("Could not convert params to map")?;

        let code = formatdoc! {r#"
            "use strict";
            Object.assign(globalThis, {params});
        "#};

        worker
            .execute_script("Params.js", code.into())
            .context("Error injecting params as globals")?;
    }

    Ok(worker)
}

// NB: Due to the new PKU feature introduced in V8 11.6, we need to init the V8
// platform on the parent thread that will spawn V8 isolates (in main.rs).
// See https://github.com/denoland/deno/blob/v1.43/cli/main.rs
pub fn init_v8() {
    // Tigthen up V8 security while sacrificing performance
    // To get a list of supported flags: deno run --v8-flags=-help
    let unknown_flags = &deno_core::v8_set_flags(vec![
        "UNUSED_BUT_NECESSARY_ARG0".into(), // See https://github.com/denoland/deno/blob/v1.37/cli/util/v8.rs#L17
        // "--jitless".into(),                 // Disable runtime allocation of executable memory
        "--no-expose-wasm".into(), // Don't expose wasm interface to JavaScript
        // "--no-use-ic".into(),      // Don't use inline caching
        "--disallow-code-generation-from-strings".into(), // Disallow eval and friends
        "--memory-protection-keys".into(), // Protect code memory with PKU if available
        "--clear-free-memory".into(),      // Initialize free memory with 0
    ])[1..];
    assert_eq!(
        unknown_flags,
        Vec::<String>::new(),
        "unknown V8 flags specified"
    );

    JsRuntime::init_platform(None);
}

#[allow(clippy::too_many_arguments)]
#[instrument(skip_all, err)]
pub(crate) async fn execute_js(
    code: String,
    js_params: Option<serde_json::Value>,
    auth_context: Option<serde_json::Value>,
    http_headers: BTreeMap<String, String>,
    timeout_ms: Option<u64>,
    memory_limit_mb: Option<usize>,
    outbound_tx: flume::Sender<tonic::Result<ExecuteJsResponse>>,
    inbound_rx: flume::Receiver<ExecuteJsRequest>,
) -> Result<()> {
    // Don't output ANSI escape sequences, e.g. when formatting JS errors
    static COLOR_INIT: Once = Once::new();
    COLOR_INIT.call_once(|| deno_runtime::colors::set_use_color(false));

    let timeout_ms = timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);
    let memory_limit_mb = memory_limit_mb.unwrap_or(DEFAULT_MEMORY_LIMIT_MB);

    let mut worker = build_main_worker_and_inject_sdk(
        &js_params,
        &auth_context,
        http_headers,
        Some(memory_limit_mb),
    )
    .context("Error building main worker")
    .map_err(|e| anyhow!("{e:#}"))?; // Ensure to keep context when downcasting JS errors later

    let op_state = worker.js_runtime.op_state();
    {
        // scope the borrow
        let mut state = op_state.borrow_mut();
        state.put(outbound_tx);
        state.put(inbound_rx);
        drop(state);
    }

    let (halt_isolate_tx, mut halt_isolate_rx) = oneshot::channel::<ExecutionResult>();
    let (memory_limit_tx, memory_limit_rx) = mpsc::unbounded_channel::<usize>();

    start_controller_thread(
        &mut worker.js_runtime,
        timeout_ms,
        memory_limit_rx,
        halt_isolate_tx,
    );

    // Terminate isolate when approaching memory limit
    worker
        .js_runtime
        .add_near_heap_limit_callback(move |current_limit, _initial_limit| {
            let _ = memory_limit_tx.send(current_limit);
            current_limit * 2
        });

    if let Err(e) = worker
        .js_runtime
        .execute_script("<user_provided_script>", code)
    {
        // Delay error handling if the controller has terminated the isolate,
        // in which case halt_isolate_rx will tell the reason.
        if e.to_string() != EXECUTION_TERMINATED_ERROR {
            bail!(e);
        }
    };

    loop {
        tokio::select! {
            biased;

            execution_result = &mut halt_isolate_rx => {
                break match execution_result {
                    Ok(ExecutionResult::Complete) => Ok(()),
                    Ok(ExecutionResult::Timeout) => bail!(Status::deadline_exceeded(format!("Your function exceeded the maximum runtime of {timeout_ms}ms and was terminated."))),
                    Ok(ExecutionResult::OutOfMemory) => bail!(Status::resource_exhausted(format!("Your function exceeded the maximum memory of {memory_limit_mb} MB and was terminated."))),
                    Err(e) => Err(e),
                }
            }
            event_loop_result = worker.run_event_loop(false) => {
                debug!("Event loop has completed: {event_loop_result:?}");
                match event_loop_result {
                    Ok(()) => break Ok(()),
                    // Despite biased polling, the event loop may still complete first,
                    // e.g. when a promise resolves after 3s and the timeout is also 3s.
                    Err(e) if e.to_string() != EXECUTION_TERMINATED_ERROR => bail!(e),
                    Err(_) => {} // continue loop to wait for halt_isolate_rx
                }
            }
        }
    }?;

    Ok(())
}

fn start_controller_thread(
    js_runtime: &mut JsRuntime,
    worker_timeout_ms: u64,
    mut memory_limit_rx: mpsc::UnboundedReceiver<usize>,
    halt_isolate_tx: oneshot::Sender<ExecutionResult>,
) -> thread::JoinHandle<ExecutionResult> {
    let isolate_handle = js_runtime.v8_isolate().thread_safe_handle();

    thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("create tokio runtime for controller thread");

        let res = runtime.block_on(async move {
            tokio::select! {
                _ = tokio::time::sleep(Duration::from_millis(worker_timeout_ms)) => {
                    if isolate_handle.terminate_execution() {
                        debug!("Terminated isolate after timeout of {worker_timeout_ms}ms");
                        ExecutionResult::Timeout
                    } else {
                        // Isolate already terminated
                        ExecutionResult::Complete
                    }
                }
                limit = memory_limit_rx.recv() => {
                    if let Some(limit) = limit {
                        if isolate_handle.terminate_execution() {
                            debug!("Terminated isolate after reaching memory limit of {limit} bytes");
                            ExecutionResult::OutOfMemory
                        } else {
                            // Isolate already terminated
                            ExecutionResult::Complete
                        }
                    } else {
                        // Sender dropped before exceeding limits, stop controller thread
                        ExecutionResult::Complete
                    }
                }
            }
        });

        // Ignore error of receiver being dropped
        let _ = halt_isolate_tx.send(res);

        debug!("start_controller_thread: {res:?}");

        res
    })
}
