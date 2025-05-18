#!/usr/bin/env bash
cd "$(dirname "$0")"

echo "BUILDING LOCAL INFRASTRUCTURE..."
echo " -> RUNNING DOCKER..."

docker compose -f ./docker-services.local.yml up -d 

echo " -> DONE RUNNING DOCKER."

echo " -> INITIATING LOCAL ENVIRONMENT..."

tflocal -chdir=./terraform apply -auto-approve -var-file="local.tfvars"

echo " -> DONE INITIATING LOCAL ENVIRONMENT."

echo " -> SEEDING SOME DATA..."

chmod u+x ./test-data-seed/test-data-seed.local.sh
./test-data-seed/test-data-seed.local.sh

echo " -> DONE SEEDING SOME DATA."

echo "DONE BUILDING LOCAL INFRASTRUCTURE."

cd ../domains/
cargo run