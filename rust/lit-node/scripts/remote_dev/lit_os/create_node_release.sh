#!/bin/bash

source ./scripts/remote_dev/vars.sh

# parse the new subnet id
new_staking_contract_address=$(jq -r '.stakingContractAddress' ../../blockchain/contracts/deployed-lit-node-contracts-temp.json | cut -c 3-)
echo "New staking contract address: $new_staking_contract_address"

# create
./scripts/remote_dev/lit_os/expect/create_node_template_and_release.exp "$PROV_HOST_IP" "$new_staking_contract_address" "$LIT_DEV_PROV_WALLET_PRIVATE_KEY" "$LIT_DEV_PROV_ADMIN_PRIVATE_KEY"

