#!/bin/bash

INITIAL_PORT=7470
NODENUM=$1

CMDS=()

for (( c=0; c<$NODENUM; c++ ))
do
	PORT=$(($INITIAL_PORT + $c ))
  CMDS+=("curl http://127.0.0.1:$PORT/clear_node")
done

concurrently "${CMDS[@]}"

sleep 0.5

CMDS=()

for (( c=0; c<$NODENUM; c++ ))
do
	PORT=$(($INITIAL_PORT + $c ))
  CMDS+=("curl -d '{\"id\": \"AAH\", \"chain\": \"hardhat\", \"key_type\": \"ECDSA\", \"iat\": 0, \"exp\": 0}' -H \"Content-Type: application/json\" -X POST http://127.0.0.1:$PORT/web/pkp/new")

done

concurrently "${CMDS[@]}"



    sig: \"0xdc009b92222e2770cf41436a9969b014dd7881229daa3c9b2e425057363b8d2e638a8f065554383ca5f0c68b1af8a96b2f004356bb1af3feb91d14a8e819303f1c\",            
        derived_via: \"web3.eth.personal.sign",
        signed_message: "Hello World",
        address: "0x70997970c51812dc3a010c7d01b50e0d17dc79c8",