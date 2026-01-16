# SCHEMA COMPARISON - LOGIN 500 ERROR ROOT CAUSE ANALYSIS
**ICT 7 Principal Engineer Grade - Production Database Investigation**

## CRITICAL ERROR
```
Database error: column "password" does not exist
Position: 25 in SQL query
```

## 1. BACKEND RUST MODEL EXPECTS
**File:** `backend-rust/src/models/user.rs:37-38`

```rust
pub const SELECT_COLUMNS: &'static str =
    "id, name, email, password, role, created_at, updated_at";
```

**Expected Columns:**
- `id` (i64)
- `name` (String)
- `email` (String)
- **`password`** ← **THIS COLUMN DOES NOT EXIST IN PRODUCTION**
- `role` (String)
- `created_at` (NaiveDateTime)
- `updated_at` (NaiveDateTime)

**Optional Columns (with `#[sqlx(default)]`):**
- `first_name` (Option<String>)
- `last_name` (Option<String>)
- `avatar_url` (Option<String>)
- `email_verified_at` (Option<NaiveDateTime>)
- `remember_token` (Option<String>)
- `stripe_customer_id` (Option<String>)

## 2. MIGRATION SCHEMA (0001_create_users_table.sql)
**File:** `backend-rust/migrations/0001_create_users_table.sql`

```sql
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),  -- ⚠️ UUID not i64
    name VARCHAR(255) NOT NULL,
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,  -- ✓ password exists in migration
    avatar_url VARCHAR(500),
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    email_verified_at TIMESTAMPTZ,
    remember_token VARCHAR(100),
    stripe_customer_id VARCHAR(255),
    settings JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);
```

## 3. PRODUCTION DATABASE ACTUAL SCHEMA
**Source:** Legacy Laravel/PHP migration (api/migrations/001_initial_schema.sql)

The production database was created by a **DIFFERENT migration system** (Node.js/PHP) that:
- Does NOT have a `password` column
- Likely uses `password_hash` or similar Laravel convention
- Uses different column names than Rust expects

**Evidence:**
1. Error: `column "password" does not exist`
2. Migration 0001 was "previously applied but is missing" (warning in logs)
3. Production DB predates Rust backend

## 4. FRONTEND EXPECTATIONS
**File:** `frontend/src/routes/api/auth/login/+server.ts:26`

Frontend sends:
```typescript
{
  email: string,
  password: string
}
```

Frontend expects response:
```typescript
{
  user: {
    id: number,
    name: string,
    email: string,
    role: string,
    // ... other fields
  },
  access_token: string,
  refresh_token: string,
  expires_in: number
}
```

## 5. ROOT CAUSE ANALYSIS

### THE PROBLEM
1. **Production database** was created by legacy PHP/Laravel migrations
2. **Legacy schema** likely uses `password_hash` or different column name
3. **Rust backend** expects `password` column
4. **SQLx migrations** are NOT running on production (migration 0 missing warning)
5. **Column mismatch** causes SQL query to fail with "column does not exist"

### WHY IT WORKED BEFORE
- Either:
  - A) Production DB had `password` column that was dropped
  - B) Different database was being used
  - C) Backend was using different query

## 6. IMMEDIATE FIX REQUIRED

### Option A: Add password column to production DB
```sql
ALTER TABLE users ADD COLUMN password VARCHAR(255);
```

### Option B: Find actual password column name and update backend
Need to query production DB to find actual column name:
```sql
SELECT column_name FROM information_schema.columns 
WHERE table_name = 'users' 
ORDER BY ordinal_position;
```

## 7. NEXT STEPS

1. **Query production DB** to get actual schema
2. **Update User model** to match production column names
3. **Test login** with correct column mapping
4. **Document** actual production schema for future reference

## 8. PRODUCTION DATABASE ACCESS

**Connection String:** `postgres://postgres:vOUkDxw1Git2UQo@revolution-db.flycast:5432/postgres`

**Access Method:**
```bash
flyctl ssh console -a revolution-trading-pros-api
# Install psql in container
# Query actual schema
```
