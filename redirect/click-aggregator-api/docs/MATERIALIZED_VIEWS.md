# ClickStream Materialized Views

This document describes the materialized views created for efficient click stream analytics.

## Overview

Materialized views pre-aggregate data from the `click_stream` table, providing fast query performance for analytics dashboards and reports. All views automatically update as new data arrives.

## Available Materialized Views

### 1. Hourly Aggregation (`click_stream_hourly_mv`)

**Purpose**: Hourly click aggregation with geographic and device breakdown

**Columns**:
- `owner_id`, `creator_id`, `route_id`, `workspace_id`
- `hour` - Start of hour timestamp
- `country`, `continent`
- `device_family`, `user_agent_family`
- `total_clicks` - Total number of clicks
- `unique_clicks` - Number of unique visitor clicks
- `bot_clicks` - Number of bot clicks
- `human_clicks` - Non-bot clicks
- `unique_ips` - Count of unique IP addresses
- `unique_sessions` - Count of unique sessions

**Example Queries**:

```sql
-- Get hourly clicks for a specific route (last 24 hours)
SELECT
    hour,
    sum(total_clicks) AS clicks,
    sum(unique_clicks) AS unique_visitors,
    sum(human_clicks) AS human_traffic
FROM click_stream_hourly_mv
WHERE route_id = 'route-123'
  AND hour >= now() - INTERVAL 24 HOUR
GROUP BY hour
ORDER BY hour DESC;

-- Top countries by hour
SELECT
    hour,
    country,
    sum(total_clicks) AS clicks
FROM click_stream_hourly_mv
WHERE owner_id = 'user-123'
  AND hour >= today() - INTERVAL 7 DAY
GROUP BY hour, country
ORDER BY hour DESC, clicks DESC
LIMIT 100;

-- Device breakdown by hour
SELECT
    hour,
    device_family,
    sum(total_clicks) AS clicks,
    sum(unique_clicks) AS unique_visitors
FROM click_stream_hourly_mv
WHERE route_id = 'route-123'
  AND hour >= today()
GROUP BY hour, device_family
ORDER BY hour DESC, clicks DESC;
```

---

### 2. Daily Aggregation (`click_stream_daily_mv`)

**Purpose**: Daily click aggregation for trend analysis

**Columns**:
- `owner_id`, `creator_id`, `route_id`, `workspace_id`
- `date` - Date of clicks
- `total_clicks`, `unique_clicks`, `bot_clicks`, `human_clicks`
- `unique_ips`, `unique_sessions`
- `avg_session_clicks` - Average clicks per session

**Example Queries**:

```sql
-- Daily trend for last 30 days
SELECT
    date,
    sum(total_clicks) AS total_clicks,
    sum(unique_clicks) AS unique_visitors,
    sum(human_clicks) AS human_clicks,
    avg(avg_session_clicks) AS avg_session_length
FROM click_stream_daily_mv
WHERE route_id = 'route-123'
  AND date >= today() - INTERVAL 30 DAY
GROUP BY date
ORDER BY date DESC;

-- Week-over-week comparison
SELECT
    toMonday(date) AS week_start,
    sum(total_clicks) AS weekly_clicks,
    sum(unique_clicks) AS weekly_visitors
FROM click_stream_daily_mv
WHERE owner_id = 'user-123'
  AND date >= today() - INTERVAL 60 DAY
GROUP BY week_start
ORDER BY week_start DESC;

-- Month-over-month growth
SELECT
    toStartOfMonth(date) AS month,
    sum(total_clicks) AS monthly_clicks,
    sum(unique_clicks) AS monthly_visitors
FROM click_stream_daily_mv
WHERE workspace_id = 'workspace-456'
  AND date >= today() - INTERVAL 1 YEAR
GROUP BY month
ORDER BY month DESC;
```

---

### 3. Geographic Analytics (`click_stream_geographic_mv`)

**Purpose**: Geographic distribution of clicks

**Columns**:
- `owner_id`, `creator_id`, `route_id`, `workspace_id`
- `date`, `continent`, `country`, `location`
- `total_clicks`, `unique_clicks`, `unique_ips`, `unique_sessions`

**Example Queries**:

