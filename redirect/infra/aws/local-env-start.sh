#!/usr/bin/env bash
cd "$(dirname "$0")"

echo "BUILDING LOCAL INFRASTRUCTURE..."
echo " -> RUNNING DOCKER..."

docker compose -f ./docker-services.local.yml up -d 

echo " -> DONE RUNNING DOCKER."

echo " -> INITIATING LOCAL ENVIRONMENT..."

tflocal -chdir=./terraform apply -auto-approve -var-file="local.tfvars"

echo " -> DONE INITIATING LOCAL ENVIRONMENT."

echo " -> INITIATING KAFKA TOPICS."

docker exec kafka kafka-topics.sh --create --bootstrap-server localhost:9092 --partitions 16 --replication-factor 1 --topic hit-stream-local 
docker exec kafka kafka-topics.sh --create --bootstrap-server localhost:9092 --partitions 16 --replication-factor 1 --topic click-aggs-local

echo " -> DONE INITIATING KAFKA TOPICS."

echo " -> SEEDING SOME DATA..."

chmod u+x ./test-data-seed/test-data-seed.local.sh
./test-data-seed/test-data-seed.local.sh

echo " -> DONE SEEDING SOME DATA."

echo "DONE BUILDING LOCAL INFRASTRUCTURE."
