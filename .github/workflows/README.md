# GitHub Actions Workflows

## Overview

This directory contains CI/CD workflows for the Revolution Trading Pros platform.

## Workflows

### 1. `deploy-cloudflare.yml` - Frontend Deployment
Deploys the SvelteKit frontend to Cloudflare Pages.

**Triggers:**
- Push to `main` → Production deployment
- Push to `develop` → Staging deployment
- Pull requests → Preview deployment
- Manual dispatch

**Required Secrets:**
- `CLOUDFLARE_API_TOKEN` - Cloudflare API token with Pages permissions
- `CLOUDFLARE_ACCOUNT_ID` - Your Cloudflare account ID

**Optional Secrets:**
- `CLOUDFLARE_ZONE_ID` - For cache purging (production only)
- `LHCI_GITHUB_APP_TOKEN` - For Lighthouse CI reports
- `SLACK_WEBHOOK_URL` - For failure notifications (currently disabled)

**Optional Variables:**
- `SITE_URL` - Custom site URL (defaults to Cloudflare Pages URL)

### 2. `e2e.yml` - End-to-End Testing
Runs Playwright E2E tests across multiple browsers.

**Triggers:**
- Pull requests to `main`
- Push to `main`
- Manual dispatch

**Test Suites:**
- **Smoke Tests** - Quick validation (< 2 min)
- **Full E2E** - Comprehensive tests across browsers
- **Mobile Tests** - Mobile viewport testing
- **API Tests** - Backend integration (optional, requires backend)

## IDE Warnings Explained

### "Context access might be invalid" Warnings

You may see warnings like:
```
Context access might be invalid: SITE_URL
Context access might be invalid: CLOUDFLARE_API_TOKEN
```

**These warnings are EXPECTED and SAFE.** Here's why:

1. **GitHub Actions Limitation**: The IDE can't verify if secrets/variables exist at edit-time
2. **Runtime Safety**: Our workflows check for secret existence before using them
3. **Graceful Degradation**: Missing secrets cause skips, not failures

**Example - Our Secret Validation:**
```yaml
- name: Check Cloudflare secrets
  run: |
    if [ -z "${{ secrets.CLOUDFLARE_API_TOKEN }}" ]; then
      echo "has_secrets=false"
    fi

- name: Deploy
  if: steps.check-secrets.outputs.has_secrets == 'true'
  # Only runs if secrets exist
```

**What We've Done:**
- ✅ Added secret validation before deployment
- ✅ Graceful skips with helpful notices
- ✅ Continue-on-error for optional features
- ✅ Clear error messages in Actions summary

**Action Required:**
- None for warnings - they're informational only
- Configure secrets in repository settings to enable features

## Configuring Secrets

**Settings → Secrets and variables → Actions → New repository secret**

### Cloudflare Deployment
1. `CLOUDFLARE_API_TOKEN` - Get from Cloudflare Dashboard → API Tokens
2. `CLOUDFLARE_ACCOUNT_ID` - Found in Cloudflare Dashboard URL
3. `CLOUDFLARE_ZONE_ID` - Optional, for cache purging

### Lighthouse CI (Optional)
1. `LHCI_GITHUB_APP_TOKEN` - For performance reports

## Troubleshooting

### Deployment Skipped
**Symptom:** "Deployment Skipped - secrets not configured"
**Solution:** Add required Cloudflare secrets (see above)

### E2E Tests Failing
**Symptom:** Smoke tests fail with "no tests found"
**Solution:** Tests are in `frontend/tests/e2e/smoke/` - ensure directory exists

### Cache Purge Skipped
**Symptom:** Cache purge step skipped
**Solution:** Add `CLOUDFLARE_ZONE_ID` secret (optional feature)

## Local Testing

### Run E2E Tests Locally
```bash
cd frontend
npm run test:e2e:smoke
```

### Build for Cloudflare
```bash
cd frontend
DEPLOY_TARGET=cloudflare npm run build
```

## Maintenance

- Workflows are self-documenting with comments
- Secret validation prevents silent failures
- Continue-on-error for optional features
- Clear error messages in GitHub Actions UI
