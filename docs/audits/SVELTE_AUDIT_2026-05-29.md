# Svelte Frontend Audit — 2026-05-29

**Auditor:** Claude (principal-engineer-level sweep)
**Scope:** All `frontend/src/**/*.svelte` (1,127 component files) + the SvelteKit configuration.
**Method:** Project gate (`svelte-check`), the official Svelte MCP `svelte-autofixer`,
and pattern sweeps for every Svelte-4→5 anti-pattern and PE7 code smell.
**Branch:** `claude/svelte-files-audit-gSHK7`

---

## TL;DR

The frontend is in **excellent** Svelte 5 health. Baseline gate is clean:

```
svelte-check: 4765 files, 0 errors, 0 warnings
```

There is **one genuinely broken/incomplete file** (an unbuilt CMS feature scaffold —
*lost, not dead*) and a handful of **medium-priority idiom cleanups** that need
per-file judgment. The quick wins in this pass are already applied (see below).

---

## ✅ What was fixed this pass (quick wins, gate-verified)

| # | Change | Files | Risk |
|---|--------|-------|------|
| 1 | Removed stale `@migration-task` marker comments left by the Svelte 4→5 auto-migrate tool. Each file already compiles clean — the markers were misleading debt implying the files were unmigrated. | `ui/select/select.svelte`, `forms/FormBuilder.svelte`, `admin/popups/new/+page.svelte`, `admin/popups/[id]/edit/+page.svelte`, `admin/boards/settings/+page.svelte`, `dashboard/indicators/[id]/+page.svelte` | None — comment-only; gate re-run green |
| 2 | (Prior commit) `forms/FieldEditor.svelte` full PE7 refactor — dropped 2 `$effect`-driven state syncs, switched to function bindings, fixed a deep-clone aliasing bug. | `forms/FieldEditor.svelte` | Verified via autofixer + gate |

After these changes: **still 0 errors / 0 warnings across 4765 files.**

Each marker was verified safe to remove because `bind_invalid_expression` and
`store_invalid_subscription` (the errors the markers referenced) are **hard
compiler errors** — if the underlying code were still unmigrated, `svelte-check`
would fail. It passes, so the code was already hand-migrated and only the marker
comment remained.

---

## 🔴 Broken / "lost" file — needs a decision (DO NOT DELETE per owner)

### `src/routes/admin/cms/pages/[id]/edit/+page.svelte`

**Status:** Will not build if ever wired in. Scaffold for a CMS page editor whose
implementation was never written.

**Findings:**

1. **Missing dependencies (never committed to git history):**
   - `import PageEditor from '$lib/components/cms/PageEditor.svelte'` → **file does not exist**
   - `import { getCmsPage, type CmsPage } from '$lib/api/cms-pages'` → **module does not exist**
   - Confirmed via `git log --all` — these were *never* committed. The feature was
     scaffolded top-down but the editor component + API layer were never built.

2. **Legacy patterns (the `@migration-task` marker here is *genuine*, not stale):**
   - `import { page } from '$app/stores'` + `page.subscribe(...)` inside `onMount`,
     then immediate `unsubscribe()` — a Svelte-4 store dance. Svelte 5 / Kit 2
     idiom is `import { page } from '$app/state'` and read `page.params.id` directly.
   - `<svelte:component this={undefined} />` at the end of the template — deprecated
     in Svelte 5 runes mode **and** dead (renders nothing). Pure cruft.

3. **Reachability:** Nothing links to `/admin/cms/pages/[id]/edit`. The entire
   `admin/cms` route tree contains only this stub + an unrelated `datasources` page.

**Why `svelte-check` currently reports it clean:** the route is not present in the
generated `.svelte-kit/tsconfig.json` include set, so its missing-import errors are
not surfaced by the gate. This is a blind spot — the file is effectively invisible
to CI until someone links to it.

