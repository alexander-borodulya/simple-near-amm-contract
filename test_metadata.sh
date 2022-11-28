#!/bin/bash
set -e
cd "`dirname $0`"

# Common variables
DIR="${BASH_SOURCE%/*}"
if [[ ! -d "$DIR" ]]; then DIR="$PWD"; fi
. "$DIR/incl.sh"

# Check that files exist
[ -f $TOKEN_CONTRACT_FILE ] || { echo "$TOKEN_CONTRACT_FILE does not exist! Build required"; ./build.sh; }
[ -f $AMM_CONTRACT_FILE ] || { echo "$AMM_CONTRACT_FILE does not exist! Build required"; ./build.sh; }

# Prints a metadata of the deployed contracts
near view $AMM_CONTRACT_ID tokens_full_info
near view $TOKEN_A_CONTRACT_ID ft_metadata
near view $TOKEN_B_CONTRACT_ID ft_metadata
near view $AMM_CONTRACT_ID token_info_by_id '{ "token_contract_id":"'$TOKEN_A_CONTRACT_ID'" }'
near view $AMM_CONTRACT_ID token_info_by_id '{ "token_contract_id":"'$TOKEN_B_CONTRACT_ID'" }'
near view $AMM_CONTRACT_ID get_tokens_ratio
