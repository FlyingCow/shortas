-- Add missing columns to route_domains table

ALTER TABLE route_domains
    ADD COLUMN IF NOT EXISTS verification_reason VARCHAR(255) DEFAULT 'not_checked',
    ADD COLUMN IF NOT EXISTS last_verification_check TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS next_verification_check TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS custom_index_url TEXT,
    ADD COLUMN IF NOT EXISTS custom_not_found_url TEXT;
