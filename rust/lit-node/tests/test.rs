// Please add all tests to this file, and keep them out of the root folder.
// Each root test file creates it's own crate, thus creating lots of warnings about unused code that may be valid.

// these tests emulate calls from a web browser, using our SDK.
pub mod acceptance;
// individual component testing - test a single component in isolation, like the DKG
pub mod component;
// integration tests - test the nodes in full compilation in a local network configuration
pub mod integration;
// sdk tests - downloads the latest SDK & test it against the nodes in full compilation in a local network configuration
pub mod sdk;
// upgrade tests - test the upgrade process
pub mod upgrades;
// fault tests - test the fault tolerance of the network
pub mod toxiproxy;

// external tests - external tests that require a network to be running
#[macro_use]
extern crate rocket;
pub mod external;
