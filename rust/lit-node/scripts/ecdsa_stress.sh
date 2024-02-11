#!/bin/bash

INITIAL_PORT=7470
NODENUM=$1
TESTSNUM=$2
CMDS=()




for (( t=0; t<$TESTSNUM; t++ ))
do
  for (( c=0; c<$NODENUM; c++ ))
  do
    PORT=$(($INITIAL_PORT + $c ))    
    CMDS+=("curl -d '{\"message\": [76, 76, 76], \"chain\": \"truffle\", \"iat\": 0, \"exp\": 0}' -H \"Content-Type: application/json\" -X POST http://127.0.0.1:$PORT/web/signing/sign_message_ecdsa")
  done
done

concurrently "${CMDS[@]}"

  

# curl -d '{"message": "Lit Protocol rocks $TESTNUM", "chain": "truffle", "iat": 0, "exp": 0}' -H "Content-Type: application/json" -X POST http://127.0.0.1:$PORT/web/signing/sign_message_ecdsa