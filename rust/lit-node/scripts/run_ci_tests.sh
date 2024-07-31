#!/bin/bash

# exit if any command fails
set -e

SCRIPTS_DIR=$(dirname "$0")

cargo fmt -- --check
./${SCRIPTS_DIR}/run_clippy.sh
RUST_LOG=warn,test=trace cargo nextest run --final-status-level pass --lib --bin lit_node --nocapture -- 
RUST_LOG=warn,test=trace cargo nextest run --final-status-level pass --test test --nocapture --

echo "All tests passed!"
