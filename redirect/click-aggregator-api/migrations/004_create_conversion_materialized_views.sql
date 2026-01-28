-- Materialized Views for Conversion Analytics
-- Pre-aggregated views for fast conversion reporting and analysis

-- 1. Conversion Rates by Route and Time
-- Tracks conversion rates over time for each route
CREATE MATERIALIZED VIEW IF NOT EXISTS conversion_rates_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (owner_id, route_id, date, conversion_type)
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    toDate(created) AS date,
    conversion_type,
    conversion_name,
    count() AS total_conversions,
    sum(conversion_value) AS total_conversion_value,
    avg(conversion_value) AS avg_conversion_value,
    max(conversion_value) AS max_conversion_value,
    min(conversion_value) AS min_conversion_value,
    uniqExact(user_id) AS unique_converting_users,
    uniqExact(session_id) AS unique_converting_sessions,
    uniqExact(ip) AS unique_converting_ips
FROM conversions
GROUP BY
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    date,
    conversion_type,
    conversion_name;

-- 2. Conversion Attribution Analysis
-- Analyzes which clicks lead to conversions
CREATE MATERIALIZED VIEW IF NOT EXISTS conversion_attribution_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (owner_id, route_id, date, attribution_type)
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    route_id,
    workspace_id,
    toDate(conversion_created) AS date,
    attribution_type,
    count() AS attribution_count,
    sum(attribution_weight) AS total_attribution_weight,
    avg(time_to_conversion_seconds) AS avg_time_to_conversion,
    min(time_to_conversion_seconds) AS min_time_to_conversion,
    max(time_to_conversion_seconds) AS max_time_to_conversion,
    uniqExact(conversion_id) AS unique_conversions,
    uniqExact(click_id) AS unique_clicks,
    uniqExact(user_id) AS unique_users
FROM conversion_attribution
GROUP BY
    owner_id,
    route_id,
    workspace_id,
    date,
    attribution_type;

-- 3. Conversion Funnel Performance
-- Tracks funnel completion rates and drop-off points
CREATE MATERIALIZED VIEW IF NOT EXISTS conversion_funnels_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (owner_id, funnel_name, date, step_position)
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    workspace_id,
    funnel_name,
    toDate(step_created) AS date,
    step_name,
    step_position,
    count() AS step_completions,
    uniqExact(user_id) AS unique_users_at_step,
    uniqExact(session_id) AS unique_sessions_at_step,
    sum(step_value) AS total_step_value,
    avg(step_value) AS avg_step_value
FROM conversion_funnels
WHERE step_completed = 1
GROUP BY
    owner_id,
    workspace_id,
    funnel_name,
    date,
    step_name,
    step_position;

-- 4. Revenue Analytics
-- Tracks revenue and ROI metrics
CREATE MATERIALIZED VIEW IF NOT EXISTS conversion_revenue_mv
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
    count() AS total_conversions,
    sum(conversion_value) AS total_revenue,
    avg(conversion_value) AS avg_order_value,
    uniqExact(user_id) AS unique_customers,
    uniqExact(session_id) AS unique_converting_sessions,
    -- Revenue per click (requires join with click data)
    sum(conversion_value) AS revenue_sum_for_join
FROM conversions
WHERE conversion_type = 'purchase' AND conversion_value > 0
GROUP BY
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    date;

-- 5. Geographic Conversion Analysis
-- Conversion rates by geographic location
CREATE MATERIALIZED VIEW IF NOT EXISTS conversion_geographic_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (owner_id, route_id, date, country)
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
    conversion_type,
    count() AS total_conversions,
    sum(conversion_value) AS total_conversion_value,
    uniqExact(user_id) AS unique_converting_users,
    uniqExact(ip) AS unique_converting_ips
FROM conversions
WHERE country != '_unknown'
GROUP BY
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    date,
    continent,
    country,
    location,
    conversion_type;

