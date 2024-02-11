use crate::server::hyper::handler::router::Router;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use hyperlocal::UnixServerExt;
#[allow(unused_imports)]
use log::{as_error, debug, warn};
use sd_notify::NotifyState;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::net::UnixStream;

pub mod handler;

pub async fn bind_unix_socket(socket_path: PathBuf, r: Router) {
    let r = Arc::new(r);

    let make_service = make_service_fn(move |conn: &UnixStream| {
        debug!("conn: {:?}", conn);

        let r = r.clone();

        let service = service_fn(move |req| {
            let r = r.clone();

            #[allow(clippy::redundant_async_block)]
            async move {
                r.route(req).await
            }
        });
        async move { Ok::<_, hyper::Error>(service) }
    });

    let t_socket_path = socket_path.clone();
    thread::spawn(move || {
        for _ in 0..100 {
            if t_socket_path.exists() {
                break;
            }

            thread::sleep(Duration::from_millis(10));
        }

        if t_socket_path.exists() {
            if let Err(e) = sd_notify::notify(true, &[NotifyState::Ready]) {
                warn!(error = as_error!(e); "failed to send systemd notify");
            }
        } else {
            warn!("gave up waiting for socket to appear, not sending systemd notify");
        }
    });

    Server::bind_unix(&socket_path)
        .unwrap_or_else(|_| panic!("Unable to bind to Unix socket: {:?}", &socket_path))
        .serve(make_service)
        .await
        .expect("failed to await on unix socket");
}
