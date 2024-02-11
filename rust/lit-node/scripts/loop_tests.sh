#!/bin/bash

# this script will just run teh integration tests over and over forever until you kill it
# it's useful for debugging tests that hang or fail intermittently
export RUST_LOG="lit_node=trace,rocket::server=warn,_=warn"

while true; do
    echo "*********** Running Test ***********"
     RUST_LOG=integration_tests=trace cargo test --test integration_tests -- --nocapture --test-threads 1  --show-output
done
