#!/bin/bash
set -e
cd "`dirname $0`"

# Common variables
DIR="${BASH_SOURCE%/*}"
if [[ ! -d "$DIR" ]]; then DIR="$PWD"; fi
. "$DIR/incl.sh"

# Check that files exist
[ -f $AMM_CONTRACT_FILE ] || { echo "$AMM_CONTRACT_FILE does not exist! Build required"; ./build.sh; }

# Create AMM account
near create-account $AMM_CONTRACT_ID \
    --masterAccount $MASTER_ACCOUNT_ID \
    --initialBalance $DEFAULT_INITIAL_BALANCE_NEAR

# Deploy token A, token B and AMM account
near deploy $AMM_CONTRACT_ID --wasmFile=$AMM_CONTRACT_FILE

# Initialize AMM account
near call $AMM_CONTRACT_ID \
    new '{
        "owner_id":"'$MASTER_ACCOUNT_ID'", 
        "token_a_contract_id":"'$TOKEN_A_CONTRACT_ID'", 
        "token_b_contract_id":"'$TOKEN_B_CONTRACT_ID'"
        }' \
    --accountId=$MASTER_ACCOUNT_ID \
    --gas=$GAS_FOR_RESOLVE_TRANSFER

# Prints a metadata of the deployed contracts
near view $AMM_CONTRACT_ID get_tokens_ratio
near view $AMM_CONTRACT_ID tokens_full_info
near view $AMM_CONTRACT_ID token_info_by_id '{ "token_contract_id":"'$TOKEN_A_CONTRACT_ID'" }'
near view $AMM_CONTRACT_ID token_info_by_id '{ "token_contract_id":"'$TOKEN_B_CONTRACT_ID'" }'
