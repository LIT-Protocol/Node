#!/bin/bash

source ./scripts/remote_dev/vars.sh

for i in "${HOSTS[@]}"
do
  echo "Copying file $1 on $i"
  ./scripts/remote_dev/single_node_scripts/copy_file.sh "$i" "$1" &
done
