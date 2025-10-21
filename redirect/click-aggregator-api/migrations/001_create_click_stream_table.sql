-- Create click_stream table
-- Non-nullable schema with default values for optional data
-- This improves query performance and simplifies analytics

CREATE TABLE IF NOT EXISTS click_stream (
    -- Core identifiers
    id String,
    owner_id String,
    creator_id String,
    route_id String,
    workspace_id String,

    -- Timestamps
    created DateTime,

    -- Request data
    dest String,
    ip String,

    -- Geographic data (defaults to '_unknown')
    continent String DEFAULT '_unknown',
    country String DEFAULT '_unknown',
    location String DEFAULT '_unknown',

    -- Operating system data (defaults to '_unknown')
    os_family String DEFAULT '_unknown',
    os_version String DEFAULT '_unknown',

    -- User agent data (defaults to '_unknown')
    user_agent_family String DEFAULT '_unknown',
    user_agent_version String DEFAULT '_unknown',

    -- Device data (defaults to '_unknown')
    device_brand String DEFAULT '_unknown',
    device_family String DEFAULT '_unknown',
    device_model String DEFAULT '_unknown',

    -- Session data (defaults to epoch and 0)
    session_first DateTime DEFAULT toDateTime('1970-01-01 00:00:00'),
    session_clicks UInt128 DEFAULT 0,

    -- Flags
    is_unique UInt8,
    is_bot UInt8
) ENGINE = MergeTree()
ORDER BY id;
