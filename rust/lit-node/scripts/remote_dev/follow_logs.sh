#!/bin/bash

source ./scripts/remote_dev/vars.sh

WHICH_NODE_INDEX_TO_FOLLOW=${1:-0} # this means use default node index 0 if $1 not specified

GUEST="${GUESTS_WITH_SM[$WHICH_NODE_INDEX_TO_FOLLOW]}"
IP_ADDRESS="${GUEST%/*}"

ssh $IP_ADDRESS /bin/bash << EOF
  source .profile
  sudo journalctl -f -u lit-node
EOF