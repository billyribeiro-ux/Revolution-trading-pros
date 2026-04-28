# Deployment Guide

**Status:** Deploy target TBD as of 2026-04-28.

The previous deployment guide targeted Fly.io. Those instructions were removed when the team paused on the deploy target choice. The original Fly.io guide is preserved in [`backups/fly-io-removed-2026-04-28.md`](../../backups/fly-io-removed-2026-04-28.md) and can be restored verbatim when the team commits to a deploy target.

## Local development

For local development setup, see [`docs/development/LOCAL_DEV.md`](../development/LOCAL_DEV.md).

## When ready to deploy

1. Choose a deploy target (Fly.io, Railway, Render, AWS ECS, GCP Cloud Run, etc.).
2. Restore or rewrite this guide for the chosen target.
3. Add the corresponding CI workflow under `.github/workflows/`.
4. Configure secrets:
   - `STRIPE_SECRET` (sk_live_…), `STRIPE_PUBLISHABLE_KEY` (pk_live_…), `STRIPE_WEBHOOK_SECRET`
   - `DATABASE_URL` (production Postgres)
   - `JWT_SECRET` (32+ random bytes)
   - `POSTMARK_TOKEN` (when email is wired up)
   - Any storage / OAuth / observability tokens needed for the chosen target.
5. Run all four gates locally before any deploy:
   - `pnpm check` (frontend typecheck)
   - `pnpm test:unit` (vitest)
   - `cd frontend && pnpm exec playwright test tests/e2e --project=chromium`
   - `cd api && cargo check`
