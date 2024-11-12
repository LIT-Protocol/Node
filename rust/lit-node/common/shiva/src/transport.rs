use std::{
    net::IpAddr,
    thread::{self, JoinHandle},
};

use flume::Sender;
use rocket::routes;
use tokio::runtime::Runtime;
use tracing::info;

use crate::{client::ShivaClient, models::TestNetMessage};

pub enum TransportType {
    HTTP = 0,
}

pub trait Transport {
    fn bind(
        &self,
        runtime: Runtime,
        client: ShivaClient,
        quit_tx: tokio::sync::broadcast::Sender<bool>,
        tnm_tx: flume::Sender<TestNetMessage>,
    ) -> Result<JoinHandle<()>, anyhow::Error>;
}

pub struct HttpTransport {
    pub port: u16,
    pub address: IpAddr,
}

impl Transport for HttpTransport {
    fn bind(
        &self,
        runtime: Runtime,
        client: ShivaClient,
        quit_tx: tokio::sync::broadcast::Sender<bool>,
        tnm_tx: Sender<TestNetMessage>,
    ) -> Result<JoinHandle<()>, anyhow::Error> {
        let port = self.port.clone();
        let addr = self.address.clone();
        info!("Found transport to be HTTP. Binding web server");
        let web_handler = thread::spawn(move || {
            runtime.block_on(async {
                let ws = rocket::build()
                    .mount(
                        "/",
                        routes![
                            crate::routes::create_testnet,
                            crate::routes::delete_testnet,
                            crate::routes::poll_testnet,
                            crate::routes::get_info_testnet,
                            crate::routes::stop_random_node_testnet,
                            crate::routes::stop_random_node_and_wait_testnet,
                            crate::routes::get_testnets,
                            crate::routes::transition_epoch_and_wait
                        ],
                    )
                    .manage(quit_tx.clone())
                    .manage(tnm_tx.clone())
                    .manage(client)
                    .configure(rocket::Config {
                        address: addr,
                        port: port,
                        workers: 1, // Only allow the web server to use a single thread
                        ..rocket::Config::default()
                    });

                let _ = ws.launch().await;
            });
        });

        Ok(web_handler)
    }
}
