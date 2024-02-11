use crate::auth::auth_material::JsonAuthSig;
use crate::config::LitNodeConfig;
use crate::error::{memory_limit_err, timeout_err, unexpected_err, Result};
use crate::models;
use lit_api_core::config::LitApiConfig;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tracing::instrument;

pub mod aes;
pub mod bindings;
pub mod js_runtime;
pub mod spawner;
#[cfg(test)]
mod tests;

pub const MAX_FETCH_COUNT: u32 = 15;
pub const MAX_CONTRACT_CALL_COUNT: u32 = 5;
pub const MAX_RESPONSE_LENGTH: usize = 1024 * 100; // 100KB
pub const MAX_SIGN_COUNT: u32 = 5; // 5 signature requests per action execution
pub const MAX_CONSOLE_LOG_LENGTH: usize = 1024 * 100; // 100KB

const DEFAULT_JS_TIMEOUT_MS: u64 = 30000; // 30s
const DEFAULT_JS_MEMORY_LIMIT_MB: usize = 256;

const EXECUTION_TERMINATED_ERROR: &str = "Uncaught Error: execution terminated";

#[derive(Debug, Copy, Clone, PartialEq)]
enum ExecutionResult {
    Complete,
    Timeout,
    OutOfMemory,
}

#[allow(clippy::too_many_arguments)]
#[cfg(feature = "lit-actions")]
#[instrument(name = "execute_js", skip_all)]
pub async fn execute_js(
    code_to_run: String,
    action_ipfs_id: Option<String>,
    auth_sig: JsonAuthSig,
    js_params: Option<serde_json::Value>,
    deno_execution_env: models::DenoExecutionEnv,
    request_id: String,
    timeout_ms: Option<u64>,
    memory_limit_mb: Option<usize>,
) -> Result<bindings::RustJsComms> {
    let auth_context = deno_execution_env.clone().auth_context;

    // we need to add the current ipfs id to the end of the action_ipfs_ids vec
    let mut new_action_ipfs_ids = auth_context.action_ipfs_ids.clone();
    if let Some(action_ipfs_id) = action_ipfs_id.clone() {
        new_action_ipfs_ids.push(action_ipfs_id);
    }
    let auth_context_with_ipfs_ids = models::AuthContext {
        action_ipfs_ids: new_action_ipfs_ids,
        auth_sig_address: auth_context.auth_sig_address,
        auth_method_contexts: auth_context.auth_method_contexts,
        resources: auth_context.resources,
    };

    let timeout_ms = timeout_ms.unwrap_or(DEFAULT_JS_TIMEOUT_MS);
    let memory_limit_mb = memory_limit_mb.unwrap_or(DEFAULT_JS_MEMORY_LIMIT_MB);

    let mut worker = js_runtime::build_main_worker_and_inject_sdk(
        &js_params,
        &auth_context_with_ipfs_ids,
        Some(memory_limit_mb),
    )
    .map_err(|e| unexpected_err(e, Some("Error building main worker".into())))?;

    let op_state = worker.js_runtime.op_state();

    // insert the RustJsComms struct with the node_address prefilled
    let cfg = deno_execution_env.cfg.clone();
    let port = cfg.external_port()?.to_string();
    let domain_name = cfg.api_domain()?;
    let addr = format!("{}:{}", domain_name, port);

    debug!("node_address: {}", addr);

    {
        // scope the borrow
        let mut mutable_op_state = op_state.borrow_mut();
        mutable_op_state.put(bindings::RustJsComms {
            claim_data: HashMap::new(),
            node_address: addr,
            node_port: port,
            signed_data: HashMap::new(),
            decrypted_data: HashMap::new(),
            deno_execution_env,
            lit_action_ipfs_id: action_ipfs_id,
            error: None,
            claim_count: 0,
            fetch_count: 0,
            contract_call_count: 0,
            sign_count: 0,
            auth_sig,
            response: "".to_string(),
            logs: "".to_string(),
            auth_context: auth_context_with_ipfs_ids,
            request_id,
        });
        drop(mutable_op_state);
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
        .execute_script("<user_provided_script>", code_to_run.into())
    {
        // Delay error handling if the controller has terminated the isolate,
        // in which case halt_isolate_rx will tell the reason.
        if e.to_string() != EXECUTION_TERMINATED_ERROR {
            error!("Error running worker.execute_script(): {e:?}");
            return Err(unexpected_err(e, None));
        }
    };

    loop {
        tokio::select! {
            biased;

            execution_result = &mut halt_isolate_rx => {
                break match execution_result {
                    Ok(ExecutionResult::Complete) => Ok(()),
                    Ok(ExecutionResult::Timeout) => Err(timeout_err(format!("Your function exceeded the maximum runtime of {timeout_ms}ms and was terminated."), None)),
                    Ok(ExecutionResult::OutOfMemory) => Err(memory_limit_err(format!("Your function exceeded the maximum memory of {memory_limit_mb} MB and was terminated."), None)),
                    Err(e) => Err(unexpected_err(e, None)),
                }
            }
            event_loop_result = worker.js_runtime.run_event_loop(false) => {
                debug!("Event loop has completed: {event_loop_result:?}");
                match event_loop_result {
                    Ok(_) => break Ok(()),
                    // Despite biased polling, the event loop may still complete first,
                    // e.g. when a promise resolves after 3s and the timeout is also 3s.
                    Err(e) if e.to_string() != EXECUTION_TERMINATED_ERROR => break Err(unexpected_err(e, None)),
                    Err(_) => {} // continue loop to wait for halt_isolate_rx
                }
            }
        }
    }?;

    let rust_js_comms = op_state.borrow_mut().take::<bindings::RustJsComms>();

    Ok(rust_js_comms)
}

fn start_controller_thread(
    js_runtime: &mut deno_runtime::deno_core::JsRuntime,
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
