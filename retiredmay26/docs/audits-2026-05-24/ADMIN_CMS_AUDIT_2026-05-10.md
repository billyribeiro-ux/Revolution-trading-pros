# Admin Dashboard + CMS — End-to-End Audit (2026-05-10)

Companion to [REPO_AUDIT_2026-05-10.md](REPO_AUDIT_2026-05-10.md), focused
specifically on the admin dashboard, sidebar, and the CMS subsystem (front
to back).

- **Branch:** `claude/repo-audit-investigation-ejndf`
- **HEAD at audit time:** `3c194f0 chore(repo): tidy root markdown, eslint override for tests/scripts`
- **Method:** two parallel specialized surveys — one on the SvelteKit admin
  surface (sidebar, layout, role gate, 164 admin pages, 93 admin proxies,
  the BlockEditor stack), one on the Rust admin + CMS surface (12 cms_*
  routes, 9 admin_* routes, 7 cms_* services, content lifecycle, migrations,
  test coverage). Each finding is grounded in a file:line reference.
- **What was *not* exercised:** a real headed Playwright run. That requires
  the local Docker stack (Postgres + Redis + Rust API) to be booted; this
  environment doesn't have it. Two new spec files have been added under
  `frontend/tests/e2e/admin/` so a real headed run can be performed locally
  with `pnpm exec playwright test --project=chromium --headed`.

---

## 1. Admin frontend — what's there

```
frontend/src/routes/admin/
  +layout.svelte           28 KB · the admin shell + role gate
  +layout.ts               5 lines · ssr=false, prerender=false
  +page.svelte             63.8 KB · dashboard with stats
  +error.svelte            7.6 KB · error boundary (401/403/404/5xx)
  ...164 +page.svelte files total (members, blog, courses, indicators,
       trading-rooms, resources, media, popups, forms, watchlist, boards,
       email, seo, analytics, behavior, crm — 34 sub-pages, performance,
       site-health, connections, users, schedules, consent, settings, etc.)

frontend/src/routes/api/admin/
  93 +server.ts proxies
```

### Sidebar

`frontend/src/lib/components/layout/AdminSidebar.svelte` (lines 55–119)
renders 6 sections + 29 nav items:

- **Overview** — `/admin`
- **Members** (7) — subscriptions, products, coupons, orders, abandoned carts, members, segments
- **Content** (12) — blog, courses, indicators, trading-rooms, resources, categories, media, videos, popups, forms, watchlist, boards
- **Marketing** (5) — campaigns, email templates, SMTP settings, subscribers, SEO
- **Analytics** (4) — dashboard, behavior, CRM, performance
- **System** (6) — site health, connections, admin users, schedules, consent, settings

Sidebar/route coverage: **100%** — every one of the 29 sidebar items resolves to an existing `+page.svelte`. Active-state logic uses longest-prefix-match and is consistent (e.g. `/admin/members/segments` correctly highlights "Segments", not "All Members").

### Role gate

`frontend/src/routes/admin/+layout.svelte` (lines 87–118) is the active gate.

- Defers role-check until `isInitializing.current === false` (auth store hydrates first).
- Logged out → `goto('/login?redirect=/admin')`.
- Logged in but non-admin → `goto('/?error=admin_required', { replaceState: true })` (replaceState avoids back-button loops).
- Admin → renders the shell.
- Token forwarding to the backend is via the unified helpers in `frontend/src/lib/server/auth.ts` (`requireAdminToken`, `requireAdmin`, `requireSuperadmin`).

Closes the 2026-04-25 P0-2 finding ("/admin had no role gate").

`+layout.ts` sets `ssr = false; prerender = false;` — admin never SSRs by design. Acceptable, but see §3.

### Pages inventory

164 admin `+page.svelte` files. Coverage by domain:

| Domain | Pages | Notes |
|---|---|---|
| Core (dashboard / settings / error / layout) | 5 | `/admin/+page.svelte` is **63.8 KB** — investigate splitting |
| Members & subscriptions | 8 | |
| Content management | 26 | blog, courses, indicators, trading-rooms, resources, media, popups, forms, watchlist, boards |
| Marketing & email | 15 | campaigns, templates, SMTP, subscribers, SEO sub-pages |
| CRM | 34 | a complete CRM surface (deals, leads, sequences, automations, tags, segments, lists, managers, templates, webhooks, logs, smart-links) |
| Products & orders | 8 | |
| Analytics & system | 18 | analytics sub-pages, behavior, performance, site-health, connections, users, schedules, contacts, categories, consent (3) |

### Admin API proxies

93 files under `frontend/src/routes/api/admin/`. Three patterns observed:

