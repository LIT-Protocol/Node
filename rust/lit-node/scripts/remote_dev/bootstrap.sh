#!/bin/bash

source ./scripts/remote_dev/vars.sh

for i in "${GUESTS[@]}"
do
  echo "Setting up $i"
  ./scripts/remote_dev/single_node_scripts/bootstrap_remote_host.sh "$i" &
done
