#!/bin/bash

cargo build
cd ../lit-actions
cargo build
cd ..
# now in lit-assets/rust/

./lit-node/scripts/multi-stop.sh

cp lit-node/target/debug/lit_node lit-node/
cp lit-actions/target/debug/lit_actions lit-actions/

./lit-node/scripts/multi-start.sh
