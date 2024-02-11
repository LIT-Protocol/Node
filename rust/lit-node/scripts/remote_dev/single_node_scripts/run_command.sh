#!/bin/bash

source ./scripts/remote_dev/vars.sh

echo "Running command $2 on $1"

ssh -o StrictHostKeyChecking=accept-new "$1" /bin/bash << EOF
  $2
EOF

echo "Done running command $2 on $1"