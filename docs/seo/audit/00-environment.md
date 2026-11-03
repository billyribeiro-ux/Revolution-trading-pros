# Phase 0 — Preflight & Environment Verification

**Audit Date:** 2026-11-01
**Branch:** `seo/audit-2026-11-01`

---

## 1. Standard & Svelte MCP Verification

- **SEO Standard:** `docs/seo/SEO_STANDARD.md` exists and was read in full (`[CODE docs/seo/SEO_STANDARD.md:1-209]`).
- **Svelte MCP (`TOOL-01`):** Official `@sveltejs/mcp` server (`svelte`) is connected and verified via `list-sections` (returned 205 sections across `ai/*`, `cli/*`, `kit/*`, `svelte/*`) (`[DOC svelte:list-sections]`).

---

## 2. Resolved Package Versions (Rule 3: Installed Version Wins)

All versions below are verified against `package.json`, `frontend/package.json`, `pnpm-lock.yaml`, and `frontend/node_modules/*/package.json`:

| Package / Runtime                  | Specifier (`frontend/package.json`)             | Resolved Version (`pnpm-lock.yaml` & `node_modules`) | Evidence Tag                                                                                            |
| :--------------------------------- | :---------------------------------------------- | :--------------------------------------------------- | :------------------------------------------------------------------------------------------------------ |
| **Node.js**                        | `>=26.13.0` (`[CODE frontend/package.json:9]`)  | `v26.13.0`                                           | `[CMD node -v -> exit 0: v26.13.0]`                                                                     |
| **pnpm**                           | `>=12.16.1` (`[CODE package.json:25]`)          | `12.16.1`                                            | `[CMD pnpm -v -> exit 0: 12.16.1]`                                                                      |
| **`svelte`**                       | `^5.57.3` (`[CODE frontend/package.json:124]`)  | `5.57.3`                                             | `[CODE pnpm-lock.yaml:406]`, `[CODE frontend/node_modules/svelte/package.json:3]`                       |
| **`@sveltejs/kit`**                | `^3.1.0` (`[CODE frontend/package.json:84]`)    | `3.1.0`                                              | `[CODE pnpm-lock.yaml:286]`, `[CODE frontend/node_modules/@sveltejs/kit/package.json:3]`                |
| **`@sveltejs/adapter-cloudflare`** | `^8.0.1` (`[CODE frontend/package.json:83]`)    | `8.0.1`                                              | `[CODE pnpm-lock.yaml:283]`, `[CODE frontend/node_modules/@sveltejs/adapter-cloudflare/package.json:3]` |
| **`@sveltejs/vite-plugin-svelte`** | `^7.3.1` (`[CODE frontend/package.json:85]`)    | `7.3.1`                                              | `[CODE pnpm-lock.yaml:289]`, `[CODE frontend/node_modules/@sveltejs/vite-plugin-svelte/package.json:3]` |
| **`vite`**                         | `^8.3.5` (`[CODE frontend/package.json:131]`)   | `8.3.5`                                              | `[CODE pnpm-lock.yaml:427]`, `[CODE frontend/node_modules/vite/package.json:3]`                         |
| **`typescript`**                   | `^6.0.3` (`[CODE frontend/package.json:129]`)   | `6.0.3`                                              | `[CODE pnpm-lock.yaml:421]`, `[CODE frontend/node_modules/typescript/package.json:3]`                   |
| **`svelte-meta-tags`**             | `5.0.3` (`[CODE frontend/package.json:69]`)     | `5.0.3`                                              | `[CODE pnpm-lock.yaml:246]`, `[CODE frontend/node_modules/svelte-meta-tags/package.json:3]`             |
| **`super-sitemap`**                | `^2.0.6` (`[CODE frontend/package.json:123]`)   | `2.0.6`                                              | `[CODE pnpm-lock.yaml:403]`, `[CODE frontend/node_modules/super-sitemap/package.json:3]`                |
| **`schema-dts`**                   | `2.1.0` (`[CODE frontend/package.json:68]`)     | `2.1.0`                                              | `[CODE pnpm-lock.yaml:243]`, `[CODE frontend/node_modules/schema-dts/package.json:3]`                   |
| **`@sveltejs/enhanced-img`**       | Not installed                                   | Not installed                                        | `[CODE frontend/package.json:45-136]`                                                                   |
| **`@inlang/paraglide-js`**         | Not installed                                   | Not installed                                        | `[CODE frontend/package.json:45-136]`                                                                   |
| **`@playwright/test`**             | `^1.64.1` (`[CODE frontend/package.json:81]`)   | `1.64.1`                                             | `[CODE pnpm-lock.yaml:277]`                                                                             |
| **`@axe-core/playwright`**         | `^4.14.0` (`[CODE frontend/package.json:77]`)   | `4.14.0`                                             | `[CODE pnpm-lock.yaml:265]`                                                                             |
| **`vitest`**                       | `^5.0.5` (`[CODE frontend/package.json:134]`)   | `5.0.5`                                              | `[CODE pnpm-lock.yaml:436]`                                                                             |
| **`svelte-check`**                 | `^4.7.7` (`[CODE frontend/package.json:125]`)   | `4.7.7`                                              | `[CODE pnpm-lock.yaml:409]`                                                                             |
| **`eslint`**                       | `^10.13.0` (`[CODE frontend/package.json:114]`) | `10.13.0`                                            | `[CODE pnpm-lock.yaml:376]`                                                                             |

