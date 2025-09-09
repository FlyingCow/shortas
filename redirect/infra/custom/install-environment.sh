#!/usr/bin/env bash
cd "$(dirname "$0")"

echo "# Creating clickhouse database"
clickhouse-client --host clickhouse --user default --password clickhouse <<EOF

CREATE DATABASE IF NOT EXISTS shortas;
USE shortas;

CREATE TABLE IF NOT EXISTS click_stream
                (
                    id String,
                    owner_id String,
                    creator_id String,
                    route_id String,
                    workspace_id String,
                    created DateTime64(3),
                    dest String,
                    ip String,
                    continent Nullable(String),
                    country Nullable(String),
                    location Nullable(String),
                    os_family Nullable(String),
                    os_version Nullable(String),
                    user_agent_family Nullable(String),
                    user_agent_version Nullable(String),
                    device_brand Nullable(String),
                    device_family Nullable(String),
                    device_model Nullable(String),
                    session_first Nullable(DateTime64(3)),
                    session_clicks Nullable(UInt128),
                    is_unique Bool,
                    is_bot Bool
                )
                ENGINE = MergeTree
                ORDER BY id;
EOF

echo "# Creating mongo collection"
mongosh "mongodb://root:example@mongo:27017/" <<EOF

use shortas

db.createCollection("core_routes_local");
db.core_routes_local.createIndex( { "switch": 1, "link": 1 }, { unique: true } )

db.createCollection("core_routes_encryption_local");
db.core_routes_encryption_local.createIndex( { "hostname": 1 }, { unique: true } )

db.createCollection("core_routes_hostname_mapping_local");
db.core_routes_hostname_mapping_local.createIndex( { "hostname": 1 }, { unique: true } )

db.createCollection("core_user_settings_local");
db.core_user_settings_local.createIndex( { "user_id": 1 }, { unique: true } )

EOF

echo "# Create fluvio "

fluvio topic create hit-stream-local --partitions 1 --replication 1
fluvio topic create click-aggs-local --partitions 1 --replication 1
