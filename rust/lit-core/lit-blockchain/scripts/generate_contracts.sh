#!/bin/bash

mkdir -p src/contracts

cd ../lit-contracts-minimal-generator

cargo run --bin generate_contracts
cd ../../lit-core/lit-blockchain
cargo fmt
cd ../../lit-node
cargo fmt