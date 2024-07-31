#!/bin/bash

source ./scripts/remote_dev/vars.sh

for i in "${GUESTS_WITH_SM[@]}"
do
  echo "Copying file $1 on $i"
  IP_ADDRESS="${i%/*}"
  ./scripts/remote_dev/single_node_scripts/copy_file.sh "$IP_ADDRESS" "$1" &
done
