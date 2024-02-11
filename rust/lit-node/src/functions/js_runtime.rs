use crate::error::{conversion_err, unexpected_err, Result as LitResult};
use crate::functions::bindings;
use crate::models;
use anyhow::{anyhow, Result};
use deno_core::error::AnyError;
use deno_core::url::Url;
use deno_core::Op;
use deno_runtime::deno_core::Extension;
use deno_runtime::deno_core::NoopModuleLoader;
use deno_runtime::deno_core::Snapshot;
use deno_runtime::deno_fs::RealFs;
use deno_runtime::permissions::{Permissions, PermissionsContainer};
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;
use deno_runtime::WorkerLogLevel;
use lit_core::error::Unexpected;
use log::error;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use tracing::{info_span, instrument};

static RUNTIME_SNAPSHOT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/RUNTIME_SNAPSHOT.bin"));

fn deno_isolate_init() -> Snapshot {
    debug!("Deno isolate init with snapshots.");
    Snapshot::Static(RUNTIME_SNAPSHOT)
}

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

struct FetchController;

impl deno_runtime::deno_fetch::FetchPermissions for FetchController {
    fn check_net_url(&mut self, _url: &Url, _api_name: &str) -> Result<(), AnyError> {
        Ok(())
    }

    fn check_read(&mut self, _p: &Path, _api_name: &str) -> Result<(), AnyError> {
        // nope, gonna disallow this
        Err(anyhow!("disallowed"))
    }
}

struct WebController;

impl deno_runtime::deno_web::TimersPermission for WebController {
    fn allow_hrtime(&mut self) -> bool {
        false
    }
}

