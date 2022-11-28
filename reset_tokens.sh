#!/bin/bash
set -e
cd "`dirname $0`"

# Common variables
DIR="${BASH_SOURCE%/*}"
if [[ ! -d "$DIR" ]]; then DIR="$PWD"; fi
. "$DIR/incl.sh"

# Delete token A, token B and AMM account
near delete --force true $TOKEN_A_CONTRACT_ID $MASTER_ACCOUNT_ID --verbose true
near delete --force true $TOKEN_B_CONTRACT_ID $MASTER_ACCOUNT_ID --verbose true

# Check that files exist
[ -f $TOKEN_CONTRACT_FILE ] && rm -fv $TOKEN_CONTRACT_FILE
