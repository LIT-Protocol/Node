#!/bin/bash

if ! command -v yq &> /dev/null; then
    echo "Error: yq is not installed. Please install yq to proceed."
    exit 1
fi

cargo build
cd ../lit-actions
cargo build
cd ..
# now in lit-assets/rust/

./lit-node/scripts/multi-stop.sh

# Update the rpc-config.yaml file
if [ -f "lit-node/rpc-config.yaml" ]; then
    echo "rpc-config.yaml file has $(yq '.chains | length' lit-node/rpc-config.yaml) network definitions (pre-update)"
else
    echo "rpc-config.yaml file does not exist, creating it from rpc-config.example.yaml"
fi

# Check if there's an overlay file and merge on top of the example to create the final rpc-config.yaml file
if [ -f "lit-node/rpc-config.overlay.yaml" ]; then
    echo "Found rpc-config.overlay.yaml, merging with base configuration..."
    yq eval-all 'select(fileIndex == 0) * select(fileIndex == 1)' lit-node/rpc-config.example.yaml lit-node/rpc-config.overlay.yaml > lit-node/rpc-config.yaml
    echo "Configuration overlay applied successfully."
else
    echo "No overlay file found, using base configuration."
    cp lit-node/rpc-config.example.yaml lit-node/rpc-config.yaml
fi
echo "rpc-config.yaml file has $(yq '.chains | length' lit-node/rpc-config.yaml) network definitions (post-update)"

cp lit-node/target/debug/lit_node lit-node/
cp lit-actions/target/debug/lit_actions lit-actions/

./lit-node/scripts/multi-start.sh
