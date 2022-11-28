#!/bin/bash
set -e
cd "`dirname $0`"

# Redeploy Tokens
./redeploy_tokens.sh

# Redeploy AMM
./redeploy_amm.sh
