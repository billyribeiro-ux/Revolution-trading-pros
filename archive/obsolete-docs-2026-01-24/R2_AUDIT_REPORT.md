# R2 Configuration Audit Report
**Date**: January 10, 2026  
**Status**: INCONSISTENT - Needs Cleanup

---

## Executive Summary

The hardcoded URL `https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev` appears in **7 locations** across the codebase. This is a placeholder/example URL that will NOT work for actual file uploads.

**Current State**: 
- ✅ Cloudflare Pages is deployed and working
- ❌ R2 storage credentials are placeholders
- ❌ Unclear if R2 bucket actually exists

---

## All R2 References Found

### 1. Backend API Configuration

**File**: `api/.env`
```bash
R2_ENDPOINT=https://xxx.r2.cloudflarestorage.com          # ❌ PLACEHOLDER
R2_ACCESS_KEY_ID=your-access-key                          # ❌ PLACEHOLDER  
R2_SECRET_ACCESS_KEY=your-secret-key                      # ❌ PLACEHOLDER
R2_BUCKET=revolution-trading-media                        # ✅ Valid name
R2_PUBLIC_URL=https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev  # ❌ HARDCODED
```

**File**: `api/src/config/mod.rs` (Line 71)
```rust
r2_public_url: std::env::var("R2_PUBLIC_URL")
    .unwrap_or_else(|_| "https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev".to_string()),
```
**Status**: Fallback default - will use this if .env is empty

---

### 2. Frontend Configuration

**File**: `frontend/.env.production`
```bash
VITE_CDN_URL=https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev  # ❌ HARDCODED
```

**File**: `frontend/wrangler.toml` (3 occurrences)
```toml
# Line 17 - Global vars
VITE_CDN_URL = "https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev"

# Line 26 - Preview environment  
VITE_CDN_URL = "https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev"

# Line 35 - Production environment
VITE_CDN_URL = "https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev"
```

**File**: `frontend/src/lib/api/config.ts` (Line 12)
```typescript
const PRODUCTION_CDN_URL = 'https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev';
```
**Status**: Hardcoded fallback constant

---

## What This URL Actually Is

`https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev` is:
- An **example/placeholder** R2 public URL
- Likely from documentation or a previous project
- **NOT** your actual R2 bucket
- Will return 404 or access denied on upload attempts

---

## Critical Questions to Answer

### 1. Does the R2 Bucket Exist?
**Bucket Name**: `revolution-trading-media`

**How to Check**:
1. Log into Cloudflare Dashboard: https://dash.cloudflare.com
2. Go to R2 Object Storage (left sidebar)
3. Look for bucket named `revolution-trading-media`

**Possible Outcomes**:
- ✅ Bucket exists → Just need to get correct credentials
- ❌ Bucket doesn't exist → Need to create it

---

### 2. What's the Real R2 Public URL?

If bucket exists, the real URL format is:
```
https://pub-[YOUR_HASH].r2.dev
```

Where `[YOUR_HASH]` is a unique identifier assigned by Cloudflare.

**How to Find It**:
1. Cloudflare Dashboard → R2
2. Click on `revolution-trading-media` bucket
3. Settings → Public Access
4. Copy the public bucket URL

---

## Action Plan to Fix

### Option A: Bucket Already Exists
1. Log into Cloudflare Dashboard
2. Navigate to R2 → `revolution-trading-media`
3. Get the 5 required values:
   - Account ID (from R2 overview page)
   - Access Key ID (R2 → Manage API Tokens)
   - Secret Access Key (same place)
   - Bucket name (already correct: `revolution-trading-media`)
   - Public URL (from bucket settings)
4. Update these 7 files with real values:
   - `api/.env` (5 values)
   - `frontend/.env.production` (1 value)
   - `frontend/wrangler.toml` (1 value, 3 places)

### Option B: Bucket Doesn't Exist
1. Log into Cloudflare Dashboard
2. R2 → Create Bucket
3. Name: `revolution-trading-media`
4. Location: Automatic or US East
5. Enable public access
6. Create API token with Read & Write permissions
7. Update all 7 files with new credentials

---

## Files That Need Updating

| File | Line(s) | Current Value | Needs Update |
|------|---------|---------------|--------------|
| `api/.env` | 27 | `https://xxx.r2.cloudflarestorage.com` | ✅ Yes |
| `api/.env` | 28 | `your-access-key` | ✅ Yes |
| `api/.env` | 29 | `your-secret-key` | ✅ Yes |
| `api/.env` | 31 | `https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev` | ✅ Yes |
| `frontend/.env.production` | 7 | `https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev` | ✅ Yes |
| `frontend/wrangler.toml` | 17, 26, 35 | `https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev` | ✅ Yes |
| `frontend/src/lib/api/config.ts` | 12 | `https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev` | ⚠️ Optional (fallback) |
| `api/src/config/mod.rs` | 71 | `https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev` | ⚠️ Optional (fallback) |

---

## Testing After Fix

Once updated, test with:

```bash
# Start backend
cd api
cargo run

# In another terminal, test upload
curl -X POST http://localhost:8080/api/media/upload \
  -H "Content-Type: application/json" \
  -d '{"filename": "test.jpg", "content_type": "image/jpeg"}'
```

Should return a presigned upload URL with your real R2 domain.

---

## Next Steps

1. **Check Cloudflare Dashboard** - Does `revolution-trading-media` bucket exist?
2. **Get Real Credentials** - Collect all 5 R2 values
3. **Update Files** - Replace placeholders in 7 locations
4. **Test Upload** - Verify R2 uploads work
5. **Commit Changes** - Update repo with real config

---

**Report Generated**: January 10, 2026  
**Action Required**: Verify R2 bucket existence and update credentials
