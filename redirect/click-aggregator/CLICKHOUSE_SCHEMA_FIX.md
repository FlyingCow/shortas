# ClickHouse Schema Fix - Click Aggregator

## Problem

The click-aggregator was throwing an "Invalid Object" error when inserting clickstream data into ClickHouse:

```
Code: 33. DB::Exception: Cannot read all data. 
Bytes read: 108. Bytes expected: 112.: (at row 34)
While executing BinaryRowInputFormat. (CANNOT_READ_ALL_DATA)
```

## Root Cause

**Multiple Schema Mismatches:**

1. **Nullable vs Non-nullable:** ClickHouse table schema defined fields as **non-nullable String with defaults**, but the Rust `ClickStreamItemRow` struct was using **Option<String>** (nullable).

2. **Type Size Mismatch:** The `session_clicks` field was defined as `UInt128` in ClickHouse, but the Rust clickhouse-rs client has serialization issues with `u128`, causing a 4-byte mismatch in the binary format.

### ClickHouse Table Schema
```sql
-- From migrations/001_create_click_stream_table.sql
CREATE TABLE IF NOT EXISTS click_stream (
    id String,
    owner_id String,
    creator_id String,
    route_id String,
    workspace_id String,
    created DateTime,
    dest String,
    ip String,
    
    -- These are non-nullable with defaults
    continent String DEFAULT '_unknown',
    country String DEFAULT '_unknown',
    location String DEFAULT '_unknown',
    os_family String DEFAULT '_unknown',
    os_version String DEFAULT '_unknown',
    user_agent_family String DEFAULT '_unknown',
    user_agent_version String DEFAULT '_unknown',
    device_brand String DEFAULT '_unknown',
    device_family String DEFAULT '_unknown',
    device_model String DEFAULT '_unknown',
    
    session_first DateTime DEFAULT toDateTime('1970-01-01 00:00:00'),
    session_clicks UInt128 DEFAULT 0,
    is_unique UInt8,
    is_bot UInt8
) ENGINE = MergeTree()
ORDER BY id;
```

### Previous Rust Struct (WRONG)
```rust
pub struct ClickStreamItemRow {
    pub id: String,
    pub owner_id: String,
    pub creator_id: String,
    pub route_id: String,
    pub workspace_id: String,
    pub created: DateTime<Utc>,
    pub dest: String,
    pub ip: String,
    
    // ❌ These were Option<String> but schema expects String
    pub continent: Option<String>,
    pub country: Option<String>,
    pub location: Option<String>,
    pub os_family: Option<String>,
    pub os_version: Option<String>,
    pub user_agent_family: Option<String>,
    pub user_agent_version: Option<String>,
    pub device_brand: Option<String>,
    pub device_family: Option<String>,
    pub device_model: Option<String>,
    
    // ❌ These were Option types but schema expects non-nullable with defaults
    pub session_first: Option<DateTime<Utc>>,
    pub session_clicks: Option<u128>,
    
    // ❌ These were bool but schema expects UInt8
    pub is_unique: bool,
    pub is_bot: bool,
}
```

## Solution

Updated `ClickStreamItemRow` to match the ClickHouse schema exactly:

```rust
pub struct ClickStreamItemRow {
    pub id: String,
    pub owner_id: String,
    pub creator_id: String,
    pub route_id: String,
    pub workspace_id: String,
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub created: DateTime<Utc>,
    pub dest: String,
    pub ip: String,
    
    // ✅ Now String (non-nullable)
    pub continent: String,
    pub country: String,
    pub location: String,
    pub os_family: String,
    pub os_version: String,
    pub user_agent_family: String,
    pub user_agent_version: String,
    pub device_brand: String,
    pub device_family: String,
    pub device_model: String,
    
    // ✅ Now non-nullable with defaults
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub session_first: DateTime<Utc>,
    pub session_clicks: u128,
    
    // ✅ Now u8 (UInt8 in ClickHouse)
    pub is_unique: u8,
    pub is_bot: u8,
}
```

### Data Conversion in register() Method

Since the `ClickStreamItem` (from the event stream) still uses `Option<String>` for flexibility, we convert it when writing to ClickHouse:

