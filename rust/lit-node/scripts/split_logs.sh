#!/bin/bash

# Check if there's no argument provided
if [ "$#" -ne 1 ]; then
    echo "Error: You must provide the path to the log file."
    echo "Usage: $0 <path_to_log_file>"
    exit 1
fi

LOGFILE="$1"

# Ensure the log file exists
if [ ! -f "$LOGFILE" ]; then
    echo "Error: Log file '$LOGFILE' not found."
    exit 1
fi

# Split the log file based on ports
for port in {7470..7479}; do
    grep "\"port\":$port }" "$LOGFILE" > "log_port_$port.log"
done

echo "Logs have been split into separate files."
