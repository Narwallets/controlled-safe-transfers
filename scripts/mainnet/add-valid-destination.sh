#!/bin/bash
__dir=$(dirname "$0")
. $__dir/mainnet-set-vars.sh

if [ $# -ne 1 ]; then
  echo "Error: Please provide exactly 1 argument"
  echo "receiver_id"
  exit 1
fi

set -ex
NEAR_ENV=mainnet near call $CONTRACT_ADDRESS \
    add_valid_destination '{"account_id":"'$1'"}' \
    --depositYocto 1 --accountId meta-pool-dao.near
