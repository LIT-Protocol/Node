#!/bin/bash

# sleep 10

# curl http://127.0.0.1:7470/init &
# curl http://127.0.0.1:7471/init &
# curl http://127.0.0.1:7472/init &
# curl http://127.0.0.1:7473/init &
# curl http://127.0.0.1:7474/init &
# curl http://127.0.0.1:7475/init &
# curl http://127.0.0.1:7476/init &
# curl http://127.0.0.1:7477/init &
# curl http://127.0.0.1:7478/init &
# curl http://127.0.0.1:7479/init &


INITIAL_PORT=7470
NODENUM=$1

CMDS=()

for (( c=0; c<$NODENUM; c++ ))
do
	PORT=$(($INITIAL_PORT + $c ))
  CMDS+=("curl http://127.0.0.1:$PORT/refresh")
done

concurrently "${CMDS[@]}"
