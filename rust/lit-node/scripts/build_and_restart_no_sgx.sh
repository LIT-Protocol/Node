#!/bin/bash

cargo build --features lit-actions

./scripts/multi-stop.sh

cp target/debug/lit_node .

./scripts/multi-start.sh
