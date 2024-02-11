#!/bin/bash

cd ..
echo "Signing condition count:"
sqlite3 node_7470.db "select count(*) from signing_conditions;"
echo ""
echo "Encryption condition count:"
sqlite3 node_7470.db "select count(*) from encryption_conditions;"
