#!/bin/bash
#

count=$1
if [ -z "$count" ]; then
  count=100
fi

seed=$(date +%s)
sha512="/usr/bin/sha512sum"
if [ ! -f "${sha512}" ]; then
  sha512="sha3-512sum"
fi

for i in $(seq 0 $count); do
  public_key="04$(echo "${seed}${i}" | $sha512 | awk '{print $1}')"
  key_dir="./node_keys/ecdsa/${public_key:129:1}/${public_key:128:1}/${public_key:127:1}"
  key_file="Key-H-${public_key}-1.cbor"

  mkdir -p "${key_dir}"
  dd if=/dev/urandom bs=1024 count=5 of="${key_dir}/${key_file}" status=none
done
