-- Conversion events. Read with FINAL: rows with equal keys replace each other.
-- Not partitioned, because rows are only replaced within a partition.

CREATE TABLE IF NOT EXISTS conversion_events (
    workspace_id String,
    conversion_id String,
    -- What counts as the same event
    dedupe_key String,
    -- Earlier receipt wins
    version UInt64,
    id String,
    kind LowCardinality(String),
    -- attributed or unattributed. Refunds are unattributed.
    status LowCardinality(String),
    reason LowCardinality(String),
    source LowCardinality(String),
    test UInt8,
    step String,
    step_order UInt32,
    is_final UInt8,
    event_id String,
    external_id String,
    refund_id String,
    click_id String,
    value Nullable(Decimal(18, 4)),
    currency LowCardinality(String),
    occurred_at DateTime64(3, 'UTC'),
    received_at DateTime64(3, 'UTC'),
    -- From the click, when attributed
    click_at DateTime64(3, 'UTC'),
    route_id String,
    country String,
    device_family String,
    os_family String,
    user_agent_family String
) ENGINE = ReplacingMergeTree(version)
ORDER BY (workspace_id, conversion_id, dedupe_key)
SETTINGS storage_policy = 's3_main';

-- Events waiting for their click. Resolved rows only hide the waiting ones.
CREATE TABLE IF NOT EXISTS conversion_pending (
    id String,
    workspace_id String,
    conversion_id String,
    resolved UInt8,
    payload String,
    inserted_at DateTime64(3, 'UTC')
) ENGINE = ReplacingMergeTree(resolved)
ORDER BY id
TTL toDateTime(inserted_at) + INTERVAL 1 DAY DELETE WHERE resolved = 1
SETTINGS storage_policy = 's3_main';

CREATE TABLE IF NOT EXISTS conversion_rejected (
    workspace_id String,
    conversion_id String,
    id String,
    reason LowCardinality(String),
    field LowCardinality(String),
    kind LowCardinality(String),
    source LowCardinality(String),
    test UInt8,
    step String,
    event_id String,
    external_id String,
    refund_id String,
    click_id String,
    value Nullable(Decimal(18, 4)),
    currency LowCardinality(String),
    occurred_at DateTime64(3, 'UTC'),
    received_at DateTime64(3, 'UTC')
) ENGINE = ReplacingMergeTree
ORDER BY (workspace_id, conversion_id, id)
TTL toDateTime(received_at) + INTERVAL 30 DAY
SETTINGS storage_policy = 's3_main';

-- Refunds with the part of each that reduces revenue. Worked out on reading,
-- so the result does not depend on the order of arrival.
CREATE OR REPLACE VIEW conversion_refunds AS
SELECT
    r.*,
    o.original_id AS original_id,
    o.original_occurred_at AS original_occurred_at,
    o.original_click_at AS original_click_at,
    o.original_status AS original_status,
    o.original_source AS original_source,
    o.original_route_id AS original_route_id,
    o.original_country AS original_country,
    o.original_device_family AS original_device_family,
    o.original_os_family AS original_os_family,
    o.original_user_agent_family AS original_user_agent_family,
    if(o.original_id != '' AND o.original_currency = r.currency,
        least(toDecimal128(r.value, 4),
            greatest(toDecimal128(0, 4), toDecimal128(o.original_value, 4) - r.refunded_before)),
        toDecimal128(0, 4)) AS applied,
    multiIf(o.original_id = '', 'original_not_found',
        o.original_currency != r.currency, 'currency_mismatch',
        applied < r.value, 'exceeds_original',
        '') AS invalid_reason
FROM (
    SELECT
        id, refund_id, external_id, test, source, currency, occurred_at, received_at,
        ifNull(value, 0) AS value,
        toDecimal128(sum(ifNull(value, 0)) OVER (
            PARTITION BY external_id, test, currency
            ORDER BY received_at, id
            ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
        ), 4) - ifNull(value, 0) AS refunded_before
    FROM conversion_events FINAL
    WHERE workspace_id = {workspace_id:String}
        AND conversion_id = {conversion_id:String}
        AND kind = 'refund'
) AS r
LEFT JOIN (
    SELECT
        external_id,
        test,
        argMin(id, k) AS original_id,
        argMin(currency, k) AS original_currency,
        argMin(ifNull(value, 0), k) AS original_value,
        argMin(occurred_at, k) AS original_occurred_at,
        argMin(click_at, k) AS original_click_at,
        argMin(status, k) AS original_status,
        argMin(source, k) AS original_source,
        argMin(route_id, k) AS original_route_id,
        argMin(country, k) AS original_country,
        argMin(device_family, k) AS original_device_family,
        argMin(os_family, k) AS original_os_family,
        argMin(user_agent_family, k) AS original_user_agent_family
    FROM (
        SELECT *, (received_at, id) AS k
        FROM conversion_events FINAL
        WHERE workspace_id = {workspace_id:String}
            AND conversion_id = {conversion_id:String}
            AND kind = 'event' AND is_final = 1 AND external_id != ''
    )
    GROUP BY external_id, test
) AS o ON o.external_id = r.external_id AND o.test = r.test;
