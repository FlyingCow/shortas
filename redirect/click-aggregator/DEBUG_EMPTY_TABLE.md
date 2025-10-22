# Debugging: Empty click_stream Table

## Current Situation
✅ No errors appearing in logs  
❌ click_stream table is empty  
❓ Need to trace where data is getting lost  

## Debug Steps

### Step 1: Check if Clicks Are Being Generated

```bash
# Test by clicking a short link
curl -L http://localhost:3001/testlink

# Or use your actual domain
curl -L http://yourdomain.com/yourlink
```

### Step 2: Check Click-Tracker Logs

```bash
# Look for click events being tracked
docker logs click-tracker 2>&1 | grep -i "click\|track" | tail -20

# Or if running locally:
tail -f /path/to/click-tracker/logs
```

**What to look for:**
- Click events being created
- Route information being loaded
- IP addresses being captured

### Step 3: Check Kafka/Fluvio Queue

The click-tracker sends events to Kafka/Fluvio. Check if events are there:

```bash
# For Kafka:
docker exec -it kafka kafka-console-consumer \
  --bootstrap-server localhost:9092 \
  --topic click-aggs-local \
  --from-beginning \
  --max-messages 5

# For Fluvio:
fluvio consume click-aggs-local --from-beginning -d
```

**Expected:** You should see JSON clickstream events

**If empty:** Click-tracker isn't sending events → check click-tracker configuration

### Step 4: Check Click-Aggregator Is Running

```bash
# Check if the service is running
ps aux | grep click-aggregator

# Check if it's consuming from Kafka/Fluvio
docker logs click-aggregator 2>&1 | tail -50
```

### Step 5: Check Debug Logs (NEW)

With the new debug logging, you should see:

```bash
# Watch aggregator logs in real-time
docker logs -f click-aggregator 2>&1

# Or locally:
tail -f /path/to/click-aggregator/logs
```

**Look for these messages:**

✅ **Valid records being written:**
```
[DEBUG] Writing clickstream: id=abc123, route_id=xyz, workspace_id=ws-123, dest=https://..., ip=1.2.3.4
[DEBUG] Successfully queued clickstream record to inserter buffer
```

⚠️ **Invalid records being skipped:**
```
[WARN] Skipping invalid clickstream item - missing required fields: id=..., owner_id=, creator_id=...
```

❌ **Errors:**
```
[ERROR] Failed to write to ClickHouse inserter: ...
[ERROR] Failed to commit to ClickHouse: ...
```

### Step 6: Check ClickHouse Connection

```bash
# Test connection
docker exec -it clickhouse clickhouse-client --query "SELECT 1"

# Check if database exists
docker exec -it clickhouse clickhouse-client --query "SHOW DATABASES"

# Check if table exists
docker exec -it clickhouse clickhouse-client --query "SHOW TABLES FROM shortas"

# Check table schema
docker exec -it clickhouse clickhouse-client --query "DESCRIBE shortas.click_stream"
```

### Step 7: Manually Insert Test Data

```bash
# Insert a test record directly to verify table works
docker exec -it clickhouse clickhouse-client --query "
INSERT INTO shortas.click_stream 
(id, owner_id, creator_id, route_id, workspace_id, created, dest, ip) 
VALUES 
('test-id', 'owner-123', 'creator-123', 'route-123', 'workspace-123', now(), 'https://example.com', '127.0.0.1')
"

# Check if it was inserted
docker exec -it clickhouse clickhouse-client --query "
SELECT * FROM shortas.click_stream WHERE id = 'test-id'
"
```

**If this works:** ClickHouse is fine, issue is with aggregator  
**If this fails:** Check ClickHouse table schema

### Step 8: Check Inserter Configuration

The inserter should auto-commit based on these settings (from config/development.toml):

```toml
max_rows = 100_000      # Commit when 100k rows accumulated
period_millis = 100     # OR commit every 100ms
period_bias = 0.1       # Add 10% jitter
```

**This means:** Data should commit every ~100ms even with just 1 row!

## Common Issues & Solutions

### Issue 1: All Records Have Empty Required Fields

**Symptom:** Lots of `[WARN] Skipping invalid clickstream item` in logs

**Cause:** Routes don't have `workspace_id`, `owner_id`, or `creator_id` in properties

