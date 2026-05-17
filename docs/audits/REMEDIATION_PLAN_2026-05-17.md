# Remediation Plan — 2026-05-17

Staged, root-cause remediation of every finding in
[`FULL_REPO_AUDIT_2026-05-17.md`](FULL_REPO_AUDIT_2026-05-17.md).

- **Branch base:** `claude/full-repo-audit-KQ9dT` (HEAD `fb7a401`)
- **Discipline:** principal-engineer fixes only. Root cause, fixed at
  the contract/boundary, one canonical implementation per concern, no
  band-aids, **delete dead code (do not comment it out)**, every fix
  lands with a regression test that runs in now-blocking CI. No skipped
  / aspirational tests. Match house style.

## Standing constraints

- **Bunny CDN account is DOWN (contact: Errol).** Expect `4xx/5xx` from
  `api.bunny.net` / `*.b-cdn.net` on any video upload/playback path.
  **No stage is blocked on live Bunny verification.** Bunny-adjacent
  logic is verified by (a) the opaque-GUID authorization branch in
  integration tests with seeded enrollments, (b) mocking the Bunny
  client at its trait boundary, (c) a single deferred
  `POST-BUNNY-RESTORE` live-playback smoke checklist. Items that touch
  Bunny are tagged **🐰** below.
- **No live DB / Docker stack / Playwright in this environment.**
  Stage 1 builds the verification substrate (real-module tests +
  real-JWT integration harness + blocking CI) *first* so every later
  stage is provable, not asserted.
- **Owner-decision gates** (Stage 0) must be signed off before the
  irreversible actions they guard. They are tracked, not silently
  executed.

## Root-cause clustering

Findings are not fixed one-by-one; they are collapsed into clusters
that share a root cause and a single corrective change.

| Cluster | Root cause | Findings |
|---|---|---|
| **A — Secret exposure** | secrets in VCS + email-string used as an authz boundary | P0-2, P1-1, P2-H, L1/L2, security-M1 |
| **B — Auth authority** | missing server-side authority + no token lifecycle | P0-3, P1-2, P1-3, P1-4, P1-6, P2-F, security-M3 |
| **C — Payments/money integrity** | non-atomic webhook + money-as-float + contract drift | P0-1, P0-4, P0-5, P1-7, P1-8, P2-A, P2-B, P2-C, P2-I |
| **D — Output safety** | hand-rolled serialization into HTML | P0-6 |
| **E — Data exposure / paywall** 🐰 | read endpoint with no entitlement filter | P0-7 |
| **F — Honest data / observability** | stubbed data shipped as real; errors swallowed | P1-5, P2-D, P3 log-level |
| **G — Egress boundary** | server fetch to caller-influenced URL | P2-E |
| **H — CI/test enforcement** | gates advisory; tests exercise copies | P1-9, P1-10 |
| **I — Persistence correctness** | process-local fallback masking DB failure | P3 consent OnceLock |
| **J — Structural debt** | duplication, dead code, oversized units | P2-G, P2-J, all P3 hygiene |

## Master tracking matrix

Status: ☐ todo · ◐ in progress · ☑ done · ⏸ blocked on owner gate ·
🐰 Bunny-deferred verification.

