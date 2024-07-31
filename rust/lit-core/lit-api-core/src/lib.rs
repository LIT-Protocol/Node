#[macro_use]
#[cfg(feature = "server")]
extern crate rocket;

#[cfg(feature = "client")]
pub mod client;
pub mod config;
#[cfg(feature = "server")]
pub mod context;
pub mod error;
#[cfg(feature = "server")]
pub mod http;
pub mod logging;
pub mod server;

#[cfg(feature = "server")]
pub use http::rocket::engine::Engine;
#[cfg(feature = "server")]
pub use http::rocket::event::Event;
#[cfg(feature = "server")]
pub use http::rocket::launcher::Launcher;
