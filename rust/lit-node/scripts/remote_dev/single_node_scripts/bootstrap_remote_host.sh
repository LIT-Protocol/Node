#!/bin/bash

  ssh -o StrictHostKeyChecking=accept-new "$1" /bin/bash << EOF
    source .profile
    sudo systemctl stop lit-node
    curl https://sh.rustup.rs -sSf | sh -s -- -y
EOF

echo "Done bootstrapping $1"