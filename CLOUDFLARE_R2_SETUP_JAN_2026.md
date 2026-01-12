# Cloudflare R2 Setup Guide - January 2026 UI

**Updated**: January 10, 2026  
**For**: Revolution Trading Pros

---

## Step 1: Access Cloudflare Dashboard

1. Go to **https://dash.cloudflare.com**
2. Log in with your Cloudflare account (same account used for Pages deployment)
3. You should see your main dashboard

---

## Step 2: Navigate to R2

**Current UI (January 2026)**:
- Look at the **left sidebar**
- Find and click **"R2 Object Storage"** or just **"R2"**
- You'll see the R2 overview page

**First Time Setup**:
- If this is your first time, you may need to **"Purchase R2"** or **"Enable R2"**
- R2 has a **free tier**: 10GB storage, 1 million Class A operations/month
- Click **"Purchase R2"** or **"Get Started"** (no credit card required for free tier)

---

## Step 3: Create Your Bucket

1. On the R2 page, click **"Create bucket"** button (usually top right)

2. **Bucket Configuration**:
   - **Name**: `revolution-trading-media`
   - **Location**: Choose **"Automatic"** or **"North America East (ENAM)"**
   - **Storage Class**: Leave as **"Standard"** (default)

3. Click **"Create bucket"**

---

## Step 4: Enable Public Access (Important!)

After bucket is created:

1. Click on your bucket name: `revolution-trading-media`
2. Go to **"Settings"** tab
3. Scroll to **"Public access"** section
4. Click **"Allow Access"** or toggle **"Public access"** to ON
5. You'll see a warning - confirm that you want to enable public access

**You'll now see your Public Bucket URL**:
```
https://pub-[RANDOM_HASH].r2.dev
```

**Copy this entire URL** - this is your `R2_PUBLIC_URL`

Example: `https://pub-a1b2c3d4e5f6g7h8.r2.dev`

---

## Step 5: Get Your Account ID

1. While still in R2 section, look at the **right sidebar** or **top of page**
2. You should see **"Account ID"** displayed
3. Copy this value (format: `abc123def456ghi789`)

Your R2 endpoint will be:
```
https://[YOUR_ACCOUNT_ID].r2.cloudflarestorage.com
```

Example: `https://abc123def456ghi789.r2.cloudflarestorage.com`

---

## Step 6: Create API Token

1. In R2 section, click **"Manage R2 API Tokens"** (usually top right)
   - Or go to: **R2** → **Overview** → **"Manage R2 API Tokens"**

2. Click **"Create API Token"**

3. **Token Configuration**:
   - **Token name**: `revolution-api-token`
   - **Permissions**: Select **"Object Read & Write"**
   - **TTL (Time to Live)**: Leave as default or set to "Forever"
   - **Bucket scope**: 
     - Select **"Apply to specific buckets only"**
     - Choose: `revolution-trading-media`

4. Click **"Create API Token"**

5. **IMPORTANT - Copy Immediately** (shown only once):
   - **Access Key ID**: `abc123xyz...` (like AWS access key)
   - **Secret Access Key**: `secret123...` (like AWS secret key)
   
   **Save these somewhere safe!** You cannot view the secret again.

---

## Step 7: Update Your Configuration Files

Now update these files with your real values:

### `api/.env`
```bash
R2_ENDPOINT=https://[YOUR_ACCOUNT_ID].r2.cloudflarestorage.com
R2_ACCESS_KEY_ID=[YOUR_ACCESS_KEY_ID]
R2_SECRET_ACCESS_KEY=[YOUR_SECRET_ACCESS_KEY]
R2_BUCKET=revolution-trading-media
R2_PUBLIC_URL=https://pub-[YOUR_HASH].r2.dev
```

### `frontend/.env.production`
```bash
VITE_CDN_URL=https://pub-[YOUR_HASH].r2.dev
```

### `frontend/wrangler.toml`
Replace all 3 occurrences (lines 19, 28, 37):
```toml
VITE_CDN_URL = "https://pub-[YOUR_HASH].r2.dev"
```