| ID | Cluster | Stage | Status |
|---|---|---|---|
| P0-2 secret/cred exposure | A | 0 | ☑ (PR #587 — env-sourced via `_creds.ts`/`.env`; literals gone, `pnpm check` 0/0. Rotation owner-deferred per owner: not in prod, rotate later, no history rewrite) |
| P1-1 Google key committed | A | 0 | ☑ (PR #587 — centralized to one env-overridable `google.ts`; literal in 1 place. Rotation + Google Cloud restriction owner-deferred) |
| Bunny account restore | — | 0 | ☐ ⏸ (Errol — external; code paths done, live smoke deferred) |
| R2 rotation (carried from CHANGELOG C-3) | A | 0 | ☐ ⏸ (owner-external rotation; no in-tree literal to source) |
| 🔴 P0 schema not reproducible (G0.3, 8/60 migrations fail fresh) | — | 0 | ☑ (PR #587 — reconstructed prod-safe `api/migrations/schema.sql`+`sqlx-seed.sql`; verified 60/60 replay, from-zero 0 err/204 tables, gated no-op proof test, `0*.sql` byte-identical. ONE bounded owner cutover step remains — see G0_3 note) |
| P2-H email-verify bypass | A | 2 | ☑ (PR #584 — bypass requires `email_verified_at`+DB role; email-list secondary only) |
| security-M1 email elevation | A | 2 | ☑ (PR #584 strict helpers + #586 admin-extractor companion) |
| P1-9 CI not enforcing | H | 1 | ☑ (eslint+clippy blocking; dead workflow removed; pnpm single-source) |
| P1-10 tests test copies | H | 1 | ◐ (1a: no-DB tests bind real crate ✓ #574; 1b: real-JWT DB harness **now UNBLOCKED by G0.3 #587** — `schema.sql`+seed gives a fresh DB; activation pending) |
| CI fmt-red since 4420d1d | H | 1 | ☑ (pre-existing `cargo fmt` drift in stripe.rs:826/842 fixed) |
| P0-3 /admin gate client-only | B | 2 | ☑ (PR #576 — server +layout.server.ts gate, DRY isAdminRole) |
| P1-2 token revocation | B | 2 | ☑ (PR #583 per-user token_version epoch + #584 ban-path; Redis runtime authority. Durable DB-column = G0.3-gated follow-up) |
| P1-3 spoofable client IP | B | 2 | ☑ (PR #579 — ConnectInfo + trusted-proxy CIDR, 19 no-DB tests) |
| P1-4 unauth endpoints | B | 2 | ☑ (PR #579 — AdminUser/SuperAdminUser + MONITORING_TOKEN) |
| P1-6 set-session | B | 2 | ☑ (PR #576 — insecure POST deleted) |
| P2-F proxy RBAC | B | 2 | ☑ (PR #578 — 51 proxies onto requireAdmin/requireSuperadmin) |
| security-M3 refresh TTL | B | 2 | ☑ (folded into PR #583 — token_version epoch covers C1/M3; refresh() re-checks before rotation) |
| CI red root causes (pnpm/toolchain/OOM-heap/API host/prerender SQLITE_BUSY) | H | 1 | ☑ (PR #579/#580/#581 + #585 prerender concurrency:1 — Backend+Cloudflare green; Frontend SQLITE_BUSY root-caused+fixed) |
| P1-7 money→cents schema | C | 3 | ☐ (UNBLOCKED by G0.3 #587; not yet done — highest-risk money-schema, needs dedicated careful pass) |
| P2-I migration 068 / sqlx state | C | 3 | ☐ (UNBLOCKED by G0.3 #587; not yet done) |
| P0-1 checkout contract | C | 3 | ☑ (PR #582 — real CheckoutRequest contract, dead client removed) |
| P0-4 webhook atomicity | C | 3 | ☑ (PR #582 — single-tx webhook, dedup on processed_at, Stripe-retry reprocesses) |
| P2-A persisted idempotency | C | 3 | ☑ (PR #582 — deterministic SHA-256 refund Idempotency-Key) |
| P2-C webhook DB errors swallowed | C | 3 | ☑ (PR #582 — ?/500 propagation) |
| P0-5 plan price normalizer | C | 3 | ☑ (PR #582 — normalizePlanPayload, /price flow preserved) |
| P1-8 expand=items | C | 3 | ☑ (PR #582 — get_subscription expand[]=items.data) |
| P2-B refund authz | C | 3 | ☑ (PR #582 — AdminUser-gated create_refund) |
| P0-6 JSON-LD XSS | D | 4 | ☑ (PR #579 — canonical serializeJsonLd, all 5 sites; +10 tests) |
| P0-7 course paywall | E | 4 | ☑ (PR #579 — 403 + per-lesson entitlement + GUID withholding; 🐰 live deferred) |
| P1-5 fabricated analytics | F | 5 | ☑ (PR #579 — real revenue/AOV; honest 501 for churn) |
| P2-D unwrap_or_default swallow | F | 5 | ☑ (PR #579 — ?-propagated 500s) |
| P3 log-level default | F | 5 | ☑ (PR #582 — EnvFilter debug→info) |
| P2-E webhook SSRF | G | 5 | ☑ (PR #579 — https-only + private-range block; +16 tests) |
| P3 consent OnceLock | I | 5 | ◐ (UNBLOCKED by G0.3; agent in flight — replace process-local fallback with fail-closed DB authority) |
| P2-G SW auth-HTML cache | J | 6 | ☑ (PR #582 — isPrivatePath guard) |
| P2-J component collisions | J | 6 | ◐ (agent in flight — canonicalize/rename, behavior-preserving) |
| P3 dead module/components | J | 6 | ☑ (PR #582 — courses.rs/cms_scheduler.rs + 7 dead components deleted) |
| P3 mega-components | J | 6 | ☐ (open — large decomposition; dedicated careful pass) |
| P3 a11y suppressions | J | 6 | ◐ (10 fixed at source PR #582; ~77 `svelte-ignore a11y` remain — open) |
| P3 /signup vs /register | J | 6 | ☐ (open — route duplication, both exist) |
| P3 ~36MB cruft | J | 6 | ☑ (PR #581 — 161 files/~36MB; + PR #587 dropped 268K PII scrape) |
| P3 stale comments / X-XSS hdr | J | 6 | ☑ (PR #582 X-XSS removed; PR #587 Laravel-rot comments exterminated) |

---

## Stage 0 — Containment & owner-decision gates (no code)

**Goal:** stop active exposure; record irreversible decisions.

- ☐ **Rotate** the superadmin password (`Davedicenso01!`) and the
  Google API key (`AIzaSyBTC-…`). Out-of-band; owner/ops.
- ☐ **R2 credential rotation** — carried open from CHANGELOG C-3.
- ☐ **Bunny account** — owner engages Errol; blocks only live video
  verification (🐰 items), not code.
- ☐ **Decision gate G0.1 — git history rewrite.** Rotation neutralizes
  the leaked secrets; scrubbing them from history
  (`git filter-repo`/BFG) is a force-rewrite of shared history and
  needs explicit owner approval + a coordination window. Default if not
  approved: rotate-only, document the exposed-then-rotated secrets.
- ☐ **Decision gate G0.2 — prod migration apply.** Stage 3's
  money-schema migrations and the existing `068` are operator-applied
  against prod per CLAUDE.md. Capture the `_sqlx_migrations` table
  state vs disk (resolves P2-I uncertainty) before any deploy.
- ◐ **🔴 P0 / G0.3 — reconstructed baseline shipped; fresh-env mechanism
  proven; ONE owner cutover step remains.** CONFIRMED 2026-05-17 by full
  fresh-DB replay that of 60 migrations 8 fail on a clean DB. **Root
  cause re-derived empirically** (the original "8 fail" list was a
  cascade): the historical chain assumes a Laravel-era substrate that no
  migration creates — vestigial table `user_sessions`, vestigial FK
  target `traders` (036's `trader_id BIGINT REFERENCES traders(id)` sits
  in a `DO $$ … EXCEPTION WHEN others THEN NULL` block, so its absence
  silently rolled back ALL of 036's column adds → the apparent
  `trading_room_id`/`content_type`/`section`/`resource_date` failures),
  `unified_videos.tags` (JSONB — `jsonb_path_ops` GIN index rejects
  TEXT), and `user_memberships.trial_ends_at`/`grace_period_end` (053
  indexes them 7 migrations before 060 adds them). Plus three genuine
  intra-chain defects (041 invalid `UNIQUE … WHERE` table constraint;
  050 duplicate function; 054 missing `performed_by`).
  **Shipped (prod-safe):**
  `api/migrations/schema.sql` — a `pg_dump --schema-only --no-owner` of a
  DB built by replaying ALL 60 migrations on top of a reconstructed
  pre-sqlx baseline. **Proven:** `REPLAY 60/60 OK`; `schema.sql` loads
  into a brand-new empty DB with **0 errors / 204 tables**; and the gated
  test `api/tests/g03_schema_baseline_test.rs` proves that after
  `schema.sql` + a `_sqlx_migrations` seed, the **exact** production call
  `sqlx::migrate!("./migrations").run()` is a clean no-op (all 60 SHA-384
  checksums match the embedded ones — no `VersionMismatch`). Every
  reconstructed column type is traced to the Rust app contract in
  `api/src/` (see `docs/audits/G0_3_SCHEMA_BASELINE_2026-05-17.md`).
  Production is untouched: `git diff --stat -- api/migrations/0*.sql` is
  EMPTY; the app still replays the committed chain unchanged.
  This **unblocks Stage 1b activation** and DB-backed verification for
  CI/local/DR via `api/migrations/README.md`.
  **Remaining owner-gated step (the only one):** a coordinated
  maintenance-window cutover of the *production* DB onto this baseline —
  `pg_dump --schema-only` prod, `diff` vs this `schema.sql`, reconcile
  any prod-only objects, reset prod's `_sqlx_migrations` lineage. The
  exact runbook + diff command is in
  `docs/audits/G0_3_SCHEMA_BASELINE_2026-05-17.md`; with the
  reconstructed `schema.sql` in hand this is a ~30-min owner task, not a
  research project. Evidence:
  `docs/audits/G0_3_SCHEMA_BASELINE_2026-05-17.md`,
  `docs/audits/FULL_REPO_AUDIT_2026-05-17.md` §Schema-Reproducibility.

**Exit:** secrets rotated; G0.1, G0.2, G0.3 answered (recorded here).

---

## Stage 1 — Verification substrate (Cluster H) — *do first*

**Goal:** make every later stage provable in CI, not asserted. You
cannot principal-engineer a payments rewrite without a real test
harness.

- ☑ **P1-10 (1a)** Deleted the duplicated `mod stripe`/`mod utils`
  copies; `stripe_test.rs`/`utils_test.rs` now bind the real crate
  (`revolution_api::services::stripe::StripeService::verify_webhook`,
  `revolution_api::utils`) via the public builder. Added a single
  canonical Stripe-signature simulator (`tests/common/stripe_sig.rs`,
  reused by Stage 3). New regression guards that the copies could never
  catch: HS256-pinning (forged `alg:none` rejected), access/refresh
  token-type segregation, webhook replay-window (stale/future ts), and
  strict-UTF-8 payload rejection. Verified: stripe_test 19/19,
  utils_test 18/18, clippy `-D warnings` clean, fmt clean.
- ☑ **CI fmt-red root cause** Commit `4420d1d` (the prior "P0 fix")
  added the idempotency-key headers at `stripe.rs:826/842` without
  `cargo fmt`, so the **blocking** `cargo fmt --all --check` step has
  failed since — backend CI was red on every PR. Fixed (2-line
  indentation, no behavior change). New finding; folded into the audit.
- ◐ **P1-10 (1b)** Real-JWT admin integration harness shipped
  (`api/tests/common/mod.rs`): the fake `"test_admin_token"` /
  `"test_user_token"` are gone — `setup_test_app_with_admin` /
  `_with_user_token` now `seed_user(...)` a real row and mint a real
  HS256 JWT via the production `revolution_api::utils::create_jwt`, and
  `setup_test_app` calls `db.migrate()`. The harness is correct, but
  running it surfaced **🔴 P0 G0.3**: a fresh DB cannot migrate (8/60
  migrations fail; see Stage 0). This is the *honest* boundary — the
  old fake-token harness hid it. **Stage 1b activation (and the
  Postgres-in-CI integration job) is blocked by G0.3** and is NOT
  wired into CI (a permanently-red gate would defeat Stage 1's
  purpose). Harness ships now so it is ready the moment G0.3 lands.
- ☑ **P1-9** Blocking `eslint` (frontend job) and
  `cargo clippy --locked --all-targets -- -D warnings` (backend job)
  added to the canonical `.github/workflows/ci.yml`;
  `continue-on-error` removed from clippy; `--locked` on check/test;
  pnpm version de-pinned so `pnpm/action-setup` derives it from
  `package.json#packageManager` (one source of truth). Deleted the dead
  `frontend/.github/workflows/ci.yml` (wrong location + npm-in-pnpm).
- ◐ Add a Postgres+Redis service to CI so the integration tests run on
  every PR (pairs with 1b; gates Stages 2–5).

**Exit (1a — met):** CI red on a real `services/stripe.rs` /
`utils::verify_jwt` regression; lint+clippy+fmt block merges.
**Exit (1b — pending):** admin integration tests run with real JWTs
against a CI Postgres.

---

## Stage 2 — Auth authority (Clusters B + A-runtime)

**Goal:** one server-side source of authorization truth; tokens have a
lifecycle.

- ☐ **P0-3** Add `frontend/src/routes/admin/+layout.server.ts` whose
  `load` 303-redirects when
  `!locals.user || !ADMIN_ROLES.has(locals.user.role)` (reuse the role
  set in `lib/server/auth.ts`). Remove reliance on the client `onMount`
  redirect. Keep `ssr=false` only if independently justified; the gate
  no longer depends on it.
- ☐ **P1-2 + security-M3** Root fix for the whole "stolen token
  survives" class: per-user `token_version` (i64) in DB + Redis,
  embedded as a JWT claim, checked in the `User` extractor; bumped on
  logout-all / password-reset / ban. One change closes access-token
  *and* refresh-token survival.
- ☐ **P1-3** Replace `client_ip()` with
  `ConnectInfo<SocketAddr>`-derived peer IP; honor `X-Forwarded-For`
  only from a configured trusted-proxy CIDR allowlist.
- ☐ **P2-F** One canonical `requireAdmin`/`requireSuperadmin` proxy
  guard in `lib/server/auth.ts`; apply to **all** `routes/api/admin/**`
  proxies. Delete the empty-`200`-on-missing-token anti-pattern in
  `admin/coupons/+server.ts`.
- ☐ **P1-4** Add the missing extractor to
  `GET /api/admin/coupons/validate/:code`; gate `/init-db` &
  `setup_db`/`run_migrations` behind `SuperAdminUser` (not a defaultable
  env string); network-ACL `/monitoring/metrics*`.
- ☐ **P1-6** Delete the `POST /api/auth/set-session` handler (OAuth
  callback no longer needs it); keep only the `DELETE`.
- ☐ **P2-H + security-M1** Email-list membership is no longer an authz
  boundary: require DB role **and** `email_verified_at` for elevation;
  the email list becomes a non-authoritative convenience hint only.

**Verify:** real-JWT integration tests (Stage 1) — member token →
403 on every admin route and the admin shell; tampered/old-version
token → 401; spoofed `X-Forwarded-For` no longer resets the limiter.

**Exit:** no client-only authz; one stale-token check; email never
elevates.

---

## Stage 3 — Payments & money integrity (Cluster C)

**Goal:** money is integer cents end-to-end; checkout has one contract;
the webhook is atomic and idempotent. Schema first — everything sits on
it.

- ☐ **P1-7 + P2-I** Money-schema migration: widen `orders`,
  `membership_plans.price`, `products.price`, `order_items` and the
  residual `courses.price_cents`/`indicators.price_cents` to `BIGINT`
  cents; supersede/sequence cleanly with the existing operator-gated
  `068`. Replace every `$n::BIGINT / 100.0` float SQL
  (`payments.rs:480-482,1321`, `checkout.rs:293,343`) with integer-cents
  columns. Apply is **G0.2-gated**.
- ☐ **P0-1** Define one canonical `CheckoutRequest`/`CheckoutResponse`
  contract. Wire `checkout/+page.svelte` → the real transactional
  `create_checkout` (`checkout.rs:65`) via `POST /api/checkout` with the
  correct body. **Delete** both dead `createCheckoutSession`
  implementations (`cart.ts:1115`, `lib/api/checkout.ts:149`).
- ☐ **P0-4 + P2-A + P2-C** Rewrite each webhook event handler around a
  single `Pool::begin()` … `tx.commit()`; set
  `webhook_events.processed_at` only post-commit; reprocess rows where
  `processed_at IS NULL` instead of skipping by event-id presence;
  derive the Stripe `Idempotency-Key` deterministically from the
  order/subscription id (persisted), not a per-call UUID; propagate DB
  errors via `?` so Stripe retries (kill the `.ok()` swallows at
  `payments.rs:1127,1183,1905`).
- ☐ **P0-5** Single `normalizePlanPayload` chokepoint emitting
  `price_cents = Math.round(price*100)` for `subscriptionPlansApi` and
  the hand-built body in `plans/+page.svelte` — mirror the
  audit-verified-correct coupon normalizer. Confirm intended flow vs
  the separate `/plans/:id/price` endpoint first.
- ☐ **P1-8** Add `expand[]=items.data` to `get_subscription`
  (`stripe.rs:794-808`); membership periods stop landing on the epoch.
- ☐ **P2-B** Gate `/payments/refund` behind an explicit owner-refund
  policy + rate limit (or `AdminUser`).

**Verify:** Stripe **test keys** (no Bunny dependency). Integration
tests: full checkout creates a consistent order+membership atomically;
a forced mid-handler failure rolls back fully and the Stripe retry
completes it; plan create/edit round-trips `price_cents`.

**Exit:** no float money; one checkout path; webhook all-or-nothing &
replay-safe.

---

## Stage 4 — Output safety & paywall (Clusters D + E 🐰)

- ☐ **P0-6** One hardened JSON-LD serializer (escape `<`,`>`,`&`,
  U+2028, U+2029 — or a vetted helper) in **one** component; route
  `SEOHead.svelte`, `seo/SeoHead.svelte`, `store/scanners/+page.svelte`,
  `tools/options-calculator/+page.svelte` through it. Delete the
  duplicate inline implementations.
- ☐ **P0-7 🐰** Enforce entitlement in `get_course_player`
  (`member_courses.rs:255-353`): not-free **and** `enrollment.is_none()`
  **and** not admin → `403`; additionally strip
  `bunny_video_guid`/`downloads`/non-preview lessons from any
  non-entitled response (defense-in-depth). **Bunny down:** verify the
  authorization branch with seeded enrolled/non-enrolled users (the
  GUID is opaque — logic is fully testable without Bunny); live
  playback → `POST-BUNNY-RESTORE` checklist.

**Verify:** XSS payload in a CMS schema field no longer breaks out
(unit test on the serializer with the OWASP `</script>` + U+2028
corpus); non-enrolled user gets 403 and zero GUIDs.

---

## Stage 5 — Honest data, egress, persistence (Clusters F + G + I)

- ☐ **P1-5** `analytics_churn_reasons`/`analytics_revenue` compute from
  real tables, or return `501` / `data_available:false` — never
  fabricated figures.
- ☐ **P2-D** Replace `unwrap_or_default()` swallows
  (`admin_members.rs:1143`, `room_resources.rs:1825`, `posts.rs:900`)
  with `?`-propagated 500s, matching house pattern.
- ☐ **P3** `EnvFilter` default → `info` (env may raise it).
- ☐ **P2-E** In `cms_webhooks.rs::deliver_webhook`, resolve the host
  and reject loopback/link-local/RFC-1918 before egress; https-only
  allowlist.
- ☐ **P3** Land a dedicated `consent_settings` table; treat a DB write
  failure as `500` — delete the process-local `OnceLock` fallback
  (root cause: it masks failure and is multi-replica-inconsistent).

---

## Stage 6 — Structural debt & hygiene (Cluster J)

Lower risk; principal-engineer = delete/dedupe/decompose, not annotate.

- ☐ **P2-G** Service worker: never cache navigations to `/dashboard`,
  `/account`, `/admin`, `/checkout`, `/trading-room`.
- ☐ **P2-J** Dedupe the byte-identical `RepeaterField`; namespace the
  remaining 15 component-name collisions by domain prefix.
- ☐ **P3** Delete dead Rust `courses` module + orphan
  `cms_scheduler::start_scheduler` (or wire it — owner call); delete
  the 7 zero-ref Svelte components (incl. 3,548 LOC dead
  `CourseBuilder`/`IndicatorBuilder`); add `LIMIT`+pagination to
  unbounded list queries.
- ☐ **P3** Decompose the worst route mega-components
  (`admin/courses/create` 4,892, `admin/videos` 3,746,
  `admin/trading-rooms/[slug]` 3,278) behind stable interfaces.
- ☐ **P3** Fix the 7 real `a11y_label_has_associated_control` and 3
  deprecated `<svelte:component>` at source; remove the `svelte-ignore`.
- ☐ **P3** Consolidate `/signup` + `/register` → one route + 308.
- ☐ **P3** `git rm` ~36 MB cruft (`Implementation/`, `Do's/`,
  `retired/`, orphan `frontend/e2e/`, `docs/forensics/`); prune
  `docs/audits/`.
- ☐ **P3** Remove stale Fly.io comments; drop the deprecated
  `X-XSS-Protection` header.

---

## Execution rules

1. One stage = one branch off `claude/full-repo-audit-KQ9dT` = one PR;
   merge-blocked by the Stage-1 gates.
2. Update the master matrix in this file in the same commit as each fix
   (the matrix is the single source of "what's left").
3. Every fix ships its regression test; no `it.skip`.
4. Stages 0→1→2→3 are ordered (each depends on the prior). Stages
   4/5/6 may parallelize after Stage 1.
5. 🐰 items: code + non-Bunny verification land now; live-Bunny smoke
   goes on the `POST-BUNNY-RESTORE` checklist, not the critical path.
6. ⏸ items do not start until their Stage-0 gate is recorded answered.

## POST-BUNNY-RESTORE checklist (deferred, non-blocking)

- ☐ Live playback smoke for an enrolled user after the P0-7 fix.
- ☐ Video upload happy/error path smoke (any video-upload-adjacent
  change made in Stages 4/6).

---

## Isolated & root-caused — owner / follow-up gated (not code-fixable in scope)

These were chased to the actual root with reproducible hard evidence;
each needs a decision/asset outside the audit's app-code scope.

### CI-1 — GitHub-Actions "Frontend" red — ✅ FIXED (was ROOT-CAUSED)

**Resolution (committed):** `frontend/svelte.config.js`
`prerender.concurrency: 8 → 1`. `adapter-cloudflare` spins one emulated
Miniflare/`workerd` per prerender worker; 8 concurrent instances
contended on workerd's internal SQLite lock → `SQLITE_BUSY` →
`ERR_RUNTIME_FAILURE`. Serializing prerender removes the contention.
Decisive cold-build proof: same fresh-clone methodology that
reproduced `BUILD:1`+`SQLITE_BUSY` now yields `INSTALL:0 BUILD:0 ✔
done`, zero `SQLITE_BUSY`/`ERR_RUNTIME_FAILURE`. Production-safe:
build-time only; the real Cloudflare runtime provides `platform`
unaffected; no risky workerd/miniflare version pins. Original
root-cause analysis retained below for history.

**Status:** NOT a product/code defect. Cloudflare Pages (the primary
production deploy) builds + deploys the identical code **green on every
push** (verified across #581/#582/#583). Cold repro on exact
`origin/main`: `install`/`check`/`lint`/`vitest` **all pass**; `vite
build` itself **succeeds** (`✓ built`). The job then fails because
SvelteKit `@sveltejs/adapter-cloudflare@7.2.8` starts **Miniflare 4 /
`workerd`** to emulate `event.platform` for prerendered routes and
workerd dies:

```
workerd/util/sqlite.c++:842: failed: SENTRY_DO SQLite failed;
  NOSENTRY database is locked: SQLITE_BUSY
MiniflareCoreError [ERR_RUNTIME_FAILURE]: The Workers runtime failed to start
```

Concurrent Miniflare/workerd instances during the build-time prerender
contend on workerd's internal SQLite lock (`SQLITE_BUSY`). Disproven
along the way (with evidence): not pnpm-ignored-builds (install green),
not unbuilt `workerd` (`pnpm rebuild` → binary present → still fails),
not vitest/jsdom (vitest green), not the heap/`API_BASE_URL`
placeholder. It is a **third-party build-tool concurrency regression**
(`workerd@1.20260515.1` + `miniflare@4.20260515.0`), timing-sensitive
(explains intermittency), surfacing only in the GH-Actions adapter
prerender path — never in the Cloudflare production build.

**Root-cause fix options (follow-up; touch adapter/prerender or 3rd-party
versions, outside the audit's app-code remediation scope):**
1. Eliminate build-time `platform` emulation: guard every
   `event.platform` access behind `building` from `$app/environment`,
   or set `export const prerender = false` on the specific prerendered
   route(s) that touch `platform` (short trace to identify them) → the
   adapter never starts Miniflare → no `SQLITE_BUSY`.
2. Pin `workerd` / `miniflare` / `@sveltejs/adapter-cloudflare` to a
   combination without this `SENTRY_DO`/`SQLITE_BUSY` regression.
3. CI-only mitigation: isolate Miniflare's persistence dir per
   invocation and/or force single-threaded prerender.

Recommended: option 1 (correct + removes a heavy build-time dependency).
Deliberately **not** done mid-audit: it is non-product-blocking, needs
a route-level trace + adapter behavior change, and the user scoped this
class as "isolate, then root-cause later."

### CI-2 — `Build + Deploy` (secondary `deploy-cloudflare.yml`) red
Owner-gated: needs repo secrets `CLOUDFLARE_API_TOKEN` /
`CLOUDFLARE_ACCOUNT_ID`, or disable this redundant secondary workflow
(the primary Cloudflare Pages git-integration deploy is green). Also
still hardcodes the stale `PNPM_VERSION: "10.33.2"`. Pre-existing.

### Carry-over owner-gated
🔴→◐ G0.3 — reconstructed baseline `api/migrations/schema.sql` shipped
(proven 60/60 replay + from-zero load 0 err/204 tables + embedded-migrator
no-op test). **Only the prod-DB maintenance-window cutover remains
owner-gated** — exact `pg_dump`/`diff` runbook in
`docs/audits/G0_3_SCHEMA_BASELINE_2026-05-17.md`. · rotate leaked secrets
P0-2/P1-1/R2 + G0.1 history-rewrite decision · Bunny account (Errol) ·
deferred-by-risk: P2-J 16 collisions, mega-component decomposition,
/signup-vs-/register.
