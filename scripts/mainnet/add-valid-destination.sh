#!/bin/bash
__dir=$(dirname "$0")
. $__dir/mainnet-set-vars.sh

set -ex
NEAR_ENV=mainnet near call $CONTRACT_ADDRESS \
    add_valid_destination '{"account_id":"meta-pool-dao-3.near"}' --depositYocto 1 --accountId meta-pool-dao.near
