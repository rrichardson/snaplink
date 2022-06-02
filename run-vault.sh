#!/bin/bash

docker start vault || docker run -d --name vault vault:1.10.3 --environment 'VAULT_DEV_ROOT_TOKEN_ID=devtoken' -p 9000:8200 --cap-add IPC_LOCK

export VAULT_TOKEN=devtoken
export VAULT_ADDR=http://localhost:9000

until vault status
do
  echo "Waiting (1 sec) for vault to start"
  sleep 1
done
vault secrets enable transit
vault write -f transit/keys/ssn
vault write -f transit/keys/id-evidence
cat <<EOF | vault policy write core-keys -
  path "transit/encrypt/ssn" {
    capabilities = [ "update" ]
  }
  path "transit/decrypt/ssn" {
    capabilities = [ "update" ]
  }
  path "transit/encrypt/id-evidence" {
    capabilities = [ "update"]
  }
  path "transit/decrypt/id-evidence" {
    capabilities = [ "update" ]
  }
EOF

