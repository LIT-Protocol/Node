#!/bin/bash

# curl http://127.0.0.1:7470/find_peers &
# curl http://127.0.0.1:7471/find_peers &
# curl http://127.0.0.1:7472/find_peers &
# curl http://127.0.0.1:7473/find_peers &
# curl http://127.0.0.1:7474/find_peers &
# curl http://127.0.0.1:7475/find_peers &
# curl http://127.0.0.1:7476/find_peers &
# curl http://127.0.0.1:7477/find_peers &
# curl http://127.0.0.1:7478/find_peers &
# curl http://127.0.0.1:7479/find_peers &

INITIAL_PORT=7470
NODENUM=$1

CMDS=()

for (( c=0; c<$NODENUM; c++ ))
do
	PORT=$(($INITIAL_PORT + $c ))
  CMDS+=("curl http://127.0.0.1:$PORT/find_peers")
done

concurrently "${CMDS[@]}"