#!/bin/bash

INITIAL_PORT=7470
NODENUM=$1

CMDS=()

for (( c=0; c<$NODENUM; c++ ))
do
	PORT=$(($INITIAL_PORT + $c ))
  CMDS+=("curl http://127.0.0.1:$PORT/clear_node")
done

concurrently "${CMDS[@]}"

