#!/bin/bash

source ./scripts/remote_dev/vars.sh

for i in "${GUESTS_WITH_SM[@]}"
do
  IP_ADDRESS="${i%/*}"
  echo "Running command $1 on $IP_ADDRESS"
  ./scripts/remote_dev/single_node_scripts/run_command.sh "$IP_ADDRESS" "sudo rm -rf /home/*/lit-assets/rust/lit-node/target" &
done
