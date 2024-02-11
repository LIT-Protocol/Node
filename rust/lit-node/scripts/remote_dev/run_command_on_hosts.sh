#!/bin/bash

source ./scripts/remote_dev/vars.sh

for i in "${HOSTS[@]}"
do
  echo "Running command $1 on $i"
  ./scripts/remote_dev/single_node_scripts/run_command.sh "$i" "$1" &
done
