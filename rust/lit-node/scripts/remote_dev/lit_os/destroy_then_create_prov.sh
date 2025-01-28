#!/bin/bash

source ./scripts/remote_dev/vars.sh

# # deploy the contracts
cd ../../blockchain/contracts
npm install
npx ts-node scripts/deploy.ts --deploy-config ../../rust/lit-node/scripts/remote_dev/lit_os/deploy-config.json

# parse the old subnet id
networks_path="../../../networks/internal-dev/deployed-lit-node-contracts-temp.json"
old_staking_contract_address=$(jq -r '.stakingContractAddress' $networks_path | cut -c 3-)
echo "Old staking contract address: $old_staking_contract_address"

# parse the new subnet id
new_staking_contract_address=$(jq -r '.stakingContractAddress' deployed-lit-node-contracts-temp.json | cut -c 3-)
echo "New staking contract address: $new_staking_contract_address"

echo "Please go into Cloudflare and replace the load balancer for $old_staking_contract_address with $new_staking_contract_address"

read -e -p "Press enter to continue once you've updated cloudflare"

# copy the contracts to the networks directory
cp deployed-lit-node-contracts-temp.json ../../../networks/internal-dev/
cp deployed-lit-core-contracts-temp.json ../../../networks/internal-dev/

# copy the configs to the Secrets directory
secrets_path="../../../SecretsAndKeysAndPrivateKeys/InternalDev"
cp node_configs/*.toml "$secrets_path/"
cp deployed-lit-node-contracts-temp.json "$secrets_path/"
cp deployed-lit-core-contracts-temp.json "$secrets_path/"
newest_wallet_file=$(ls -t wallets | head -n 1)
echo "Newest wallet file: $newest_wallet_file"
cp wallets/"$newest_wallet_file" "$secrets_path/"

cd ../../rust/lit-node

# destroy
./scripts/remote_dev/lit_os/expect/destroy_prov.exp "$PROV_HOST_IP" "$old_staking_contract_address"

# create template
./scripts/remote_dev/lit_os/expect/create_prov_template.exp "$PROV_HOST_IP"

# create prov instance
./scripts/remote_dev/lit_os/expect/create_prov.exp "$PROV_HOST_IP" "$PROV_GUEST_IP" "$PROV_GUEST_GATEWAY" "$new_staking_contract_address" "$LIT_DEV_PROV_WALLET_PRIVATE_KEY" "$LIT_DEV_PROV_ADMIN_PRIVATE_KEY"

echo "Please commit changes in the networks directory"