---

## Step 8: Test Your Setup

### Test 1: Check Bucket Access
```bash
# Install AWS CLI (R2 is S3-compatible)
brew install awscli  # macOS
# or: apt-get install awscli  # Linux

# Configure AWS CLI for R2
aws configure
# Enter your R2 Access Key ID
# Enter your R2 Secret Access Key
# Region: auto
# Output format: json

# Test listing bucket
aws s3 ls s3://revolution-trading-media \
  --endpoint-url https://[YOUR_ACCOUNT_ID].r2.cloudflarestorage.com
```

### Test 2: Upload Test File
```bash
# Create test file
echo "Hello R2" > test.txt

# Upload to R2
aws s3 cp test.txt s3://revolution-trading-media/test.txt \
  --endpoint-url https://[YOUR_ACCOUNT_ID].r2.cloudflarestorage.com

# Verify public access
curl https://pub-[YOUR_HASH].r2.dev/test.txt
# Should return: Hello R2
```

### Test 3: Backend API Upload
```bash
# Start your backend
cd api
cargo run

# In another terminal, test upload endpoint
curl -X POST http://localhost:8080/api/media/upload \
  -H "Content-Type: application/json" \
  -d '{"filename": "test.jpg", "content_type": "image/jpeg"}'

# Should return presigned upload URL with your real R2 domain
```

---

## Troubleshooting

### "Bucket already exists" error
- Bucket names are globally unique across all Cloudflare accounts
- Try: `revolution-trading-media-[your-initials]`
- Update `R2_BUCKET` in all config files

### "Access Denied" on public URL
- Make sure you enabled **Public Access** in bucket settings
- Check that the object was uploaded successfully
- Wait 1-2 minutes for DNS propagation

### "Invalid credentials" error
- Double-check Access Key ID and Secret Access Key
- Make sure you copied them correctly (no extra spaces)
- Regenerate token if needed

### Can't find Account ID
- Go to: Cloudflare Dashboard → R2 → Overview
- Look at URL bar: `dash.cloudflare.com/[ACCOUNT_ID]/r2/overview`
- Or check right sidebar for "Account ID"

---

## Current Cloudflare Dashboard Layout (Jan 2026)

```
┌─────────────────────────────────────────────────┐
│ Cloudflare Dashboard                            │
├─────────────────────────────────────────────────┤
│                                                 │
│  Left Sidebar:                                  │
│  ├─ Home                                        │
│  ├─ Websites                                    │
│  ├─ Workers & Pages                             │
│  ├─ R2 Object Storage  ← Click here             │
│  ├─ Stream                                      │
│  └─ ...                                         │
│                                                 │
│  Main Content (R2 Page):                        │
│  ┌───────────────────────────────────────────┐ │
│  │ R2 Overview                               │ │
│  │ [Create bucket]  [Manage R2 API Tokens]  │ │
│  ├───────────────────────────────────────────┤ │
│  │ Buckets:                                  │ │
│  │ • revolution-trading-media                │ │
│  │   10 objects | 2.5 MB | Public            │ │
│  └───────────────────────────────────────────┘ │
│                                                 │
│  Right Sidebar:                                 │
│  Account ID: abc123...                          │
│                                                 │
└─────────────────────────────────────────────────┘
```

---

## Summary Checklist

- [ ] Log into Cloudflare Dashboard
- [ ] Navigate to R2 Object Storage
- [ ] Enable/Purchase R2 (if first time)
- [ ] Create bucket: `revolution-trading-media`
- [ ] Enable public access on bucket
- [ ] Copy public bucket URL
- [ ] Note your Account ID
- [ ] Create API token with Read & Write permissions
- [ ] Copy Access Key ID and Secret Access Key
- [ ] Update `api/.env` with 5 values
- [ ] Update `frontend/.env.production` with CDN URL
- [ ] Update `frontend/wrangler.toml` with CDN URL (3 places)
- [ ] Test upload with AWS CLI or backend API

---

**Next**: Once R2 is configured, move on to Bunny.net and Stripe setup.
