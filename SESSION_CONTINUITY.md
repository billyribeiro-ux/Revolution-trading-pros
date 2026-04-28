# Session Continuity — pick up here next time

## What was completed this session (2026-04-28)

### Svelte MCP autofixer full sweep — ALL folders done ✅

Ran `svelte-autofixer` on every `.svelte` file in the repo. The only
issue class found was missing `{#each}` key expressions. All fixed,
all committed, all clean at `pnpm check` (0 errors / 0 warnings / 5215 files).

Commits landed:
```
8ef40b48f fix(svelte): autofixer pass — routes/ (non-admin) keyed {#each} blocks  ← stragglers
b23a97213 fix(svelte): autofixer pass — routes/ (non-admin) keyed {#each} blocks
b8c81807e fix(svelte): autofixer pass — routes/admin/ keyed {#each} blocks
cd30995a9 fix(svelte): autofixer pass — lib/components/ + lib/ keyed {#each} blocks
```

**The Svelte autofixer audit is 100% complete. Do not re-run it.**

---

## What is NOT done — next tasks in priority order

Source of truth: `docs/audits/DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md` §9

### Tier 0 — blocking (do these first)
- [ ] **Fly Postgres** — DB outage from prior audit. Nothing works until login works.
- [ ] **CMS toolbar** — `frontend/src/routes/cms/editor/+page.svelte:113` — headline feature can't author content.
- [ ] **Stripe Checkout-Session creation** — `api/src/routes/subscriptions.rs:446` — revenue blocker.

### Tier 1 — under a day each, high payoff
- [ ] Frontend `/admin` role gate — `frontend/src/routes/admin/+layout.server.ts`
- [ ] Favorites proxy cookie fix — `frontend/src/routes/api/favorites/+server.ts:32`
- [ ] WebSocket JWT validation — `api/src/websocket.rs:344`
- [ ] Sanitize 3 unsanitized `{@html}` components (grep for `{@html` — exclude JSON-LD which is safe)
- [ ] Rate-limit register + forgot-password + reset-password endpoints
- [ ] Centralize 14 hardcoded `fly.dev` URLs into `frontend/src/lib/config/api.ts`
- [ ] Unify env-var precedence between catch-all proxy and Axum client
- [ ] Remove unused deps: `vivus`, `lottie-web`
- [ ] Fix `SETUP_GUIDE.md:75` doc drift

### Tier 2 — ~a week
- [ ] Wrap multi-step admin mutations in transactions (`grant_membership`, `bulk_assign_tags`, role updates)
- [ ] Migrate `admin.rs` to `AdminUser` extractor uniformly
- [ ] Activate MFA at login (service is complete, just not wired at login)
- [ ] Fix `let _ = sqlx::query(…)` swallowing in `cms_scheduling.rs` + `admin_courses.rs:1829`
- [ ] Replace 7 handler-level `.unwrap()` with `?` propagation
- [ ] Migrate 5 dashboard `+page.server.ts` files to `.remote.ts`
- [ ] Fix 2 correctness issues in existing `.remote.ts` files (hardcoded refresh keys; unawaited `.refresh()`)
- [ ] Move `SITEMAP_CACHE` off `RwLock<HashMap>` (no TTL) to a proper TTL'd cache
- [ ] Move `cms_seo.rs:351` regex to `LazyLock`
- [ ] Add CMS audit coverage for admin user-management mutations
- [ ] Add `aria-hidden` to ~3900 decorative icons
- [ ] Add `alt` to 92 images
- [ ] Fix 30+ static GSAP imports — convert to dynamic-import per page

### Tier 3 — larger efforts
- [ ] Test coverage: `routes/auth.rs`, `routes/oauth.rs`, `routes/checkout.rs`, `services/mfa.rs`, `services/cms_workflow.rs`
- [ ] E2E specs: sign-up, login, OAuth, logout, password reset, subscribe, indicator purchase, course enrollment
- [ ] Lighthouse CI + perf budget
- [ ] Frontend admin/dashboard: migrate `let props = $props()` shadow-state to destructured `$bindable()` (Tier 3 — 30 in CLAUDE.md)
- [ ] CMS editor UX: autosave, undo/redo, drag-reorder, AI panel, preset picker
- [ ] Tighten CSP from `unsafe-inline` to nonces
- [ ] i18n with Paraglide

---

## Rules for next session

1. **One folder at a time** — finish it, confirm clean, write changelog, commit, then next.
2. **Never commit without explicit user approval** per `memory/feedback_no_commit_without_approval.md`.
3. **Never delete files** — orphan = build the missing side.
4. **Stay inside this repo** — never touch sibling repos.
5. Run `pnpm check` after every fix batch before declaring clean.

## Current branch state
- Branch: `main`
- Ahead of `origin/main` by 5 commits (not pushed yet)
- Working tree: clean (only untracked `ADMIN_SYSTEM_DISCOVERY.md`)
