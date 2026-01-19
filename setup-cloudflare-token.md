# Cloudflare API Token Setup for GitHub Actions

## The Problem
The current `CLOUDFLARE_API_TOKEN` secret is using a Global API Key format, which doesn't work with the wrangler-action. We need a proper scoped API Token.

## Solution: Create a Scoped API Token

### Step 1: Go to Cloudflare Dashboard
Open: https://dash.cloudflare.com/profile/api-tokens

### Step 2: Create Custom Token
1. Click **"Create Token"**
2. Click **"Create Custom Token"** (or use "Edit Cloudflare Workers" template)

### Step 3: Configure Permissions
Set these permissions:

**Account Permissions:**
- Cloudflare Pages → **Edit**

**Account Resources:**
- Include → **Your Account** (should auto-select your account)

### Step 4: Optional Settings
- **Client IP Address Filtering**: Leave empty (or add GitHub Actions IP ranges if you want extra security)
- **TTL**: Leave as default or set expiration

### Step 5: Create and Copy Token
1. Click **"Continue to summary"**
2. Click **"Create Token"**
3. **COPY THE TOKEN** (you won't see it again!)

### Step 6: Update GitHub Secret
Run this command with your new token:

```bash
echo 'YOUR_NEW_TOKEN_HERE' | gh secret set CLOUDFLARE_API_TOKEN
```

### Step 7: Trigger Deployment
```bash
git push origin main
```

Or manually trigger the workflow:
```bash
gh workflow run deploy-cloudflare.yml
```

## Verification
After updating the token, the deployment should succeed. Check the workflow run:
```bash
gh run list --workflow=deploy-cloudflare.yml --limit 1
```
