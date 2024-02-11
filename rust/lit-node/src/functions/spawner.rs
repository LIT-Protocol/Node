use tokio::runtime::Builder;
use tokio::sync::{mpsc, oneshot};
use tokio::task::LocalSet;
use tracing::instrument;

use crate::auth::auth_material::JsonAuthSig;
use crate::error::{unexpected_err, Result};
use crate::functions::bindings::RustJsComms;
use crate::models::DenoExecutionEnv;

use lit_api_core::context::{with_context, Tracing};

#[derive(Debug)]
pub enum Task {
    ExecuteJs(
        String,                               // code to run
        Option<String>,                       // action_ipfs_id
        JsonAuthSig,                          // auth_sig
        Option<serde_json::Value>,            // js_params
        DenoExecutionEnv,                     // deno_execution_env
        String,                               // request_id
        oneshot::Sender<Result<RustJsComms>>, // response sender
    ),
}

#[derive(Clone)]
pub struct LocalSpawner {
    send: mpsc::UnboundedSender<Task>,
}

impl LocalSpawner {
    #[instrument(name = "LocalSpawner::new", skip_all)]
    pub fn new() -> Self {
        let (send, mut recv) = mpsc::unbounded_channel();

        let runtime = Builder::new_multi_thread()
            .thread_name("node-tasks")
            .enable_all()
            .build()
            .expect("create tokio runtime for deno");

        std::thread::spawn(move || {
            let local = LocalSet::new();

            local.spawn_local(async move {
                while let Some(new_task) = recv.recv().await {
                    debug!("Spawning local");

                    let _handle = tokio::task::spawn_local(run_task(new_task));

                    debug!("All spawning done");
                }
                // If the while loop returns, then all the LocalSpawner
                // objects have have been dropped.
            });

            // This will return once all senders are dropped and all
            // spawned tasks have returned.
            runtime.block_on(local);
        });

        Self { send }
    }

    #[instrument(name = "LocalSpawner::spawn", skip_all)]
    pub fn spawn(&self, task: Task) -> Result<()> {
        self.send
            .send(task)
            .map_err(|e| unexpected_err(e, Some("Thread with LocalSet has shut down.".into())))
    }
}

#[instrument(name = "spawner::run_task", skip_all)]
async fn run_task(task: Task) {
    match task {
        Task::ExecuteJs(
            code_to_run,
            action_ipfs_id,
            auth_sig,
            js_params,
            deno_execution_env,
            request_id,
            response,
        ) => {
            with_context(Tracing::new(request_id.clone()), async move {
                let rust_js_comms_result = crate::functions::execute_js(
                    code_to_run.to_string(),
                    action_ipfs_id,
                    auth_sig,
                    js_params,
                    deno_execution_env,
                    request_id.to_string(),
                    None,
                    None,
                )
                .await
                .map_err(|e| {
                    error!("error in Js comms result: {:?}", e);
                    e
                });

                let _ = response.send(rust_js_comms_result);
            })
            .await;
        }
    }
}
