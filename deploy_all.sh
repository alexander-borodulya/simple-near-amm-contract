#!/bin/bash
set -e
cd "`dirname $0`"

# Common variables
DIR="${BASH_SOURCE%/*}"
if [[ ! -d "$DIR" ]]; then DIR="$PWD"; fi
. "$DIR/incl.sh"

# Deploy Tokens
./deploy_tokens.sh

# Deploy AMM
./deploy_amm.sh

# Prints a metadata of the deployed contracts
near view $TOKEN_A_CONTRACT_ID ft_metadata
near view $TOKEN_B_CONTRACT_ID ft_metadata

near view $AMM_CONTRACT_ID get_tokens_ratio
near view $AMM_CONTRACT_ID tokens_full_info
near view $AMM_CONTRACT_ID token_info_by_id '{ "token_contract_id":"'$TOKEN_A_CONTRACT_ID'" }'
near view $AMM_CONTRACT_ID token_info_by_id '{ "token_contract_id":"'$TOKEN_B_CONTRACT_ID'" }'
