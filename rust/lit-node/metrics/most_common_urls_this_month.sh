#!/bin/bash

t_first_of_month=$(date -d "`date +%Y%m01`" +%Y-%m-%d)

echo "Getting metrics from $t_first_of_month until now..."

echo "This will probably take like 5 mins to run.  Go get a cup of coffee."
sudo journalctl -u lit-node@0 --since "$t_first_of_month 00:00:00" | egrep -o 'URI: [^\\]*'| sort | uniq -c | sort -nr