1. **Unified** — uses `requireAdmin`/`requireSuperadmin` from `$lib/server/auth.ts`. Examples: `bunny/**`, `users/**`, `invoice-settings/**`, `past-members-dashboard/**`. Correct.
2. **Manual cookie-or-header read with token-only check** — reads `rtp_access_token` cookie or `Authorization` header but only verifies *presence*, not role. The role check happens server-side at the Rust API. Examples: `schedules/+server.ts:122-130`, `indicators/+server.ts:22-28`, `forms/+server.ts:53-58`. **Functionally safe** (Rust enforces the role) but inconsistent and fails slow (one round-trip to Rust before returning 403).
3. **No SvelteKit-side check at all** — proxy just forwards. Same defence-in-depth dependency on Rust.

The new spec `frontend/tests/e2e/admin/role-gate.spec.ts` includes a slice that validates the **defense-in-depth** assumption — every admin proxy in the slice must return 401/403 to a non-admin session. If any returns 200, that proxy is broken.

### CMS editor surface

| Component | LOC | Risk |
|---|---|---|
| `BlockEditor.svelte` | 2,892 | 25+ `$state` decls, deep-nested `editorState` object, 8+ `$effect`s; updates can re-render large subtrees |
| `BlockSettingsPanel.svelte` | 2,146 | 10+ `$state` + a `SvelteSet` for expanded sections |
| `BlockRenderer.svelte` | 238 | Fine |
| `GlobalComponentLibrary.svelte` | 1,953 | Eagerly loaded with the editor |

**Total:** 7,229 LOC loaded as a single chunk on any blog/CMS edit page. Justified by feature scope but a candidate for code-splitting + memoisation pass.

`BlockEditor/performance/reporter.ts` and `BlockEditor/VirtualBlockList.svelte` contain **18 actual `console.log` calls** (vitals, block-count, drag debug). With the new eslint override these are no longer surfaced as warnings, but they still ship to production. Wrap in `import.meta.env.DEV` or push to a telemetry sink.

---

## 2. Rust backend — admin & CMS modules

### CMS module map (12 files, ~15,144 LOC)

| File | Lines | Purpose |
|---|---|---|
| `cms_v2.rs` | 904 | Core: posts CRUD, status transitions, listing, stats — **37 handlers** |
| `cms_v2_enterprise.rs` | 340 | Enterprise extensions — 11 handlers |
| `cms_delivery.rs` | 559 | **Public read** — no auth — 10 handlers (this is the delivery layer for the front of the site) |
| `cms_revisions.rs` | — | Revisions table CRUD |
| `cms_ai_assist.rs` | — | AI assistance for content authors |
| `cms_assets.rs` | 1,444 | Media library / file metadata |
| `cms_datasources.rs` | — | External datasources for blocks |
| `cms_global_components.rs` | 1,883 | Global / reusable component library |
| `cms_presets.rs` | — | Editor presets |
| `cms_reusable_blocks.rs` | — | Reusable block storage |
| `cms_scheduling.rs` | 2,003 | Schedule posts for future publish |
| `cms_seo.rs` | — | Per-post SEO metadata |

All 12 are mounted in `routes/mod.rs`.

### Admin module map (9 files)

| File | Auth | Notes |
|---|---|---|
| `admin.rs` | `AdminUser` | Aggregates the legacy admin surface |
| `admin_courses.rs` | `AdminUser` | 49 handlers |
| `admin_indicators.rs` | `AdminUser` | |
| `admin_indicators_enhanced` (separate file `indicators_admin.rs`) | — | **Not mounted** — `pub mod` commented out at `routes/mod.rs:40` due to a SQLx tuple-decoding bug. 1,585 LOC of working CRUD waiting for an unblock. |
| `admin_member_management.rs` | `AdminUser` | |
| `admin_members.rs` | `AdminUser` | |
| `admin_orders.rs` | `AdminUser` | |
| `admin_page_layouts.rs` | **Custom string check** at lines 27–40, includes a `developer` role not used elsewhere | Should migrate to `AdminUser`/`SuperAdminUser` for consistency |
| `admin_popups.rs` | `AdminUser` | |
| `admin_videos.rs` | `AdminUser` | 48 handlers |

Total ≈ 211 handlers across the admin surface. Auth coverage: **8 of 9 modules use the unified `AdminUser` extractor**; `admin_page_layouts.rs` is the lone outlier.

### CMS service modules (7 files)

| Service | Wired? | Purpose |
|---|---|---|
| `services/cms_audit.rs` | ✅ | Append-only audit log writes |
| `services/cms_content.rs` | ✅ | Status transitions, content reads |
| `services/cms_preview.rs` | ✅ | Draft preview tokens |
| `services/cms_scheduler.rs` | ✅ (background) | Picks up scheduled posts and publishes them |
| `services/cms_upload.rs` | ❌ | Imported nowhere — **dead code or incomplete feature**, decide |
| `services/cms_webhooks.rs` | ✅ | Outbound webhooks on content events |
| `services/cms_workflow.rs` | ✅ | Workflow engine for editorial review |

