#!/usr/bin/env bash
cd "$(dirname "$0")"

mongosh "mongodb://root:example@mongo:27017/" <<EOF
use shortas

db.core_user_settings_local.insertOne({
    user_id: "01K4QMZ2G52WNRXFEDQ75KKAEE",
    user_email: "test@shortas.com"
});

db.core_routes_local.insertOne({
    switch: "main",
    link: "localhost%2Ftest",
    dest: "https://google.com",
    properties: {
        route_id: "01K4QMW3F04N7WB04NBQHQ7CTB",
        domain_id: "01K4QMY8M79SAQ98QEW9FVKDCV",
        owner_id: "01K4QMZ2G52WNRXFEDQ75KKAEE",
        creator_id: "01K4QMZ9ZD2PH7N1QGK9F2ZF7E",
        workspace_id: "01K4QMZG5CFHSN2801QAC2E1GR",
        allow_debug: true
    }
});
EOF