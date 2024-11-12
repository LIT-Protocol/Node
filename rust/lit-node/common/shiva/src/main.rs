pub mod client;
pub mod manager;
pub mod models;
pub mod routes;
pub mod runtime;
pub mod testnet_instance;
pub mod transport;
use manager::start_runtime;
use test_common::init_test_config;

pub fn main() {
    // setup the logger
    init_test_config();
    // start the runtime
    start_runtime(transport::TransportType::HTTP).unwrap();
}
