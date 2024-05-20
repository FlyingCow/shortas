#!/usr/bin/env bash
cd "$(dirname "$0")"

echo "DATA-SEED TRACKER..."
export AWS_DEFAULT_REGION="us-east-1"
export LOCALSTACK_ENDPOINT_URL="http://localhost:4566"
export AWS_ACCESS_KEY_ID="local"
export AWS_SECRET_ACCESS_KEY="local"
awslocal dynamodb list-tables

awslocal s3 mb s3://my-bucket
awslocal sqs create-queue --queue-name my-queue

echo "------INITIALIZING DYNAMO TABLE"
awslocal dynamodb put-item \
    --table-name core-routes-local  \
    --item \
        '{"switch": {"S": "main"}, "link": {"S": "localhost%2ftest"}, "dest": {"S": "https://google.com"}}'

awslocal dynamodb put-item \
    --table-name core-routes-local  \
    --item \
        '{"switch": {"S": "main"}, "link": {"S": "localhost%2fattr"}, "dest": {"S": "https://google.com"}, "attributes": {"M": {"type": {"S":"test"}, "env": {"S": "local"}}}}'


awslocal dynamodb put-item \
    --table-name core-routes-local  \
    --item \
        '{"switch": {"S": "main"}, "link": {"S": "localhost%2fblocked"}, "dest": {"S": "https://google.com"}, "blocked": {"BOOL": true}, "blocked.reason": {"S": "Spam"}}'

awslocal dynamodb get-item --table-name core-routes-local --key '{"link": {"S": "localhost%2ftest"}, "switch": {"S": "main"}}'

python3 ./add-certificate.local.py localhost ../certs/cert.pem ../certs/key.pem
