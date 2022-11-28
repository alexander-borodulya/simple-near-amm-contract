#!/bin/bash
set -e
cd "`dirname $0`"

# Common variables
DIR="${BASH_SOURCE%/*}"
if [[ ! -d "$DIR" ]]; then DIR="$PWD"; fi
. "$DIR/incl.sh"

# Check that files exist
[ -f $TOKEN_CONTRACT_FILE ] || { echo "$TOKEN_CONTRACT_FILE does not exist! Build required"; ./build.sh; }

# Create token A, token B
near create-account $TOKEN_A_CONTRACT_ID --masterAccount $MASTER_ACCOUNT_ID --initialBalance $DEFAULT_INITIAL_BALANCE_NEAR
near create-account $TOKEN_B_CONTRACT_ID --masterAccount $MASTER_ACCOUNT_ID --initialBalance $DEFAULT_INITIAL_BALANCE_NEAR

# Deploy token A, token B
near deploy $TOKEN_A_CONTRACT_ID --wasmFile=$TOKEN_CONTRACT_FILE
near deploy $TOKEN_B_CONTRACT_ID --wasmFile=$TOKEN_CONTRACT_FILE

# Initialize token A, token B and AMM account
near call $TOKEN_A_CONTRACT_ID \
    new '{
        "owner_id":"'$MASTER_ACCOUNT_ID'", 
        "name":"Token A", 
        "symbol":"tkn_A", 
        "total_supply":"'$TOKEN_A_TOTAL_SUPPLY'", 
        "decimals": '$TOKEN_A_DECIMALS'
        }' \
    --accountId=$MASTER_ACCOUNT_ID

near call $TOKEN_B_CONTRACT_ID \
    new '{
        "owner_id":"'$MASTER_ACCOUNT_ID'", 
        "name":"Token B", 
        "symbol":"tkn_B", 
        "total_supply":"'$TOKEN_B_TOTAL_SUPPLY'", 
        "decimals": '$TOKEN_B_DECIMALS'
        }' \
    --accountId=$MASTER_ACCOUNT_ID

# Prints a metadata of the deployed contracts
near view $TOKEN_A_CONTRACT_ID ft_metadata
near view $TOKEN_B_CONTRACT_ID ft_metadata
