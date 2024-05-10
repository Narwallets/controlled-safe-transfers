#!/bin/bash
__dir=$(dirname "$0")
. $__dir/set-vars.sh

echo DEPLOYING $NEAR_ENV $CONTRACT_ADDRESS
set -ex
near deploy $CONTRACT_ADDRESS $CONTRACT_WASM \
    --initFunction new --initArgs "$ARGS"

# register so the contract can receive mpDAO
near call $MPDAO_TOKEN_ADDRESS storage_deposit '{"account_id":"'$CONTRACT_ADDRESS'"}' --accountId $OWNER_ID --amount 0.0125
