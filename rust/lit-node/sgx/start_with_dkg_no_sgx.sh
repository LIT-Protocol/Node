#!/bin/bash

NODENUM=$1

cargo build
./start_dev.sh $NODENUM &
sleep 10
./find_node_peers.sh $NODENUM
sleep 5
./dkg.sh $NODENUM