#!/bin/bash
set -e
export NEAR_ENV="testnet"
NETWORK=testnet

CONTRACT_WASM="target/wasm32-unknown-unknown/release/controlled_transfer_contract.wasm"
ls -l $CONTRACT_WASM

CONTRACT_ADDRESS=meta-pool-dao-7.$NETWORK
OWNER_ID=meta-pool-dao.$NETWORK
OPERATOR_ID=operator.$CONTRACT_ADDRESS

# open a stNEAR account for $CONTRACT_ADDRESS
REGISTER_TOKEN_ADDRESS="meta-v2.pool.testnet"

echo $NEAR_ENV $METAVOTE_CONTRACT_ADDRESS $(date) 

YOCTO_UNITS="000000000000000000000000"
TOTAL_PREPAID_GAS="200000000000000"
