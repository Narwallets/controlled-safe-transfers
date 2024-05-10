#!/bin/bash
__dir=$(dirname "$0")
. $__dir/set-vars.sh


# Redeploy Contract
echo Re-DEPLOY ONLY
set -ex
near deploy $CONTRACT_ADDRESS $CONTRACT_WASM
