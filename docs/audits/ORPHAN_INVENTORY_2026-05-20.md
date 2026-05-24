# Orphans — files & their folder locations

Read-only inventory produced 2026-05-20. **No files were modified.** Each entry below is a CANDIDATE FOR RECONNECTION per the hard rule "CREATE, never DELETE — orphan = build the missing side."

For the full audit with evidence, reconnection map, and rationale, see `/tmp/maint-reports/ORPHAN_INVENTORY_2026-05-20.md` (also backed up at `~/Desktop/ORPHAN_INVENTORY_2026-05-20.md`).

Paths are relative to repo root: `/Users/billyribeiro/Desktop/my-websites/Revolution-trading-pros/`

---

## 1. Frontend `lib/api/` orphans (FE methods with no BE handler)

### 1a) Banner-flagged orphans (`FIX-2026-04-26: ORPHAN`)

| File | Folder |
|---|---|
| `abandoned-carts.ts` | `frontend/src/lib/api/` |
| `bannedEmails.ts` | `frontend/src/lib/api/` |
| `behavior.ts` | `frontend/src/lib/api/` |
| `bing-seo.ts` | `frontend/src/lib/api/` |
| `boards.ts` | `frontend/src/lib/api/` |
| `campaigns.ts` | `frontend/src/lib/api/` |
| `learning-content.ts` | `frontend/src/lib/api/` |
| `past-members.ts` | `frontend/src/lib/api/` |
| `past-members-dashboard.ts` | `frontend/src/lib/api/` |
| `popup-branding.ts` | `frontend/src/lib/api/` |
| `spx-profit-pulse.ts` | `frontend/src/lib/api/` |
| `timers.ts` | `frontend/src/lib/api/` |
| `trade-alerts.ts` | `frontend/src/lib/api/` |
| `trading-room-sso.ts` | `frontend/src/lib/api/` |
| `workflow.ts` | `frontend/src/lib/api/` |

### 1b) Zero-importer orphans (BE may exist; needs reconnection by import)

| File | Folder | Note |
|---|---|---|
| `articles.ts` | `frontend/src/lib/api/` | |
| `consent-settings.ts` | `frontend/src/lib/api/` | **BE already built** at `api/src/routes/consent.rs:218` — just needs import |
| `courses-enhanced.ts` | `frontend/src/lib/api/` | **BE already built** at `api/src/routes/courses_admin/` (mounted) — just needs import |
| `media.ts` | `frontend/src/lib/api/` | |
| `popups.ts` (one importer only) | `frontend/src/lib/api/` | Verify reach |
| `room-resources.ts` | `frontend/src/lib/api/` | |
| `video-advanced.ts` | `frontend/src/lib/api/` | |
| `boards/types.ts` (`CustomFieldValue`, `ActivityChange`) | `frontend/src/lib/boards/` | Type-only orphan |

---

## 2. Frontend proxy orphans (proxies with no backend route)

| File | Folder |
|---|---|
| `+server.ts` (popups/active) | `frontend/src/routes/api/popups/active/` |
| `+server.ts` (behavior/events) | `frontend/src/routes/api/behavior/events/` |
| `+server.ts` (dashboard/spx-profit-pulse) | `frontend/src/routes/api/dashboard/spx-profit-pulse/` |
| `+server.ts` (dashboard/spx-profit-pulse/alerts) | `frontend/src/routes/api/dashboard/spx-profit-pulse/alerts/` |
| `+server.ts` (dashboard/spx-profit-pulse/alerts/[slug]) | `frontend/src/routes/api/dashboard/spx-profit-pulse/alerts/[slug]/` |
| `+server.ts` (experiments/config) | `frontend/src/routes/api/experiments/config/` |
| `+server.ts` (prices) | `frontend/src/routes/api/prices/` |
| `+server.ts` (admin/invoice-settings) | `frontend/src/routes/api/admin/invoice-settings/` |
| `+server.ts` (admin/past-members-dashboard/[...path]) | `frontend/src/routes/api/admin/past-members-dashboard/[...path]/` |

---

## 3. Backend route module orphans (declared but not mounted)

| Module | File:Line | Note |
|---|---|---|
| `indicators_admin` | `api/src/routes/mod.rs:43` | Commented out — SQLx tuple decoding TODO (1,585 LOC) |
| `indicators` | `api/src/routes/mod.rs:14` | Commented out — legacy (superseded by `member_indicators`) |
| `settings` | `api/src/routes/mod.rs:17` | Commented out — overlap audit with `admin.rs` needed |
| `profile_router` | `api/src/routes/organization.rs:1217` | Defined but never mounted in `mod.rs` |
| `taxonomy_router` | `api/src/routes/admin_courses/mod.rs:163` | Defined but never mounted (R13-B aware) |
| `mod.rs:23` | `api/src/routes/` | Other commented `pub mod` lines (cross-check) |
| `mod.rs:10` | `api/src/services/` | Commented `pub mod` line |

