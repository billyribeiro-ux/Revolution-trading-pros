# 🔐 Secrets Configuration - Revolution Trading Pros

> **AI AGENTS: READ THIS FILE FIRST when debugging deployment or secret issues.**

## Quick Reference

| Secret Name | Where to Configure | Required For |
|-------------|-------------------|--------------|
| `CLOUDFLARE_API_TOKEN` | **Repository Secrets** | Frontend deployment |
| `CLOUDFLARE_ACCOUNT_ID` | **Repository Secrets** | Frontend deployment |
| `FLY_API_TOKEN` | **Repository Secrets** | API deployment |
| `CLOUDFLARE_ZONE_ID` | **Repository Secrets** (optional) | Cache purging |
| `LHCI_GITHUB_APP_TOKEN` | **Repository Secrets** (optional) | Lighthouse CI |

---

## ⚠️ CRITICAL: Why Secrets "Get Lost"

The `deploy-cloudflare.yml` workflow uses **GitHub Environments** (`production`, `staging`, `preview`).

**Problem:** When a job specifies `environment: name`, GitHub creates a separate secret scope:
1. Environment-specific secrets are checked FIRST
2. Repository secrets are fallback ONLY
3. **If an environment exists but has NO secrets, repository secrets are NOT used**

**Solution:** Configure secrets at **REPOSITORY LEVEL** (not environment level).

---

## 🔧 How to Configure Secrets

### Step 1: Go to Repository Secrets

```
https://github.com/billyribeiro-ux/Revolution-trading-pros/settings/secrets/actions
```

**Navigation:** Repository → Settings → Secrets and variables → Actions → **Repository secrets**

### Step 2: Add Required Secrets

Click "New repository secret" for each:

#### CLOUDFLARE_API_TOKEN
- **Where to get:** https://dash.cloudflare.com/profile/api-tokens
- **Create token with:** 
  - Template: "Edit Cloudflare Workers" OR
  - Custom permissions: Account → Cloudflare Pages → Edit

#### CLOUDFLARE_ACCOUNT_ID
- **Where to get:** https://dash.cloudflare.com → Workers & Pages → Right sidebar → "Account ID"
- **Format:** 32-character hex string (e.g., `a1b2c3d4e5f6...`)

#### FLY_API_TOKEN
- **Where to get:** Run `flyctl tokens create deploy` in terminal
- **Format:** `FlyV1 fm2_...` (long token string)

---

## 📍 GitHub Environments (DO NOT ADD SECRETS HERE)

The workflow creates these environments automatically:
- `production` - main branch deployments
- `staging` - develop branch deployments  
- `preview` - PR and feature branch deployments

**DO NOT add secrets to individual environments.** Use repository-level secrets only.

If you accidentally added secrets to environments:
1. Go to: https://github.com/billyribeiro-ux/Revolution-trading-pros/settings/environments
2. Click each environment (production, staging, preview)
3. Check "Environment secrets" section
4. **DELETE any Cloudflare secrets there** (they override repository secrets)

---

## 🔍 Debugging Checklist

When deployments fail with "secrets not configured":

1. **Check Repository Secrets exist:**
   - Go to: Settings → Secrets → Actions → Repository secrets
   - Verify `CLOUDFLARE_API_TOKEN` is listed
   - Verify `CLOUDFLARE_ACCOUNT_ID` is listed

2. **Check Environments don't have conflicting secrets:**
   - Go to: Settings → Environments
   - For each environment (production, staging, preview):
     - Click environment name
     - Check "Environment secrets" section
     - If any CLOUDFLARE secrets exist, DELETE them

3. **Re-run the workflow:**
   - Go to: Actions tab
   - Select failed workflow
   - Click "Re-run all jobs"

---

## 📂 Related Files

- `.github/workflows/deploy-cloudflare.yml` - Frontend deployment workflow
- `.github/workflows/deploy-fly.yml` - API deployment workflow
- `.github/workflows/README.md` - Workflow documentation
- `.github/workflows/TECHNICAL_NOTES.md` - IDE warning explanations

---

## 🔑 Current Secret Values (for reference only)

### Cloudflare (need to obtain from dashboard)
- Account ID: Check https://dash.cloudflare.com → Workers & Pages
- API Token: Create at https://dash.cloudflare.com/profile/api-tokens

### Fly.io
See `DEPLOYMENT_COMPLETE.md` for the FLY_TOKEN value.

---

## 📝 For AI Agents

When user reports deployment failures:

1. **First check:** Is the error "secrets not configured"?
2. **If yes:** Direct user to this file's "How to Configure Secrets" section
3. **Common fix:** Delete environment-level secrets, ensure repository-level secrets exist
4. **Verify:** Ask user to confirm secrets are at REPOSITORY level, not ENVIRONMENT level

**Key insight:** The workflow's `environment:` declaration creates separate scopes. Secrets must be at repository level to work across all environments.

---

**Last Updated:** January 17, 2026
**Status:** Active Configuration Document
