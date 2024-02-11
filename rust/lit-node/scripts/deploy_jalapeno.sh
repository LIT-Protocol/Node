#!/bin/bash

ssh node2 /bin/bash << EOF
source .profile
cd lit_node_rust
git pull
./build_and_restart.sh
EOF