#!/bin/bash

source ./scripts/remote_dev/vars.sh

for i in "${HOSTS[@]}"
do
  IP_ADDRESS="${i%/*}"
  (./scripts/remote_dev/single_node_scripts/run_command.sh "$IP_ADDRESS" "$1" | sed -e "s/^/$IP_ADDRESS: /;") &
done
