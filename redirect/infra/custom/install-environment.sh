#!/usr/bin/env bash
cd "$(dirname "$0")"

echo "# Creating clickhouse database"
clickhouse-client --host clickhouse --user default --password clickhouse <<EOF

CREATE DATABASE IF NOT EXISTS shortas;
USE shortas;

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
