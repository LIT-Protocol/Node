#!/bin/bash

source ./scripts/remote_dev/vars.sh

for i in "${GUESTS[@]}"
do
  echo "Downloading file $3 on $i:$2"
  ./scripts/remote_dev/single_node_scripts/download_file.sh "$1/$i" "$2" "$3" &
done
