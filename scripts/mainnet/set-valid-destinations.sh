#!/bin/bash
__dir=$(dirname "$0")
. $__dir/mainnet-set-vars.sh

NEAR_ENV=mainnet near call $CONTRACT_ADDRESS add_valid_destination '{"account_id":"mpdao-vote.near"}' --depositYocto 1 --accountId meta-pool-dao.near
