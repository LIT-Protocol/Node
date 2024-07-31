#!/bin/bash

source ./scripts/remote_dev/vars.sh

for i in "${GUESTS_WITH_SM[@]}"
do
  IP_ADDRESS="${i%/*}"
  ./scripts/remote_dev/single_node_scripts/run_command.sh "$IP_ADDRESS" "$1" &
  sleep 1
done