**Recommendation (see Plan §A):** keep the file (owner's call), but track the
CMS-page-editor feature as explicit backlog. When picked up, build `cms-pages` API
+ `PageEditor` component and modernize this route's two legacy patterns.

---

## 🟡 Medium-priority: state written inside `$effect` (needs per-file judgment)

A heuristic sweep found **~14 files** that assign to a `$state` variable inside an
`$effect`. **This is not automatically a bug** — it must be triaged by category:

### Category A — true anti-pattern (prop → local-state sync)
These mirror the FieldEditor issue: local `$state` is rebuilt from a prop inside an
`$effect`. Preferred fixes: `$derived`, a `{#key}`-block remount, or initialise-once
with `untrack` (as done in FieldEditor).

| File | ~writes | Notes |
|------|---------|-------|
| `lib/components/admin/MemberFormModal.svelte` | 9 | Syncs ~9 form fields from `member` prop when the modal opens. Classic case. Fix needs care around the open/close lifecycle (re-seed on open, not on every prop tick). |
| `lib/components/admin/CourseDetailDrawer.svelte` | 6 | Resets drawer state (`courseData`, `activeTab`, `error`, …) when opened. Candidate for `{#key drawerOpenId}` remount. |
| `lib/components/ClassVideos.svelte` | 2 | `courseData = initialData; loading = false` — should be `$derived`/init-once. |

### Category B — legitimate `$effect` (event/observer-driven; leave as-is)
These write state in response to **external events**, which is exactly what `$effect`
is for. No change recommended.

| File | Why it's fine |
|------|---------------|
| `lib/components/ClassDownloads.svelte` | `viewportWidth = window.innerWidth` inside a resize listener. |
| `lib/components/OfflineIndicator.svelte` | `justCameOnline` toggled on `online`/`offline` events. |

### Category C — single-write files, need individual review
~9 files with one state-in-effect write each, e.g.:
`routes/dashboard/explosive-swings/components/PnLChart.svelte`,
`routes/admin/blog/edit/[id]/_components/SaveBar.svelte`,
`routes/admin/email/campaigns/new/_components/StepReview.svelte`,
`routes/admin/email/campaigns/[id]/report/_components/EmailPerformanceChart.svelte`,
and others. Most are chart/canvas sync effects (likely Category B) but each should
be eyeballed.

> Note: this is a static heuristic (regex), not a compiler analysis. Treat the list
> as leads to review, not a defect count.

---

## 🟢 Low-priority observations (healthy, tracked for completeness)

| Area | Finding | Verdict |
|------|---------|---------|
| **`{@html}` (XSS surface)** | 34 component files. All real render sites are routed through `sanitizeHtml`/`sanitizePopupContent`/`sanitizeBlogContent`/DOMPurify, or are JSON-LD (`application/ld+json`) built from `JSON.stringify`'d schema. The 2 "unsanitized"-looking hits were code comments, not render tags. | **No action** — surface is sanitized. |
| **`svelte-ignore` suppressions** | ~84 total, dominated by a11y (`a11y_no_noninteractive_element_interactions` ×41, `a11y_no_static_element_interactions` ×12, `a11y_click_events_have_key_events` ×8). | **Plan §C** — a11y debt review; mostly interactive-div patterns that want keyboard handlers or role changes. |
| **`state_referenced_locally`** | 3 suppressions. One (`state_referenced_locally_INVALID`) is a **typo'd, non-existent ignore code** living in the broken CMS file. | Folded into Plan §A. |
| **`: any` typed props** | ~72 files use `any` in a `Props` interface or annotation. | **Plan §D** — incremental typing; low urgency, no runtime effect. |
| **`class:` directive** | 402 files. PE7/Svelte docs *prefer* clsx-style class arrays, but `class:` is fully supported and not deprecated. | **No action** — stylistic only; not worth churn. |
| **`use:` actions** | 12 files. Docs nudge toward `{@attach}`, but actions remain supported. | **No action** — not worth churn. |
| **`onMount`** | 286 files. Many are legitimate (client-only init); some could be `$effect`/`{@attach}`. | **No action** — case-by-case, low value. |
| **Legacy syntax** (`on:event`, `export let`, `$$props`, `$:`, `<slot>`, `createEventDispatcher`, `beforeUpdate`/`afterUpdate`) | **Zero occurrences** outside the broken CMS file. | **Migration is essentially complete.** |

---

## Plan — prioritized backlog

### §A — CMS page editor feature (the "lost" file) — *owner-gated*
1. Decide product intent: is `/admin/cms/pages/[id]/edit` a real planned feature?
2. If **yes**: build `src/lib/api/cms-pages.ts` (`CmsPage` type, `getCmsPage`,
   `saveCmsPage`) and `src/lib/components/cms/PageEditor.svelte` (Svelte 5, wired to
   the existing `cms/blocks/*` + `PageRenderer`). Then modernize the route:
   `$app/stores`→`$app/state`, drop the `<svelte:component this={undefined}/>`,
   remove the (then-resolved) `@migration-task` marker.
3. If **deferred**: leave as-is (per owner) but add a `// TODO(cms-editor):` header
   so the next reader knows it's an intentional unfinished scaffold, and add the
   route to a CI "known-incomplete" allowlist so the missing-import blind spot is
   documented.

### §B — `$effect`→`$derived` cleanups (Category A files)
Tackle in this order (highest payoff / clearest pattern first):
1. `ClassVideos.svelte` (smallest, 2 writes) — init-once or `$derived`.
2. `CourseDetailDrawer.svelte` — `{#key}` remount on open.
3. `MemberFormModal.svelte` — re-seed form on open; most care needed.
Each fix: run `svelte-autofixer` until clean, then `pnpm check`.

### §C — Accessibility debt
Review the ~61 a11y suppressions. The dominant pattern is click-handlers on
non-interactive elements; remediation is usually adding `onkeydown` + `role` +
`tabindex` or switching to a `<button>`. Batch by component family (CRM modals,
blog BlockEditor, cms blocks).

### §D — Type tightening
Replace `: any` in `Props` interfaces with real types, file by file, lowest-traffic
first. Pure type-safety; no runtime risk.

### §E — Optional idiom modernization (low ROI, defer)
`class:`→clsx arrays, `use:`→`{@attach}`, `onMount`→`{@render}`/`$effect` where
genuinely better. Only touch when already editing a file for another reason —
not worth a dedicated churn pass.

---

## Verification

- `pnpm check` → **0 errors / 0 warnings / 4765 files** (before and after this pass).
- `svelte-autofixer` → clean on every file edited this pass.
- No file deleted. No route removed. All changes are additive or comment-removal.
