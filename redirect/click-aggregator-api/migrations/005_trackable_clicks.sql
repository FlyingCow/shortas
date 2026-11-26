-- Clicks whose destination was given a click ID, so a conversion can follow

ALTER TABLE click_stream ADD COLUMN IF NOT EXISTS is_trackable UInt8 DEFAULT 0;

-- Every dimension is in the sorting key: merges add up rows with equal keys.
CREATE MATERIALIZED VIEW IF NOT EXISTS trackable_clicks_daily_mv
ENGINE = SummingMergeTree(clicks)
PARTITION BY toYYYYMM(date)
ORDER BY (workspace_id, route_id, date, country, device_family, os_family, user_agent_family)
SETTINGS storage_policy = 's3_main'
AS SELECT
    workspace_id,
    route_id,
    toDate(created) AS date,
    country,
    device_family,
    os_family,
    user_agent_family,
    count() AS clicks
FROM click_stream
WHERE is_trackable = 1
GROUP BY
    workspace_id,
    route_id,
    date,
    country,
    device_family,
    os_family,
    user_agent_family;
