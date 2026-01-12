# Get R2 API Credentials - Final Step

Your R2 bucket is fully configured! You just need API credentials for uploads.

---

## Current R2 Setup ✅

- **Bucket Name**: `revolution-trading-media`
- **Location**: Eastern North America (ENAM)
- **Public Access**: Enabled ✅
- **Public URL**: `https://pub-2e5bd1b702b440bd888a0fc47f3493ae.r2.dev` ✅
- **S3 API Endpoint**: `https://9c72eb0d1b0b7891aca6532fe709cacc.r2.cloudflarestorage.com` ✅

---

## Get API Credentials (2 Minutes)

### Step 1: Navigate to API Tokens
From your current R2 page:
1. Look at the **top navigation bar** or **top right**
2. Click **"Manage R2 API Tokens"** button

Or go directly to:
- Cloudflare Dashboard → R2 → Overview → **"Manage R2 API Tokens"**

### Step 2: Create API Token
1. Click **"Create API Token"** button

2. Fill in the form:
   - **Token name**: `revolution-api-token`
   - **Permissions**: Select **"Object Read & Write"**
   - **TTL**: Leave default or select "Forever"
   - **Bucket scope**: 
     - Choose **"Apply to specific buckets only"**
     - Select: `revolution-trading-media`

3. Click **"Create API Token"**

### Step 3: Copy Credentials (IMPORTANT!)
You'll see a screen with:

```
Access Key ID: abc123xyz456...
Secret Access Key: secret789abc123...
```

**⚠️ CRITICAL**: Copy both values immediately! The secret will only be shown once.

---

## Update Your Configuration

Once you have the credentials, update `api/.env`:

```bash
R2_ACCESS_KEY_ID=abc123xyz456...
R2_SECRET_ACCESS_KEY=secret789abc123...
```

Replace lines 22-23 in `/Users/billyribeiro/CascadeProjects/Revolution-trading-pros/api/.env`

---

## Test R2 Upload

After updating credentials:

```bash
# Start backend
cd api
cargo run

# In another terminal, test upload
curl -X POST http://localhost:8080/api/media/upload \
  -H "Content-Type: application/json" \
  -d '{"filename": "test.jpg", "content_type": "image/jpeg"}'
```

Should return a presigned upload URL with your R2 domain.

---

## What's Already Configured ✅

All files have been updated with your R2 URLs:
- ✅ `api/.env` - Endpoint and public URL
- ✅ `frontend/.env.production` - CDN URL
- ✅ `frontend/wrangler.toml` - CDN URL (all 3 environments)
- ✅ `frontend/src/lib/api/config.ts` - Fallback CDN URL
- ✅ `api/src/config/mod.rs` - Fallback public URL

**Only missing**: Access Key ID and Secret Access Key

---

**Next**: Get those 2 credentials and R2 is 100% complete!