**Solution:**
```sql
-- Check routes without workspace
SELECT COUNT(*) FROM "Routes" 
WHERE "Properties" IS NULL 
OR "Properties"->>'WorkspaceId' IS NULL 
OR "Properties"->>'WorkspaceId' = '';

-- Update existing routes (if you have a default workspace)
UPDATE "Routes" 
SET "Properties" = jsonb_set(
  COALESCE("Properties", '{}'::jsonb),
  '{WorkspaceId}',
  '"your-default-workspace-id"'::jsonb
)
WHERE "Properties"->>'WorkspaceId' IS NULL;
```

### Issue 2: No Clicks Being Generated

**Symptom:** No logs at all in click-aggregator

**Cause:** Click-tracker isn't sending events to Kafka/Fluvio

**Solution:**
1. Check click-tracker is running
2. Check Kafka/Fluvio is running and reachable
3. Verify topic names match in both services
4. Check click-tracker configuration

### Issue 3: Kafka/Fluvio Connection Issues

**Symptom:** Aggregator starts but no consumption happens

**Cause:** Can't connect to message queue

**Solution:**
```bash
# Check Kafka is accessible
docker exec -it click-aggregator ping kafka

# Check Fluvio is accessible
fluvio cluster status

# Verify topic exists
docker exec -it kafka kafka-topics --list --bootstrap-server localhost:9092
```

### Issue 4: ClickHouse Connection Issues

**Symptom:** Errors about connection refused

**Cause:** Can't connect to ClickHouse

**Solution:**
```bash
# Check ClickHouse is running
docker ps | grep clickhouse

# Test connection from aggregator container
docker exec -it click-aggregator curl http://clickhouse:8123

# Check firewall/network
docker network inspect shortas_network
```

## Verification Checklist

Run through this checklist:

- [ ] Click-tracker is running
- [ ] Kafka/Fluvio is running
- [ ] Click-aggregator is running
- [ ] ClickHouse is running
- [ ] Routes have workspace_id in properties
- [ ] Users have been initialized with workspaces
- [ ] Message queue has clickstream events
- [ ] Debug logs show `[DEBUG] Writing clickstream`
- [ ] No `[ERROR]` messages in logs
- [ ] Table schema matches expected structure
- [ ] Manual insert test works

## Expected Log Flow

When everything is working, you should see:

```
1. Click happens on short link

2. Click-tracker logs:
   "Processing click for route xyz"
   "Publishing clickstream event"

3. Kafka/Fluvio has event in queue

4. Click-aggregator logs:
   [DEBUG] Writing clickstream: id=..., route_id=..., workspace_id=...
   [DEBUG] Successfully queued clickstream record to inserter buffer

5. ClickHouse (after ~100ms):
   SELECT COUNT(*) FROM click_stream;
   -- Returns: 1 (or more)
```

## Quick Test Script

```bash
#!/bin/bash
echo "=== Testing Clickstream Flow ==="

echo "1. Clicking test link..."
curl -L http://localhost:3001/testlink -o /dev/null -s

echo "2. Waiting 2 seconds for processing..."
sleep 2

echo "3. Checking ClickHouse..."
COUNT=$(docker exec -it clickhouse clickhouse-client --query "SELECT COUNT(*) FROM shortas.click_stream" 2>/dev/null | tr -d '\r')

echo "Records in click_stream: $COUNT"

if [ "$COUNT" -gt 0 ]; then
    echo "✅ SUCCESS! Data is flowing to ClickHouse"
    docker exec -it clickhouse clickhouse-client --query "SELECT id, route_id, workspace_id, created FROM shortas.click_stream ORDER BY created DESC LIMIT 3"
else
    echo "❌ FAILED! No data in ClickHouse"
    echo ""
    echo "Checking aggregator logs for errors..."
    docker logs click-aggregator 2>&1 | grep -E "\[ERROR\]|\[WARN\]" | tail -10
fi
```

## Next Steps

Based on what you find:

1. **No events in Kafka/Fluvio** → Fix click-tracker
2. **Events in queue but not consumed** → Fix aggregator consumer
3. **Consumed but all skipped** → Fix route properties (workspace_id, etc.)
4. **No errors, data written, but table empty** → Check ClickHouse table/database
5. **Connection errors** → Fix network/configuration

## Get Help

If still stuck, provide:
1. Output of step 5 (debug logs)
2. Count of messages in Kafka/Fluvio
3. Sample route from database (with properties)
4. ClickHouse table schema output