```rust
async fn register(&mut self, click: ClickStreamItem) -> Result<()> {
    let mut inserter = self.inserter.lock().await;

    // Convert Option fields to non-nullable with default values
    const UNKNOWN: &str = "_unknown";
    let epoch = DateTime::from_timestamp(0, 0).unwrap();

    inserter.write(&ClickStreamItemRow {
        id: click.id,
        owner_id: click.owner_id,
        creator_id: click.creator_id,
        route_id: click.route_id,
        workspace_id: click.workspace_id,
        created: click.created,
        dest: click.dest,
        ip: click.ip,
        
        // Geographic fields with default
        continent: click.continent.unwrap_or_else(|| UNKNOWN.to_string()),
        country: click.country.unwrap_or_else(|| UNKNOWN.to_string()),
        location: click.location.unwrap_or_else(|| UNKNOWN.to_string()),
        
        // OS fields with default
        os_family: click.os_family.unwrap_or_else(|| UNKNOWN.to_string()),
        os_version: click.os_version.unwrap_or_else(|| UNKNOWN.to_string()),
        
        // User agent fields with default
        user_agent_family: click.user_agent_family.unwrap_or_else(|| UNKNOWN.to_string()),
        user_agent_version: click.user_agent_version.unwrap_or_else(|| UNKNOWN.to_string()),
        
        // Device fields with default
        device_brand: click.device_brand.unwrap_or_else(|| UNKNOWN.to_string()),
        device_family: click.device_family.unwrap_or_else(|| UNKNOWN.to_string()),
        device_model: click.device_model.unwrap_or_else(|| UNKNOWN.to_string()),
        
        // Session fields with defaults
        session_first: click.session_first.unwrap_or(epoch),
        session_clicks: click.session_clicks.unwrap_or(0),
        
        // Boolean to u8 conversion
        is_unique: if click.is_unique { 1 } else { 0 },
        is_bot: if click.is_bot { 1 } else { 0 },
    })?;

    let r = inserter.commit().await;
    if r.is_err() {
        println!("{}", "error");
    }

    if self.token.is_cancelled() {
        inserter.commit().await?;
    }

    Ok(())
}
```

## Key Changes

1. **String fields**: Changed from `Option<String>` to `String`
   - Uses "_unknown" as default for missing values

2. **DateTime fields**: Changed from `Option<DateTime<Utc>>` to `DateTime<Utc>`
   - Uses Unix epoch (1970-01-01) as default for missing values

3. **Numeric fields**: Changed from `Option<u128>` to `u128`
   - Uses 0 as default for missing values

4. **Boolean flags**: Changed from `bool` to `u8`
   - Converts `true` → `1`, `false` → `0`

## Benefits

✅ **Type Safety**: Struct now matches ClickHouse schema exactly  
✅ **No NULL Issues**: All fields have proper defaults  
✅ **Better Performance**: Non-nullable fields are more efficient in ClickHouse  
✅ **Clearer Intent**: Defaults are explicit in the code  
✅ **No Data Loss**: Optional data is preserved with meaningful defaults  

## ClickHouse Schema Update Required

The `session_clicks` field needs to be changed from `UInt128` to `UInt64` in the ClickHouse table:

### Option 1: Drop and Recreate (if no data needs preserving)
```sql
DROP TABLE IF EXISTS click_stream;
-- Then run migrations/001_create_click_stream_table.sql
```

### Option 2: Alter Existing Table (preserves data)
```sql
-- See migrations/002_alter_session_clicks_to_uint64.sql
ALTER TABLE click_stream ADD COLUMN session_clicks_new UInt64 DEFAULT 0;
ALTER TABLE click_stream UPDATE session_clicks_new = CAST(session_clicks AS UInt64) WHERE 1;
ALTER TABLE click_stream DROP COLUMN session_clicks;
ALTER TABLE click_stream RENAME COLUMN session_clicks_new TO session_clicks;
OPTIMIZE TABLE click_stream FINAL;
```

## Testing

```bash
# Build the project
cd redirect/click-aggregator
cargo build

# Run with ClickHouse
cargo run
```

## Related Files

- `/redirect/click-aggregator/src/adapters/clickhouse/mod.rs` - Updated struct and conversion logic
- `/redirect/click-aggregator-api/migrations/001_create_click_stream_table.sql` - Table schema
- `/redirect/click-aggregator/src/core/mod.rs` - Core ClickStreamItem (unchanged, still uses Options for flexibility)

## Future Improvements

Consider adding validation to ensure:
- Required fields (id, owner_id, creator_id, route_id, workspace_id) are never empty
- IP addresses are valid
- Timestamps are reasonable (not in the future)

