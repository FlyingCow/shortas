-- Routes whose conversion coverage has not been sent to the router yet.
-- Normally none, so the coverage sync's checks read almost nothing.
CREATE INDEX IF NOT EXISTS idx_route_properties_pending_coverage
    ON route_properties (route_id)
    WHERE track_conversions <> track_conversions_sent;
