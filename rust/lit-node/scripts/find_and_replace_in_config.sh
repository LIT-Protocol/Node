#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Your must pass 2 parameters: the string to find and the string to replace it with."
    exit 2
fi

cd config
sed -i "s/$1/$2/g" *.toml
