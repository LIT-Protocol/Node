#!/bin/bash

ARCH_LIBDIR=/lib/x86_64-linux-gnu
NODEJS_DIR=/usr/bin
SGX_SIGNER_KEY=/home/ubuntu/.config/gramine/enclave-key.pem

gramine-manifest -Darch_libdir=$ARCH_LIBDIR -Dnodejs_dir=$NODEJS_DIR lit_node.manifest.template lit_node.manifest
echo "manifest generated"
gramine-sgx-sign --key $SGX_SIGNER_KEY --manifest lit_node.manifest -o lit_node.manifest.sgx
echo "manifest signed"
gramine-sgx-get-token --output lit_node.token --sig lit_node.sig
echo "got token.  should be good to go."