// using the worker built into deno
#[instrument(name = "build_main_worker_and_inject_sdk", skip_all)]
pub fn build_main_worker_and_inject_sdk(
    globals_to_inject: &Option<serde_json::Value>,
    auth_context: &models::AuthContext,
    memory_limit_mb: Option<usize>,
) -> LitResult<MainWorker> {
    // Build a deno_core::Extension providing custom ops
    let exts = Extension {
        name: "lit",
        ops: vec![
            bindings::op_aes_decrypt::DECL,
            bindings::op_call_child::DECL,
            bindings::op_call_contract::DECL,
            bindings::op_check_conditions::DECL,
            bindings::op_claim_key_identifier::DECL,
            bindings::op_get_latest_nonce::DECL,
            bindings::op_increment_fetch_count::DECL,
            bindings::op_pkp_permissions_get_permitted_auth_method_scopes::DECL,
            bindings::op_pkp_permissions_get_permitted::DECL,
            bindings::op_pkp_permissions_is_permitted_auth_method::DECL,
            bindings::op_pkp_permissions_is_permitted::DECL,
            bindings::op_pubkey_to_token_id::DECL,
            bindings::op_set_response::DECL,
            bindings::op_sign_ecdsa_eth_personal_sign_message::DECL,
            bindings::op_sign_ecdsa::DECL,
        ]
        .into(),
        // NOTE: middleware only seems to work with op2 bindings
        middleware_fn: Some(Box::new(|op| match op.name {
            "op_print" => bindings::op_print::DECL,
            _ => op,
        })),
        ..Default::default()
    };

    let perms = Permissions {
        read: Permissions::new_read(&None, &None, false)
            .map_err(|e| unexpected_err(e, Some("Unable to create new read permissions".into())))?,
        write: Permissions::new_write(&None, &None, false).map_err(|e| {
            unexpected_err(e, Some("Unable to create new write permissions".into()))
        })?,
        net: Permissions::new_net(&Some(vec![]), &None, false)
            .map_err(|e| unexpected_err(e, Some("Unable to create new net permissions".into())))?, // network access
        env: Permissions::new_env(&None, &None, false)
            .map_err(|e| unexpected_err(e, Some("Unable to create new env permissions".into())))?,
        sys: Permissions::new_sys(&None, &None, false)
            .map_err(|e| unexpected_err(e, Some("Unable to create new sys permissions".into())))?,
        run: Permissions::new_run(&None, &None, false)
            .map_err(|e| unexpected_err(e, Some("Unable to create new run permissions".into())))?,
        ffi: Permissions::new_ffi(&None, &None, false)
            .map_err(|e| unexpected_err(e, Some("Unable to create new ffi permissions".into())))?,
        hrtime: Permissions::new_hrtime(false, false),
    };

    // someday, we can implement this to make it so that the user can load JS modules
    // let module_loader = Rc::new(module_loader::EmbeddedModuleLoader {
    //     eszip: eszip::EszipV2::default(),
    //     maybe_import_map_resolver: None,
    // });

    let options = WorkerOptions {
        bootstrap: BootstrapOptions {
            args: vec![],
            cpu_count: 1,
            enable_testing_features: false,
            locale: "en".to_string(),
            location: None,
            log_level: WorkerLogLevel::Info,
            no_color: false,
            is_tty: false,
            runtime_version: "x".to_string(),
            ts_version: "x".to_string(),
            unstable: false,
            user_agent: "lit_protocol_node".to_string(),
            inspect: false,
            has_node_modules_dir: false,
            maybe_binary_npm_command_name: None,
        },
        feature_checker: Default::default(),
        extensions: vec![exts],
        startup_snapshot: Some(deno_isolate_init()),
        create_params: memory_limit_mb.map(|limit| {
            deno_core::v8::CreateParams::default().heap_limits(0, limit * 1024 * 1024)
        }),
        unsafely_ignore_certificate_errors: None,
        root_cert_store_provider: None,
        seed: None,
        fs: Arc::new(RealFs),
        module_loader: Rc::new(NoopModuleLoader), // avoid loading any modules
        npm_resolver: None,
        create_web_worker_cb: Arc::new(|_| unimplemented!("web workers are not supported")),
        format_js_error_fn: None,
        source_map_getter: None,
        maybe_inspector_server: None,
        should_break_on_first_statement: false,
        should_wait_for_inspector_session: false,
        get_error_class_fn: Some(&get_error_class_name),
        cache_storage_dir: None,
        origin_storage_dir: None,
        blob_store: Default::default(),
        broadcast_channel: Default::default(),
        shared_array_buffer_store: None,
        compiled_wasm_module_store: None,
        stdio: Default::default(),
    };

    set_v8_flags();

    let main_module = deno_core::resolve_path("empty.js", Path::new(env!("CARGO_MANIFEST_DIR")))
        .map_err(|e| unexpected_err(e, Some("Error resolving empty.js".into())))?;

    let mut worker =
        MainWorker::bootstrap_from_options(main_module, PermissionsContainer::new(perms), options);

    // inject auth
    {
        let _span = info_span!("AuthContext.js").entered();
        let lit_auth_string = format!(
            "const LitAuth = {:}; Object.freeze(LitAuth);",
            serde_json::to_string(auth_context)
                .map_err(|e| conversion_err(e, Some("Could not serialize auth_context".into())))?,
        );
        let script_execution_result = worker
            .js_runtime
            .execute_script("AuthContext.js", lit_auth_string.into())
            .map_err(|e| {
                unexpected_err(
                    e,
                    Some("Error running worker.execute_script(lit_auth_string)".into()),
                )
            })?;
    }

    // inject final Lit namespace functions
    {
        let _span = info_span!("LitNamespacing.js").entered();
        let lit_actions_namespacing =
            "const Lit = {}; Lit.Actions = LitActions; Lit.Auth = LitAuth; Object.freeze(Lit);";
        let script_execution_result = worker
            .js_runtime
            .execute_script_static("LitNamespacing.js", lit_actions_namespacing)
            .map_err(|e| {
                unexpected_err(
                    e,
                    Some(
                        "Error running worker.execute_script_static(lit_actions_namespacing)"
                            .into(),
                    ),
                )
            })?;
    }

    // inject js_params
    if let Some(params) = globals_to_inject {
        // it should be a map
        let params_map = params
            .as_object()
            .expect_or_err("Could not convert params to map")?;

        // expose the js params as globals by declaring them at the top of the scope as const's
        let params_js_option = params_map
            .into_iter()
            .map(|(k, v)| {
                if k == "Lit" {
                    // this namespace is taken by Lit
                    return "".to_string();
                }

                let value_str_result = match serde_json::to_string(v).map_err(|e| {
                    conversion_err(
                        e,
                        Some("Could not serialize value to string in js_params".into()),
                    )
                }) {
                    Ok(s) => s,
                    Err(e) => {
                        error!("{}", format!("{}", e));
                        return "".to_string();
                    }
                };

                format!("const {} = {};", k, value_str_result)
            })
            .reduce(|acculum, item| format!("{}\n{}", acculum, item));

        if let Some(params_js_code_to_run) = params_js_option {
            let _span = info_span!("ParamsJSCode.js").entered();

            debug!("Running params_js_code_to_run: {}", params_js_code_to_run);
            let script_execution_result = worker
                .js_runtime
                .execute_script("ParamsJSCode.js", params_js_code_to_run.into())
                .map_err(|e| {
                    unexpected_err(
                        e,
                        Some("Error running worker.execute_script(params_js_code_to_run)".into()),
                    )
                })?;
        } else {
            warn!("No params_js_code_to_run so not setting any jsParams globals");
            return Ok(worker);
        }
    }

    Ok(worker)
}

fn set_v8_flags() {
    // Tigthen up V8 security while sacrificing performance
    // To get a list of supported flags: deno run --v8-flags=-help
    let unknown_flags = &deno_core::v8_set_flags(vec![
        "UNUSED_BUT_NECESSARY_ARG0".into(), // See https://github.com/denoland/deno/blob/v1.37/cli/util/v8.rs#L17
        "--jitless".into(),                 // Disable runtime allocation of executable memory
        "--no-expose-wasm".into(),          // Don't expose wasm interface to JavaScript
        "--no-use-ic".into(),               // Don't use inline caching
        "--disallow-code-generation-from-strings".into(), // Disallow eval and friends
        "--write-protect-code-memory".into(), // Write protect code memory
        "--clear-free-memory".into(),       // Initialize free memory with 0
    ])[1..];
    assert_eq!(
        unknown_flags,
        Vec::<String>::new(),
        "unknown V8 flags specified"
    );
}
