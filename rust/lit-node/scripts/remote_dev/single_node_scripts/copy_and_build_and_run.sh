#!/bin/bash

source ./scripts/remote_dev/vars.sh

rsync -e "ssh -o StrictHostKeyChecking=no" -avhr --include='**.gitignore' --exclude='/.git' --filter=':- .gitignore' --delete-after ../../ "$1:~/lit-assets"

if [ $? -ne 0 ]; then
	RED='\033[0;31m'
	NC='\033[0m' # No Color
	printf "${RED} Error with rsync!!${NC}\n"
	exit 1
fi

ssh "$1" /bin/bash <<EOF
  source .profile
  sudo systemctl stop lit-node
  sudo systemctl stop lit-actions
  sudo systemctl stop lit-attestation
  sudo systemctl stop lit-logging

  cd lit-assets/rust/lit-node
  cargo build
  export BINPATH="\$(pwd)/target/debug/lit_node"
  echo "lit_node BINPATH: \$BINPATH"
  sudo sed -i "/^ExecStart=/c\ExecStart=\$BINPATH" /etc/systemd/system/lit-node.service

  cd ../lit-actions
  cargo build
  export BINPATH="\$(pwd)/target/debug/lit_actions"
  echo "lit_actions BINPATH: \$BINPATH"
  sudo sed -i "/^ExecStart=/c\ExecStart=\$BINPATH" /etc/systemd/system/lit-actions.service

  cd ../lit-os/lit-attestation-service
  cargo build --release
  cd .. # pwd is now lit-os
  export BINPATH="\$(pwd)/target/release/lit-attestation-service"
  echo "lit-attestation-service BINPATH: \$BINPATH"
  sudo sed -i "/^ExecStart=/c\ExecStart=\$BINPATH" /etc/systemd/system/lit-attestation.service

  cd lit-logging-service
  cargo build --release
  cd .. # pwd is now lit-os
  export BINPATH="\$(pwd)/target/release/lit-logging-service"
  echo "lit-logging-service BINPATH: \$BINPATH"
  sudo sed -i "/^ExecStart=/c\ExecStart=\$BINPATH" /etc/systemd/system/lit-logging.service

  sudo systemctl daemon-reload
  sudo systemctl start lit-logging
  sleep 5 # can't start node until logging is running...
  sudo systemctl start lit-attestation
  sudo systemctl start lit-node
  sudo systemctl start lit-actions
EOF

echo "Done building $1"