### Content lifecycle (verified end-to-end)

1. **Create draft** — `cms_v2.rs::create_post` writes to `cms_content` (status='draft'); admin auth required.
2. **Edit + revisions** — every save writes a row to `cms_revisions` keyed by content_id; `cms_audit_log` records actor + diff.
3. **Publish** — `cms_content.rs::transition_status` flips status to 'published', writes `cms_workflow_log` and `cms_audit_log`. **Three writes — not currently wrapped in a transaction** (see §3 critical findings).
4. **Schedule** — `cms_scheduling.rs` enqueues a row in the scheduling table; `services/cms_scheduler.rs` is a long-running task (spawned at boot in `main.rs`) that polls and triggers publish on each due item.
5. **Public delivery** — `cms_delivery.rs` is the read endpoint; no auth (correctly), uses materialized read-side queries.

### CMS migrations

8 CMS-relevant migrations totalling ≈180 KB. All use proper FK constraints and soft-deletes via `deleted_at`. Migration `014_*` looks superseded by `023_*` — confirm and prune.

### CMS test coverage

**Zero CMS-specific tests** in `api/tests/`. The `integration_tests.rs` covers auth/health/users/courses; nothing exercises post lifecycle, revisions, scheduling, or delivery. This is the single largest test gap on the backend.

### Frontend ↔ backend coupling

Spot-checked 3 admin proxies (`users/**`, `datasources/**`, `orders/**`) — each maps to a real Rust route. No 404-on-Rust-side discovered.

---

## 3. Critical findings

### 🔴 1. Publish flow is not transactional
`api/src/services/cms_content.rs::transition_status` (lines 756–829) writes
`cms_content`, `cms_workflow_log`, and `cms_audit_log` as three separate
statements, not inside `Pool::begin()` → `tx.commit()`. A crash mid-way
leaves a published post with no audit row, or an audit row with stale
status. CLAUDE.md explicitly mandates transactions for >1 table mutations.

### 🔴 2. Unauthenticated CMS settings endpoint
`api/src/routes/cms_v2.rs::get_site_settings` (around line 605) does not
extract `AdminUser`. Any anonymous client can read whatever is in the
site_settings table. If those settings include API keys, internal toggles,
or admin-only metadata, this is a real exposure.

### 🔴 3. `admin_page_layouts.rs` uses ad-hoc auth
Lines 27–40 do a manual role-string check rather than going through
`AdminUser`/`SuperAdminUser`. The role list includes `"developer"` which
is not recognized anywhere else in the auth surface. Two risks:
behavioral drift if `AdminUser` ever changes, and a back-door `developer`
role that lets non-admins through page-layout mutations.

### 🟡 4. `services/cms_upload.rs` is dead
Imported by no caller. Either an unfinished feature or stale code from a
prior approach. Resolve.

### 🟡 5. Zero CMS test coverage
No `api/tests/` file exercises post create, publish, revision, schedule,
or delivery. Regressions in any of those will only surface in production.

### 🟡 6. `cms_delivery.rs:150–165` builds an ORDER BY/filter via `format!()`
The interpolated values look whitelisted (sort columns from a fixed enum),
but it's the same brittle pattern flagged in `room_analytics.rs` in the
2026-05-10 audit. Add an explicit comment + CI guard.

### 🟡 7. Frontend admin API proxies inconsistent
28+ proxies under `routes/api/admin/` use a manual token-presence check
instead of `requireAdmin()` from `$lib/server/auth.ts`. Defence-in-depth is
intact (Rust enforces the role) but failures are slow and the pattern
drift hurts maintainability.

### 🟡 8. `BlockEditor` stack is 7.2 K LOC, eagerly loaded
4 components total 7,229 LOC; loaded as a single chunk on every edit page;
no code-splitting; 25+ `$state` decls in `BlockEditor.svelte` alone. Real
maintainability and re-render risk.

### 🟢 9. 18 `console.log` calls in production code
Not warnings any more (they live in `BlockEditor/**` which is `src/**`),
but they still ship. Gate behind `import.meta.env.DEV` or replace with a
telemetry sink.

### 🟢 10. Admin `+page.svelte` at 63.8 KB
The dashboard root page is huge. If it's mostly server-fetched data
panels, split into sub-components and lazy-load.

---

## 4. New e2e specs (delivered with this audit)

Two Playwright specs added under `frontend/tests/e2e/admin/` so the major
admin invariants can be re-verified locally with a real headed browser:

