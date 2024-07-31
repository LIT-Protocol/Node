#![allow(unused_variables)]
#[macro_use]
pub extern crate rocket;

// pub mod beavermanager;
pub mod config;
pub mod endpoints;
pub mod models;
pub mod peers;
pub mod siwe_db;
pub mod version;

pub mod access_control;
#[allow(dead_code)]
pub mod auth;
pub mod constants;
pub mod contracts;
#[cfg(feature = "lit-actions")]
pub mod functions;
pub mod jwt;
pub mod networking;
pub mod node_state;
// pub mod peerreviewer;
pub mod p2p_comms;
pub mod pkp;
pub mod rate_limiting;
pub mod tss;
pub mod utils {
    pub mod attestation;
    pub mod chain;
    pub mod consensus;
    pub mod contract;
    pub mod cose_keys;
    pub mod encoding;
    pub mod future;
    pub mod networking;
    pub mod rocket;
    pub mod serde_encrypt;
    pub mod siwe;
    #[allow(dead_code)]
    pub mod web;
}
pub mod error;
pub mod services;
pub mod tasks;

#[cfg(test)]
mod tests;
