# Backlog

Forward-looking work captured as it surfaces during shipped batches.
Items here are intentionally not yet scoped to a specific branch — when
a batch picks them up, it pulls them out of this file into a
`MERGE_BATCH<n>.md` plan and removes the entry once shipped.

---

## Batch 4 candidates

### 1. Admin edit-coupon Stripe-coupon recreation flow
**Severity:** P1 (operators can silently desync coupons)

When admin edits discount math (percent_off, amount_off, duration),
backend creates a new Stripe coupon, flips `stripe_coupon_id` pointer on
the DB row, optionally deletes the old Stripe coupon. (Stripe Coupons
are immutable; existing subscriptions keep the discount they redeemed
per Stripe's `duration` semantics, which is the desired behavior.) The
current `update_coupon` only updates DB metadata, so an operator
changing 10% → 15% in our admin UI silently leaves the Stripe coupon at
10% and customers continue redeeming at 10%.

### 2. Coupon backfill admin tool
**Severity:** P2 (currently no rows affected; future-proof)

Add admin endpoint `POST /api/admin/coupons/:id/sync-to-stripe` that
mirrors a DB row whose `stripe_coupon_id IS NULL` into a fresh Stripe
Coupon and stores the id. Useful if a Stripe API outage during create
left a row half-mirrored, or if rows ever get imported from elsewhere.

### 3. Frontend admin coupon form fields for `duration` / `duration_in_months`
**Severity:** P1 (form is now incomplete vs. backend contract)

Add a `duration` dropdown (once / forever / repeating) and a
`duration_in_months` input (visible only when `duration=repeating`) to
the `/admin/coupons` create + edit pages. The backend now accepts and
requires these fields per Migration 062 + Batch 3.5; the frontend form
hasn't caught up yet.

---

## SvelteKit 3 follow-ups

Deferred from the SvelteKit 3 migration. Everything **removed** in Kit 3
was migrated; the items below are **deprecated-but-working** APIs and
advisory findings from `sv migrate`'s generated task list. None of them
block the build, typecheck, lint, or tests.

### 1. `invalidateAll` → `refreshAll`
**Severity:** P2 (deprecated, still functional)

~110 call sites still use `invalidateAll()` / `goto(..., { invalidateAll: true })`.
The Kit 3 replacement is `refreshAll()`, but it is **not** a pure rename:
`refreshAll()` preserves `page.state` where `invalidateAll()` reset it.
Each call site needs a look at whether retained page state changes
behaviour, so this was deliberately kept out of the dependency-bump
change rather than sed-replaced.

### 2. `json()` / `text()` → `Response.json()` / `new Response()`
**Severity:** P3 (deprecated, still functional)

~200 `+server.ts` handlers import `json` from `@sveltejs/kit`. Mechanical,
but large and touches every API proxy, so it belongs in its own pass.

### 3. `replaceState` → `replace` in `goto` options
**Severity:** P3 (deprecated, still functional)

Small rename, safe to fold into whichever pass touches navigation next.

### 4. Pre-existing a11y failure: form inputs without labels
**Severity:** P2 (pre-existing, not a migration regression)

`tests/accessibility/a11y-audit.spec.ts` → "form inputs should have
associated labels" fails on `/admin/blog/new`. Verified failing
identically on the pre-migration baseline (`e5cf441`), so it predates the
Kit 3 work — but it is a real WCAG gap worth closing.

### 5. TypeScript 7
**Severity:** P2 (blocked upstream)

TypeScript 7.0.2 is released but unusable here: `svelte-check@4.7.6` peers
on `^5 || ^6`, and `@sveltejs/kit@3.0.0-next.23` requires `typescript ^6`.
Revisit once the Svelte toolchain supports 7.

### 6. SvelteKit 3 is a prerelease
**Severity:** P1 (track to stable)

The branch pins `@sveltejs/kit@3.0.0-next.23` and
`@sveltejs/adapter-cloudflare@8.0.0-next.6`. There is no `3.0.0` stable and
no `rc` dist-tag on npm yet. Re-pin to the stable releases when they land.
Known upstream rough edge: adapter-cloudflare 8 prints
"Reading `config.kit` inside adapters is deprecated" on every build.