---

## 4. Frontend component orphans (declared, zero importers in src/)

| File | Folder |
|---|---|
| `DynamicIcon.svelte` | `frontend/src/lib/components/` |
| `LazySection.svelte` | `frontend/src/lib/components/` |
| `ChartIcons.svelte` | `frontend/src/lib/components/` |
| `RateLimitIndicator.svelte` | `frontend/src/lib/components/` |
| `ConnectionHealthPanel.svelte` | `frontend/src/lib/components/` |
| `OfflineIndicator.svelte` | `frontend/src/lib/components/` |
| `Modal.svelte` (duplicate of ui/Modal) | `frontend/src/lib/components/` |
| `BatchOperations.svelte` (duplicate of ui/BatchOperations) | `frontend/src/lib/components/` |
| `DashboardWidgetManager.svelte` (duplicate of ui/DashboardWidgetManager) | `frontend/src/lib/components/` |
| `BatchOperations.svelte` (duplicate) | `frontend/src/lib/components/ui/` |
| `DashboardWidgetManager.svelte` (duplicate) | `frontend/src/lib/components/ui/` |
| `select.svelte` (duplicate of ui/select/select) | `frontend/src/lib/components/ui/select/` |

### CRM stub pages (22 — intentional placeholders per `06-crm-DEFERRED.md`)

All in `frontend/src/routes/admin/crm/*/new/` and `frontend/src/routes/admin/crm/*/[id]/edit/` — flagged with banner `Stub page — audit 06-crm.md P0 #1`. Not bugs; tracked for follow-up.

---

## 5. Type-shim orphans (`*.d.ts`)

| File | Folder | Note |
|---|---|---|
| `d3.d.ts` | `frontend/src/lib/types/` | Was a shadow shim overriding `@types/d3` — R24-A neutralized body to `export {}`. Kept on disk per CREATE-not-DELETE. |
| `diff-match-patch.d.ts` | `frontend/src/lib/types/` | Verify importers |
| `dom.d.ts` | `frontend/src/lib/types/` | Verify against `app.d.ts` overlap |
| `lottie-web.d.ts` | `frontend/src/lib/types/` | Zero importers via grep |
| `svelte-app.d.ts` | `frontend/src/lib/types/` | Likely legacy — shadowed by root `src/types.d.ts` |
| `workflow.ts` | `frontend/src/lib/types/` | Cross-check vs `workflow.ts` api orphan |
| `types.ts` | `frontend/src/routes/dashboard/explosive-swings/` | Lines 112,114,130 unused exports |

---

## 6. Migration markers (`@migration-task` — Svelte 4→5 conversion stragglers)

Files where the migrator gave up; need manual conversion:

| File | Folder |
|---|---|
| `FormBuilder.svelte` | (find via `grep -rln "@migration-task" frontend/src/lib/components/forms/`) |
| `FieldEditor.svelte` | (same dir as FormBuilder) |
| `select.svelte` | `frontend/src/lib/components/ui/select/` |
| `+page.svelte` (popups/new) | `frontend/src/routes/admin/popups/new/` |
| `+page.svelte` (popups/[id]/edit) | `frontend/src/routes/admin/popups/[id]/edit/` |
| `+page.svelte` (boards/[id]/settings) | `frontend/src/routes/admin/boards/[id]/settings/` |
| `+page.svelte` (indicators/[id]) | `frontend/src/routes/admin/indicators/[id]/` |

Full list: `grep -rln "@migration-task" frontend/src/`

---

## 7. Rust function orphans (`pub fn` with no callers)

Sampled — full audit requires `cargo +nightly rustc -- -W dead_code`.

| Function | File |
|---|---|
| `profile_router` | `api/src/routes/organization.rs:1217` |
| `taxonomy_router` | `api/src/routes/admin_courses/mod.rs:163` |

---

## Reconnection summary

- **15 FE clients need new BE routes** (category 1a)
- **2 FE clients have BE already built — just need import** (`consent-settings.ts`, `courses-enhanced.ts`)
- **5 BE module orphans with explicit re-enable steps** (category 3)
- **9 proxy orphans pointing to nonexistent backend routes** (category 2)
- **12+ duplicate-name components** in `lib/components/` vs `lib/components/ui/` — dedup candidates
- **22 CRM stub pages** intentional per `06-crm-DEFERRED.md`
- **7 `@migration-task` Svelte 4→5 stragglers**

For evidence (line numbers, importer counts, banner text), grep patterns used, and full reconnection map with backend routes to build, see `/tmp/maint-reports/ORPHAN_INVENTORY_2026-05-20.md` (also at `~/Desktop/ORPHAN_INVENTORY_2026-05-20.md`).
