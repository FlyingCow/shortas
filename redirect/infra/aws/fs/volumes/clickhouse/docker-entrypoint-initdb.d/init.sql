CREATE DATABASE IF NOT EXISTS shortas;

CREATE OR REPLACE TABLE shortas.click_stream_raw (
    message String
)
ENGINE = MergeTree()
ORDER BY tuple();

CREATE OR REPLACE TABLE shortas.click_stream
(
    id FixedString(26),
    owner_id FixedString(26),
    creator_id FixedString(26),
    route_id FixedString(26),
    workspace_id FixedString(26),
    inserted DateTime MATERIALIZED now(),
    created DateTime,
    dest Nullable(String),
    ip LowCardinality(Nullable(String)),
    continent LowCardinality(Nullable(String)),
    country LowCardinality(Nullable(FixedString(2))),
    location LowCardinality(Nullable(String)),
    os_family LowCardinality(Nullable(String)),
    os_version LowCardinality(Nullable(String)),
    user_agent_family LowCardinality(Nullable(String)),
    user_agent_version LowCardinality(Nullable(String)),
    device_brand LowCardinality(Nullable(String)),
    device_family LowCardinality(Nullable(String)),
    device_model LowCardinality(Nullable(String)),
    session_first Nullable(DateTime),
    session_clicks Nullable(UInt128),
    is_unique Bool,
    is_bot Bool
)
ENGINE = MergeTree
ORDER BY id;
