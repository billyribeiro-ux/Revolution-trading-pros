# Running Database Migrations - AI Agent Guide

## Problem
When SQLx compile-time verification fails with errors like:
```
error: error returned from database: type "cms_content_type" does not exist
error: error returned from database: relation "cms_reusable_blocks" does not exist
```

This means **database migrations have not been run on the production database**.

## Root Cause
SQLx performs compile-time verification by connecting to the database specified in `DATABASE_URL` during compilation. If the schema doesn't match the queries in the code, compilation fails.

## Solution: Run Migrations via API Endpoint

### Method 1: Using the `/init-db` Endpoint (Recommended)

This endpoint runs all pending migrations and doesn't require authentication:

```bash
curl -X POST https://revolution-trading-pros-api.fly.dev/init-db
```

**Expected Response:**
```json
{
  "success": true,
  "message": "Migrations and bootstrap completed from environment variables"
}
```

### Method 2: Using SQLx CLI (If Available in Container)

If you have shell access to the container with sqlx installed:

```bash
fly ssh console -a revolution-trading-pros-api -C "sqlx migrate run --source /app/migrations"
```

**Note:** This usually fails because `sqlx` CLI is not included in the production Docker image.

### Method 3: Local Migration (Development Only)

For local development with direct database access:

```bash
cd api
sqlx migrate run --database-url "postgres://postgres:password@localhost:5432/revolution"
```

## Verification

After running migrations, verify they succeeded:

1. **Check the endpoint response** - Should return `success: true`
2. **Trigger a new CI build** - Push an empty commit to re-run GitHub Actions:
   ```bash
   git commit --allow-empty -m "chore(ci): trigger build after database migrations"
   git push
   ```
3. **Monitor GitHub Actions** - The build should now pass without SQLx errors

## What Gets Created

The migrations create the following CMS v2 schema:

### Enums
- `cms_content_type` - Content type enum (article, page, etc.)
- `cms_content_status` - Workflow status enum

### Tables
- `cms_content` - Main content table
- `cms_revisions` - Content revision history
- `cms_reusable_blocks` - Reusable content blocks
- `cms_reusable_block_usage` - Block usage tracking
- `cms_users` - CMS user management
- `cms_webhooks` - Webhook configurations
- `cms_webhook_deliveries` - Webhook delivery logs
- `cms_navigation_menus` - Navigation menu system
- `cms_ai_assist_history` - AI assistance tracking

### Functions
- `cms_get_content_full()` - Content retrieval with joins
- Various trigger functions for audit logging

## Important Notes

1. **Production Database Access** - The Fly.io database (`revolution-db.flycast`) is only accessible from within the Fly.io network, not from local machines.

2. **Migration Files Location** - All migration SQL files are in `/api/migrations/`

3. **SQLx Offline Mode** - If you need to bypass compile-time checks temporarily, set:
   ```yaml
   env:
     SQLX_OFFLINE: true
   ```
   in `.github/workflows/deploy-cloudflare.yml`

4. **Migration Order** - Migrations run in numerical order (001, 002, 003, etc.)

## Troubleshooting

### Error: "Missing or invalid authorization header"
- You tried `/run-migrations` which requires admin auth
- Use `/init-db` instead (no auth required)

### Error: "nodename nor servname provided"
- You tried to connect to `.flycast` domain from local machine
- Use the API endpoint method instead

### Error: "executable file not found in $PATH"
- The production container doesn't have `sqlx` CLI installed
- Use the API endpoint method instead

## For AI Agents

When you encounter SQLx compilation errors about missing database objects:

1. ✅ **DO**: Run `curl -X POST https://revolution-trading-pros-api.fly.dev/init-db`
2. ✅ **DO**: Verify the response shows `success: true`
3. ✅ **DO**: Trigger a new CI build with an empty commit
4. ❌ **DON'T**: Try to run `sqlx migrate run` locally against production
5. ❌ **DON'T**: Try to SSH into Fly.io and run migrations manually
6. ❌ **DON'T**: Modify the code to work around missing schema

## Reference

- Migration endpoint code: `/api/src/routes/health.rs` (lines 331-345)
- Migration files: `/api/migrations/*.sql`
- Database config: `/api/src/db/mod.rs`
