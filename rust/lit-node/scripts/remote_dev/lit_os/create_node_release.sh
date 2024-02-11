#!/bin/bash

source ./scripts/remote_dev/vars.sh

# parse the new subnet id
new_staking_contract_address=$(jq -r '.stakingContractAddress' ../../blockchain/contracts/deployed-lit-node-contracts-temp.json | cut -c 3-)
echo "New staking contract address: $new_staking_contract_address"

# change this as needed, this is set to the internal dev prov host.
prov_ip="198.7.56.184"

# create
./scripts/remote_dev/lit_os/expect/create_node_template_and_release.exp "$prov_ip" "$new_staking_contract_address" "$LIT_DEV_PROV_WALLET_PRIVATE_KEY" "$LIT_DEV_PROV_ADMIN_PRIVATE_KEY"

