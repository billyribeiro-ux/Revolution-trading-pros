# Backlog

Forward-looking work captured as it surfaces during shipped batches.
Items here are intentionally not yet scoped to a specific branch — when
a batch picks them up, it pulls them out of this file into a
`MERGE_BATCH<n>.md` plan and removes the entry once shipped.

---

## Batch 4 candidates

### 0. Shrink or box `ApiError` instead of raising clippy's error-size bar
**Severity:** P3 (code health; no runtime impact)

Rust 1.98's clippy extended `result_large_err` to `async fn`, so ~150
route handlers returning `Result<T, ApiError>` began failing the
`-D warnings` gate on 2026-08-30. `ApiError` is 168 bytes (3×String,
2×Option<String>, Option<HashMap>), over the 128-byte default. The
dependency sweep unblocked CI with `api/clippy.toml`
(`large-error-threshold = 176`) rather than reshaping ~40 modules
mid-upgrade.

The real fix is to make the error small: box it (`Result<T, Box<ApiError>>`
serializes identically, so the JSON contract is unaffected) or collapse
`status`/`code`/`timestamp` into cheaper representations (a status enum
and a derived timestamp). Then drop `clippy.toml` back to the default and
delete the ~12 now-redundant `#[allow(clippy::result_large_err)]`
attributes still scattered through the routes.


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

### 7. Remaining Svelte-5 modernisation surface
**Severity:** P3 (all currently supported; no warnings fired)

Measured 2026-08-14 against the official [Svelte best practices](https://svelte.dev/docs/svelte/best-practices)
list. The codebase is already fully migrated on the big-ticket items —
**0** `export let`, **0** `on:` event directives, **0** `$:` reactive statements,
**0** `class:` directives, and **0** real `<slot>` or `<svelte:component>` uses
(the few textual matches are comments recording that they were replaced).

What is left, in full:

| Pattern | Count | Recommended replacement | Notes |
|---------|-------|-------------------------|-------|
| `svelte/store` imports | 28 files | classes with `$state` fields | The largest remaining item. |
| `setContext` / `getContext` | 24 calls | `createContext` | Docs prefer it for type safety. |
| `use:tilt` action | 1 | `{@attach ...}` | The other two `use:` directives are not legacy: `use:enhance` is SvelteKit's own API and `use:emblaCarouselSvelte` is the library's public interface. |

None of these emit warnings — `pnpm check:strict` and `pnpm check:a11y` are both
clean at 0 errors / 0 warnings across 4,847 files — so this is voluntary
modernisation, not remediation.

### 8. adapter-cloudflare 8.0.0-next.6 reads deprecated `config.kit`
**Severity:** P3 (warning only, no behavioural effect)

Every production build prints:

```
Reading `config.kit` inside adapters is deprecated — it should access
configuration on the `config` object directly. You may need to update your adapter
```

This comes from inside the prerelease adapter, not from repo code, so there is
nothing to fix here — it clears when adapter-cloudflare ships a build that reads
`config` directly. Re-check when the SvelteKit 3 pins are moved off the
`next` line (item 1).

### 9. `$app/env`'s `version` is unusable in service workers
**Severity:** P3 (worked around; upstream issue)

`@sveltejs/kit@3.0.0-next.23` defines it as
`BROWSER ? payload.version : __SVELTEKIT_APP_VERSION__`, and `payload` is only
ever populated by the client app boot — so in a `ServiceWorkerGlobalScope` it is
`undefined`. `$service-worker` supplied a real string in SvelteKit 2 and nothing
in `$app/*` replaces it today.

Worked around by injecting `__APP_VERSION__` via Vite's `define` and pinning
`kit.version.name` to the same value (see `frontend/vite.config.ts`). If a later
Kit 3 build fixes the export, that indirection can be dropped and the import
restored. Worth reporting upstream if it is still present at 3.0.0 stable.
