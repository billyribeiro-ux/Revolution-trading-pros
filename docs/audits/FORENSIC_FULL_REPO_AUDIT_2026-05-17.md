# Forensic Full-Repository Audit — 2026-05-17

Independent end-to-end audit of the Revolution Trading Pros monorepo
(SvelteKit 2 / Svelte 5 frontend, Rust + Axum backend, PostgreSQL).

- **HEAD audited:** `16b350b` (Merge PR #589) — **4 PRs (#586–#589) newer**
  than the prior [`FULL_REPO_AUDIT_2026-05-17.md`](FULL_REPO_AUDIT_2026-05-17.md)
  (audited `c31c43d`).
- **Date:** 2026-05-17
- **Corpus:** ~767k LOC — Rust API 106k / 148 files · frontend 660k
  (.svelte 471k/891 files, .ts 188k/704 files) · 62 SQL migrations
  (35k LOC) · 643 SvelteKit route files.
- **Method:** every available quality gate run to ground truth + **10
  parallel deep-read agents** (backend auth/security, payments/Stripe,
  CMS/admin routes, migrations/schema, services/jobs, SvelteKit
  proxies, Svelte-5 hygiene, frontend money/lib, XSS/SEO, CI/deps) each
  tracing its domain line-by-line + **forensic hand-verification in the
  main thread** of every load-bearing P0/P1 and every cross-agent
  discrepancy. Items proven by hand are marked **[FV]**.

> **Scope honesty:** 767k LOC cannot be read sequentially by one reader
> in one pass. Coverage = all gates (which transitively compile/
> type-check every file the build sees) + 10 domain agents reading their
> subsystem source + hand-verification of every headline finding and
> every place two agents disagreed. Not covered: Playwright e2e (no
> Docker stack), live migration replay (no DB), `cargo machete`/`deny`
> (absent), `svelte-autofixer` MCP (**permission-denied to agents** —
> CLAUDE.md-mandated discipline could not be exercised this session).

---

## 1. Quality gates — actual ground-truth results

| Gate | Result | Detail |
|---|---|---|
| `cargo check --locked --all-targets` | ✅ | clean compile, 0 errors (5m02s) |
| `pnpm install --frozen-lockfile` | ✅ | succeeded (15s) |
| `pnpm check` / `vitest` / `eslint` / `cargo clippy` | ✅ (per CI) | now **all blocking in CI** (`ci.yml`) — see §5 P1-9 |
| `serializeJsonLd` unit tests | ✅ | 10/10 (agent ran vitest) — confirms P0-6 fix |
| `svelte-autofixer` MCP | ⚠ | **permission-denied** — Svelte-5 autofix coverage gap |
| Playwright e2e / DB migration replay | ⏸ | no Docker / no DB in env |

### 1a. Systemic gate blind spot — **[FV]**

`SQLX_OFFLINE = "true"` (`api/.cargo/config.toml:46`) but only **2 files
/ 27 calls** use compile-time `query!`/`query_as!` macros; **~1,630–1,755
runtime `sqlx::query(` calls have zero compile-time SQL verification**;
the `.sqlx/` cache holds **22** entries. **CI verifies ≈1.5% of the SQL
surface against the schema.** A green `cargo check`/`clippy` does **not**
mean the SQL matches the database. Any column rename/drop/type-change in
a migration is invisible to all four CLAUDE.md gates and surfaces only as
a runtime `sqlx::Error` on the first production request. This is the
multiplier behind every "latent until runtime" finding below.

---

## 2. Verdict on prior-audit P0/P1 (status at HEAD `16b350b`)

PRs #586–#589 landed real fixes. Independently re-verified:

| Prior finding | Status | Proof |
|---|---|---|
| P0-1 checkout route mismatch | **FIXED** [FV] | `cart.ts:1246` POSTs `${API_URL}/checkout` → catch-all → backend `checkout.rs` `create_checkout`; old `/create-session` gone; dead 2nd fn removed |
| P0-2 committed superadmin creds | **PARTIAL / OPEN** [FV] | worktree `.env.production` sanitized, but creds remain in **git history** *and* plaintext in 4 tracked docs (`FULL_REPO_AUDIT_2026-05-17.md`, `REMEDIATION_PLAN_2026-05-17.md`, `BATCH4_RESULT.md`, `BATCH7_RESULT.md`). Needs rotation + history rewrite — file deletion is insufficient |
| P0-3 `/admin` gate client-side only | **FIXED** [FV] | `frontend/src/routes/admin/+layout.server.ts:28-37` server `load` 303-redirects on `!isAdminRole`; runs even with `ssr=false` |
| P0-4 webhook multi-table writes non-atomic | **FIXED** [FV] | `payments.rs:282` `pool.begin()`; all writes `&mut **tx`; `processed_at` stamped last before `tx.commit()`; dedup now gated on `processed_at IS NOT NULL` (retry-safe) |
| P0-5 sub-plan create/edit drops price | **FIXED** [FV] | `admin.ts:1269-1286` `normalizePlanPayload` → `Math.round(price*100)`, wired into create/update |
| P0-6 JSON-LD `{@html}` stored-XSS | **FIXED** | all 6 sinks → `serializeJsonLd.ts` `escapeForScriptContext` (escapes `< > & U+2028/9`); 10/10 tests pass |
| P0-7 course paywall bypass | **FIXED** | `member_courses.rs:341-354` hard 403 `if !free && !enrolled && !admin`; per-lesson GUID/download strip `:408-439` |
| P1-1 Google API key committed | **PARTIAL** [FV] | centralized + env-overridable (`lib/config/google.ts`) but **live key still the hardcoded fallback** (`google.ts:38`) → ships in bundle |
| P1-2 stolen JWT survives logout-all/reset/ban | **FIXED (password path)** | `tv` claim + epoch check fail-closed (`utils/mod.rs`, `middleware/auth.rs:147-180`, `redis.rs:288-347`) — **but see NF-3: OAuth path bypasses it** |
| P1-3 spoofable XFF rate-limit IP | **FIXED** | trusted-proxy CIDR + `ConnectInfo` (`config/mod.rs`, `auth.rs:80-177`, `main.rs:309`) |
| P1-4 unauth admin/infra endpoints | **FIXED** | `coupons/validate` `AdminUser` (`admin.rs:1318`); init/setup/run-migrations `SuperAdminUser` (`health.rs`); monitoring token-gated fail-closed |
| P1-5 fabricated admin analytics | **FIXED (residue)** | revenue/churn now real SQL or honest `501`; **`analytics_cohorts` still hardcoded** (`admin_members.rs:1062-1070`) — NF-9 |
| P1-6 `/api/auth/set-session` session fixation | **FIXED** | unauthenticated POST deleted; only `DELETE` remains |
| P1-7 money not stored as integer cents | **OPEN** | DB columns still `DECIMAL(10,2)`; float boundary conv `checkout.rs:293/343`, `payments.rs:583`; refund float accumulation `payments.rs:1507`; mig 068 operator-gated |
| P1-8 `get_subscription` missing `expand=items` | **FIXED (handler)** / partial | `stripe.rs:813` fixed; `list_subscriptions` still un-expanded → NF-5 |
| P1-9 CI doesn't enforce lint/clippy | **FIXED** [FV] | `ci.yml` eslint blocking; clippy `-D warnings` no `continue-on-error`; dead npm workflow deleted |
| P1-10 tests exercise duplicated logic | **FIXED** | `stripe_test.rs:12` uses real `StripeService`; duplicated `mod stripe` gone; real-JWT harness `common/mod.rs:129-132` |
| P2-D errors swallowed `unwrap_or_default` | **PARTIAL** | fixed at cited spots; still open `admin_members.rs:1267/1324`, `member_courses.rs:254/361/375/382`, `services/search.rs:124` |
| P2-E cms_webhooks SSRF | **FIXED (dead code)** | robust `validate_webhook_url` (`cms_webhooks.rs:166-224`); subsystem has no live caller |
| P2-G service worker caches auth HTML | **FIXED** | `service-worker.ts:89-112` `isPrivatePath()` excludes `/dashboard /account /admin /checkout /trading-room /my` |
| P2-J 16 component-name collisions | **REFUTED** | 0 non-route basename collisions; the "byte-identical RepeaterField" was a phantom diff of two empty subshells |
| P3 ~36MB dead files | **FIXED** | all 6 dirs deleted (commit `012901f`) |
| G0.3 schema reproducibility | **PARTIAL** [FV] | `schema.sql` exists & checksum-correct, but **not wired into CI/test harness**; from-zero `sqlx::migrate!` still aborts; `041` `UNIQUE…WHERE` syntax error still OPEN in the committed chain |

**Net:** the prior audit's seven P0s and most P1/P2s are genuinely
closed at this HEAD — substantial, real remediation. **But the same
audit missed an entire systemic class (§3) more severe than anything it
listed.**

---

## 3. NEW release-blocking findings (the headline)

The dominant pattern, independently surfaced by **four** agents and
hand-verified: **systemic broken access control on the paid-content
surface.** P0-7 fixed the *courses* instance; the identical bug class
was never checked on rooms, video, search, or WebSocket.

### NF-1 [FV] · P0 · Entire room-content read+write surface is anonymous
`routes/mod.rs:170` mounts `room_content::public_router()` with **no
guard layer**. Handlers `list_trade_plans` (`room_content.rs:385`),
`list_alerts` (`:754`), `list_trades` (`:1449`), `get_weekly_video`
(`:1113`), `list_weekly_videos`/`list_archived_videos`/`get_room_stats`
take `(State, Path(room_slug), Query)` — **no `User` extractor, no
membership check**. `mark_alert_read` (`:1076`) is an **anonymous POST**
running `UPDATE room_alerts SET is_new=false WHERE id=$1` on a
client-supplied id. The core paid trading-room product (trade plans,
live alerts, trades, weekly videos) is served to the open internet.
**Fix:** `User` + active-membership check for `:room_slug` on every
handler; scope `mark_alert_read` to the owning user.

### NF-2 [FV] · P0 · `/video-advanced` router fully unauthenticated incl. destructive ops
`routes/mod.rs:146` mounts `admin_videos::analytics_router()` with no
`route_layer`. ~35 routes (`admin_videos.rs:796-853`) — `delete_series`,
`delete_chapter`, `clone_video`, `bulk_edit_videos`, `export_videos_csv`,
`purge_video_cdn`, `purge_all_cdn` (`:1408` `(State(_state))` — no
extractor), `analytics_dashboard` — are anonymous. Any internet user can
destroy series/chapters, purge all CDN, and exfiltrate the full video
catalog as CSV. Only `bulk_update_tags`/`bulk_feature` carry `AdminUser`.
**Fix:** apply an `AdminUser` `route_layer` to the whole router.

### NF-3 [FV] · P0 · `change_plan` never calls Stripe — free upgrades / billing divergence
`subscriptions.rs:942-1079`. Extractor is `user: User` (not admin). It
computes a `ProrationPreview` then does **only** `UPDATE user_memberships
SET plan_id` (`:1043-1059`) — **zero Stripe API call**. Any authenticated
user can POST their own plan change and receive the higher-tier
entitlement **with no charge**; downgrades grant credit-free changes;
the Stripe subscription keeps billing the old price. Direct revenue loss
+ entitlement integrity break. **Fix:** modify the Stripe subscription
item (with `proration_behavior`) and let `customer.subscription.updated`
drive the DB write; require payment confirmation before granting access.

### NF-4 · P1 · WebSocket room subscription has no authorization — **[FV]**
`websocket.rs:376-394` validates only that the JWT is a valid `access`
token; the requested `room` is never checked against membership.
`:440-460`/`:505-525` subscribe and stream all `broadcast_to_room`
traffic. Any account with any valid token receives live paid-room
alerts/trades. (Same class as NF-1, real-time channel.) **Fix:**
per-room entitlement check before `subscribe_room`; close 4003 otherwise.

### NF-5 · P1 · Room-search & unified-video media ignore membership
- `room_search.rs:151-386` (`/api/room-search/:room_slug/*`): `User`
  only, **no per-room membership** — free accounts full-text search any
  paid room's alerts/trade-plans.
- `videos.rs:349-381` (`GET /api/videos/:id_or_slug`, no extractor):
  returns `video_url` + computed Bunny `embed_url` for any `is_published`
  row, ignoring `unified_videos.room_id` — members-only media leaked
  anonymously (P0-7 sibling).

### NF-3b · P1 · OAuth login bypasses the P1-2 token-version epoch — **[FV]**
`oauth.rs:527-540` `create_oauth_auth_response` calls the
`create_jwt`/`create_refresh_token` shims, which `utils/mod.rs:245/283`
hardcode `token_version: 0`. The shim's own doc says production must use
the `_versioned` form. Consequence: (a) the P1-2 revocation guarantee
does **not** hold for Google/Apple-issued tokens; (b) after any
logout-all/reset/ban bumps a user's epoch ≥1, a subsequent OAuth login
mints v0 → instantly rejected by the extractor (OAuth login DoS for any
user who ever revoked). **Fix:** use `*_versioned` with
`redis.get_token_version(user.id)`.

---

## 4. NEW high / medium findings

| ID | Sev | Title | file:line | Fix |
|---|---|---|---|---|
| NF-6 | P1 | Over-refund: refund validated vs *original* total, ignores prior refunds | `orders.rs:1065-1079` | cap `requested + already_refunded ≤ total_cents` |
| NF-7 | P1 | Reconcile job can never correct period drift on Stripe basil+ API (`list_subscriptions` no `expand=items`) | `stripe.rs:1469-1509`, `reconcile_stripe.rs:185` | add `expand[]=data.items` |
| NF-8 | P1 | Unsupervised background worker — any panic permanently kills email/webhook/scheduled jobs (no health signal) | `main.rs:84-87`, `queue/worker.rs` | supervised restart / `catch_unwind`; surface liveness |
| NF-9 | P1 | `analytics_cohorts` still returns hardcoded data (P1-5 residue) | `admin_members.rs:1062-1070` | real SQL or honest `501` |
| NF-10 | P1 | G0.3 dormant — `schema.sql` not wired into CI/harness; from-zero replay still aborts (`041` syntax error, `053` ordering) | `ci.yml:159`, `tests/common/mod.rs:68`, `041_cms_presets.sql:86` | CI job: `psql -f schema.sql && sqlx-seed` then migrate-verify |
| NF-11 | P1/Sec | Tracked DB password fallback in orphan script; live Google key in bundle; creds in 4 tracked docs + history | `api/run-migrations.js:7`, `lib/config/google.ts:38`, docs | rotate all; history rewrite; purge docs |
| NF-12 | P1/Sec | No SCA/SAST in CI; `rsa 0.9.10` (RUSTSEC-2023-0071) ignored w/o expiry; dup `reqwest 0.11`+`0.12` | `ci.yml`, `.cargo/audit.toml`, `Cargo.toml:76` | add blocking `cargo deny`/`audit`; dedupe reqwest |
| NF-13 | Med | 21/93 admin proxies are token-presence-only (no server role) | `$lib/server/auth.ts:71` consumers | `requireAdminToken`→`requireAdmin`/`requireSuperadmin` |
| NF-14 | Med | JWT in WebSocket URL query string (leaks via logs/Referer) | `coupons.ts:607` | send token in first WS message / short-lived ticket |
| NF-15 | Med | §7 "0 shadow-state" **refuted** — 18 files use `$effect`-resync shadow-state (incl. `VideoEmbed.svelte:337`) | 18 files | `$bindable`/`$derived`; prior grep was too narrow |
| NF-16 | Med | Poison job stuck `processing` forever + double-grab race (lock not held across select→mark) | `queue/worker.rs:579-642` | one tx select→mark; reclaim sweep |
| NF-17 | Med | Multi-replica schedulers double-run (no advisory lock/leader) | `reconcile_stripe.rs:33`, `email_reminders.rs:37` | `pg_try_advisory_lock` |
| NF-18 | Med | Redis `KEYS` (blocking O(N)) on every cache invalidation; comments falsely say "SCAN" | `redis.rs:499-511`, `cache/service.rs:449` | `SCAN`+`UNLINK` |
| NF-19 | Med | Idempotency keys random per-call & not persisted (only refund deterministic) | `stripe.rs:396…1445` | persist deterministic key per logical op |
| NF-20 | Med | `formatGuideHtml` rendered via regex-only sanitizer (not DOMPurify) — XSS on CMS-authored install guide | `dashboard/indicators/[id]/+page.svelte:525` | route through `$lib/sanitize` |
| NF-21 | Med | Apple ID-token nonce skippable when claim absent; Apple unverified email auto-marked verified | `oauth.rs:276-295,450` | require `nonce` present; gate on `claims.email_verified` |
| NF-22 | Med | Percent-coupon `discount_value*100` cross-boundary contract fragile (no type branch) | `admin.ts:829-838`, `coupons.rs:234` | branch on `discount_type`; integer-cents math |
| NF-23 | Low | 271 `ON DELETE CASCADE` FKs; non-idempotent `DROP TABLE…CASCADE` in fwd migrations `007`/`016` | migrations | soft-delete guard; `CREATE IF NOT EXISTS`+`ALTER` |
| NF-24 | Low | Login proxy echoes tokens in JSON body alongside httpOnly cookies | `routes/api/auth/login/+server.ts:55-76` | strip tokens from body |
| NF-25 | Low | Catch-all proxy forwards client `X-Forwarded-For`/`Host` verbatim | `routes/api/[...path]/+server.ts:182` | derive from `getClientAddress()` |
| NF-26 | Low | `health/detailed` (un-gated) leaks DB/Redis up/down+latency+env | `health.rs:63-132` | gate or trim |
| NF-27 | Low | `realtime/events` streams all CMS editorial activity to any authed user | `realtime.rs:331-347` | scope events to entitlement |
| NF-28 | Low | DOMPurify global `addHook`/`removeHook` race under SSR concurrency | `lib/utils/sanitize.ts:177-208` | register hook once / per-call instance |
| NF-29 | Low | postMessage origin substring checks (`includes('mediadelivery.net')`/youtube) | `BunnyVideoPlayer.svelte:224`, `sanitize.ts:188` | hostname-suffix match |
| NF-30 | P3 | Latent SQLi in dead `domain/content/repository.rs:218,232` (`ORDER BY {}` no whitelist) | — | whitelist or delete dead repo |

---

## 5. Forensic reconciliation — do the agents "all match"?

**Yes, after resolving three discrepancies:**

1. **Agent-4's headline "P0 BIGINT→i32 decode panic"** (`indicator_enhanced.rs:54` vs migration `061`) — **downgraded to P3 [FV].** The
   `IndicatorEnhanced` struct is referenced *only* by `indicators_admin.rs`,
   which `routes/mod.rs:40,111` shows is **commented out / not mounted /
   not even compiled**. The mounted `admin_indicators.rs` uses its own
   `IndicatorRow{price_cents:i64}` against the `indicators` table. Real
   type-debt in dead code, **not a live P0**. (Still a valid example of
   the §1a sqlx blind spot.)
2. **Prior audit factual error corrected [FV]:** it claimed `EnvFilter`
   defaults to `debug` in prod — `main.rs:32` actual default is
   `"revolution_api=info,tower_http=info"` (**info**, no debug leak).
3. **Count corrections:** proxy contract is **151 files / 151 compliant
   (0 hardcoded prod URLs)**, not "129/129" (denominator was wrong, claim
   true in spirit); P2-J collisions **refuted**; oversized components
   **55** (not 57); suppressed a11y **77** (not 90); dead Svelte
   components **3** (`*Legacy`) not 7.

**Cross-corroboration (high confidence):** the broken-access-control
class (§3 NF-1/2/4/5/3b) was independently surfaced by the CMS,
services, auth, and proxy agents converging on the same root cause —
routers mounted without a guard layer / handlers missing the `User`
extractor. The money-divergence class (NF-3, NF-6) corroborated by the
payments + frontend-money agents. The secret-residue (NF-11) corroborated
by the CI agent + main-thread `git`/`git grep`. No unresolved
contradictions remain.

---

## 6. Verified-clean (independently confirmed solid at this HEAD)

JWT (HS256 pinned, access/refresh segregated, 7 tests) · Argon2id OWASP
params + bcrypt-Laravel rehash-on-login · token blacklist SHA-256
fail-closed · rate limiters fail-closed · MFA/TOTP constant-time + atomic
· OAuth PKCE/state single-use, no token-in-URL · webhook signature
constant-time HMAC over raw body, ±300s, fail-closed, **now atomic**
(P0-4) · coupon increment exactly-once in the tx · `create_checkout`
properly tx-wrapped · CORS explicit origins · CSP no `unsafe-inline` in
script-src · cookies HttpOnly+SameSite+Secure · 84 `{@html}` sinks — all
DOMPurify/escaped except NF-20 · proxy backend-URL contract 151/151 ·
P0-3/P0-4/P0-5/P0-6/P0-7/P1-3/P1-4/P1-6/P1-9/P1-10 genuinely fixed ·
`schema.sql`/`sqlx-seed.sql` checksum-correct · no committed
`node_modules`/`target` · no live `sk_`/`whsec_` in tracked source.

---

## 7. Prioritized backlog

**P0 (release-blocking, NEW):** NF-1 anonymous room-content surface ·
NF-2 unauthenticated `/video-advanced` (destructive) · NF-3 `change_plan`
free upgrades/no-Stripe · P0-2 rotate creds + history rewrite + purge
docs.

**P1:** NF-4 WS room authz · NF-5 room-search/video membership · NF-3b
OAuth token-version · NF-6 over-refund · NF-7 reconcile expand-items ·
NF-8 supervised worker · NF-9 cohorts fabricated · NF-10 wire G0.3 into
CI + fix `041`/`053` · P1-7 money→BIGINT cents · NF-11 secrets · NF-12
SCA/SAST + rsa + reqwest dedupe.

**P2:** NF-13 admin-proxy RBAC · NF-14 JWT-in-WS-URL · NF-15 shadow-state
sweep (18) · NF-16 poison job · NF-17 scheduler leader-lock · NF-18 Redis
SCAN · NF-19 persist idempotency keys · NF-20 install-guide sanitizer ·
NF-21 Apple nonce/email · NF-22 percent-coupon contract · P2-D residual
error-swallow.

**P3:** NF-23 cascade/idempotent migrations · NF-24/25/26/27 info-leak
hardening · NF-28/29 sanitizer races/origin checks · NF-30 dead-repo
SQLi · §1a — adopt compile-time `query!` or a DB-backed CI smoke so the
~1,630 runtime queries get schema verification · split 55 oversized
components · consolidate `/signup`+`/register` · run `svelte-autofixer`
on a host where the MCP is permitted.

---

## 8. Bottom line

The team has genuinely closed every prior P0 and most P1/P2 since the
2026-05-17 baseline — the security posture of *audited* paths (auth
crypto, webhook atomicity, checkout, JSON-LD) is now strong. **However,
this audit found a systemic broken-access-control class the prior audits
missed entirely: the paid product's core data and a destructive video
admin router are exposed to anonymous internet users (NF-1, NF-2), and
any user can grant themselves a paid upgrade for free (NF-3).** These
three are more severe than anything in the prior backlog and are not
caught by any current gate (no e2e, no DB tests, lint can't see a
missing extractor). The `SQLX_OFFLINE` blind spot (§1a) means a green
build is not evidence the data layer is correct. Address the §7 P0s
before any release.
