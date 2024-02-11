#!/bin/bash


highestBlockFromSequencer=$(curl -X POST -H 'Content-Type: application/json' --data '{"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["latest", false],"id":1}' https://chain-rpc.litprotocol.com/http | jq -r '.result.number' | sed 's/0x//' | tr '[:lower:]' '[:upper:]' | xargs -I {} echo "obase=10; ibase=16; {}" | bc | awk '{printf "%'"'"'.0f\n", $0}')

currentSyncedBlock=$(curl -X POST -H 'Content-Type: application/json' --data '{"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["latest", false],"id":1}' http://localhost:8549 | jq -r '.result.number' | sed 's/0x//'  | tr '[:lower:]' '[:upper:]' | xargs -I {} echo "obase=10; ibase=16; {}" | bc | awk '{printf "%'"'"'.0f\n", $0}')

echo "Highest block from sequencer: $highestBlockFromSequencer"
echo "Current synced block: $currentSyncedBlock"