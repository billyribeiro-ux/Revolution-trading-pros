# Tailwind to Scoped CSS Baseline - 2026-06-05

Branch: `codex/tailwind-to-page-css-migration`

## Objective

Convert the frontend from Tailwind-dependent styling to plain CSS with Svelte-scoped
page/component styles. Keep global CSS limited to typography, minimal reset, and
accessibility primitives.

## Svelte MCP Evidence

Relevant Svelte documentation sections reviewed before implementation:

- `svelte/scoped-styles` - component and page styles are scoped by default.
- `svelte/global-styles` - global selectors must be explicit.
- `svelte/custom-properties` - CSS variables remain valid for shared values.
- `svelte/class` - Svelte 5 supports semantic class composition and directives.
- `cli/sv-check` - project-wide validation is required.
- `cli/tailwind` - documents the Tailwind integration being removed.

## Current Tailwind Surface

Direct Tailwind infrastructure:

- `frontend/src/app.css`
  - `@import 'tailwindcss'`
  - `@import 'tw-animate-css'`
  - `@custom-variant dark`
  - `@theme inline`
  - Tailwind `@layer` utility/base blocks
- `frontend/vite.config.ts`
  - imports and registers `@tailwindcss/vite`
- `frontend/package.json`
  - `@tailwindcss/vite`
  - `tailwindcss`
  - `tw-animate-css`
  - `tailwind-merge`
  - `tailwind-variants`
- `frontend/src/lib/utils.ts`
  - imports `twMerge` from `tailwind-merge`
- `frontend/src/lib/components/ui/badge/badge.types.ts`
  - imports `tv`/`VariantProps` from `tailwind-variants`
- `frontend/src/lib/components/ui/button/button.types.ts`
  - imports `tv`/`VariantProps` from `tailwind-variants`

## `@apply` / `@reference` Blockers

These files require Tailwind compilation today and must be converted first:

- `frontend/src/app.css`
- `frontend/src/routes/media/+page.svelte`
- `frontend/src/routes/analytics/+page.svelte`
- `frontend/src/routes/behavior/+page.svelte`
- `frontend/src/routes/email/+page.svelte`
- `frontend/src/lib/components/media/FolderTree.svelte`
- `frontend/src/lib/components/media/UploadDropzone.svelte`
- `frontend/src/lib/components/analytics/AIInsightsPanel.svelte`
- `frontend/src/lib/components/analytics/EventExplorer.svelte`
- `frontend/src/lib/components/analytics/RetentionCurve.svelte`
- `frontend/src/lib/components/analytics/RevenueBreakdown.svelte`
- `frontend/src/lib/components/analytics/UserJourneyMap.svelte`

## Quantified Migration Scope

Current static scan counts:

- Page files with Tailwind-like utility class markup: `120`
- Svelte files with Tailwind-like utility class markup: `247`
- `+page.svelte` files without a scoped `<style>` block: `49`

The utility-class scan is intentionally conservative and may include false positives
for semantic classes such as `active`; each file must be inspected before editing.

## Execution Rules

- Convert one batch at a time and keep commits focused.
- Prefer semantic classes over utility-like names.
- Page-specific layout/section styling belongs in the page `<style>`.
- Reusable component internals belong in the component `<style>`.
- Do not remove Tailwind packages or the Vite plugin until zero-reference scans pass.
- Run Svelte MCP `svelte_autofixer` on every touched `.svelte` file until it reports
  zero issues and zero suggestions.
