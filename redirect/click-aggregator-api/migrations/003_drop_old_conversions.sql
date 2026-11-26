-- Migration 003: Remove the old conversion tables and views
-- Earlier versions of 003 and 004 created them. Conversion events now live in 006.

DROP VIEW IF EXISTS conversion_rates_mv;
DROP VIEW IF EXISTS conversion_attribution_mv;
DROP VIEW IF EXISTS conversion_funnels_mv;
DROP VIEW IF EXISTS conversion_revenue_mv;
DROP VIEW IF EXISTS conversion_geographic_mv;
DROP VIEW IF EXISTS conversion_device_mv;
DROP VIEW IF EXISTS conversion_hourly_mv;
DROP VIEW IF EXISTS conversion_goals_performance_mv;
DROP VIEW IF EXISTS conversion_multi_touch_mv;
DROP VIEW IF EXISTS conversion_cohort_mv;

DROP TABLE IF EXISTS conversions;
DROP TABLE IF EXISTS conversion_attribution;
DROP TABLE IF EXISTS conversion_funnels;
DROP TABLE IF EXISTS conversion_goals;