```sql
-- Top countries
SELECT
    country,
    sum(total_clicks) AS clicks,
    sum(unique_clicks) AS unique_visitors,
    sum(unique_ips) AS unique_ips
FROM click_stream_geographic_mv
WHERE route_id = 'route-123'
  AND date >= today() - INTERVAL 30 DAY
GROUP BY country
ORDER BY clicks DESC
LIMIT 20;

-- Geographic distribution by continent
SELECT
    continent,
    count(DISTINCT country) AS countries,
    sum(total_clicks) AS clicks,
    sum(unique_clicks) AS unique_visitors
FROM click_stream_geographic_mv
WHERE owner_id = 'user-123'
  AND date >= today() - INTERVAL 7 DAY
GROUP BY continent
ORDER BY clicks DESC;

-- City-level breakdown
SELECT
    country,
    location,
    sum(total_clicks) AS clicks,
    sum(unique_clicks) AS unique_visitors
FROM click_stream_geographic_mv
WHERE route_id = 'route-123'
  AND date >= today() - INTERVAL 30 DAY
  AND location IS NOT NULL
GROUP BY country, location
ORDER BY clicks DESC
LIMIT 50;
```

---

### 4. Device Analytics (`click_stream_device_mv`)

**Purpose**: Device, OS, and hardware analytics

**Columns**:
- `owner_id`, `creator_id`, `route_id`, `workspace_id`
- `date`, `device_family`, `device_brand`, `device_model`
- `os_family`, `os_version`
- `total_clicks`, `unique_clicks`, `unique_ips`

**Example Queries**:

```sql
-- Device type distribution
SELECT
    device_family,
    sum(total_clicks) AS clicks,
    sum(unique_clicks) AS unique_visitors,
    (sum(total_clicks) * 100.0 / (SELECT sum(total_clicks) FROM click_stream_device_mv WHERE route_id = 'route-123' AND date >= today() - INTERVAL 30 DAY)) AS percentage
FROM click_stream_device_mv
WHERE route_id = 'route-123'
  AND date >= today() - INTERVAL 30 DAY
GROUP BY device_family
ORDER BY clicks DESC;

-- Operating system breakdown
SELECT
    os_family,
    os_version,
    sum(total_clicks) AS clicks,
    sum(unique_clicks) AS unique_visitors
FROM click_stream_device_mv
WHERE owner_id = 'user-123'
  AND date >= today() - INTERVAL 7 DAY
GROUP BY os_family, os_version
ORDER BY clicks DESC
LIMIT 20;

-- Mobile vs Desktop
SELECT
    CASE
        WHEN device_family IN ('Mobile', 'Tablet') THEN 'Mobile'
        WHEN device_family = 'Desktop' THEN 'Desktop'
        ELSE 'Other'
    END AS device_category,
    sum(total_clicks) AS clicks,
    sum(unique_clicks) AS unique_visitors
FROM click_stream_device_mv
WHERE route_id = 'route-123'
  AND date >= today() - INTERVAL 30 DAY
GROUP BY device_category
ORDER BY clicks DESC;
```

---

### 5. Browser Analytics (`click_stream_browser_mv`)

**Purpose**: Browser/user agent analytics

**Columns**:
- `owner_id`, `creator_id`, `route_id`, `workspace_id`
- `date`, `user_agent_family`, `user_agent_version`
- `total_clicks`, `unique_clicks`, `unique_ips`

**Example Queries**:

```sql
-- Top browsers
SELECT
    user_agent_family,
    sum(total_clicks) AS clicks,
    sum(unique_clicks) AS unique_visitors
FROM click_stream_browser_mv
WHERE route_id = 'route-123'
  AND date >= today() - INTERVAL 30 DAY
GROUP BY user_agent_family
ORDER BY clicks DESC
LIMIT 15;

-- Browser version distribution
SELECT
    user_agent_family,
    user_agent_version,
    sum(total_clicks) AS clicks
FROM click_stream_browser_mv
WHERE owner_id = 'user-123'
  AND date >= today() - INTERVAL 7 DAY
  AND user_agent_family = 'Chrome'
GROUP BY user_agent_family, user_agent_version
ORDER BY clicks DESC;
```

---

### 6. Route Performance (`click_stream_route_performance_mv`)