### SvelteKit 3.1.0 Installed Type Contracts (`node_modules/@sveltejs/kit/types/index.d.ts`)

1. **Configuration:** SvelteKit 3 configures options directly inside `sveltekit({ ... })` in `frontend/vite.config.ts` (`[CODE frontend/vite.config.ts:30-168]`, `[CODE frontend/node_modules/@sveltejs/kit/types/index.d.ts:1607-2209]`). There is no `svelte.config.js`.
2. **Subpath Imports:** `#lib/*` maps to `./src/lib/*` via `package.json#imports` (`[CODE frontend/package.json:137-143]`).
3. **Environment Variables:** Declared in `frontend/src/env.ts` using `defineEnvVars` from `@sveltejs/kit/env` (`[CODE frontend/src/env.ts:1-113]`) and imported via `$app/env`, `$app/env/public`, or `$app/env/private` (`[CODE frontend/node_modules/@sveltejs/kit/types/index.d.ts:1153-1234]`, `[CODE frontend/node_modules/@sveltejs/kit/types/index.d.ts:2387-2407]`).
4. **Page State:** `page`, `navigating`, `updated` are imported from `$app/state` (`[CODE frontend/node_modules/@sveltejs/kit/types/index.d.ts:3835-3890]`).
5. **Super Sitemap:** `super-sitemap@2.0.6` exports `./sveltekit` (`import * as sitemap from 'super-sitemap/sveltekit'`) (`[CODE frontend/node_modules/super-sitemap/package.json:35-40]`, `[CODE frontend/node_modules/super-sitemap/dist/adapters/sveltekit/index.d.ts:1-4]`).

---

## 3. Baseline Preflight Commands

| Command                                     | Exit Code | Warnings / Errors                                                                                                                                                                            | Output Excerpt                                                                                                                            |
| :------------------------------------------ | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `pnpm install --frozen-lockfile`            | `0`       | 0                                                                                                                                                                                            | `[CMD pnpm install --frozen-lockfile -> exit 0: "Lockfile is up to date, resolution step is skipped. Done in 218ms using pnpm v12.16.1"]` |
| `pnpm check`                                | `0`       | 0 errors, 0 warnings                                                                                                                                                                         | `[CMD pnpm check -> exit 0: "svelte-check found 0 errors and 0 warnings"]`                                                                |
| `pnpm lint`                                 | `0`       | 0 errors, 0 warnings                                                                                                                                                                         | `[CMD pnpm lint -> exit 0: "$ NODE_OPTIONS='--max-old-space-size=4096' eslint ."]`                                                        |
| `pnpm --filter revolution-svelte test:unit` | `0`       | 63 passed, 2778 tests passed (32 skipped)                                                                                                                                                    | `[CMD pnpm --filter revolution-svelte test:unit -> exit 0: "Test Files 63 passed (63), Tests 2778 passed                                  | 32 skipped (2810)"]` |
| `pnpm build`                                | `0`       | 0 build errors (note: prerender crawled several 404 internal links ignored by custom `handleHttpError`, and `handleMissingId: 'ignore'`, `handleUnseenRoutes: 'ignore'` in `vite.config.ts`) | `[CMD pnpm build -> exit 0: "✓ built in 68ms ... Using @sveltejs/adapter-cloudflare ... ✔ done"]`                                         |
