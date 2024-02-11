#!/bin/bash

cargo build --release

./multi-stop.sh

cp target/release/lit_node .

./sign_sgx.sh

./multi-start.sh