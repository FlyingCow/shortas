-- Check Routes Configuration for Clickstream Compatibility
-- Run this against your PostgreSQL database to see if routes are properly configured

-- 1. Count total routes
SELECT 'Total Routes' as check_name, COUNT(*) as count FROM "Routes";

-- 2. Count routes WITHOUT workspace_id
SELECT 'Routes WITHOUT workspace_id' as check_name, COUNT(*) as count 
FROM "Routes" 
WHERE "Properties" IS NULL 
   OR "Properties"->>'WorkspaceId' IS NULL 
   OR "Properties"->>'WorkspaceId' = '';

-- 3. Count routes WITHOUT owner_id
SELECT 'Routes WITHOUT owner_id' as check_name, COUNT(*) as count 
FROM "Routes" 
WHERE "Properties" IS NULL 
   OR "Properties"->>'OwnerId' IS NULL 
   OR "Properties"->>'OwnerId' = '';

-- 4. Count routes WITHOUT creator_id
SELECT 'Routes WITHOUT creator_id' as check_name, COUNT(*) as count 
FROM "Routes" 
WHERE "Properties" IS NULL 
   OR "Properties"->>'CreatorId' IS NULL 
   OR "Properties"->>'CreatorId' = '';

-- 5. Show sample routes with missing fields
SELECT 
    "Id",
    "Link",
    "Dest",
    "Properties"->>'WorkspaceId' as workspace_id,
    "Properties"->>'OwnerId' as owner_id,
    "Properties"->>'CreatorId' as creator_id
FROM "Routes" 
WHERE "Properties" IS NULL 
   OR "Properties"->>'WorkspaceId' IS NULL 
   OR "Properties"->>'WorkspaceId' = ''
   OR "Properties"->>'OwnerId' IS NULL
   OR "Properties"->>'OwnerId' = ''
   OR "Properties"->>'CreatorId' IS NULL
   OR "Properties"->>'CreatorId' = ''
LIMIT 10;

-- 6. Show sample VALID routes (that should work with clickstream)
SELECT 
    "Id",
    "Link",
    "Dest",
    "Properties"->>'WorkspaceId' as workspace_id,
    "Properties"->>'OwnerId' as owner_id,
    "Properties"->>'CreatorId' as creator_id
FROM "Routes" 
WHERE "Properties" IS NOT NULL
  AND "Properties"->>'WorkspaceId' IS NOT NULL
  AND "Properties"->>'WorkspaceId' != ''
  AND "Properties"->>'OwnerId' IS NOT NULL
  AND "Properties"->>'OwnerId' != ''
  AND "Properties"->>'CreatorId' IS NOT NULL
  AND "Properties"->>'CreatorId' != ''
LIMIT 5;

-- 7. Count workspaces
SELECT 'Total Workspaces' as check_name, COUNT(*) as count FROM "Workspaces";

-- 8. Count users with workspaces
SELECT 'Users with Workspaces' as check_name, COUNT(DISTINCT "UserId") as count FROM "UserWorkspaces";

-- 9. Show workspace assignments
SELECT 
    uw."UserId",
    uw."WorkspaceId",
    w."Name" as workspace_name,
    uw."Role"
FROM "UserWorkspaces" uw
JOIN "Workspaces" w ON w."Id" = uw."WorkspaceId"
LIMIT 10;

