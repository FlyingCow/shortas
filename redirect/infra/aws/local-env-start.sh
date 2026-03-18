#!/usr/bin/env bash
cd "$(dirname "$0")"

echo "BUILDING LOCAL INFRASTRUCTURE..."
echo " -> RUNNING DOCKER..."

docker compose -f ./docker-services.local.yml up -d 

echo " -> DONE RUNNING DOCKER."

echo " -> ADDING FLUVIO TOPICS..."

fluvio topic create hit-stream-local --partitions 1 --replication 1
fluvio topic create click-aggs-local --partitions 1 --replication 1

echo " -> DONE ADDING FLUVIO TOPICS..."

echo " -> INITIATING LOCAL ENVIRONMENT..."

tflocal -chdir=./terraform apply -auto-approve -var-file="local.tfvars"

echo " -> DONE INITIATING LOCAL ENVIRONMENT."

echo " -> SEEDING SOME DATA..."

chmod u+x ./test-data-seed/test-data-seed.local.sh
./test-data-seed/test-data-seed.local.sh

echo " -> DONE SEEDING SOME DATA."

echo "DONE BUILDING LOCAL INFRASTRUCTURE."