-- 6. Device Conversion Analysis
-- Conversion rates by device type
CREATE MATERIALIZED VIEW IF NOT EXISTS conversion_device_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (owner_id, route_id, date, device_family)
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
    user_agent_family,
    user_agent_version,
    conversion_type,
    count() AS total_conversions,
    sum(conversion_value) AS total_conversion_value,
    uniqExact(user_id) AS unique_converting_users
FROM conversions
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
    os_version,
    user_agent_family,
    user_agent_version,
    conversion_type;

-- 7. Hourly Conversion Tracking
-- Real-time conversion monitoring
CREATE MATERIALIZED VIEW IF NOT EXISTS conversion_hourly_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(hour)
ORDER BY (owner_id, route_id, hour, conversion_type)
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    toStartOfHour(created) AS hour,
    conversion_type,
    count() AS total_conversions,
    sum(conversion_value) AS total_conversion_value,
    uniqExact(user_id) AS unique_converting_users,
    uniqExact(session_id) AS unique_converting_sessions
FROM conversions
GROUP BY
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    hour,
    conversion_type;

-- 8. Conversion Goals Performance
-- Tracks performance against defined goals
CREATE MATERIALIZED VIEW IF NOT EXISTS conversion_goals_performance_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (owner_id, route_id, date, goal_name)
SETTINGS storage_policy = 's3_main'
AS SELECT
    cg.owner_id,
    cg.workspace_id,
    cg.route_id,
    cg.goal_name,
    cg.goal_type,
    cg.target_value,
    cg.target_period,
    toDate(c.created) AS date,
    count() AS actual_conversions,
    sum(c.conversion_value) AS actual_value,
    -- Calculate goal achievement percentage
    CASE 
        WHEN cg.goal_type = 'revenue' THEN (sum(c.conversion_value) / cg.target_value) * 100
        WHEN cg.goal_type = 'conversion_rate' THEN (count() / cg.target_value) * 100
        ELSE 0
    END AS goal_achievement_percentage
FROM conversion_goals cg
INNER JOIN conversions c ON cg.route_id = c.route_id 
    AND cg.owner_id = c.owner_id 
    AND cg.workspace_id = c.workspace_id
WHERE cg.is_active = 1
GROUP BY
    cg.owner_id,
    cg.workspace_id,
    cg.route_id,
    cg.goal_name,
    cg.goal_type,
    cg.target_value,
    cg.target_period,
    date;

-- 9. Multi-touch Attribution Analysis
-- Detailed analysis of multi-touch attribution paths
CREATE MATERIALIZED VIEW IF NOT EXISTS conversion_multi_touch_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (owner_id, route_id, date, attribution_position)
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    route_id,
    workspace_id,
    toDate(conversion_created) AS date,
    attribution_position,
    count() AS attribution_count,
    sum(attribution_weight) AS total_weight,
    avg(time_to_conversion_seconds) AS avg_time_to_conversion,
    uniqExact(conversion_id) AS unique_conversions,
    uniqExact(click_id) AS unique_clicks
FROM conversion_attribution
WHERE attribution_type = 'multi_touch'
GROUP BY
    owner_id,
    route_id,
    workspace_id,
    date,
    attribution_position;

-- 10. Conversion Cohort Analysis
-- Tracks conversion behavior over time for user cohorts
CREATE MATERIALIZED VIEW IF NOT EXISTS conversion_cohort_mv
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMM(cohort_date)
ORDER BY (owner_id, route_id, cohort_date, conversion_date)
SETTINGS storage_policy = 's3_main'
AS SELECT
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    toDate(funnel_started) AS cohort_date,
    toDate(step_created) AS conversion_date,
    funnel_name,
    step_name,
    count() AS conversions,
    uniqExact(user_id) AS unique_users,
    sum(step_value) AS total_value
FROM conversion_funnels
WHERE step_completed = 1
GROUP BY
    owner_id,
    creator_id,
    route_id,
    workspace_id,
    cohort_date,
    conversion_date,
    funnel_name,
    step_name;
