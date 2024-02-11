#!/bin/bash

source ./scripts/remote_dev/vars.sh

for i in "${GUESTS[@]}"
do
  echo "Running command $1 on $i"
  ./scripts/remote_dev/single_node_scripts/run_command.sh "$i" "sudo rm -rf /home/*/lit-assets/rust/lit-node/target" &
done
