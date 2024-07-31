#!/bin/bash

INITIAL_PORT=7470
NODENUM=$1

CMDS=()

for (( c=0; c<$NODENUM; c++ ))
do
	PORT=$(($INITIAL_PORT + $c ))
  CMDS+=("sudo LIT_NODE_PORT=$PORT gramine-sgx lit_node")
done

concurrently "${CMDS[@]}"

# concurrently "LIT_NODE_PORT=7470 cargo run" "LIT_NODE_PORT=7471 cargo run" "LIT_NODE_PORT=7472 cargo run" "LIT_NODE_PORT=7473 cargo run" "LIT_NODE_PORT=7474 cargo run" "LIT_NODE_PORT=7475 cargo run" "LIT_NODE_PORT=7476 cargo run" "LIT_NODE_PORT=7477 cargo run" "LIT_NODE_PORT=7478 cargo run" "LIT_NODE_PORT=7479 cargo run"