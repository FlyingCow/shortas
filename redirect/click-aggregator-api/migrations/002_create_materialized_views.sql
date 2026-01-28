-- Materialized Views for ClickStream Analytics
-- These views pre-aggregate data for faster analytics queries
-- Updated to work with non-nullable schema using '_unknown' defaults

-- 1. Hourly Click Aggregation
-- Aggregates clicks by hour with geographic and device breakdown
CREATE MATERIALIZED VIEW IF NOT EXISTS click_stream_hourly_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(hour)
ORDER BY (owner_id, route_id, hour, country, device_family, user_agent_family)
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    toStartOfHour(created) AS hour,
    country,
    continent,
    device_family,
    user_agent_family,
    count() AS total_clicks,
    sum(is_unique) AS unique_clicks,
    sum(is_bot) AS bot_clicks,
    count() - sum(is_bot) AS human_clicks,
    uniqExact(ip) AS unique_ips,
    countIf(session_first != toDateTime('1970-01-01 00:00:00')) AS unique_sessions
FROM click_stream
GROUP BY
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    hour,
    country,
    continent,
    device_family,
    user_agent_family;

-- 2. Daily Click Aggregation
-- Aggregates clicks by day for trend analysis
CREATE MATERIALIZED VIEW IF NOT EXISTS click_stream_daily_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (owner_id, route_id, date)
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    toDate(created) AS date,
    count() AS total_clicks,
    sum(is_unique) AS unique_clicks,
    sum(is_bot) AS bot_clicks,
    count() - sum(is_bot) AS human_clicks,
    uniqExact(ip) AS unique_ips,
    countIf(session_first != toDateTime('1970-01-01 00:00:00')) AS unique_sessions,
    avgIf(session_clicks, session_clicks > 0) AS avg_session_clicks
FROM click_stream
GROUP BY
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    date;

-- 3. Geographic Analytics
-- Aggregates clicks by geographic location
CREATE MATERIALIZED VIEW IF NOT EXISTS click_stream_geographic_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (owner_id, route_id, date, country, location)
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    toDate(created) AS date,
    continent,
    country,
    location,
    count() AS total_clicks,
    sum(is_unique) AS unique_clicks,
    uniqExact(ip) AS unique_ips,
    countIf(session_first != toDateTime('1970-01-01 00:00:00')) AS unique_sessions
FROM click_stream
WHERE country != '_unknown'
GROUP BY
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    date,
    continent,
    country,
    location;

-- 4. Device Analytics
-- Aggregates clicks by device information
CREATE MATERIALIZED VIEW IF NOT EXISTS click_stream_device_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (owner_id, route_id, date, device_family, os_family)
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    toDate(created) AS date,
    device_family,
    device_brand,
    device_model,
    os_family,
    os_version,
    count() AS total_clicks,
    sum(is_unique) AS unique_clicks,
    uniqExact(ip) AS unique_ips
FROM click_stream
WHERE device_family != '_unknown'
GROUP BY
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    date,
    device_family,
    device_brand,
    device_model,
    os_family,
    os_version;

-- 5. Browser Analytics
-- Aggregates clicks by browser/user agent
CREATE MATERIALIZED VIEW IF NOT EXISTS click_stream_browser_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (owner_id, route_id, date, user_agent_family)
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    toDate(created) AS date,
    user_agent_family,
    user_agent_version,
    count() AS total_clicks,
    sum(is_unique) AS unique_clicks,
    uniqExact(ip) AS unique_ips
FROM click_stream
WHERE user_agent_family != '_unknown'
GROUP BY
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    date,
    user_agent_family,
    user_agent_version;

-- 6. Route Performance
-- Top-level route performance metrics
CREATE MATERIALIZED VIEW IF NOT EXISTS click_stream_route_performance_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (owner_id, route_id, date)
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    toDate(created) AS date,
    dest,
    count() AS total_clicks,
    sum(is_unique) AS unique_visitors,
    sum(is_bot) AS bot_clicks,
    count() - sum(is_bot) AS human_clicks,
    uniqExact(ip) AS unique_ips,
    uniqExactIf(country, country != '_unknown') AS countries_reached,
    uniqExactIf(device_family, device_family != '_unknown') AS device_types,
    avgIf(session_clicks, session_clicks > 0) AS avg_session_clicks,
    maxIf(session_clicks, session_clicks > 0) AS max_session_clicks
