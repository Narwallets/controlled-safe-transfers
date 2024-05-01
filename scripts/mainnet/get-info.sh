#!/bin/bash
__dir=$(dirname "$0")
. $__dir/mainnet-set-vars.sh

NEAR_ENV=mainnet near view $CONTRACT_ADDRESS get_owner_id {}
NEAR_ENV=mainnet near view $CONTRACT_ADDRESS get_operator_id {}
NEAR_ENV=mainnet near view $CONTRACT_ADDRESS get_valid_destinations {}
