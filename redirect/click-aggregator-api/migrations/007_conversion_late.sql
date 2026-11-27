-- Events whose click was not found keep being re-checked for a while.
-- conversion_pending.resolved, the highest of which wins:
--   0  waiting for its click
--   1  late: counted as unattributed, still re-checked
--   2  done
-- Late rows normally end after 72 hours; their TTL is only a safety net.

ALTER TABLE conversion_pending MODIFY TTL
    toDateTime(inserted_at) + INTERVAL 1 DAY DELETE WHERE resolved = 2,
    toDateTime(inserted_at) + INTERVAL 4 DAY DELETE WHERE resolved = 1;
