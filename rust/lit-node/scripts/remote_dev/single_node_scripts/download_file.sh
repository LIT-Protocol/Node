#!/bin/bash

source ./scripts/remote_dev/vars.sh

# if dest directory doesn't exist, create it
if [ ! -d "$3/$1" ]; then
  mkdir -p "$3/$1"
fi

scp $1:$2 "$3/$1"

echo "Done downloading file from $1"