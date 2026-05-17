# Oversized Svelte Components — Refactor Backlog

**Created:** 2026-05-16 21:xx EDT · **Why this exists:** Svelte's value is small, focused components. Files below are too large to run through the `svelte-autofixer` MCP tool in one pass, and too large to reason about safely. They were **edited minimally** during the pnpm/Sentry/deps work and **verified via the type-aware `pnpm check` gate (0 errors / 0 warnings)** instead of the autofixer. They are parked here to be split into smaller components in a dedicated follow-up.

## Rule going forward

When a `.svelte` file is small enough, the `svelte-autofixer` MCP tool is run on it before commit (done for `select-item.svelte`, `TickerSearch.svelte`, etc. — all `issues: []`). When a file is too large for that, the change is gated by `pnpm check` instead, and the file is logged here for later decomposition.

## Files to decompose (priority by size / blast radius)

| File | Lines | Touched this session | Why it's a problem | Suggested split |
|---|---|---|---|---|
| `frontend/src/routes/admin/settings/+page.svelte` | ~2098 (was 2120) | Removed the Sentry service-definition object (pure data-array deletion) | A 2k-line route page mixing a huge service-config data array + UI + handlers. Unreviewable; can't autofix. | Extract the service-config array to `$lib/admin/service-registry.ts`; split the page into `<ServiceList>`, `<ServiceCard>`, `<ServiceFieldForm>` components. |
| `frontend/src/lib/components/admin/ServiceConnectionStatus.svelte` | ~868 | One-token change `primaryService: 'sentry'` → `'datadog'` (type-safe) | 868-line "premium" component with 4 display variants inlined + a big `FEATURE_CONFIG` map. | Pull `FEATURE_CONFIG` into a module; split the 4 variants (`card`/`inline`/`badge`/`banner`) into separate components composed by a thin wrapper. |

## Verification status of this session's edits to the above

- `admin/settings/+page.svelte`: change = deletion of one literal object from a data array (no logic/markup/reactivity). **Gate: `pnpm check` 0/0.**
- `ServiceConnectionStatus.svelte`: change = one string literal, `'datadog'` is a valid `ServiceKey` (verified the key still exists in `connections.svelte.ts`). **Gate: `pnpm check` 0/0.**

Neither edit altered component structure, so the autofixer's structural analysis was not the right tool; the type-aware compile gate is the authoritative evidence and it passed.

## Add to this list

Any future `.svelte` file > ~400 lines that gets touched: log it here with line count + the change made + the gate used, instead of silently skipping the autofixer.
