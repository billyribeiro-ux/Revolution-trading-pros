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
| P0-2 secret/cred exposure | A | 0 | ☐ ⏸ |
| P1-1 Google key committed | A | 0 | ☐ ⏸ |
| Bunny account restore | — | 0 | ☐ ⏸ (Errol) |
| R2 rotation (carried from CHANGELOG C-3) | A | 0 | ☐ ⏸ |
| P2-H email-verify bypass | A | 2 | ☐ |
| security-M1 email elevation | A | 2 | ☐ |
| P1-9 CI not enforcing | H | 1 | ☐ |
| P1-10 tests test copies | H | 1 | ☐ |
| P0-3 /admin gate client-only | B | 2 | ☐ |
| P1-2 token revocation | B | 2 | ☐ |
| P1-3 spoofable client IP | B | 2 | ☐ |
| P1-4 unauth endpoints | B | 2 | ☐ |
| P1-6 set-session | B | 2 | ☐ |
| P2-F proxy RBAC | B | 2 | ☐ |
| security-M3 refresh TTL | B | 2 | ☐ |
| P1-7 money→cents schema | C | 3 | ☐ |
| P2-I migration 068 / sqlx state | C | 3 | ☐ ⏸ |
| P0-1 checkout contract | C | 3 | ☐ |
| P0-4 webhook atomicity | C | 3 | ☐ |
| P2-A persisted idempotency | C | 3 | ☐ |
| P2-C webhook DB errors swallowed | C | 3 | ☐ |
| P0-5 plan price normalizer | C | 3 | ☐ |
| P1-8 expand=items | C | 3 | ☐ |
| P2-B refund authz | C | 3 | ☐ |
| P0-6 JSON-LD XSS | D | 4 | ☐ |
| P0-7 course paywall | E | 4 | ☐ 🐰 |
| P1-5 fabricated analytics | F | 5 | ☐ |
| P2-D unwrap_or_default swallow | F | 5 | ☐ |
| P3 log-level default | F | 5 | ☐ |
| P2-E webhook SSRF | G | 5 | ☐ |
| P3 consent OnceLock | I | 5 | ☐ |
| P2-G SW auth-HTML cache | J | 6 | ☐ |
| P2-J component collisions | J | 6 | ☐ |
| P3 dead module/components | J | 6 | ☐ |
| P3 mega-components | J | 6 | ☐ |
| P3 a11y suppressions | J | 6 | ☐ |
| P3 /signup vs /register | J | 6 | ☐ |
| P3 ~36MB cruft | J | 6 | ☐ |
| P3 stale comments / X-XSS hdr | J | 6 | ☐ |

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

**Exit:** secrets rotated; G0.1 and G0.2 answered (recorded in this
file).

---

## Stage 1 — Verification substrate (Cluster H) — *do first*

**Goal:** make every later stage provable in CI, not asserted. You
cannot principal-engineer a payments rewrite without a real test
harness.

- ☐ **P1-10** Delete the duplicated `mod stripe { … }` / `mod utils`
  copies in `api/tests/{stripe_test,utils_test}.rs`; point the tests at
  the real `api/src/` modules (expose via `pub` or a `testutil`
  feature). A regression in `services/stripe.rs` must now fail CI.
- ☐ **P1-10** Build a real-JWT admin integration harness in
  `api/tests/common/` (mint a signed token with the test
  `JWT_SECRET`, seed an admin + a member + a non-enrolled user). Delete
  the fake `"test_admin_token"`. This harness is the substrate for
  verifying Clusters B/C/E.
- ☐ **P1-9** Add **blocking** `eslint` and
  `cargo clippy -D warnings` jobs to the canonical
  `.github/workflows/ci.yml`; remove `continue-on-error` from clippy.
  Delete the dead `frontend/.github/workflows/ci.yml` (wrong location +
  npm-in-pnpm).
- ☐ Add a Postgres+Redis service to CI so the new integration tests
  run on every PR (gates Stages 2–5).

**Exit:** CI red on a real `services/stripe.rs` regression; admin
integration tests run with real JWTs in CI; lint+clippy block merges.

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
</content>