**Purpose**: Overall route performance metrics

**Columns**:
- `owner_id`, `creator_id`, `route_id`, `workspace_id`
- `date`, `dest`
- `total_clicks`, `unique_visitors`, `bot_clicks`, `human_clicks`
- `unique_ips`, `countries_reached`, `device_types`
- `avg_session_clicks`, `max_session_clicks`

**Example Queries**:

```sql
-- Top performing routes
SELECT
    route_id,
    sum(total_clicks) AS clicks,
    sum(unique_visitors) AS unique_visitors,
    sum(countries_reached) AS countries,
    avg(avg_session_clicks) AS avg_session_length
FROM click_stream_route_performance_mv
WHERE owner_id = 'user-123'
  AND date >= today() - INTERVAL 30 DAY
GROUP BY route_id
ORDER BY clicks DESC
LIMIT 20;

-- Route performance over time
SELECT
    date,
    sum(total_clicks) AS clicks,
    sum(unique_visitors) AS visitors,
    sum(human_clicks) AS human_clicks,
    sum(bot_clicks) AS bot_clicks
FROM click_stream_route_performance_mv
WHERE route_id = 'route-123'
  AND date >= today() - INTERVAL 90 DAY
GROUP BY date
ORDER BY date DESC;

-- Engagement metrics
SELECT
    route_id,
    sum(total_clicks) AS total_clicks,
    sum(unique_visitors) AS unique_visitors,
    sum(total_clicks) / sum(unique_visitors) AS clicks_per_visitor,
    avg(avg_session_clicks) AS avg_session_length
FROM click_stream_route_performance_mv
WHERE workspace_id = 'workspace-456'
  AND date >= today() - INTERVAL 30 DAY
GROUP BY route_id
ORDER BY clicks_per_visitor DESC;
```

---

### 7. Owner/Workspace Analytics (`click_stream_owner_workspace_mv`)

**Purpose**: Usage tracking by owner and workspace

**Columns**:
- `owner_id`, `creator_id`, `workspace_id`
- `date`
- `total_clicks`, `unique_visitors`, `bot_clicks`, `human_clicks`
- `routes_used`, `unique_ips`, `countries_reached`

**Example Queries**:

```sql
-- Workspace usage summary
SELECT
    workspace_id,
    sum(total_clicks) AS total_clicks,
    sum(unique_visitors) AS unique_visitors,
    sum(routes_used) AS active_routes,
    sum(countries_reached) AS countries
FROM click_stream_owner_workspace_mv
WHERE owner_id = 'user-123'
  AND date >= today() - INTERVAL 30 DAY
GROUP BY workspace_id
ORDER BY total_clicks DESC;

-- Owner daily usage
SELECT
    date,
    sum(total_clicks) AS clicks,
    sum(unique_visitors) AS visitors,
    sum(routes_used) AS routes_active
FROM click_stream_owner_workspace_mv
WHERE owner_id = 'user-123'
  AND date >= today() - INTERVAL 30 DAY
GROUP BY date
ORDER BY date DESC;

-- Top users by traffic
SELECT
    owner_id,
    sum(total_clicks) AS total_traffic,
    sum(unique_visitors) AS unique_visitors,
    count(DISTINCT workspace_id) AS workspaces
FROM click_stream_owner_workspace_mv
WHERE date >= today() - INTERVAL 30 DAY
GROUP BY owner_id
ORDER BY total_traffic DESC
LIMIT 50;
```

---

### 8. Real-time Stats (`click_stream_realtime_mv`)

**Purpose**: Minute-by-minute aggregation for real-time dashboards (7-day TTL)

**Columns**:
- `owner_id`, `route_id`
- `minute` - Start of minute timestamp
- `total_clicks`, `unique_clicks`, `bot_clicks`, `unique_ips` (AggregatingMergeTree states)

**Example Queries**:

```sql
-- Real-time clicks (last hour)
SELECT
    minute,
    countMerge(total_clicks) AS clicks,
    sumMerge(unique_clicks) AS unique,
    uniqMerge(unique_ips) AS ips
FROM click_stream_realtime_mv
WHERE route_id = 'route-123'
  AND minute >= now() - INTERVAL 1 HOUR
GROUP BY minute
ORDER BY minute DESC;

-- Current activity (last 15 minutes)
SELECT
    route_id,
    countMerge(total_clicks) AS clicks,
    sumMerge(unique_clicks) AS unique_visitors
FROM click_stream_realtime_mv
WHERE owner_id = 'user-123'
  AND minute >= now() - INTERVAL 15 MINUTE
GROUP BY route_id
ORDER BY clicks DESC;
```

---

### 9. Top Destinations (`click_stream_top_destinations_mv`)

**Purpose**: Most popular destination URLs

**Columns**:
- `owner_id`, `creator_id`, `route_id`, `workspace_id`
- `date`, `dest`
- `total_clicks`, `unique_visitors`, `countries`

**Example Queries**:

```sql
-- Most clicked destinations
SELECT
    dest,
    sum(total_clicks) AS clicks,
    sum(unique_visitors) AS unique_visitors,
    sum(countries) AS countries_reached
FROM click_stream_top_destinations_mv
WHERE route_id = 'route-123'
  AND date >= today() - INTERVAL 30 DAY
GROUP BY dest
ORDER BY clicks DESC
LIMIT 20;

-- Destination diversity
SELECT
    count(DISTINCT dest) AS unique_destinations,
    sum(total_clicks) AS total_clicks,
    sum(unique_visitors) AS unique_visitors
FROM click_stream_top_destinations_mv
WHERE owner_id = 'user-123'
  AND date >= today() - INTERVAL 30 DAY;
```

---

### 10. Traffic Type (`click_stream_traffic_type_mv`)

**Purpose**: Bot vs human traffic analysis

**Columns**:
- `owner_id`, `creator_id`, `route_id`, `workspace_id`
- `hour`, `is_bot`
- `total_clicks`, `unique_ips`, `user_agent_varieties`

**Example Queries**:

```sql
-- Bot vs human breakdown
SELECT
    CASE WHEN is_bot = 1 THEN 'Bot' ELSE 'Human' END AS traffic_type,
    sum(total_clicks) AS clicks,
    sum(unique_ips) AS unique_ips
FROM click_stream_traffic_type_mv
WHERE route_id = 'route-123'
  AND hour >= today() - INTERVAL 7 DAY
GROUP BY traffic_type;

-- Bot traffic trend
SELECT
    toDate(hour) AS date,
    sum(total_clicks) FILTER (WHERE is_bot = 1) AS bot_clicks,
    sum(total_clicks) FILTER (WHERE is_bot = 0) AS human_clicks,
    (bot_clicks * 100.0 / (bot_clicks + human_clicks)) AS bot_percentage
FROM click_stream_traffic_type_mv
WHERE owner_id = 'user-123'
  AND hour >= today() - INTERVAL 30 DAY
GROUP BY date
ORDER BY date DESC;
```

---

### 11. Session Analytics (`click_stream_session_mv`)

**Purpose**: User session analysis

**Columns**:
- `owner_id`, `creator_id`, `route_id`, `workspace_id`
- `date`
- `unique_sessions`, `avg_clicks_per_session`, `max_clicks_per_session`, `total_session_clicks` (AggregatingMergeTree states)

**Example Queries**:

```sql
-- Session engagement metrics
SELECT
    date,
    uniqMerge(unique_sessions) AS sessions,
    avgMerge(avg_clicks_per_session) AS avg_clicks,
    maxMerge(max_clicks_per_session) AS max_clicks
FROM click_stream_session_mv
WHERE route_id = 'route-123'
  AND date >= today() - INTERVAL 30 DAY
GROUP BY date
ORDER BY date DESC;

-- Overall session statistics
SELECT
    route_id,
    uniqMerge(unique_sessions) AS total_sessions,
    avgMerge(avg_clicks_per_session) AS avg_session_length,
    maxMerge(max_clicks_per_session) AS longest_session
FROM click_stream_session_mv
WHERE owner_id = 'user-123'
  AND date >= today() - INTERVAL 30 DAY
GROUP BY route_id
ORDER BY total_sessions DESC;
```

---

### 12. Recent Activity (`click_stream_recent_activity_mv`)

**Purpose**: High-granularity recent data (24-hour TTL)

