# This script is expected to be kept the same as the github clippy check in .github/workflows/rust-lit-node-unit-tests.yml
cargo clippy --version
cargo clippy --all-features -- -Dwarnings -Dclippy::unwrap_used -Aclippy::ptr_arg
