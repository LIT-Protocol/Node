#!/bin/bash

source ./scripts/remote_dev/vars.sh


scp $2 $1:~/

echo "Done copying file to $1"