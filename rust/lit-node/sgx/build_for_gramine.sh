#!/bin/bash

cargo build --release

cp target/release/lit_node .

./sign_sgx.sh