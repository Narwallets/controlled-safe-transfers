#!/bin/bash
__dir=$(dirname "$0")
. $__dir/mainnet-set-vars.sh

echo DEPLOYING $NEAR_ENV $CONTRACT_ADDRESS
set -ex
NEAR_ENV=mainnet near deploy $CONTRACT_ADDRESS $CONTRACT_WASM \
    --initFunction new --initArgs "$ARGS"

# register so the contract can receive mpDAO
NEAR_ENV=mainnet near call $MPDAO_TOKEN_ADDRESS storage_deposit '{"account_id":"'$CONTRACT_ADDRESS'"}' --accountId $OWNER_ID --amount 0.0125
