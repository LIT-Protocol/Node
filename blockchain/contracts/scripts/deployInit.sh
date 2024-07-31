#!/bin/bash

set -e

export LIT_ENV="$1"
export RESOLVER_CONTRACT_ADDRESS="$2"
export NETWORK="$3"

export SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
export LIT_OS_DIR="${SCRIPT_DIR}/lit-os"

if [ -z "${LIT_ENV}" ]; then
  echo "Usage: $0 <dev|staging|prod>"
  exit 2
fi

if [ "${LIT_ENV}" != "dev" ] && [ "${LIT_ENV}" != "staging" ] && [ "${LIT_ENV}" != "prod" ]; then
  echo "Invalid environment (valid: dev, staging, prod)"
  exit 2
fi

if [ -z "${NETWORK}" ]; then
  NETWORK="mumbai"
fi

if [ "${NETWORK}" == "localchain" ]; then
  if [ "$(pidof anvil)" != "" ]; then
    echo "Killing anvil..."
    kill $(pidof anvil)
  fi

  anvil &
fi

if [ -z "${RESOLVER_CONTRACT_ADDRESS}" ]; then
  if [ -z "${LIT_RESOLVER_CONTRACT_ADDRESS}" ]; then
    # deploy the resolver etc.
    ./scripts/deployLitOs.sh "${LIT_ENV}" "${NETWORK}"

    export RESOLVER_CONTRACT_ADDRESS=$(cat deployed-contracts-temp.json | jq -r ".contractResolver")
  else
    # Default from LIT_RESOLVER_CONTRACT_ADDRESS
    export RESOLVER_CONTRACT_ADDRESS="${LIT_RESOLVER_CONTRACT_ADDRESS}"
  fi
fi
