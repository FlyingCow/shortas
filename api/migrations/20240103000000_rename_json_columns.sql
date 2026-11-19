-- Rename JSONB columns to match Rust code expectations
-- Also change from JSONB to TEXT for compatibility with string serialization

ALTER TABLE route_properties
    RENAME COLUMN scripts TO scripts_json;

ALTER TABLE route_properties
    RENAME COLUMN tags TO tags_json;

ALTER TABLE route_properties
    RENAME COLUMN custom TO custom_json;

ALTER TABLE route_properties
    RENAME COLUMN native TO native_json;

ALTER TABLE route_properties
    RENAME COLUMN bundling TO bundling_json;

ALTER TABLE route_properties
    RENAME COLUMN qr_settings TO qr_settings_json;

-- Convert JSONB to TEXT
ALTER TABLE route_properties
    ALTER COLUMN scripts_json TYPE TEXT USING scripts_json::TEXT,
    ALTER COLUMN tags_json TYPE TEXT USING tags_json::TEXT,
    ALTER COLUMN custom_json TYPE TEXT USING custom_json::TEXT,
    ALTER COLUMN native_json TYPE TEXT USING native_json::TEXT,
    ALTER COLUMN bundling_json TYPE TEXT USING bundling_json::TEXT,
    ALTER COLUMN qr_settings_json TYPE TEXT USING qr_settings_json::TEXT;

-- Also convert routes.policy_json to TEXT if it's JSONB
ALTER TABLE routes
    ALTER COLUMN policy_json TYPE TEXT USING policy_json::TEXT;
