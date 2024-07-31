#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "You must enter exactly 1 argument - the test name to run"
    exit 1
fi


# exit if any command fails
set -e

RUST_LOG=_=warn,test=trace,lit_node=trace,lit_actions=trace cargo nextest run --features lit-actions,testing --final-status-level pass --test test --nocapture -- $1

echo "The test passed!"