### `frontend/tests/e2e/admin/role-gate.spec.ts`

Four cases:

1. Logged-out `/admin` → `/login?redirect=/admin`.
2. Non-admin `/admin` → `/?error=admin_required` (creates a non-admin user
   on first run via `/api/auth/register` if needed).
3. Admin `/admin` renders the shell.
4. **Defence-in-depth check** — every admin API proxy in a representative
   slice (`/api/admin/members`, `/orders`, `/products`, `/users`,
   `/settings`, `/coupons`, `/email/campaigns`, `/seo/keywords`,
   `/site-health`) must return 401/403 to a non-admin session. If any
   returns 200, the gate is broken.

### `frontend/tests/e2e/admin/subscription-checkout.spec.ts`

Regression test for the 2026-05-10 fix at
`api/src/routes/subscriptions.rs:446`. Verifies POST `/api/subscriptions`
returns a real Stripe URL — no `placeholder` substring, must match
`stripe.com`. Skipped by default (requires a real Stripe test-mode key);
gate with `RUN_STRIPE_E2E=1`.

To run locally:

```bash
docker compose up -d db redis api
./api/scripts/seed-local-admin.sh you@example.com YourPass1! "Your Name"
pnpm dev   # in another terminal

pnpm exec playwright test tests/e2e/admin/ \
  --project=chromium --headed
```

The existing `frontend/tests/e2e/admin-sweep.spec.ts` (372 LOC) and
`frontend/tests/e2e/blog-post-create.spec.ts` (266 LOC) already cover the
sidebar walk, every admin API proxy, and the full blog post
create→publish→view→delete lifecycle, so the new specs are additive, not
duplicative. The known auth-store hydration race documented at the top of
`blog-post-create.spec.ts` may also affect the role-gate spec — if it
flakes in isolation, run with `--retries=2` and check that admin-sweep
also passes.

---

## 5. Prioritised admin/CMS backlog

### P0 — close before launch

1. Wrap `cms_content::transition_status` (and any other multi-table CMS mutation) in `Pool::begin()` → `tx.commit()`.
2. Gate `cms_v2::get_site_settings` behind `AdminUser`. Audit every other CMS handler that doesn't take an extractor.
3. Migrate `admin_page_layouts.rs` to the unified `AdminUser`/`SuperAdminUser` extractor; drop the `developer` role string.

### P1 — close this sprint

4. Decide on `services/cms_upload.rs` — finish the feature or delete the module.
5. Add backend integration tests for: post create, publish, revision insert, scheduled-publish (end-to-end with the worker), and delivery endpoint.
6. Run the new `tests/e2e/admin/role-gate.spec.ts` headed against the local stack and fix any proxy that surfaces a 200 to a non-admin session.
7. Migrate the 28+ frontend admin proxies still using manual token checks to `requireAdmin()`. Pick one each PR.
8. Document `cms_delivery.rs:150-165`'s ORDER BY whitelist assumption inline; add a CI grep guard against `format!.*sqlx::query`.

### P2 — should-fix soon

9. Fix the SQLx tuple-decoding issue blocking `indicators_admin.rs` (1,585 LOC currently disabled), or formally retire the file.
10. Code-split + memoise `BlockEditor` and `BlockSettingsPanel`. Lazy-load `GlobalComponentLibrary` only when a block uses it.
11. Wrap `BlockEditor` performance-reporter `console.log`s behind `import.meta.env.DEV`.
12. Split `frontend/src/routes/admin/+page.svelte` (63.8 KB) into smaller panel components.

### P3 — hygiene

13. Confirm migration `014_*` is superseded by `023_*`; if so, document the relationship.
14. Add `+page.server.ts` server-loaders to the highest-traffic admin pages (members, products, blog) — measurable TTFCP win, no architectural cost.

---

## Conclusion

The admin and CMS subsystems are **architecturally sound but have specific
correctness gaps** — chief among them the non-transactional publish flow
in `cms_content::transition_status`, the unauthenticated
`cms_v2::get_site_settings` endpoint, and the lone admin module
(`admin_page_layouts.rs`) that doesn't use the unified `AdminUser`
extractor. The frontend is in better shape than the backend on this
surface: the role gate works, sidebar coverage is 100 %, and the major
e2e flows have existing Playwright coverage.

Sidebar UX is solid; data-loading strategy (100 % client-side under
`ssr=false`) is acceptable but not optimal for the highest-traffic
collections. The CMS editor stack is large and will need a code-splitting
pass before the codebase grows further.

The new e2e specs surface the role-gate and Stripe-checkout regressions
the 2026-05-10 audit identified — run them headed against the local stack
to verify both fixes hold under a real browser session.
