#!/bin/bash

t_first_of_month=$(date -d "`date +%Y%m01`" +%Y-%m-%d)

echo "Getting metrics from $t_first_of_month until now..."

echo "Signing condition requests:"
sudo journalctl -u lit-node@0 --since "$t_first_of_month 00:00:00" | grep "signing_retrieve, request:" | wc -l
echo ""
echo "Encryption condition requests:"
sudo journalctl -u lit-node@0 --since "$t_first_of_month 00:00:00" | grep "encryption_retrieve, request:" | wc -l
echo ""
echo "Lit action execution requests:"
sudo journalctl -u lit-node@0 --since "$t_first_of_month 00:00:00" | grep "execute, request:" | wc -l