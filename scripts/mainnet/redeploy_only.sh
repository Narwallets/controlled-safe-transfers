#!/bin/bash
__dir=$(dirname "$0")
. $__dir/mainnet-set-vars.sh


# Redeploy Contract
echo Re-DEPLOY ONLY
NEAR_ENV=mainnet near deploy $CONTRACT_ADDRESS $CONTRACT_WASM
