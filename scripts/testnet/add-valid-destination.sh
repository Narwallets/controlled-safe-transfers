#!/bin/bash
__dir=$(dirname "$0")
. $__dir/set-vars.sh

set -ex
near call $CONTRACT_ADDRESS add_valid_destination '{"account_id":"v1.mpdao-vote.'$NETWORK'"}' --depositYocto 1 --accountId $OWNER_ID
