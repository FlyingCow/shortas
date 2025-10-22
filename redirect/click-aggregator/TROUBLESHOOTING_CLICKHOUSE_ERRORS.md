# Troubleshooting ClickHouse Inserter Errors

## Error: "Cannot read all data. Bytes read: 17. Bytes expected: 117"

### Symptoms
- Periodic errors in logs
- No data appearing in `click_stream` table
- Error occurs at specific row numbers (e.g., "at row 8")

### Root Cause
**Missing or empty required fields** in clickstream data. The ClickHouse binary format expects:
- 7 required String fields: `id`, `owner_id`, `creator_id`, `route_id`, `workspace_id`, `dest`, `ip`
- 1 DateTime field: `created`

When any of these fields are empty or missing, the binary serialization fails with a byte count mismatch.

### Solution Applied

Added validation in `/redirect/click-aggregator/src/adapters/clickhouse/mod.rs`:

```rust
// Validate required fields - skip rows with empty required fields
if click.id.is_empty() || click.owner_id.is_empty() || click.creator_id.is_empty() 
    || click.route_id.is_empty() || click.workspace_id.is_empty() 
    || click.dest.is_empty() || click.ip.is_empty() {
    eprintln!(
        "[WARN] Skipping invalid clickstream item - missing required fields: ..."
    );
    return Ok(()); // Skip this record
}
```

### What This Means

**Invalid records will be skipped** and you'll see warnings in logs like:
```
[WARN] Skipping invalid clickstream item - missing required fields: 
id=abc123, owner_id=, creator_id=, route_id=, workspace_id=, dest=, ip=
```

This prevents the entire batch from failing due to one bad record.

## How to Fix Upstream Data Issues

### 1. Check Click-Tracker Configuration

The click-tracker should always populate these fields:

```rust
pub struct ClickStreamItem {
    pub id: String,              // ✅ Must not be empty
    pub owner_id: String,        // ✅ Must not be empty
    pub creator_id: String,      // ✅ Must not be empty
    pub route_id: String,        // ✅ Must not be empty
    pub workspace_id: String,    // ✅ Must not be empty
    pub dest: String,            // ✅ Must not be empty
    pub ip: String,              // ✅ Must not be empty
    pub created: DateTime<Utc>,  // ✅ Must be valid
    // ... optional fields with defaults
}
```

### 2. Verify Route Properties

Routes must have these properties set:
- `properties.owner_id`
- `properties.creator_id`  
- `properties.workspace_id`

### 3. Check User Initialization

Ensure the `/api/v1/user/initialize` endpoint is being called after user registration to create default workspace and settings.

## Monitoring

### Check Logs for Skipped Records

```bash
# In click-aggregator logs, look for:
grep "WARN.*Skipping invalid clickstream" /path/to/logs

# Or in real-time:
tail -f /path/to/logs | grep "WARN"
```

### Verify Data in ClickHouse

```sql
-- Check if data is being inserted
SELECT COUNT(*) FROM click_stream;

-- Check for missing workspace IDs (shouldn't happen now)
SELECT COUNT(*) FROM click_stream WHERE workspace_id = '';

-- Check recent inserts
SELECT * FROM click_stream ORDER BY created DESC LIMIT 10;
```

## Performance Improvements

### Batching Configuration

The inserter is configured to batch writes:

```rust
let inserter = client
    .inserter::<ClickStreamItemRow>(&settings.table)?
    .with_period(Some(Duration::from_millis(settings.period_millis)))
    .with_period_bias(settings.period_bias)
    .with_max_rows(settings.max_rows);
```

**Benefits:**
- Writes are buffered and sent in batches
- Auto-commits when `period_millis` expires OR `max_rows` is reached
- More efficient than committing after every single row

**Configuration Options** (in `config.toml`):
```toml
[clickstream_store]
period_millis = 5000  # Commit every 5 seconds
max_rows = 1000       # OR when 1000 rows accumulated
period_bias = 0.1     # Add 10% random jitter to spread load
```

## Complete Error Reference

### Error Pattern 1: Small byte mismatch (4 bytes)
```
Cannot read all data. Bytes read: 150. Bytes expected: 154
```
**Cause:** Type size mismatch (e.g., u128 vs u64)  
**Solution:** Use u64 for `session_clicks` ✅ FIXED

### Error Pattern 2: Large byte mismatch (100+ bytes)
```
Cannot read all data. Bytes read: 17. Bytes expected: 117
```
**Cause:** Missing required string fields  
**Solution:** Added validation to skip invalid records ✅ FIXED

### Error Pattern 3: Specific row failures
```
at row 8
```
**Cause:** One specific record has bad data  
**Solution:** Validation now skips that record, rest of batch proceeds ✅ FIXED

## Validation Rules

### Required Fields (non-empty)
- ✅ `id` - Unique identifier
- ✅ `owner_id` - Route owner
- ✅ `creator_id` - Route creator
- ✅ `route_id` - Route identifier
- ✅ `workspace_id` - Workspace identifier
- ✅ `dest` - Destination URL
- ✅ `ip` - Client IP address

### Optional Fields (defaults applied)
- `continent` → "_unknown"
- `country` → "_unknown"
- `location` → "_unknown"
- `os_family` → "_unknown"
- `os_version` → "_unknown"
- `user_agent_family` → "_unknown"
- `user_agent_version` → "_unknown"
- `device_brand` → "_unknown"
- `device_family` → "_unknown"
- `device_model` → "_unknown"
- `session_first` → Unix epoch (1970-01-01)
- `session_clicks` → 0
- `is_unique` → false (0)
- `is_bot` → false (0)

## Testing

### 1. Check Service Status
```bash
# Verify click-aggregator is running
ps aux | grep click-aggregator

# Check logs
tail -f /path/to/click-aggregator/logs
```

### 2. Send Test Click
```bash
# Click a short link
curl -L http://yourdomain.com/testlink

# Check if it appears in ClickHouse
clickhouse-client --query "SELECT * FROM click_stream ORDER BY created DESC LIMIT 1"
```

### 3. Monitor Error Rate
```bash
# Count errors in logs
grep "ERROR.*ClickHouse" /path/to/logs | wc -l

# Count warnings (skipped records)
grep "WARN.*Skipping invalid" /path/to/logs | wc -l
```

## Recovery Steps

If the click_stream table gets corrupted or needs reset:

```sql
-- Drop and recreate
DROP TABLE IF EXISTS click_stream;

-- Run migration
SOURCE /path/to/migrations/001_create_click_stream_table.sql;

-- Verify schema
DESCRIBE click_stream;
```

## Related Files

- `/redirect/click-aggregator/src/adapters/clickhouse/mod.rs` - Inserter implementation with validation
- `/redirect/click-aggregator-api/migrations/001_create_click_stream_table.sql` - Table schema
- `/redirect/click-tracker/src/core/mod.rs` - ClickStreamItem source structure

## Future Improvements

1. **Metrics**: Add counters for skipped records
2. **Dead Letter Queue**: Store invalid records for later review
3. **Alerting**: Send notifications when skip rate exceeds threshold
4. **Validation Service**: Validate data before it reaches aggregator

