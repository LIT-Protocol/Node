#!/bin/bash


NODENUM=$1
INITIAL_PORT=${2:-7470} # this means use default port 7470 if $2 not specified
HOST=${3:-"http://127.0.0.1"} # this means use default protocol http if $3 not specified
CHAIN=${4:-"alfajores"} # this means use default chain alfajores if $4 not specified

# CMDS=()

# for (( c=0; c<$NODENUM; c++ ))
# do
# 	PORT=$(($INITIAL_PORT + $c ))
#   CMDS+=("curl $HOST:$PORT/clear_node")
# done

# concurrently "${CMDS[@]}"

# sleep 0.5

CMDS=()

for (( c=0; c<$NODENUM; c++ ))
do
	PORT=$(($INITIAL_PORT + $c ))
  CMDS+=("curl -d '{\"id\": \"AAH\", \"chain\": \"$CHAIN\", \"key_type\": \"ECDSA\", \"iat\": 0, \"exp\": 0}' -H \"Content-Type: application/json\" -X POST $HOST:$PORT/web/pkp/new")

done

concurrently "${CMDS[@]}"
