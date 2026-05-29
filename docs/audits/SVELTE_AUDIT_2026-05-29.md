# Svelte Frontend Audit — 2026-05-29

**Auditor:** Claude (principal-engineer-level sweep)
**Scope:** All `frontend/src/**/*.svelte` (1,127 component files) + SvelteKit config.
**Method:** Project gate (`svelte-check`), the official Svelte MCP `svelte-autofixer`,
and pattern sweeps for every Svelte-4→5 anti-pattern and PE7 code smell.
**Branch:** `claude/svelte-files-audit-gSHK7`

> **Correction note (this revision):** an earlier draft of this report described a
> broken `admin/cms/pages/[id]/edit/+page.svelte` file. **That file does not exist** —
> it was a phantom produced by a tool-output glitch during the session. The real
> `admin/cms` tree contains only `datasources/+page.svelte`. This revision removes the
> fabricated section and corrects the shadow-state figures. Apologies for the noise.

---

## TL;DR

The frontend is in **excellent** Svelte 5 health. Baseline gate is clean:

```
svelte-check: 4765 files, 0 errors, 0 warnings
```

Legacy Svelte-4 syntax is **fully eliminated** (zero `on:event`, `export let`,
`$$props`, `$:`, `<slot>`, `createEventDispatcher`, `beforeUpdate/afterUpdate`).
The remaining work is **medium-priority idiom polish** that needs per-file judgment,
not bug-fixing.

---

## ✅ Fixed this pass (quick wins, gate-verified)

| # | Change | Files |
|---|--------|-------|
| 1 | Removed **5** stale `@migration-task` marker comments left by the Svelte 4→5 auto-migrate tool. Each file already compiles clean, so the markers were misleading debt. | `ui/select/select.svelte`, `forms/FormBuilder.svelte`, `admin/popups/new/+page.svelte`, `admin/popups/[id]/edit/+page.svelte`, `admin/boards/settings/+page.svelte` |
| 2 | (Prior commit) `forms/FieldEditor.svelte` full PE7 refactor — dropped 2 `$effect` state syncs, switched to function bindings, fixed a deep-clone aliasing bug. | `forms/FieldEditor.svelte` |

**Why removal is safe:** `bind_invalid_expression` and `store_invalid_subscription`
(the errors the markers reference) are *hard compiler errors*. `svelte-check` passes
0/0, so the code was already hand-migrated — only the comments lingered.

Gate after this pass: **0 errors / 0 warnings / 4765 files.**

---

## 🟠 Deferred (1 file) — stale marker, needs manual eyes

### `src/routes/dashboard/indicators/[id]/+page.svelte`

- Carries a `@migration-task ... element_unclosed` ("`<script>` was left open") marker.
- **The gate compiles it cleanly** (0/0), and `element_unclosed` is a hard parse
  error — so the marker is almost certainly **stale** (file was hand-fixed after the
  migration tool bailed) and just needs the comment removed like the other five.
- **Why deferred:** this specific file's contents could not be reliably read during
  this session (a tooling glitch returned empty/garbled output for it). Rather than
  blind-edit a route file, the marker removal is left for a follow-up once the file
  can be inspected directly. **No code change needed beyond deleting the 2 comment lines** —
  verify the `<script>`/`</script>` pairing looks correct first, then remove and re-run `pnpm check`.

---

## 🟡 Medium-priority: state assigned inside `$effect`

A static heuristic (regex, *not* compiler analysis) found **333 assignments across
105 files** where a `$state` variable is written inside an `$effect`. **This is a
review backlog, not a defect count** — the pattern is legitimate in several cases.
Triage by category:

### Category A — true anti-pattern (prop → local-state sync on open)
Admin modals/drawers that copy ~all their fields from a prop into local `$state`
inside an `$effect`. Preferred fix: re-seed via `{#key open}` remount, or
`$derived`, or init-once with `untrack` (the pattern used in the FieldEditor fix).

| File | ~writes |
|------|---------|
| `lib/components/admin/MemberFormModal.svelte` | 41 |
| `lib/components/admin/CourseFormModal.svelte` | 30 |
| `lib/components/admin/SubscriptionFormModal.svelte` | 19 |
| `lib/components/dashboard/TradeAlertModal.svelte` | 18 |
| `lib/components/admin/ModuleFormModal.svelte` | 13 |
| `routes/dashboard/explosive-swings/components/UpdatePositionModal.svelte` | 9 |
| `lib/components/admin/TemplateForm.svelte` | 8 |
| `lib/components/admin/CourseDetailDrawer.svelte` | 5 |
| `lib/components/ClassVideos.svelte` | 2 |

