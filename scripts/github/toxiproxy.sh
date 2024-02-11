#!/bin/bash

set -e

stderr() {
  echo "${@}" >&2
}

install_and_start_bg() {
    wget -O toxiproxy-server-2.5.0 https://github.com/Shopify/toxiproxy/releases/download/v2.5.0/toxiproxy-server-linux-amd64
    chmod +x toxiproxy-server-2.5.0
    ./toxiproxy-server-2.5.0 &
}

while [ -n "$1" ]; do
  case "$1" in
  --install-and-start-bg)
    install_and_start_bg
    ;;
  *)
    stderr "ERROR: Invalid option: $1"
    ;;
  esac

  shift
done

exit 0