FROM click_stream
GROUP BY
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    date,
    dest;

-- 7. Owner/Workspace Analytics
-- Aggregates by owner and workspace for usage tracking
CREATE MATERIALIZED VIEW IF NOT EXISTS click_stream_owner_workspace_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (owner_id, workspace_id, date)
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    creator_id,
    workspace_id,
    toDate(created) AS date,
    count() AS total_clicks,
    sum(is_unique) AS unique_visitors,
    sum(is_bot) AS bot_clicks,
    count() - sum(is_bot) AS human_clicks,
    uniqExact(route_id) AS routes_used,
    uniqExact(ip) AS unique_ips,
    uniqExactIf(country, country != '_unknown') AS countries_reached
FROM click_stream
GROUP BY
    owner_id,
    creator_id,
    workspace_id,
    date;

-- 8. Real-time Stats (Last Hour)
-- Fast aggregation for real-time dashboards
CREATE MATERIALIZED VIEW IF NOT EXISTS click_stream_realtime_mv
ENGINE = AggregatingMergeTree()
PARTITION BY toYYYYMMDD(created)
ORDER BY (owner_id, route_id, toStartOfMinute(created))
TTL created + INTERVAL 7 DAY
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    route_id,
    toStartOfMinute(created) AS minute,
    countState() AS total_clicks,
    sumState(is_unique) AS unique_clicks,
    sumState(is_bot) AS bot_clicks,
    uniqState(ip) AS unique_ips
FROM click_stream
GROUP BY
    owner_id,
    route_id,
    minute;

-- 9. Top Destinations
-- Tracks most popular destination URLs
CREATE MATERIALIZED VIEW IF NOT EXISTS click_stream_top_destinations_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (owner_id, route_id, date, dest)
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    toDate(created) AS date,
    dest,
    count() AS total_clicks,
    sum(is_unique) AS unique_visitors,
    uniqExactIf(country, country != '_unknown') AS countries
FROM click_stream
GROUP BY
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    date,
    dest;

-- 10. Bot vs Human Traffic
-- Separates bot and human traffic for analysis
CREATE MATERIALIZED VIEW IF NOT EXISTS click_stream_traffic_type_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(hour)
ORDER BY (owner_id, route_id, hour, is_bot)
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    toStartOfHour(created) AS hour,
    is_bot,
    count() AS total_clicks,
    uniqExact(ip) AS unique_ips,
    uniqExactIf(user_agent_family, user_agent_family != '_unknown') AS user_agent_varieties
FROM click_stream
GROUP BY
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    hour,
    is_bot;

-- 11. Session Analytics
-- Analyzes user sessions
CREATE MATERIALIZED VIEW IF NOT EXISTS click_stream_session_mv
ENGINE = AggregatingMergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (owner_id, route_id, date)
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    toDate(created) AS date,
    uniqStateIf(session_first, session_first != toDateTime('1970-01-01 00:00:00')) AS unique_sessions,
    avgStateIf(session_clicks, session_clicks > 0) AS avg_clicks_per_session,
    maxStateIf(session_clicks, session_clicks > 0) AS max_clicks_per_session,
    sumStateIf(session_clicks, session_clicks > 0) AS total_session_clicks
FROM click_stream
WHERE session_first != toDateTime('1970-01-01 00:00:00')
GROUP BY
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    date;

-- 12. Minute-by-Minute for Recent Activity (Last 24h)
-- High-granularity recent data with auto-expiration
CREATE MATERIALIZED VIEW IF NOT EXISTS click_stream_recent_activity_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMMDD(minute)
ORDER BY (owner_id, route_id, minute)
TTL minute + INTERVAL 24 HOUR
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    toStartOfMinute(created) AS minute,
    count() AS total_clicks,
    sum(is_unique) AS unique_clicks,
    uniqExact(ip) AS unique_ips
FROM click_stream
GROUP BY
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    minute;
