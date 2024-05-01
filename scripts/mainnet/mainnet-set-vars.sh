#!/bin/bash
set -e
export NEAR_ENV="mainnet"

CONTRACT_WASM="target/wasm32-unknown-unknown/release/controlled_transfer_contract.wasm"
ls -l $CONTRACT_WASM

CONTRACT_ADDRESS=meta-pool-dao-safe.near
OWNER_ID="meta-pool-dao.near"
OPERATOR_ID="operator.$CONTRACT_ADDRESS"

MPDAO_TOKEN_ADDRESS="mpdao-token.near"

echo $NEAR_ENV $METAVOTE_CONTRACT_ADDRESS $(date) 

YOCTO_UNITS="000000000000000000000000"
TOTAL_PREPAID_GAS="200000000000000"

ARGS=$(cat <<EOA
{
"owner_id":"$OWNER_ID","operator_id":"$OPERATOR_ID"
}
EOA
)