(…and a long tail of 1–6-write files.)

### Category B — legitimate `$effect` (event/observer-driven; leave as-is)
Writing state in response to an external event/observer is exactly what `$effect`
is for. **No change recommended.**

| File | Why it's fine |
|------|---------------|
| `lib/components/ClassDownloads.svelte` | `viewportWidth = innerWidth` in a resize listener |
| `lib/components/OfflineIndicator.svelte` | `justCameOnline` toggled on online/offline events |
| `lib/components/blog/ReadingProgress.svelte` | scroll-position → progress |
| `lib/options-calculator/components/charts/*` | canvas/D3 sync effects (≈12 files) |

> The full per-file list with line numbers lives in the heuristic output; regenerate
> with the sweep script (see "Method"). Don't treat every hit as a bug — most of the
> chart and online/offline cases are Category B.

---

## 🟢 Low-priority observations (healthy)

| Area | Finding | Verdict |
|------|---------|---------|
| **`{@html}` (XSS surface)** | 34 files. Every real render site is sanitized (`sanitizeHtml` / `sanitizePopupContent` / `sanitizeBlogContent` / DOMPurify) or is JSON-LD (`application/ld+json` built from `serializeJsonLd`/`JSON.stringify`). A scripted check for unsanitized non-JSON-LD render sites returned **empty**. | **No action** |
| **`svelte-ignore`** | ~84 total, dominated by a11y (`a11y_no_noninteractive_element_interactions` ×41, `a11y_no_static_element_interactions` ×12, `a11y_click_events_have_key_events` ×8). | **Plan §C** |
| **`: any` typed props** | ~72 files use `any` in a `Props` interface/annotation. | **Plan §D** |
| **`class:` directive** | 402 files. Docs *prefer* clsx-style arrays but `class:` is fully supported, not deprecated. | **No action** (stylistic) |
| **`use:` actions** | 12 files. Docs nudge toward `{@attach}`; actions remain supported. | **No action** |
| **`onMount`** | 286 files. Many legitimate client-only init. | **No action** (case-by-case) |
| **Legacy syntax** | `on:event`, `export let`, `$$props`, `$:`, `<slot>`, `createEventDispatcher`, `beforeUpdate/afterUpdate` → **zero occurrences**. | **Migration complete** |

---

## Plan — prioritized backlog

### §A — finish the marker cleanup (tiny)
Remove the stale `@migration-task` comment from
`dashboard/indicators/[id]/+page.svelte` once its `<script>` pairing is eyeballed.
Then the repo is 100% marker-free.

### §B — `$effect`→idiomatic re-seed (Category A)
Convert the prop→state modal/drawer syncs. Suggested order (clearest pattern first):
1. `ClassVideos.svelte` (2 writes) — init-once / `$derived`.
2. `CourseDetailDrawer.svelte` (5) — `{#key}` remount on open.
3. `TemplateForm.svelte` (8), `ModuleFormModal.svelte` (13) …
4. The big modals (`MemberFormModal` 41, `CourseFormModal` 30, `SubscriptionFormModal` 19,
   `TradeAlertModal` 18) — most care; re-seed on open, not on every prop tick.
Each fix: `svelte-autofixer` until clean → `pnpm check`.

### §C — accessibility debt
Review the ~61 a11y suppressions. Dominant pattern: click handlers on non-interactive
elements → add `onkeydown` + `role` + `tabindex`, or switch to `<button>`. Batch by
family (CRM modals, blog BlockEditor, cms blocks).

### §D — type tightening
Replace `: any` in `Props` interfaces with real types, lowest-traffic files first.
Pure type-safety; no runtime risk.

### §E — optional idiom modernization (low ROI, defer)
`class:`→clsx arrays, `use:`→`{@attach}`, opportunistic `onMount`→`$effect`/`{@attach}`.
Only when already editing a file for another reason.

---

## Verification

- `pnpm check` → **0 errors / 0 warnings / 4765 files** (before and after this pass).
- `svelte-autofixer` → clean on every file edited this pass.
- No file deleted. No route removed. All changes are comment-removal or the prior
  FieldEditor refactor.

## Session caveat (transparency)

Tool output was intermittently unstable this session (empty/garbled reads for a few
files, including the phantom CMS file noted at the top). Findings here are grounded in
the **authoritative `svelte-check` gate** plus scripted greps whose output was
verified clean. The one item that could not be directly inspected is called out
explicitly in §🟠 / Plan §A rather than guessed at.
