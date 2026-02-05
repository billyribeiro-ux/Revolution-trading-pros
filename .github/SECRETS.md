# GitHub Secrets Configuration
**ICT 7 Apple Principal Engineer Grade**  
**Revolution Trading Pros CI/CD Pipeline**

## Overview

This document lists all GitHub secrets required for the CI/CD pipeline. Secrets are configured in:
**Repository Settings → Secrets and variables → Actions**

## Required Secrets (Deployment)

### Cloudflare Pages Deployment

| Secret Name | Required | Used In | Description |
|------------|----------|---------|-------------|
| `CLOUDFLARE_API_TOKEN` | ✅ Yes | `deploy-preview`, `deploy-production` | Cloudflare API token with Pages write access |
| `CLOUDFLARE_ACCOUNT_ID` | ✅ Yes | `deploy-preview`, `deploy-production` | Cloudflare account ID |

**How to get:**
1. Go to [Cloudflare Dashboard](https://dash.cloudflare.com)
2. Navigate to: Profile → API Tokens
3. Create token with "Cloudflare Pages" template
4. Copy Account ID from any Pages project URL

## Optional Secrets (Features)

### AI Features (Anthropic Claude)

| Secret Name | Required | Used In | Description |
|------------|----------|---------|-------------|
| `VITE_ANTHROPIC_API_KEY` | ⚠️ Optional | `build`, `deploy-production` | Anthropic API key for AI writing assistant |

**Impact if missing:** AI writing features disabled, build succeeds  
**How to get:** [Anthropic Console](https://console.anthropic.com) → API Keys

### Supabase (Legacy - Not Currently Used)

| Secret Name | Required | Used In | Description |
|------------|----------|---------|-------------|
| `VITE_PUBLIC_SUPABASE_URL` | ⚠️ Optional | `deploy-production` | Supabase project URL |
| `VITE_PUBLIC_SUPABASE_ANON_KEY` | ⚠️ Optional | `deploy-production` | Supabase anonymous key |

**Impact if missing:** No impact, Supabase not currently integrated  
**Status:** Reserved for future use

## Automatic Secrets (Provided by GitHub)

| Secret Name | Description |
|------------|-------------|
| `GITHUB_TOKEN` | Automatically provided by GitHub Actions for API access |

## Secret Validation

### Check if secrets are configured:
```bash
# This will show which secrets exist (not their values)
gh secret list
```

### Test deployment secrets:
```bash
# Trigger a workflow manually to test
gh workflow run ci.yml
```

## Adding Secrets

### Via GitHub UI:
1. Go to repository on GitHub
2. Settings → Secrets and variables → Actions
3. Click "New repository secret"
4. Enter name and value
5. Click "Add secret"

### Via GitHub CLI:
```bash
# Set a secret
gh secret set CLOUDFLARE_API_TOKEN

# Set from file
gh secret set CLOUDFLARE_API_TOKEN < token.txt

# Set from environment variable
gh secret set CLOUDFLARE_API_KEY --body "$CLOUDFLARE_TOKEN"
```

## Security Best Practices

### ✅ DO:
- Rotate secrets regularly (every 90 days)
- Use least-privilege tokens (only required permissions)
- Store secrets in GitHub Secrets (never commit to code)
- Use separate tokens for staging/production
- Enable secret scanning in repository settings

### ❌ DON'T:
- Commit secrets to code or `.env` files
- Share secrets via Slack/email
- Use personal API keys for CI/CD
- Log secret values in workflow outputs
- Use the same secrets across multiple projects

## Troubleshooting

### "Context access might be invalid" warnings in IDE

**Cause:** IDE cannot verify if secrets exist in GitHub  
**Solution:** This is expected - warnings will disappear once secrets are configured  
**Workaround:** Secrets have `|| ''` fallback to allow builds without them

### Deployment fails with "unauthorized"

**Cause:** Missing or invalid `CLOUDFLARE_API_TOKEN`  
**Solution:**
1. Verify token exists: `gh secret list`
2. Check token permissions in Cloudflare Dashboard
3. Regenerate token if expired
4. Update secret: `gh secret set CLOUDFLARE_API_TOKEN`

### Build succeeds but features don't work

**Cause:** Optional secrets not configured  
**Solution:** Add the optional secret for the feature you need  
**Note:** Build will succeed without optional secrets

## Secrets Audit Log

| Date | Action | Secret | Performed By |
|------|--------|--------|--------------|
| 2026-02-05 | Documentation created | All | Cascade AI |
| TBD | Initial setup | `CLOUDFLARE_API_TOKEN` | Admin |
| TBD | Initial setup | `CLOUDFLARE_ACCOUNT_ID` | Admin |

## Related Documentation

- [GitHub Actions Secrets](https://docs.github.com/en/actions/security-guides/encrypted-secrets)
- [Cloudflare Pages Deployment](https://developers.cloudflare.com/pages/how-to/use-direct-upload-with-continuous-integration/)
- [Anthropic API Keys](https://docs.anthropic.com/claude/reference/getting-started-with-the-api)

---
**Last Updated:** 2026-02-05  
**Maintained By:** DevOps Team  
**Review Frequency:** Quarterly
