#!/bin/bash

source ./scripts/remote_dev/vars.sh

toml_path="$1"
default_toml_path="../../blockchain/contracts/node_configs"

if [ -z "$toml_path" ]; then
    echo "Usage: $0 <path to toml files>"
    echo "Using default TOML path since you didn't pass one: $default_toml_path"
    toml_path="$default_toml_path"
fi

if [ ! -d "$toml_path" ]; then
    echo "Error: $toml_path is not a directory"
    exit 1
fi
counter=0
for i in "${HOSTS[@]}"; do
    file="$toml_path/lit_config$counter.toml"
    if [ -f "$file" ]; then
        echo "Creating node $counter with file $file"
        export host_ip="${i%/*}"
        export ip_address=${GUESTS_WITH_SM[$counter]}
        export gw=${GUEST_GATEWAYS[$counter]}
        export subnet_id=`stoml "$file" subnet.id`
        export staker_address=`stoml "$file" node.staker_address`
        export wallet_key=`stoml "$file" blockchain.wallet.default.private_key`
        export coms_sender_key=`stoml "$file" node.coms_keys_sender_privkey`
        export coms_receiver_key=`stoml "$file" node.coms_keys_receiver_privkey`
        ./scripts/remote_dev/lit_os/expect/destroy_then_create_node.exp "$host_ip" "$ip_address" "$gw" "$subnet_id" "$staker_address" "$wallet_key" "$coms_sender_key" "$coms_receiver_key" &
        counter=$((counter+1))
        sleep 30
    fi
done

# set host_ip [lindex $argv 0]
# set ip_address [lindex $argv 1]
# set gw [lindex $argv 2]
# set subnet_id [lindex $argv 3]
# set staker_address [lindex $argv 4]
# set wallet_key [lindex $argv 5]
# set coms_sender_key [lindex $argv 6]
# set coms_receiver_key [lindex $argv 7]