# GitHub Actions Secrets

Configured under **Repository Settings → Secrets and variables →
Actions**. The two workflows in this repo (`ci.yml`,
`deploy-cloudflare.yml`) reference exactly two secrets — nothing else.

## Secrets actually used

| Secret | Workflow | Purpose |
|--------|----------|---------|
| `CLOUDFLARE_API_TOKEN` | `deploy-cloudflare.yml` (job `build-and-deploy`) | Cloudflare API token with "Pages: Edit" permission |
| `CLOUDFLARE_ACCOUNT_ID` | `deploy-cloudflare.yml` (job `build-and-deploy`) | Cloudflare account ID |

`ci.yml` uses no secrets at all.

**Behavior when missing:** deploy-cloudflare.yml does not fail. A gate
step checks both values and skips the wrangler deploy with a
`::notice::` if either is empty. The build itself still runs.

**How to get them:**

1. [Cloudflare Dashboard](https://dash.cloudflare.com) → Profile → API
   Tokens → create a token with the "Cloudflare Pages" template
   (Pages: Edit).
2. Account ID is shown in the dashboard sidebar / any Pages project URL.

## Managing secrets

```bash
gh secret list                          # names only, never values
gh secret set CLOUDFLARE_API_TOKEN      # prompts for value
gh secret set CLOUDFLARE_API_TOKEN < token.txt
```

To verify end-to-end, dispatch a manual preview deploy:

```bash
gh workflow run deploy-cloudflare.yml -f branch=preview
```

## Practices

- Least-privilege tokens (Pages: Edit only); rotate periodically.
- Never commit secrets or log them in workflow output.
- Don't reuse these tokens outside this repo.

## History

Older versions of this document listed `VITE_ANTHROPIC_API_KEY`, Supabase
keys, `CLOUDFLARE_ZONE_ID`, `LHCI_GITHUB_APP_TOKEN`, and
`SLACK_WEBHOOK_URL` for jobs named `deploy-preview` / `deploy-production`
and a Fly.io workflow. Fly.io was removed 2026-04-28; none of those
secrets or jobs exist in the current workflows. If a future workflow
needs a new secret, add it to the table above in the same change.

---
**Last updated:** 2026-07-19
