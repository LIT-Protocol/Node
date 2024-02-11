#!/bin/bash


source ./scripts/remote_dev/vars.sh

rsync -e "ssh" -avhr --include='**.gitignore' --exclude='/.git' --filter=':- .gitignore' --delete-after ../../ "$1:~/lit-assets"

if [ $? -ne 0 ]
then
  RED='\033[0;31m'
  NC='\033[0m' # No Color
  printf "${RED} Error with rsync!!${NC}\n"
  exit 1
fi

ssh  "$1" /bin/bash << EOF
  source .profile
  export OPENSSL_INCLUDE_DIR=/usr/local/include/openssl
  export OPENSSL_LIB_DIR=/usr/local/lib64
  sudo systemctl stop lit-node
  cd lit-assets/rust/lit-node
  cargo build
  export BINPATH="\$(pwd)/target/debug/lit_node"
  echo "BINPATH: \$BINPATH"
  sudo sed -i "/^ExecStart=/c\ExecStart=\$BINPATH" /etc/systemd/system/lit-node.service
  sudo systemctl daemon-reload
  sudo systemctl start lit-node
EOF

echo "Done building $1"