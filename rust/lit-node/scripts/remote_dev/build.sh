#!/bin/bash

source ./scripts/remote_dev/vars.sh

for i in "${GUESTS[@]}"
do
  echo "Setting up $i"
  ./scripts/remote_dev/single_node_scripts/copy_and_build_and_run.sh "$i" &
done