**Columns**:
- `owner_id`, `creator_id`, `route_id`, `workspace_id`
- `minute` - Start of minute timestamp
- `total_clicks`, `unique_clicks`, `unique_ips`

**Example Queries**:

```sql
-- Last hour activity
SELECT
    minute,
    sum(total_clicks) AS clicks,
    sum(unique_clicks) AS unique_visitors
FROM click_stream_recent_activity_mv
WHERE route_id = 'route-123'
  AND minute >= now() - INTERVAL 1 HOUR
GROUP BY minute
ORDER BY minute DESC;

-- Current spike detection
SELECT
    route_id,
    sum(total_clicks) AS recent_clicks
FROM click_stream_recent_activity_mv
WHERE owner_id = 'user-123'
  AND minute >= now() - INTERVAL 5 MINUTE
GROUP BY route_id
HAVING recent_clicks > 100
ORDER BY recent_clicks DESC;
```

---

## Performance Tips

1. **Time Range Filtering**: Always filter by date/time columns for optimal performance
2. **Partition Pruning**: Queries that filter by month will only scan relevant partitions
3. **Aggregation**: Use `sum()` on SummingMergeTree views and merge functions (`countMerge`, `sumMerge`, `uniqMerge`) on AggregatingMergeTree views
4. **TTL Views**: `click_stream_realtime_mv` and `click_stream_recent_activity_mv` auto-expire old data

## View Maintenance

### Refreshing Views

Materialized views automatically update as new data arrives. No manual refresh needed.

### Dropping and Recreating

```sql
-- Drop a view
DROP VIEW IF EXISTS click_stream_hourly_mv;

-- Recreate from migration file
-- Run the CREATE MATERIALIZED VIEW statement again
```

### Checking View Status

```sql
-- View row counts
SELECT
    name,
    formatReadableSize(total_bytes) AS size,
    formatReadableQuantity(total_rows) AS rows
FROM system.tables
WHERE database = 'shortas'
  AND name LIKE '%_mv'
ORDER BY total_rows DESC;

-- View partitions
SELECT
    table,
    partition,
    formatReadableSize(bytes_on_disk) AS size,
    rows
FROM system.parts
WHERE database = 'shortas'
  AND table LIKE '%_mv'
  AND active = 1
ORDER BY table, partition;
```

## Common Analytics Queries

### Dashboard Overview

```sql
-- Complete dashboard metrics for last 30 days
WITH metrics AS (
    SELECT
        sum(total_clicks) AS clicks,
        sum(unique_clicks) AS unique_visitors,
        sum(human_clicks) AS human_clicks,
        sum(unique_ips) AS unique_ips
    FROM click_stream_daily_mv
    WHERE owner_id = 'user-123'
      AND date >= today() - INTERVAL 30 DAY
)
SELECT
    clicks,
    unique_visitors,
    human_clicks,
    unique_ips,
    clicks / unique_visitors AS clicks_per_visitor,
    (human_clicks * 100.0 / clicks) AS human_percentage
FROM metrics;
```

### Growth Analysis

```sql
-- Compare this month vs last month
SELECT
    'This Month' AS period,
    sum(total_clicks) AS clicks,
    sum(unique_clicks) AS visitors
FROM click_stream_daily_mv
WHERE owner_id = 'user-123'
  AND date >= toStartOfMonth(today())

UNION ALL

SELECT
    'Last Month' AS period,
    sum(total_clicks) AS clicks,
    sum(unique_clicks) AS visitors
FROM click_stream_daily_mv
WHERE owner_id = 'user-123'
  AND date >= toStartOfMonth(today()) - INTERVAL 1 MONTH
  AND date < toStartOfMonth(today());
```

### Geographic Heat Map

```sql
-- Data for world map visualization
SELECT
    country,
    sum(total_clicks) AS clicks,
    sum(unique_clicks) AS unique_visitors,
    (clicks * 100.0 / (SELECT sum(total_clicks) FROM click_stream_geographic_mv WHERE owner_id = 'user-123' AND date >= today() - INTERVAL 30 DAY)) AS percentage
FROM click_stream_geographic_mv
WHERE owner_id = 'user-123'
  AND date >= today() - INTERVAL 30 DAY
GROUP BY country
ORDER BY clicks DESC;
```
