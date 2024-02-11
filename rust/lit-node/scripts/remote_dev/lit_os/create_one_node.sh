#!/bin/bash

source ./scripts/remote_dev/vars.sh

node_index="$1"
toml_path="../../blockchain/contracts/node_configs"

if [ -z "$node_index" ]; then
    echo "Usage: $0 <node index>"
    echo "Error: You must pass node index"
    exit
fi

file="$toml_path/lit_config$node_index.toml"
if [ -f "$file" ]; then
    echo "Creating node $node_index with file $file"
    export host_ip=${HOSTS[$node_index]}
    export ip_address=${GUESTS_WITH_SM[$node_index]}
    export gw=${GUEST_GATEWAYS[$node_index]}
    export subnet_id=`stoml "$file" subnet.id`
    export staker_address=`stoml "$file" node.staker_address`
    export wallet_key=`stoml "$file" blockchain.wallet.default.private_key`
    export coms_sender_key=`stoml "$file" node.coms_keys_sender_privkey`
    export coms_receiver_key=`stoml "$file" node.coms_keys_receiver_privkey`
    ./scripts/remote_dev/lit_os/expect/destroy_then_create_node.exp "$host_ip" "$ip_address" "$gw" "$subnet_id" "$staker_address" "$wallet_key" "$coms_sender_key" "$coms_receiver_key" &
fi


# set host_ip [lindex $argv 0]
# set ip_address [lindex $argv 1]
# set gw [lindex $argv 2]
# set subnet_id [lindex $argv 3]
# set staker_address [lindex $argv 4]
# set wallet_key [lindex $argv 5]
# set coms_sender_key [lindex $argv 6]
# set coms_receiver_key [lindex $argv 7]