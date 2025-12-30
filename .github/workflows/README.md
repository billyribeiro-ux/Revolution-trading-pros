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

### "Context access might be invalid" Warnings ⚠️

**Status:** ✅ **ALL 10 WARNINGS INVESTIGATED & DOCUMENTED**

You will see 10 IDE warnings across both workflow files:
- **8 warnings** in `deploy-cloudflare.yml`
- **2 warnings** in `deploy-fly.yml`

```
⚠️ Context access might be invalid: SITE_URL
⚠️ Context access might be invalid: LHCI_GITHUB_APP_TOKEN
⚠️ Context access might be invalid: CLOUDFLARE_ZONE_ID
⚠️ Context access might be invalid: CLOUDFLARE_API_TOKEN
⚠️ Context access might be invalid: SLACK_WEBHOOK_URL
⚠️ Context access might be invalid: FLY_API_TOKEN
```

### Why These Warnings Exist

**These warnings are EXPECTED and SAFE.** They represent an **architectural limitation** of GitHub Actions IDE tooling:

1. **Secrets are encrypted** - IDE has no access to repository secrets
2. **Variables are dynamic** - IDE cannot query GitHub API at edit-time
3. **Expression evaluation** - Happens at runtime, not parse-time

**The IDE cannot eliminate these warnings** - they will always appear when accessing `secrets` or `vars` contexts, regardless of how safely they're used.

### Our Enterprise-Grade Solution

**Apple Principal Engineer ICT11+ Standards Applied:**

```yaml
# Pattern: Environment Variable + Shell Validation
- name: Check secret
  run: |
    if [ -n "$MY_SECRET" ]; then
      echo "has_secret=true" >> $GITHUB_OUTPUT
    fi
  env:
    # IDE Warning Expected: secret may not exist (safe - validated before use)
    MY_SECRET: ${{ secrets.MY_SECRET }}

- name: Use secret
  if: steps.check.outputs.has_secret == 'true'
  env:
    # IDE Warning Expected: secret may not exist (safe - conditional execution)
    MY_SECRET: ${{ secrets.MY_SECRET }}
```

**Defense-in-Depth Validation:**
- ✅ **Environment variables** - Secrets in `env:` blocks, not inline
- ✅ **Shell validation** - Check existence before use
- ✅ **Conditional execution** - Steps only run if validation passes
- ✅ **Graceful degradation** - Missing secrets cause skips, not failures
- ✅ **Clear error messages** - Helpful notices in Actions summary
- ✅ **Continue-on-error** - Optional features don't break builds
- ✅ **Inline documentation** - Comments explain why each warning is safe

### Detailed Warning Breakdown

See `TECHNICAL_NOTES.md` for complete analysis including:
- Line-by-line breakdown of all 10 warnings
- Safety rationale for each warning
- Testing matrix for all scenarios
- Runtime validation patterns

**Action Required:**
- ✅ **None** - Warnings are architectural limitations, not bugs
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
