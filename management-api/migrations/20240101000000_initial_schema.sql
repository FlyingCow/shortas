-- Initial schema for management-api

-- Workspaces
CREATE TABLE IF NOT EXISTS workspaces (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    type VARCHAR(50) NOT NULL DEFAULT 'Personal',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_workspaces_name ON workspaces(name);

-- User-Workspace relationships
CREATE TABLE IF NOT EXISTS user_workspaces (
    id UUID PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL DEFAULT 'Member',
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, workspace_id)
);

CREATE INDEX idx_user_workspaces_user ON user_workspaces(user_id);
CREATE INDEX idx_user_workspaces_workspace ON user_workspaces(workspace_id);

-- Route Domains
CREATE TABLE IF NOT EXISTS route_domains (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    owner_id VARCHAR(255) NOT NULL,
    is_shared BOOLEAN NOT NULL DEFAULT FALSE,
    verification_status VARCHAR(50) NOT NULL DEFAULT 'Pending',
    verification_token VARCHAR(255),
    verified_at TIMESTAMPTZ,
    not_found_page TEXT,
    index_page TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_route_domains_owner ON route_domains(owner_id);
CREATE INDEX idx_route_domains_name ON route_domains(LOWER(name));
CREATE INDEX idx_route_domains_shared ON route_domains(is_shared) WHERE is_shared = true;
CREATE INDEX idx_route_domains_verification ON route_domains(verification_status);

-- Certificates
CREATE TABLE IF NOT EXISTS certificates (
    id UUID PRIMARY KEY,
    key TEXT NOT NULL,
    cert TEXT NOT NULL,
    ocsp_resp BYTEA,
    owner_id VARCHAR(255) NOT NULL,
    domain_id UUID REFERENCES route_domains(id) ON DELETE CASCADE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_certificates_owner ON certificates(owner_id);
CREATE INDEX idx_certificates_domain ON certificates(domain_id);
CREATE INDEX idx_certificates_expires ON certificates(expires_at);

-- Routes
CREATE TABLE IF NOT EXISTS routes (
    id UUID PRIMARY KEY,
    switch VARCHAR(255) NOT NULL,
    link VARCHAR(2048) NOT NULL,
    dest VARCHAR(2048),
    dest_format VARCHAR(50) NOT NULL DEFAULT 'Http',
    code SMALLINT,
    ttl BIGINT,
    status VARCHAR(50) NOT NULL DEFAULT 'Active',
    terminal VARCHAR(50) NOT NULL DEFAULT 'External',
    policy_json JSONB NOT NULL DEFAULT '{"type": "Basic"}',
    domain_id UUID REFERENCES route_domains(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_routes_switch ON routes(switch);
CREATE INDEX idx_routes_link ON routes(link);
CREATE INDEX idx_routes_domain ON routes(domain_id);
CREATE INDEX idx_routes_switch_link ON routes(switch, link);
CREATE UNIQUE INDEX idx_routes_domain_link_switch ON routes(domain_id, link, switch);

-- Route Properties
CREATE TABLE IF NOT EXISTS route_properties (
    id UUID PRIMARY KEY,
    route_id UUID NOT NULL REFERENCES routes(id) ON DELETE CASCADE,
    domain_id VARCHAR(255),
    owner_id VARCHAR(255),
    creator_id VARCHAR(255),
    workspace_id VARCHAR(255),
    scripts JSONB,
    tags JSONB,
    custom JSONB,
    native JSONB,
    bundling JSONB,
    qr_settings JSONB,
    opengraph BOOLEAN NOT NULL DEFAULT FALSE,
    allow_debug BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_route_properties_route ON route_properties(route_id);
CREATE INDEX idx_route_properties_owner ON route_properties(owner_id);
CREATE INDEX idx_route_properties_workspace ON route_properties(workspace_id);

-- Outbox Messages (for transactional outbox pattern)
CREATE TABLE IF NOT EXISTS outbox_messages (
    id UUID PRIMARY KEY,
    message_type VARCHAR(100) NOT NULL,
    payload TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'Pending',
    retry_count INTEGER NOT NULL DEFAULT 0,
    max_retries INTEGER NOT NULL DEFAULT 3,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ,
    next_retry_at TIMESTAMPTZ
);

CREATE INDEX idx_outbox_status ON outbox_messages(status);
CREATE INDEX idx_outbox_pending ON outbox_messages(status, next_retry_at) WHERE status = 'Pending';
CREATE INDEX idx_outbox_created ON outbox_messages(created_at);
