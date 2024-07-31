pub mod grpc_transmissions;
pub mod internal;
pub mod models;

use rocket::Route;
#[allow(dead_code)]
pub(crate) fn routes() -> Vec<Route> {
    routes![internal::connect]
}
