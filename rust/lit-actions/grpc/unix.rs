use std::os::unix::fs::PermissionsExt;
use std::time::Duration;
use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use hyper_util::rt::TokioIo;
use tokio::net::{UnixListener, UnixStream};
use tokio_stream::wrappers::UnixListenerStream;
use tonic::body::BoxBody;
use tonic::transport::{Channel, Endpoint, Server, Uri};

pub async fn connect_to_socket(socket_path: impl Into<PathBuf>) -> Result<Channel> {
    const IGNORED_URI: &str = "http://[::]:50051";

    // Take a copy before moving into the `service_fn` closure so that the
    // closure can implement `FnMut`.
    let path = socket_path.into();

    // NB: .timeout() doesn't work here for some reason, which is why we use tokio::time::timeout in the client
    Endpoint::from_static(IGNORED_URI)
        .connect_timeout(Duration::from_secs(1))
        .connect_with_connector(tower::service_fn(move |_: Uri| {
            let path = path.clone();
            async move { Ok::<_, std::io::Error>(TokioIo::new(UnixStream::connect(path).await?)) }
        }))
        .await
        .map_err(Into::into)
}

pub async fn start_server<S, P, F>(
    service: S,
    socket_path: P,
    shutdown_signal: Option<F>,
) -> Result<()>
where
    S: tower::Service<
            http::Request<BoxBody>,
            Response = http::Response<BoxBody>,
            Error = std::convert::Infallible,
        > + tonic::server::NamedService
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
    P: Into<PathBuf>,
    F: std::future::Future<Output = ()>,
{
    let socket_path = socket_path.into();
    if socket_path.exists() {
        // If the file is a symlink, e.g. from /var/run to somewhere else, remove the target file
        if let Ok(target_path) = fs::read_link(&socket_path) {
            fs::remove_file(target_path)
        } else {
            fs::remove_file(&socket_path)
        }
        .context("Failed to remove existing socket file")?;
    }

    let uds = UnixListener::bind(socket_path.clone())?;

    // set permissions on socket to 777 so that the lit-node user can talk to it
    // this is safe - the lit actions runner has no secrets in it and an unauthorized user
    // could only run JS code on it.
    fs::set_permissions(&socket_path, fs::Permissions::from_mode(0o777))?;

    let uds_stream = UnixListenerStream::new(uds);

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(crate::proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    let router = Server::builder()
        .add_service(reflection)
        .add_service(service);

    if let Some(sig) = shutdown_signal {
        router.serve_with_incoming_shutdown(uds_stream, sig).await
    } else {
        router.serve_with_incoming(uds_stream).await
    }?;

    Ok(())
}
