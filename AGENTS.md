# Agent instructions — Revolution Trading Pros

## Svelte MCP (remote)

This repository is configured to use the **official remote Svelte MCP** at `https://mcp.svelte.dev/mcp`. That server covers **Svelte 5** and **SvelteKit 2** documentation and tooling in one place.

When working in Cursor, ensure **MCP is enabled** and the `svelte` server is loaded (see `.cursor/mcp.json` at the repo root, and `frontend/.cursor/mcp.json` if you open the `frontend` folder as the workspace root).

### Tools (from [Svelte MCP overview](https://mcp.svelte.dev/))

1. **list-sections** — Call first to discover documentation sections (titles, `use_cases`, paths).
2. **get-documentation** — Fetch full docs for the sections relevant to the task.
3. **svelte-autofixer** — Run on Svelte code before finalizing; iterate until clean.
4. **playground-link** — Only after user confirmation; not for code already written to the repo.

**Agents should use these tools** when answering Svelte/SvelteKit questions or editing `.svelte` / `+page` / `+layout` / `+server` files.

---

## Project conventions (Svelte 5 / SvelteKit 2)

- **Runes:** Prefer `$state`, `$derived`, `$effect`, `$props`, `$bindable` over legacy reactivity (`$:`), `export let`, and implicit `$store` subscriptions where practical.
- **Layout / pages:** Use `page` from `$app/state` (not legacy `$app/stores` `page`) unless a file is explicitly legacy.
- **Children:** Use `Snippet` + `{@render children?.()}` (or named snippets), not `<slot>` for new work.
- **Remote functions:** `svelte.config.js` enables `kit.experimental.remoteFunctions`; prefer typed `.remote.ts` modules for server/client boundaries where the feature is used.
- **SSR:** Avoid browser-only APIs in universal load or component init; gate with `import { browser } from '$app/environment'` or `onMount`.

---

## Monorepo

- **JavaScript:** Install from repository root with **pnpm** (`pnpm install`). Frontend package name: `revolution-svelte`.
- **Checks:** `pnpm --filter revolution-svelte run check` (Svelte + TS), `pnpm --filter revolution-svelte run lint`.

### Whole-project check (when MCP tools are not available)

From `frontend/`, `svelte-check` runs the same Svelte compiler diagnostics the **svelte-autofixer** MCP tool is built around. Use it in CI and locally:

```bash
pnpm --filter revolution-svelte run check
```

A clean run means **zero errors**; **warnings** (for example `state_referenced_locally` on UI primitives that mirror props into `$state`) may remain until components are refactored to `$bindable()` / `$derived` patterns or reviewed with **svelte-autofixer** in the IDE.
