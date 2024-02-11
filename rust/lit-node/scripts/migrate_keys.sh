#!/bin/bash
#

set -e

function migrate_keys() {
  while read file
  do
    public_key=$(basename $(dirname "$file"))

    if [ "${#public_key}" != 130 ]; then
      continue
    fi

    key_file="Key-H-$public_key-$(basename $file | sed -e "s/^Key-//")"
    key_dir="./node_keys/ecdsa/${public_key:129:1}/${public_key:128:1}/${public_key:127:1}"

    mkdir -p "${key_dir}" && mv "$file" "${key_dir}/${key_file}" && rm -rf "$(dirname "$file")"

    echo "Migrated: $public_key"
  done < /dev/stdin
}

find node_keys -regex '.*/Key-[0-9]*.cbor' | migrate_keys
