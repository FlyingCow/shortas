-- Denormalized counters for workspaces and domains, kept in sync by
-- application code on every route/domain/workspace/member write (see
-- route_repo.rs, domain_repo.rs, workspace_repo.rs). Lets dashboards read
-- counts in O(1) instead of scanning/joining the routes table.

CREATE TABLE IF NOT EXISTS workspace_stats (
    workspace_id UUID PRIMARY KEY REFERENCES workspaces(id) ON DELETE CASCADE,
    route_count BIGINT NOT NULL DEFAULT 0,
    active_route_count BIGINT NOT NULL DEFAULT 0,
    blocked_route_count BIGINT NOT NULL DEFAULT 0,
    member_count BIGINT NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS domain_stats (
    domain_id UUID PRIMARY KEY REFERENCES route_domains(id) ON DELETE CASCADE,
    route_count BIGINT NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Backfill from current data so existing workspaces/domains start accurate.
-- `route_properties.workspace_id` is a loose text column (not an FK), so
-- only rows that actually parse as a UUID and match a real workspace count.
INSERT INTO workspace_stats (workspace_id, route_count, active_route_count, blocked_route_count, member_count, updated_at)
SELECT
    w.id,
    COALESCE(rc.route_count, 0),
    COALESCE(rc.active_count, 0),
    COALESCE(rc.blocked_count, 0),
    COALESCE(mc.member_count, 0),
    NOW()
FROM workspaces w
LEFT JOIN (
    SELECT
        p.workspace_id::uuid AS workspace_id,
        COUNT(*) AS route_count,
        COUNT(*) FILTER (WHERE r.status = 'Active') AS active_count,
        COUNT(*) FILTER (WHERE r.status = 'Blocked') AS blocked_count
    FROM routes r
    JOIN route_properties p ON p.route_id = r.id
    WHERE r.switch = 'main' AND p.workspace_id ~ '^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$'
    GROUP BY p.workspace_id::uuid
) rc ON rc.workspace_id = w.id
LEFT JOIN (
    SELECT workspace_id, COUNT(*) AS member_count
    FROM user_workspaces
    GROUP BY workspace_id
) mc ON mc.workspace_id = w.id
ON CONFLICT (workspace_id) DO NOTHING;

INSERT INTO domain_stats (domain_id, route_count, updated_at)
SELECT d.id, COALESCE(rc.route_count, 0), NOW()
FROM route_domains d
LEFT JOIN (
    SELECT domain_id, COUNT(*) AS route_count
    FROM routes
    WHERE switch = 'main' AND domain_id IS NOT NULL
    GROUP BY domain_id
) rc ON rc.domain_id = d.id
ON CONFLICT (domain_id) DO NOTHING;
