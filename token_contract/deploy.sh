#!/bin/bash

export FT_CONTRACT_ID=finished_contract_001.testnet
export EVENTS_FT_CONTRACT_ID=events.$FT_CONTRACT_ID
export FT_ACTIVE_CONTRACT_ID=$FT_CONTRACT_ID

near create-account events.$FT_CONTRACT_ID --masterAccount $FT_CONTRACT_ID --initialBalance 25

./build.sh 

near deploy --wasmFile out/contract.wasm --accountId $FT_ACTIVE_CONTRACT_ID

near call $FT_ACTIVE_CONTRACT_ID new_default_meta '{"owner_id": "'$FT_ACTIVE_CONTRACT_ID'", "total_supply": "1000000000000000000000000000"}' --accountId $FT_ACTIVE_CONTRACT_ID

near view $FT_ACTIVE_CONTRACT_ID ft_total_supply

near view $FT_ACTIVE_CONTRACT_ID ft_balance_of '{"account_id": "'$FT_ACTIVE_CONTRACT_ID'"}'

# near view $FT_ACTIVE_CONTRACT_ID ft_balance_of '{"account_id": "benjiman.testnet"}'