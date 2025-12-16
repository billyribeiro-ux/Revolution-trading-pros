# Cloudflare Pages Setup Guide - Step by Step

## Revolution Trading Pros - Complete Cloudflare Deployment Guide

This guide walks you through every step of deploying the frontend to Cloudflare Pages.

---

## Table of Contents

1. [Create Cloudflare Account](#step-1-create-cloudflare-account)
2. [Connect GitHub Repository](#step-2-connect-github-repository)
3. [Configure Build Settings](#step-3-configure-build-settings)
4. [Set Environment Variables](#step-4-set-environment-variables)
5. [Deploy Your Site](#step-5-deploy-your-site)
6. [Set Up Custom Domain](#step-6-set-up-custom-domain)
7. [Configure SSL/HTTPS](#step-7-configure-sslhttps)
8. [Set Up R2 Storage](#step-8-set-up-r2-storage-optional)
9. [Configure GitHub Actions](#step-9-configure-github-actions)
10. [Verify Everything Works](#step-10-verify-everything-works)

---

## Step 1: Create Cloudflare Account

### 1.1 Sign Up

1. Go to [https://dash.cloudflare.com/sign-up](https://dash.cloudflare.com/sign-up)
2. Enter your email address
3. Create a strong password
4. Click **Create Account**
5. Verify your email address

### 1.2 Choose a Plan

1. For Pages, you can use the **Free** plan
2. Free plan includes:
   - 500 builds per month
   - Unlimited sites
   - Unlimited bandwidth
   - Global CDN

---

## Step 2: Connect GitHub Repository

### 2.1 Navigate to Pages

1. Log into [Cloudflare Dashboard](https://dash.cloudflare.com)
2. In the left sidebar, click **Workers & Pages**
3. Click the **Pages** tab
4. Click **Create application**
5. Click **Pages** (not Workers)
6. Click **Connect to Git**

### 2.2 Authorize GitHub

1. Click **Connect GitHub**
2. A popup will appear asking for GitHub authorization
3. Click **Authorize Cloudflare**
4. Select your GitHub account or organization: `billyribeiro-ux`
5. Choose repository access:
   - Select **Only select repositories**
   - Choose `Revolution-trading-pros`
6. Click **Install & Authorize**

### 2.3 Select Repository

1. Back in Cloudflare, you should see your repositories
2. Find and click `Revolution-trading-pros`
3. Click **Begin setup**

---

## Step 3: Configure Build Settings

### 3.1 Project Name

```
Project name: revolution-trading-pros
```

This will be your default URL: `revolution-trading-pros.pages.dev`

### 3.2 Production Branch

```
Production branch: main
```

### 3.3 Build Settings

Click **Framework preset** and select: **SvelteKit**

Then configure these settings:

| Setting | Value |
|---------|-------|
| **Framework preset** | SvelteKit |
| **Root directory** | `frontend` |
| **Build command** | `npm run build` |
| **Build output directory** | `.svelte-kit/cloudflare` |

### 3.4 Build Environment Variables (Required for Build)

Click **Environment variables (advanced)** to expand, then add:

| Variable name | Value |
|---------------|-------|
| `NODE_VERSION` | `20` |
| `DEPLOY_TARGET` | `cloudflare` |

Click **Add variable** for each one.

### 3.5 Screenshot Reference

Your settings should look like this:

```
┌─────────────────────────────────────────────────────────────┐
│ Set up builds and deployments                               │
├─────────────────────────────────────────────────────────────┤
│ Project name: revolution-trading-pros                       │
│                                                             │
│ Production branch: main                                     │
│                                                             │
│ ▼ Build settings                                            │
│   Framework preset: SvelteKit                               │
│   Root directory (advanced): frontend                       │
│   Build command: npm run build                              │
│   Build output directory: .svelte-kit/cloudflare            │
│                                                             │
│ ▼ Environment variables (advanced)                          │
│   NODE_VERSION = 20                                         │
│   DEPLOY_TARGET = cloudflare                                │
└─────────────────────────────────────────────────────────────┘
```

---

## Step 4: Set Environment Variables

### 4.1 Initial Deploy First

1. Click **Save and Deploy** to start the first build
2. Wait for the build to complete (2-5 minutes)
3. Once deployed, you'll see your site at `revolution-trading-pros.pages.dev`

### 4.2 Navigate to Environment Variables

1. Go to your project: **Workers & Pages** → **revolution-trading-pros**
2. Click **Settings** tab
3. Click **Environment variables** in the left menu

### 4.3 Add Production Variables

Click **Add variable** for each of these:

#### Required Variables

| Variable | Value | Encrypt? |
|----------|-------|----------|
| `VITE_API_URL` | `https://revolution-trading-pros.fly.dev/api` | No |
| `VITE_SITE_URL` | `https://revolutiontradingpros.com` | No |
| `VITE_SITE_NAME` | `Revolution Trading Pros` | No |

#### Analytics Variables (Add when you have the IDs)

| Variable | Value | Encrypt? |
|----------|-------|----------|
| `VITE_GTM_ID` | `GTM-XXXXXXX` | No |
| `VITE_GTAG_ID` | `G-XXXXXXXXXX` | No |
| `PUBLIC_GA4_MEASUREMENT_ID` | `G-XXXXXXXXXX` | No |
| `PUBLIC_META_PIXEL_ID` | `123456789` | Yes |

### 4.4 Environment-Specific Variables

Cloudflare lets you set different values for Production vs Preview:

1. Click on a variable
2. Choose **Specify a different value for Preview**
3. Set preview to point to staging backend:
   - Production: `https://revolution-trading-pros.fly.dev/api`
   - Preview: `https://revolution-trading-pros.fly.dev/api`

### 4.5 Screenshot Reference

```
┌─────────────────────────────────────────────────────────────┐
│ Environment variables                                        │
├─────────────────────────────────────────────────────────────┤
│ Production                           Preview                 │
│ ─────────────────────────────────────────────────────────── │
│ VITE_API_URL                                                │
│ Value: https://revolution-trading-pros.fly.dev/api         │
│ [ ] Encrypt                                                 │
│                                                             │
│ VITE_SITE_URL                                               │
│ Value: https://revolutiontradingpros.com                    │
│ [ ] Encrypt                                                 │
│                                                             │
│ [+ Add variable]                                            │
└─────────────────────────────────────────────────────────────┘
```

---

## Step 5: Deploy Your Site

### 5.1 Trigger a New Deployment

After adding environment variables:

1. Go to **Deployments** tab
2. Find your latest deployment
3. Click the **...** menu → **Retry deployment**

Or push a commit to trigger automatic deployment.

### 5.2 Monitor Build Progress

1. Click on the deployment to see build logs
2. Watch for:
   - ✅ Cloning repository
   - ✅ Installing dependencies
   - ✅ Building application
   - ✅ Deploying to Cloudflare's network

### 5.3 Build Log Example

```
Cloning repository...
Installing dependencies...
npm WARN deprecated...
Building application...
vite v5.x.x building for production...
✓ 142 modules transformed
✓ built in 12.34s
Deploying to Cloudflare's global network...
Success! Your site is live at:
https://revolution-trading-pros.pages.dev
```

---

## Step 6: Set Up Custom Domain

### 6.1 Add Domain to Cloudflare (If not already)

If your domain isn't on Cloudflare yet:

1. Go to **Websites** in the sidebar
2. Click **Add a site**
3. Enter your domain: `revolutiontradingpros.com`
4. Select **Free** plan
5. Cloudflare will scan your DNS records
6. Update your domain registrar's nameservers to Cloudflare's:
   ```
   Nameserver 1: xxx.ns.cloudflare.com
   Nameserver 2: xxx.ns.cloudflare.com
   ```
7. Wait for DNS propagation (can take up to 24 hours)

### 6.2 Connect Domain to Pages

1. Go to **Workers & Pages** → **revolution-trading-pros**
2. Click **Custom domains** tab
3. Click **Set up a custom domain**
4. Enter: `revolutiontradingpros.com`
5. Click **Continue**
6. Cloudflare will automatically configure DNS
7. Click **Activate domain**

### 6.3 Add WWW Subdomain

Repeat for www:

1. Click **Set up a custom domain**
2. Enter: `www.revolutiontradingpros.com`
3. Click **Continue** → **Activate domain**

### 6.4 DNS Records Created

Cloudflare automatically creates:

```
Type    Name    Content                               TTL     Proxy
CNAME   @       revolution-trading-pros.pages.dev    Auto    Proxied
CNAME   www     revolution-trading-pros.pages.dev    Auto    Proxied
```

### 6.5 Verify Domain Status

1. Go to **Custom domains**
2. Both domains should show **Active**:
   - ✅ revolutiontradingpros.com - Active
   - ✅ www.revolutiontradingpros.com - Active

---

## Step 7: Configure SSL/HTTPS

### 7.1 SSL/TLS Settings

1. Go to **Websites** → `revolutiontradingpros.com`
2. Click **SSL/TLS** in the sidebar
3. Click **Overview**
4. Set encryption mode to: **Full (strict)**

### 7.2 Enable Always Use HTTPS

1. Go to **SSL/TLS** → **Edge Certificates**
2. Enable **Always Use HTTPS** → Toggle ON
3. Enable **Automatic HTTPS Rewrites** → Toggle ON

### 7.3 Enable HSTS

1. Still in **Edge Certificates**
2. Find **HTTP Strict Transport Security (HSTS)**
3. Click **Enable HSTS**
4. Configure:
   - Max Age: `12 months`
   - Include subdomains: `On`
   - Preload: `On`
5. Click **Save**

### 7.4 Minimum TLS Version

1. Go to **SSL/TLS** → **Edge Certificates**
2. Set **Minimum TLS Version** to: `TLS 1.2`

---

## Step 8: Set Up R2 Storage (Optional)

R2 is for storing user uploads, images, and media files.

### 8.1 Create R2 Bucket

1. Go to **R2 Object Storage** in sidebar
2. Click **Create bucket**
3. Bucket name: `revolution-trading-media`
4. Location: **Automatic** (or choose closest to your users)
5. Click **Create bucket**

### 8.2 Enable Public Access

1. Click on your bucket `revolution-trading-media`
2. Go to **Settings** tab
3. Under **Public access**, click **Allow Access**
4. Choose **Connect a domain** or use the `r2.dev` subdomain
5. If using r2.dev:
   - Click **Allow Access**
   - Your public URL will be: `https://pub-XXXXX.r2.dev`

### 8.3 Create API Token for Backend

1. Go to **R2 Object Storage** → **Manage R2 API Tokens**
2. Click **Create API token**
3. Token name: `revolution-backend-r2`
4. Permissions: **Object Read & Write**
5. Specify bucket: `revolution-trading-media`
6. Click **Create API Token**
7. **SAVE THESE VALUES** (shown only once):
   ```
   Access Key ID: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
   Secret Access Key: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
   Endpoint: https://ACCOUNT_ID.r2.cloudflarestorage.com
   ```

### 8.4 Add R2 Credentials to Fly.io

In your Fly.io backend secrets (via `fly secrets set`):

```
R2_ACCESS_KEY_ID=your-access-key-id
R2_SECRET_ACCESS_KEY=your-secret-access-key
R2_BUCKET=revolution-trading-media
R2_ENDPOINT=https://ACCOUNT_ID.r2.cloudflarestorage.com
R2_PUBLIC_URL=https://pub-XXXXX.r2.dev
```

---

## Step 9: Configure GitHub Actions

### 9.1 Create Cloudflare API Token

1. Go to [Cloudflare API Tokens](https://dash.cloudflare.com/profile/api-tokens)
2. Click **Create Token**
3. Click **Use template** next to **Edit Cloudflare Workers**
4. Configure permissions:
   ```
   Account - Cloudflare Pages - Edit
   Account - Account Settings - Read
   Zone - Zone - Read (for cache purging)
   Zone - Cache Purge - Purge
   ```
5. Account Resources: **Include - Your Account**
6. Zone Resources: **Include - Specific zone - revolutiontradingpros.com**
7. Click **Continue to summary** → **Create Token**
8. **COPY THE TOKEN** (shown only once)

### 9.2 Get Account ID

1. Go to any page in Cloudflare Dashboard
2. Look at the URL: `https://dash.cloudflare.com/XXXXXXXX/...`
3. The `XXXXXXXX` is your Account ID
4. Or go to **Workers & Pages** → right sidebar shows **Account ID**

### 9.3 Get Zone ID

1. Go to **Websites** → `revolutiontradingpros.com`
2. On the **Overview** page, right sidebar shows **Zone ID**

### 9.4 Add Secrets to GitHub

1. Go to your GitHub repository
2. Click **Settings** → **Secrets and variables** → **Actions**
3. Click **New repository secret**
4. Add these secrets:

| Secret Name | Value |
|-------------|-------|
| `CLOUDFLARE_API_TOKEN` | (the token you created) |
| `CLOUDFLARE_ACCOUNT_ID` | (your account ID) |
| `CLOUDFLARE_ZONE_ID` | (your zone ID) |

### 9.5 Verify GitHub Actions

1. Push a commit to `main` branch
2. Go to **Actions** tab in GitHub
3. You should see "Deploy Frontend (Cloudflare Pages)" workflow running
4. Click to see the progress

---

## Step 10: Verify Everything Works

### 10.1 Test Your Site

Visit these URLs and verify they work:

- [ ] `https://revolution-trading-pros.pages.dev` (Cloudflare default)
- [ ] `https://revolutiontradingpros.com` (custom domain)
- [ ] `https://www.revolutiontradingpros.com` (www subdomain)

### 10.2 Test HTTPS

1. Visit `http://revolutiontradingpros.com` (note: http, not https)
2. Verify it redirects to `https://`

### 10.3 Test API Connection

1. Open browser developer tools (F12)
2. Go to **Network** tab
3. Navigate around your site
4. Look for API calls to your Fly.io backend
5. Verify they return 200 status

### 10.4 Test Preview Deployments

1. Create a pull request on GitHub
2. Cloudflare will automatically deploy a preview
3. Check the PR for a comment with the preview URL

### 10.5 Check SSL Certificate

1. Click the padlock icon in browser address bar
2. Verify certificate is valid
3. Should show: "Connection is secure"

---

## Troubleshooting

### Build Fails: "Cannot find module"

**Solution**: Ensure `ROOT_DIRECTORY` is set to `frontend`

### Build Fails: "adapter-cloudflare not found"

**Solution**: Add environment variable:
```
DEPLOY_TARGET=cloudflare
```

### Site Shows 404

**Solution**: Verify build output directory is `.svelte-kit/cloudflare`

### API Calls Fail (CORS Error)

**Solution**: In your Fly.io backend secrets:
```
CORS_ALLOWED_ORIGINS=https://revolutiontradingpros.com,https://www.revolutiontradingpros.com
SANCTUM_STATEFUL_DOMAINS=revolutiontradingpros.com,www.revolutiontradingpros.com
```

### Custom Domain Shows "DNS Error"

**Solution**:
1. Wait for DNS propagation (up to 24 hours)
2. Verify nameservers are set to Cloudflare at your registrar

### Environment Variables Not Working

**Solution**:
1. After adding/changing variables, trigger a new deployment
2. Variables starting with `VITE_` are available in frontend code
3. Variables starting with `PUBLIC_` are also public

---

## Quick Reference

### Important URLs

| Purpose | URL |
|---------|-----|
| Cloudflare Dashboard | https://dash.cloudflare.com |
| Pages Settings | https://dash.cloudflare.com/?to=/:account/pages/view/revolution-trading-pros |
| DNS Settings | https://dash.cloudflare.com/?to=/:account/:zone/dns |
| SSL Settings | https://dash.cloudflare.com/?to=/:account/:zone/ssl-tls |

### Build Commands

```bash
# Local build for Cloudflare
cd frontend
DEPLOY_TARGET=cloudflare npm run build

# Manual deploy with Wrangler
npx wrangler pages deploy .svelte-kit/cloudflare --project-name=revolution-trading-pros
```

### Support

- [Cloudflare Pages Docs](https://developers.cloudflare.com/pages/)
- [SvelteKit Cloudflare Adapter](https://kit.svelte.dev/docs/adapter-cloudflare)
- [Cloudflare Community](https://community.cloudflare.com/)

---

## Checklist Summary

- [ ] Created Cloudflare account
- [ ] Connected GitHub repository
- [ ] Configured build settings (SvelteKit, frontend root, .svelte-kit/cloudflare output)
- [ ] Added NODE_VERSION=20 and DEPLOY_TARGET=cloudflare
- [ ] First deployment successful
- [ ] Added VITE_API_URL environment variable
- [ ] Added analytics variables (GTM, GA4, etc.)
- [ ] Custom domain added and active
- [ ] SSL/TLS set to Full (strict)
- [ ] HTTPS always enabled
- [ ] HSTS enabled
- [ ] R2 bucket created (if needed)
- [ ] GitHub secrets configured
- [ ] GitHub Actions workflow working
- [ ] Site accessible and functional
