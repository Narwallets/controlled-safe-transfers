#!/bin/bash
__dir=$(dirname "$0")
. $__dir/set-vars.sh

near view $CONTRACT_ADDRESS get_owner_id {}
near view $CONTRACT_ADDRESS get_operator_id {}
near view $CONTRACT_ADDRESS get_valid_destinations {}
