#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
SCCACHE_URL="https://github.com/mozilla/sccache/releases/download"
PATH="$SCRIPT_DIR:$PATH"

if [ -z "${LOCAL_CACHE_DIR}" ]; then
  LOCAL_CACHE_DIR="/tmp/.cache"
fi
LOCAL_CACHE_RUST_DIR="${LOCAL_CACHE_DIR}/rust"

set -e

stderr() {
  echo "${@}" >&2
}

die() {
  stderr "ERROR: ${FUNCNAME[1]}: ${@}"
  exit 1
}

highlight() {
  echo -e "\e[1;33m${@}\e[0m"
}

echo_run() {
  echo "$@"
  $@
}

init() {
  highlight "ci-util: init"
  echo "Setting paths"

  mkdir -p "${LOCAL_CACHE_DIR}"
  mkdir -p $HOME/.local/bin
  echo "$SCRIPT_DIR" >> $GITHUB_PATH
  echo "$HOME/.local/bin" >> $GITHUB_PATH
}

install_sccache() {
  highlight "ci-util: install-sccache"

  if [ -z "$SCCACHE_VERSION" ]; then
    SCCACHE_VERSION="v0.3.3"
  fi

  local release_name="sccache-$SCCACHE_VERSION-x86_64-unknown-linux-musl"

  echo "Installing sccache"

  mkdir -p "$LOCAL_CACHE_DIR/sccache-$SCCACHE_VERSION"
  cd "$LOCAL_CACHE_DIR/sccache-$SCCACHE_VERSION"

  if [ ! -e $release_name/sccache ]; then
    curl -L "$SCCACHE_URL/$SCCACHE_VERSION/$release_name.tar.gz" | tar xz
  fi

  cp -f $release_name/sccache $HOME/.local/bin/sccache
}

checkout_build_branch() {
  highlight "ci-util: checkout-build-branch"

  if [ "${GITHUB_REF_NAME}" == "main" ]; then
    echo_run git checkout "${GITHUB_REF_NAME}"
  elif [ "${GITHUB_REF_NAME}" == "develop" ]; then
    echo_run git checkout "${GITHUB_REF_NAME}"
  else
    echo "Trying: git checkout ${GITHUB_REF_NAME}"
    if [ ! "$(git checkout "${GITHUB_REF_NAME}")" ]; then
      echo_run git checkout "develop"
    fi
  fi

  echo_run git config pull.rebase false
  echo_run git pull
}

cache_rust() {
  highlight "ci-util: cache-rust"

  local ws_rust="${GITHUB_WORKSPACE}/rust"

  if [ -d "${LOCAL_CACHE_RUST_DIR}" ]; then
    rm -rf "${LOCAL_CACHE_RUST_DIR}"
  fi
  if [ ! -d "${ws_rust}" ]; then
    die "Expected ${ws_rust} dir"
  fi

  mkdir -p ${LOCAL_CACHE_RUST_DIR}

  cd "${ws_rust}"

  while IFS= read -r -d '' src
  do
    local dst="${LOCAL_CACHE_RUST_DIR}/${src/.\//}"
    mkdir -p "$dst"
    cp -rf $src/* $dst/
  done <   <(find . -type d -name 'target' -maxdepth 2 -print0)
}

restore_rust() {
  highlight "ci-util: restore-rust"

  local ws_rust="${GITHUB_WORKSPACE}/rust"

  if [ ! -d "${LOCAL_CACHE_RUST_DIR}" ]; then
    return
  fi
  if [ ! -d "${ws_rust}" ]; then
    die "Expected ${ws_rust} dir"
  fi

  cd ${LOCAL_CACHE_RUST_DIR}

  while IFS= read -r -d '' src
  do
    local dst="${ws_rust}/${src/.\//}"
    if [ -d "${dst}" ]; then
      rm -rf "${dst}"
    fi
    mkdir -p "$dst"
    cp -rf $src/* $dst/
  done <   <(find . -type d -name 'target' -maxdepth 2 -print0)
}

while [ -n "$1" ]; do
  case "$1" in
  --init)
    init
    ;;
  --install-sccache)
    install_sccache
    ;;
  --checkout-build-branch)
    checkout_build_branch
    ;;
  --cache)
    cache_rust
    ;;
  --restore)
    restore_rust
    ;;
  *)
    stderr "ERROR: Invalid option: $1"
    ;;
  esac

  shift
done

exit 0
