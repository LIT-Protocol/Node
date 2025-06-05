use anyhow::{Context as _, Result};
use clap::Parser;
use lit_core::config::{envs::LitEnv, LitConfig};
use lit_core::utils::unix::raise_fd_limit;
use tracing::{debug, info};

#[derive(Debug, Parser)]
struct Args {
    #[arg(
        short,
        long,
        default_value = "/tmp/lit_actions.sock",
        help = "Path to Unix domain socket used by gRPC server"
    )]
    socket: std::path::PathBuf,

    #[arg(
        short,
        long,
        default_value_t = LitEnv::Dev,
        help = "Lit environment"
    )]
    env: LitEnv,
}

#[tokio::main]
async fn main() -> Result<()> {
    raise_fd_limit();

    let args = Args::parse();
    debug!("Lit Actions Launching with args: {:?}", args);

    let cfg = lit_config(args.env)?;
    init_logging(&cfg)?;

    lit_actions_server::init_v8();

    info!("Listening on {:?}", args.socket);
    let signal = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    lit_actions_server::start_server(args.socket, Some(signal)).await
}

fn lit_config(env: LitEnv) -> Result<LitConfig> {
    use lit_api_core::config::LitApiConfig;
    use lit_core::config::LitConfigBuilder;
    use lit_logging::config::LitLoggingConfig;

    let mut builder = LitConfigBuilder::default().set_default("lit.env", env.to_string());
    builder = <LitConfig as LitLoggingConfig>::apply_defaults(builder)?;
    let cfg = <LitConfig as LitApiConfig>::from_builder(builder)?;
    Ok(cfg)
}

fn init_logging(cfg: &LitConfig) -> Result<()> {
    use lit_api_core::logging::TracingPlugin;

    lit_logging::builder(cfg, env!("CARGO_BIN_NAME"))
        .plugin(TracingPlugin::new())
        .init()
        .context("failed to init logging")
}
