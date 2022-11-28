#!/bin/bash
set -e
cd "`dirname $0`"

# Common variables
DIR="${BASH_SOURCE%/*}"
if [[ ! -d "$DIR" ]]; then DIR="$PWD"; fi
. "$DIR/incl.sh"

# Delete token A, token B and AMM account
near delete --force $AMM_CONTRACT_ID $MASTER_ACCOUNT_ID --verbose true

# Check that files exist
[ -f $AMM_CONTRACT_FILE ] && rm -fv $AMM_CONTRACT_FILE
