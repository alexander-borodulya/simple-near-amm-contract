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

# Create/Register contract user accounts for the Token A contract
USER_TOKEN_A_001=user_001.$TOKEN_A_CONTRACT_ID
# USER_TOKEN_A_002=user_002.$TOKEN_A_CONTRACT_ID
# USER_TOKEN_A_003=user_003.$TOKEN_A_CONTRACT_ID

echo "================================================================"
echo "Test User 1: $USER_TOKEN_A_001"
echo ""

# Prints all accounts for the Token A contract
echo "[1] - Prints all accounts for the Token A contract before test"
echo ""
near view $TOKEN_A_CONTRACT_ID print_accounts

# Crete a new user account for the Token A contract
echo "[2]"
echo ""
near call $TOKEN_A_CONTRACT_ID \
    storage_deposit '{ "account_id":"'$USER_TOKEN_A_001'" }' \
    --accountId $MASTER_ACCOUNT_ID \
    --amount 1
    # --deposit=1 \

# Check the balance, for the new account it has to be equal to 0
echo "[3] - ft_balance_of USER_TOKEN_A_001 before transfer call"
echo ""
near view $TOKEN_A_CONTRACT_ID \
    ft_balance_of '{ "account_id": "'$USER_TOKEN_A_001'" }'

# Ask the Token A contract to transfer 50000000000000000 tokens to the user account
echo "[4] - Transfer call for the USER_TOKEN_A_001"
echo ""
near call $TOKEN_A_CONTRACT_ID \
    ft_transfer '{
        "receiver_id": "'$USER_TOKEN_A_001'", 
        "amount": "5000000000000000", 
        "memo": "ft_transfer for user 1 of token A contract"
        }' \
    --accountId $MASTER_ACCOUNT_ID \
    --depositYocto 1

# Check the balance, for the new account it's equal to 50000000000000000 tokens
echo "[5] - ft_balance_of USER_TOKEN_A_001 after transfer call"
echo ""
near view $TOKEN_A_CONTRACT_ID \
    ft_balance_of '{ "account_id":"'$USER_TOKEN_A_001'" }'

# A -> B
echo "[6] A -> B"
echo ""
near call $AMM_CONTRACT_ID \
    deposit_contract '{
        "amount":50
        }' \
    --accountId=$USER_TOKEN_A_001 \
    --gas=$GAS_FOR_RESOLVE_TRANSFER
    #  \
    # --depositYocto 1

# Prints all accounts for the Token A contract
echo "[7]"
echo ""
near view $TOKEN_A_CONTRACT_ID print_accounts

# Deposit token A, token B directly by the AMM contract
echo "[8]"
echo ""
near call $AMM_CONTRACT_ID deposit_token_contract \
    '{
        "token_contract_id":"'$TOKEN_A_CONTRACT_ID'",
        "amount":5000
    }' \
    --accountId=$MASTER_ACCOUNT_ID \
    --gas=$GAS_FOR_RESOLVE_TRANSFER
near view $AMM_CONTRACT_ID get_metadata
near view $AMM_CONTRACT_ID get_tokens_ratio

near call $AMM_CONTRACT_ID deposit_token_contract \
    '{
        "token_contract_id":"'$TOKEN_B_CONTRACT_ID'",
        "amount":5000
    }' \
    --accountId=$MASTER_ACCOUNT_ID \
    --gas=$GAS_FOR_RESOLVE_TRANSFER
near view $AMM_CONTRACT_ID get_metadata
near view $AMM_CONTRACT_ID get_tokens_ratio
