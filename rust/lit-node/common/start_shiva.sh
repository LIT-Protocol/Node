#!/bin/bash

downloadArchive () {
  if [ ! -d rust/lit-node/target/debug/lit_node_${1} ]; then
    echo "Downloading manifest at hash: $1"
    ARCHIVE=$(curl -L \
      -H "Accept: application/vnd.github+json" \
      -H "Authorization: Bearer ${GH_PAT}" \
      -H "X-GitHub-Api-Version: 2022-11-28" \
      https://api.github.com/repos/lit-protocol/lit-assets/actions/artifacts?name=lit_node_${1} \
        | jq -r ".artifacts[0].archive_download_url") 

    echo "Downloading binary from archive url: $ARCHIVE"
    curl -L \
      -H "Accept: application/vnd.github+json" \
      -H "Authorization: Bearer ${GH_PAT}" \
      -H "X-GitHub-Api-Version: 2022-11-28" \
      $ARCHIVE > archive.zip

    unzip -o -d rust/lit-node/target/debug/lit_node_${1} archive.zip  
  fi
}

cd blockchain/contracts && npm i
cd ../..

if [ ! -d rust/lit-node/target/debug ]; then
  mkdir -p rust/lit-node/target/debug
fi

if [ ! -d rust/lit-node/tmp/shiva ]; then
  echo "Directory does not exists. Creating temp directory"
  mkdir -p rust/lit-node/tmp/shiva
fi

echo "Downloading builds at hash: $HASH"
downloadArchive $HASH



cd rust/lit-node/tmp/shiva
cp -a /app/. .

cp config/test/deploy-config.json /data/rust/lit-node/config/test
cp config/test/custom_node_runtime_config.toml /data/rust/lit-node/config/test


chmod +x /data/rust/lit-node/target/debug/lit_node_${HASH}/lit-node/common/target/debug/shiva
cp /data/rust/lit-node/target/debug/lit_node_${HASH}/lit-node/common/target/debug/shiva .

cp /data/rust/lit-node/target/debug/lit_node_${HASH}/lit-node/target/debug/lit_node /usr/bin
cp /data/rust/lit-node/target/debug/lit_node_${HASH}/lit-actions/target/debug/lit_actions /usr/bin

chmod +x /usr/bin/lit_node
chmod +x /usr/bin/lit_actions

chmod +x shiva

cd /usr/local/bin && ./anvil --host 0.0.0.0 > anvil.log & 
cd /data/rust/lit-node/tmp/shiva

export IN_GITHUB_CI=1
export IN_CONTAINER=1
export RUST_LOG=DEBUG
./shiva