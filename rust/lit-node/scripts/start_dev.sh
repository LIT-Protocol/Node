#!/bin/bash

INITIAL_PORT=7470
NODENUM=${1:-10} # this means use default 10 nodes if $1 not specified

CMDS=()

pkill -9 lit_node

for (( c=0; c<$NODENUM; c++ ))
do
  # uncomment this if statement if you want to run jaeger on the first node
  # if [[ $c -eq 0 ]]
  # then
  #   CMDS+=("LIT_LOGGING_JAEGER=1 RUST_LOG=warn,lit_node=trace LIT_CONFIG_FILE=./config/lit_config$c.toml cargo run --features lit-actions")
  # else
  #   CMDS+=("RUST_LOG=warn,lit_node=trace LIT_CONFIG_FILE=./config/lit_config$c.toml cargo run --features lit-actions")
  # fi
  CMDS+=("RUST_LOG=warn,lit_node=trace,lit_blockchain=trace LIT_CONFIG_FILE=./config/lit_config$c.toml cargo run --features lit-actions-server")
done

concurrently --kill-others "${CMDS[@]}"   | grep "\[0\] "
