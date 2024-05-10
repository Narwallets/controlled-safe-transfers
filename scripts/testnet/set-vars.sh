#!/bin/bash
set -e
export NEAR_ENV="testnet"
NETWORK=testnet

CONTRACT_WASM="target/wasm32-unknown-unknown/release/controlled_transfer_contract.wasm"
ls -l $CONTRACT_WASM

CONTRACT_ADDRESS=meta-pool-dao-safe.testnet
OWNER_ID="meta-pool-dao.testnet"
OPERATOR_ID="operator.$CONTRACT_ADDRESS"

MPDAO_TOKEN_ADDRESS="mpdao-token.testnet"

echo $NEAR_ENV $METAVOTE_CONTRACT_ADDRESS $(date) 

YOCTO_UNITS="000000000000000000000000"
TOTAL_PREPAID_GAS="200000000000000"
