#!/bin/bash

if [ -z "$1" ]
  then
    echo "Usage: ./scripts/deploy_serrano.sh <release-branch>"
    exit 1
fi

ssh serrano /bin/bash << EOF
source .profile
cd lit-assets/rust/lit-node
git pull
git checkout $1
git pull
./scripts/build_and_restart_no_sgx.sh
EOF


