-- Conversions Tracking Tables
-- Extends the click tracking system with conversion attribution and analytics

-- 1. Conversions Table
-- Stores individual conversion events with metadata
CREATE TABLE IF NOT EXISTS conversions (
    -- Core identifiers
    id String,
    owner_id String,
    creator_id String,
    route_id String,
    workspace_id String,
    
    -- Conversion details
    conversion_type String,  -- 'purchase', 'signup', 'download', 'form_submission', 'custom'
    conversion_name String,   -- User-defined name for the conversion
    conversion_value Decimal64(2) DEFAULT 0,  -- Monetary value (for purchases)
    
    -- Attribution data
    attributed_click_id String,  -- The click that led to this conversion
    attribution_type String DEFAULT 'direct',  -- 'direct', 'session', 'time_based', 'multi_touch'
    attribution_window_hours UInt32 DEFAULT 24,  -- Time window for attribution
    
    -- User and session data
    user_id String DEFAULT '_unknown',  -- If user is identified
    session_id String DEFAULT '_unknown',
    ip String,
    
    -- Geographic data (defaults to '_unknown')
    continent String DEFAULT '_unknown',
    country String DEFAULT '_unknown',
    location String DEFAULT '_unknown',
    
    -- Device data (defaults to '_unknown')
    device_family String DEFAULT '_unknown',
    device_brand String DEFAULT '_unknown',
    device_model String DEFAULT '_unknown',
    os_family String DEFAULT '_unknown',
    os_version String DEFAULT '_unknown',
    user_agent_family String DEFAULT '_unknown',
    user_agent_version String DEFAULT '_unknown',
    
    -- Timestamps (DateTime64(3) for millisecond precision)
    created DateTime64(3),
    click_created DateTime64(3),  -- When the attributed click happened
    
    -- Additional metadata
    metadata String DEFAULT '{}',  -- JSON string for custom data
    referrer String DEFAULT '_unknown',
    
    -- Flags
    is_unique UInt8 DEFAULT 1,  -- First conversion for this user/route combination
    
    -- Indexing fields
    date Date MATERIALIZED toDate(created),
    hour DateTime MATERIALIZED toStartOfHour(created)
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(created)
ORDER BY (owner_id, route_id, created, conversion_type)
TTL created + INTERVAL 2 YEAR;

-- 2. Conversion Attribution Table
-- Links conversions to their originating clicks for detailed attribution analysis
CREATE TABLE IF NOT EXISTS conversion_attribution (
    -- Core identifiers
    conversion_id String,
    click_id String,
    owner_id String,
    route_id String,
    workspace_id String,
    
    -- Attribution details
    attribution_weight Decimal32(4) DEFAULT 1.0,  -- Weight of this click in attribution (for multi-touch)
    attribution_position UInt8 DEFAULT 1,  -- Position in attribution chain (1=first, 2=second, etc.)
    attribution_type String DEFAULT 'direct',  -- Type of attribution
    
    -- Time data
    click_created DateTime64(3),
    conversion_created DateTime64(3),
    time_to_conversion_seconds UInt32,  -- Seconds between click and conversion
    
    -- User journey data
    session_id String DEFAULT '_unknown',
    user_id String DEFAULT '_unknown',
    
    -- Geographic data
    country String DEFAULT '_unknown',
    device_family String DEFAULT '_unknown',
    user_agent_family String DEFAULT '_unknown',
    
    -- Indexing fields
    date Date MATERIALIZED toDate(conversion_created)
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(conversion_created)
ORDER BY (owner_id, route_id, conversion_created, attribution_position)
TTL conversion_created + INTERVAL 2 YEAR;

-- 3. Conversion Funnels Table
-- Tracks multi-step conversion processes
CREATE TABLE IF NOT EXISTS conversion_funnels (
    -- Core identifiers
    id String,
    owner_id String,
    workspace_id String,
    
    -- Funnel definition
    funnel_name String,
    funnel_steps Array(String),  -- Array of step names: ['view', 'add_to_cart', 'checkout', 'purchase']
    
    -- User journey
    user_id String DEFAULT '_unknown',
    session_id String DEFAULT '_unknown',
    route_id String,
    
    -- Step completion data
    step_name String,
    step_position UInt8,  -- Position in funnel (1, 2, 3, etc.)
    step_completed UInt8 DEFAULT 1,
    step_value Decimal64(2) DEFAULT 0,  -- Value at this step
    
    -- Timestamps
    step_created DateTime64(3),
    funnel_started DateTime64(3),
    
    -- Additional data
    metadata String DEFAULT '{}',
    
    -- Indexing fields
    date Date MATERIALIZED toDate(step_created)
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(step_created)
ORDER BY (owner_id, funnel_name, user_id, step_position)
TTL step_created + INTERVAL 1 YEAR;

-- 4. Conversion Goals Table
-- Defines conversion goals and targets for routes
CREATE TABLE IF NOT EXISTS conversion_goals (
    -- Core identifiers
    id String,
    owner_id String,
    workspace_id String,
    route_id String,
    
    -- Goal definition
    goal_name String,
    goal_type String,  -- 'conversion_rate', 'revenue', 'custom'
    target_value Decimal64(2) DEFAULT 0,
    target_period String DEFAULT 'daily',  -- 'hourly', 'daily', 'weekly', 'monthly'
    
    -- Configuration
    attribution_window_hours UInt32 DEFAULT 24,
    is_active UInt8 DEFAULT 1,
    
    -- Timestamps
    created DateTime64(3),
    updated DateTime64(3),
    
    -- Indexing fields
    date Date MATERIALIZED toDate(created)
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(created)
ORDER BY (owner_id, route_id, goal_name)
TTL created + INTERVAL 1 YEAR;
