# Initialize CLI step

The first step is `near login` CLI command, which will redirect to NEAR Wallet where the creation of a full-access key should be confirmed.


# Required accounts

`MASTER_ACCOUNT_ID=<root-amm-account-id>.testnet` - Top Level Account that manages all the contracts accounts.

`TOKEN_A_CONTRACT_ID=token_a.$MASTER_ACCOUNT_ID` - Subaccount id for the Token A contract.

`TOKEN_B_CONTRACT_ID=token_b.$MASTER_ACCOUNT_ID` - Subaccount id for the Token B contract.

`AMM_CONTRACT_ID=amm_contract.$MASTER_ACCOUNT_ID` - Subaccount id for the AMM contract.

# Create subaccounts
```
# Create token A, token B
near create-account $TOKEN_A_CONTRACT_ID --masterAccount $MASTER_ACCOUNT_ID --initialBalance $DEFAULT_INITIAL_BALANCE_NEAR
near create-account $TOKEN_B_CONTRACT_ID --masterAccount $MASTER_ACCOUNT_ID --initialBalance $DEFAULT_INITIAL_BALANCE_NEAR

# Create AMM account
near create-account $AMM_CONTRACT_ID \
    --masterAccount $MASTER_ACCOUNT_ID \
    --initialBalance $DEFAULT_INITIAL_BALANCE_NEAR
```

# Deploy contracts
```
# Deploy token A, token B
near deploy $TOKEN_A_CONTRACT_ID --wasmFile=$TOKEN_CONTRACT_FILE
near deploy $TOKEN_B_CONTRACT_ID --wasmFile=$TOKEN_CONTRACT_FILE

# Deploy AMM account
near deploy $AMM_CONTRACT_ID --wasmFile=$AMM_CONTRACT_FILE

```

# Initialize contracts

Initialize token A, token B and AMM account with the metadata provided by the user of the contract
```
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
```

Initialize AMM account
```
near call $AMM_CONTRACT_ID \
    new '{
        "owner_id":"'$MASTER_ACCOUNT_ID'", 
        "token_a_contract_id":"'$TOKEN_A_CONTRACT_ID'", 
        "token_b_contract_id":"'$TOKEN_B_CONTRACT_ID'"
        }' \
    --accountId=$MASTER_ACCOUNT_ID \
    --gas=$GAS_FOR_RESOLVE_TRANSFER
```

# Test Environment

`USER_TOKEN_A_001=user_001.$TOKEN_A_CONTRACT_ID` - Subaccount of the Token A contract that will be used for deposit and transfer to tokens.

The first step for this account is to create a record in the Token A contract:
```
near call $TOKEN_A_CONTRACT_ID \
    storage_deposit '{ "account_id":"'$USER_TOKEN_A_001'" }' \
    --accountId $MASTER_ACCOUNT_ID \
    --amount 1
```

The following steps is to send some tokens to the USER_TOKEN_A_001 account:
```
near call $TOKEN_A_CONTRACT_ID \
    ft_transfer '{
        "receiver_id": "'$USER_TOKEN_A_001'", 
        "amount": "5000000000000000", 
        "memo": "ft_transfer for user 1 of token A contract"
        }' \
    --accountId $MASTER_ACCOUNT_ID \
    --depositYocto 1
```

After the sending tokens, the number of received tokens should be received by:
```
near view $TOKEN_A_CONTRACT_ID \
    ft_balance_of '{ "account_id":"'$USER_TOKEN_A_001'" }'

```

The main function of AMM contract that exchanges Token A to the Token B for the USER_TOKEN_A_001 account:
```
near call $AMM_CONTRACT_ID deposit_token_contract \
    '{
        "token_contract_id":"'$TOKEN_A_CONTRACT_ID'",
        "amount":5000
    }' \
    --accountId=$MASTER_ACCOUNT_ID \
    --gas=$GAS_FOR_RESOLVE_TRANSFER
```

The methods for getting information about the contract:
```
near view $AMM_CONTRACT_ID tokens_full_info
near view $AMM_CONTRACT_ID token_info_by_id '{ "token_contract_id":"'$TOKEN_A_CONTRACT_ID'" }'
near view $AMM_CONTRACT_ID token_info_by_id '{ "token_contract_id":"'$TOKEN_B_CONTRACT_ID'" }'
near view $AMM_CONTRACT_ID get_tokens_ratio
```

AMM method that directly deposits specified token contracts:
```
# TOKEN A:
near call $AMM_CONTRACT_ID deposit_token_contract \
    '{
        "token_contract_id":"'$TOKEN_A_CONTRACT_ID'",
        "amount":5000
    }' \
    --accountId=$MASTER_ACCOUNT_ID \
    --gas=$GAS_FOR_RESOLVE_TRANSFER

# TOKEN B:
near call $AMM_CONTRACT_ID deposit_token_contract \
    '{
        "token_contract_id":"'$TOKEN_B_CONTRACT_ID'",
        "amount":5000
    }' \
    --accountId=$MASTER_ACCOUNT_ID \
    --gas=$GAS_FOR_RESOLVE_TRANSFER
```

# Testing
Unit tests can be run by `cargo test` command.

# Usefull scripts
All the accounts and common variables are defined in the `incl.sh` file.

Described steps above might be reproduced by running `./test_all.sh` script.

- `build.sh` - Builds all contracts (token and AMM);
- `deploy_all.sh` - Deploys both Token and AMM contracts;
- `deploy_amm.sh` - Deploys AMM contract only;
- `deploy_tokens.sh` - Deploys Token contract only;
- `incl.sh` - Common variables;

Deletes already deployed contract, rebuilds contract, deploys a contract again:
- `redeploy_all.sh`
- `redeploy_amm.sh`
- `redeploy_tokens.sh`

Deletes already deployed contract:
- `reset_all.sh`
- `reset_amm.sh`
- `reset_tokens.sh`

Simple CLI tests:
- `test_all.sh`
- `test_metadata.sh`
