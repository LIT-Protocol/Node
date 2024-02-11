#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "You must enter exactly 1 argument - the test name to run"
    exit 1
fi

# kill toxiproxy if running
pkill -f toxiproxy-serve

toxiproxy-server &> /dev/null &


# exit if any command fails
set -e

RUST_LOG=_=warn,test=trace,lit_node=trace cargo nextest run --test test --features proxy_http,testing,lit-actions --nocapture -- $1

echo "The test passed!"
