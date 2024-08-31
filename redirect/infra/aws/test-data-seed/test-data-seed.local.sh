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
    --table-name core-user-settings-local  \
    --item \
        '{"user_id": {"S": "my_users_id"}, "user_email": {"S": "my_user@email.com"}, "api_key": {"S": "my_user_api_key"}, "status": {"S": "active"}}'

awslocal dynamodb put-item \
    --table-name core-routes-local  \
    --item \
        '{"switch": {"S": "main"}, "link": {"S": "localhost%2ftest"}, "dest": {"S": "https://google.com"}, "route.id": {"S": "my_route_id"}, "owner.id": {"S": "my_users_id"}, "creator.id": {"S": "my_creator_id"}, "workspace.id": {"S": "my_workspace_id"}}'

awslocal dynamodb put-item \
    --table-name core-routes-local  \
    --item \
        '{"switch": {"S": "main"}, "link": {"S": "localhost%2fattr"}, "dest": {"S": "https://google.com"}, "route.id": {"S": "my_route_id"}, "owner.id": {"S": "my_users_id"}, "creator.id": {"S": "my_creator_id"}, "workspace.id": {"S": "my_workspace_id"}, "attributes": {"M": {"type": {"S":"test"}, "env": {"S": "local"}}}}'


awslocal dynamodb put-item \
    --table-name core-routes-local  \
    --item \
        '{"switch": {"S": "main"}, "link": {"S": "localhost%2fblocked"}, "dest": {"S": "https://google.com"}, "route.id": {"S": "my_route_id"}, "owner.id": {"S": "my_users_id"}, "creator.id": {"S": "my_creator_id"}, "workspace.id": {"S": "my_workspace_id"}, "blocked": {"BOOL": true}, "blocked.reason": {"S": "Spam"}}'



awslocal dynamodb put-item \
    --table-name core-routes-local  \
    --item \
        '{"switch": {"S": "test"}, "link": {"S": "localhost%2fconds"}, "dest": {"S": "https://google.com?q=test"}, "route.id": {"S": "my_route_id"}, "owner.id": {"S": "my_users_id"}, "creator.id": {"S": "my_creator_id"}, "workspace.id": {"S": "my_workspace_id"}}'

awslocal dynamodb put-item \
    --table-name core-routes-local  \
    --item \
        '{
        "link": {
            "S": "localhost%2fconds"
        },
        "dest": {"S": "https://google.com?q=main"},
        "routing": {
            "M": {
                "conditions": {
                    "L": [
                        {
                            "M": {
                                "key": {
                                    "S": "test"
                                },
                                "condition": {
                                    "M": {
                                        "ua": {
                                            "M": {
                                                "IN": {
                                                    "L": [
                                                        {
                                                            "S": "Edge"
                                                        },
                                                        {
                                                            "S": "Chrome"
                                                        },
                                                        {
                                                            "S": "Firefox"
                                                        }
                                                    ]
                                                }
                                            }
                                        },
                                        "day_of_month": {
                                            "M": {
                                                "IN": {
                                                    "L": [
                                                        {
                                                            "N": "7"
                                                        },
                                                        {
                                                            "N": "14"
                                                        },
                                                        {
                                                            "N": "30"
                                                        },
                                                        {
                                                            "N": "26"
                                                        }
                                                    ]
                                                }
                                            }
                                        },
                                        "and": {
                                            "L": [
                                                {
                                                    "M": {
                                                        "os": {
                                                            "M": {
                                                                "EQ": {
                                                                    "S": "Windows"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            ]
                                        }
                                    }
                                }
                            }
                        }
                    ]
                },
                "policy": {
                    "S": "conditional"
                }
            }
        },
        "debug": {"BOOL": true},
        "route.id": {"S": "my_route_id"}, "owner.id": {"S": "my_users_id"}, "creator.id": {"S": "my_creator_id"}, "workspace.id": {"S": "my_workspace_id"},
        "switch": {
            "S": "main"
        }
    }'


awslocal dynamodb get-item --table-name core-routes-local --key '{"link": {"S": "localhost%2fconds"}, "switch": {"S": "main"}}'

python3 ./add-certificate.local.py localhost ./certs/cert.pem ./certs/key.pem
