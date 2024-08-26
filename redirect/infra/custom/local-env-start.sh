#!/usr/bin/env bash
cd "$(dirname "$0")"

echo "BUILDING LOCAL INFRASTRUCTURE..."

echo " -> INITIATING LOCAL ENVIRONMENT..."

terraform -chdir=./terraform apply -auto-approve -var-file="local.tfvars"

# helm install --generate-name --namespace shortas-core oci://registry-1.docker.io/bitnamicharts/kafka

echo " -> DONE INITIATING LOCAL ENVIRONMENT."

echo "DONE BUILDING LOCAL INFRASTRUCTURE."
