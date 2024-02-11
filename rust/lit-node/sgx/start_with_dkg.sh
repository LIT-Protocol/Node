#!/bin/bash

NODENUM=$1

./build.sh

./launch_nodes.sh $NODENUM &
sleep 10
./find_node_peers.sh $NODENUM
sleep 5
./dkg.sh $NODENUM