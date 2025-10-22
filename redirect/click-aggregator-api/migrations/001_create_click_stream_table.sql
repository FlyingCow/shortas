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

    -- Timestamps (DateTime64(3) for millisecond precision)
    created DateTime64(3),

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

    -- Session data (defaults to epoch and 0, DateTime64(3) for millisecond precision)
    session_first DateTime64(3) DEFAULT toDateTime64('1970-01-01 00:00:00.000', 3),
    session_clicks UInt64 DEFAULT 0,

    -- Flags
    is_unique UInt8,
    is_bot UInt8
) ENGINE = MergeTree()
ORDER BY id;
