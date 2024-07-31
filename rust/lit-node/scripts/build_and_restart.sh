#!/bin/bash

cargo build --release

./multi-stop.sh

cp target/release/lit_node .

make clean

SGX=1 make all

./multi-start.sh