#!/bin/bash

# this script assumes we have 10 nodes
NODENUM=3
KEY_COUNT=$1


for (( c=0; c<$KEY_COUNT; c++ ))
do
  echo "Generating key $c of $KEY_COUNT"
  ./scripts/new_pkp.sh $NODENUM 7470 127.0.0.1 mumbai
  sleep 5

done

