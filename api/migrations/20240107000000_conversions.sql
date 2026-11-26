-- Conversions: what a workspace counts as a conversion and how it is tracked.

CREATE TABLE IF NOT EXISTS conversions (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Draft',
    public_key VARCHAR(64) NOT NULL UNIQUE,
    -- SHA-256 of the whole secret key, hex encoded. The key is never stored.
    secret_hash CHAR(64) NOT NULL,
    currency CHAR(3) NOT NULL,
    window_days INTEGER NOT NULL CHECK (window_days BETWEEN 1 AND 90),
    counting VARCHAR(20) NOT NULL,
    domain_id UUID REFERENCES route_domains(id) ON DELETE SET NULL,
    allowed_origins TEXT[] NOT NULL DEFAULT '{}',
    server_values_only BOOLEAN NOT NULL DEFAULT FALSE,
    -- FALSE means only the routes in conversion_routes are covered, even when
    -- that list becomes empty because its routes were deleted.
    all_routes BOOLEAN NOT NULL DEFAULT TRUE,
    version BIGINT NOT NULL DEFAULT 1,
    created_by VARCHAR(255) NOT NULL,
    -- First activation. Once set, removed steps are archived, not deleted.
    activated_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_conversions_workspace ON conversions(workspace_id);
CREATE INDEX idx_conversions_domain ON conversions(domain_id);

CREATE TABLE IF NOT EXISTS conversion_steps (
    conversion_id UUID NOT NULL REFERENCES conversions(id) ON DELETE CASCADE,
    key VARCHAR(64) NOT NULL,
    name VARCHAR(100) NOT NULL,
    position INTEGER NOT NULL,
    default_value NUMERIC(18, 4) CHECK (default_value >= 0),
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (conversion_id, key)
);

CREATE TABLE IF NOT EXISTS conversion_routes (
    conversion_id UUID NOT NULL REFERENCES conversions(id) ON DELETE CASCADE,
    route_id UUID NOT NULL REFERENCES routes(id) ON DELETE CASCADE,
    PRIMARY KEY (conversion_id, route_id)
);

CREATE INDEX idx_conversion_routes_route ON conversion_routes(route_id);

-- Whether the redirect adds a click ID. `track_conversions` is computed from
-- the conversions above; `track_conversions_sent` is the value the router last
-- received. Rows where they differ are the work left to send.
ALTER TABLE route_properties
    ADD COLUMN conversion_opt_out BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN track_conversions BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN track_conversions_sent BOOLEAN NOT NULL DEFAULT FALSE;

CREATE INDEX idx_route_properties_workspace_lower ON route_properties(LOWER(workspace_id));
CREATE INDEX idx_route_properties_coverage_pending ON route_properties(route_id)
    WHERE track_conversions <> track_conversions_sent;
