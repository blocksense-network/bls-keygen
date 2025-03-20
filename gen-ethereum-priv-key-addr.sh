#!/usr/bin/env bash

key="$(openssl ecparam -name secp256k1 -genkey -noout | openssl ec -text -noout 2>/dev/null)"
priv="$(echo "$key" | grep priv -A 3 | tail -n +2 | tr -d '\n[:space:]:' | sed 's/^00//')"

echo "Ethereum address: $(cast wallet address "$priv")"
echo "Private key: 0x$priv"
