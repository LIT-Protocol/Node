#!/bin/bash

source ./scripts/remote_dev/vars.sh

for i in "${GUESTS_WITH_SM[@]}"
do
  IP_ADDRESS="${i%/*}"
  echo "Downloading file $3 on $IP_ADDRESS"
  ./scripts/remote_dev/single_node_scripts/download_file.sh "$1/$IP_ADDRESS" "$2" "$3" &
done
