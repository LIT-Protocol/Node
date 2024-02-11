#!/bin/bash

./scripts/remote_dev/run_command_on_guests.sh "sudo sed -i 's/enable_rate_limiting = false/enable_rate_limiting = true/g' /etc/lit/node.config.toml"

sleep 10

./scripts/remote_dev/run_command_on_guests.sh "sudo systemctl restart lit-node"