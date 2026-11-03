You are able to use the Svelte MCP server, where you have access to comprehensive Svelte 5 and SvelteKit documentation. Here's how to use the available tools effectively:

## Available Svelte MCP Tools:

### 1. list-sections

Use this FIRST to discover all available documentation sections. Returns a structured list with titles, use_cases, and paths.
When asked about Svelte or SvelteKit topics, ALWAYS use this tool at the start of the chat to find relevant sections.

### 2. get-documentation

Retrieves full documentation content for specific sections. Accepts single or multiple sections.
After calling the list-sections tool, you MUST analyze the returned documentation sections (especially the use_cases field) and then use the get-documentation tool to fetch ALL documentation sections that are relevant for the user's task.

### 3. svelte-autofixer

Analyzes Svelte code and returns issues and suggestions.
You MUST use this tool whenever writing Svelte code before sending it to the user. Keep calling it until no issues or suggestions are returned.

### 4. playground-link

Generates a Svelte Playground link with the provided code.
After completing the code, ask the user if they want a playground link. Only call this tool after user confirmation and NEVER if code was written to files in their project.

---

## Verified Stack (`frontend/package.json` & `pnpm-lock.yaml`)

- `vite-plus`: `1.4.0` (`vp` unified toolchain: Vite 8.3.5, Rolldown 1.2.15, Vitest 5.0.5, tsdown 0.23.1, Oxlint 1.89.0, Oxfmt 0.74.0)
- `svelte`: `5.57.3` (Svelte 5 Runes: `$props`, `$state`, `$derived`, `$effect`)
- `@sveltejs/kit`: `3.1.0`
- `@sveltejs/adapter-cloudflare`: `8.0.0`
- `@sveltejs/enhanced-img`: `0.10.2`
- `vite`: `catalog:` (`npm:@voidzero-dev/vite-plus-core@1.4.0`)
- `typescript`: `6.0.3`
- `super-sitemap`: `2.0.6`
- `schema-dts`: `2.0.0`
- **Version-specific API notes:**
  - Import reactive page state from `$app/state` (`import { page } from '$app/state'`), never `$app/stores` (`TOOL-02`).
  - Import runtime flags (`browser`, `building`, `dev`) from `$app/environment`.

---

## The SEO Contract (enforced — see `docs/seo/SEO_STANDARD.md`)

1. **Single-Source Origin & Breadcrumb Config (`frontend/src/lib/seo/config.ts`):**
   - Exports `PRODUCTION_ORIGIN` (`https://revolutiontradingpros.com`), `TRAILING_SLASH` (`'never'`), `AI_SEARCH_OPT_OUT`, `NOINDEX_PREFIXES`, `isNoindexPath()`, and `buildBreadcrumbsForPath()`.
   - Never hardcode origins (`https://revolutiontradingpros.com` or `*.pages.dev`) outside `frontend/src/lib/seo/config.ts`.
2. **Typed SEO Input (`frontend/src/lib/seo/types.ts` & `frontend/src/app.d.ts`):**
   - `SEOInput` defines `title`, `description`, `canonical`, `robots`, `og`, `twitter`, `alternates`, and `jsonld`.
   - Declared on `App.PageData['seo']` in `frontend/src/app.d.ts` so every `+page.ts` / `+page.server.ts` load function is type-checked when returning `{ seo: { ... } satisfies SEOInput }`.
3. **Single-Source Head Renderer (`frontend/src/lib/seo/Seo.svelte`):**
   - Rendered **once** in `frontend/src/routes/+layout.svelte` using `resolveSEO()` from `frontend/src/lib/seo/resolve.ts`.
   - No other component or route may emit `<title>`, `<meta name="description">`, `<meta name="robots">`, `<link rel="canonical">`, `og:*`, `twitter:*`, or `<script type="application/ld+json">` (`META-05`).
4. **JSON-LD Graph & Safe Serialization (`frontend/src/lib/seo/jsonld.ts` & `frontend/src/lib/seo/schemas.ts`):**
   - `toJsonLdScript()` in `frontend/src/lib/seo/jsonld.ts` serializes `@graph` and escapes `<` (`\u003c`), `>` (`\u003e`), `&` (`\u0026`), `U+2028`, and `U+2029` (`SD-02`).
   - `Organization` and `WebSite` JSON-LD are emitted on `/` only (`SD-04`).
   - Every non-home indexable route automatically receives a `BreadcrumbList` node in `resolveSEO()` matching the visible breadcrumb trail (`SD-05`).
5. **Visible Breadcrumbs (`frontend/src/lib/components/seo/Breadcrumbs.svelte`):**
   - Rendered in `frontend/src/routes/+layout.svelte` on all non-home marketing pages and driven by the shared `buildBreadcrumbsForPath()` helper in `frontend/src/lib/seo/config.ts` (`SD-05`).
6. **Robots & Noindex Enforcement (`frontend/src/lib/seo/robots.ts`, `frontend/src/hooks.server.ts`, `frontend/src/routes/robots.txt/+server.ts`):**
   - `frontend/src/routes/robots.txt/+server.ts` is prerendered (`IDX-07`, `IDX-08`).
   - `frontend/src/hooks.server.ts` attaches `X-Robots-Tag: noindex, nofollow` on non-production hosts (`localhost`, `127.0.0.1`, `*.pages.dev`), `NOINDEX_PREFIXES`, and HTTP status `>= 400` (`IDX-09`).
7. **Sitemap Registration (`frontend/src/routes/sitemap.xml/+server.ts`):**
   - Powered by `super-sitemap` with `excludeRoutePatterns` for private/auth/error routes and `paramValues` for dynamic routes (`/authors/[slug]`, `/indicators/[id]`, `/blog/[slug]`) (`MAP-01`..`MAP-06`).

---

## Forbidden Patterns

1. Never use a second metadata component or raw `<title>`, `<meta name="description">`, `<link rel="canonical">`, or JSON-LD in page/layout files (`META-05`).
2. Never hardcode domain origins outside `frontend/src/lib/seo/config.ts` (`URL-02`).
3. Never set `export const ssr = false` on any indexable route (`IDX-01`).
4. Never emit `<changefreq>` or `<priority>` in XML sitemaps (`MAP-03`).
5. Never emit deprecated/retired Google rich-result schema types (`SD-10`: `FAQPage` on non-gov/health, `HowTo`, `SitelinksSearchBox`, `SpecialAnnouncement`, `CourseInfo`, `EstimatedSalary`, `LearningVideo`, `ClaimReview`, `VehicleListing`, `BookActions`, `PracticeProblems`, `Dataset`).
6. Never fabricate or hardcode `AggregateRating`, `Review`, prices, availability, author names, credentials, or dates (`SD-03`, `YMYL-01`, `YMYL-02`).
7. Never import from `$app/stores` — always use `page` from `$app/state` (`META-05`, `META-09`).
8. Never introduce `any`, `@ts-ignore`, `@ts-expect-error`, or `eslint-disable` (`TOOL-02`).
9. Never weaken prerender handlers (`handleHttpError`, `handleMissingId`, `handleUnseenRoutes` in `frontend/vite.config.ts`) or lower Playwright/Lighthouse test thresholds (`IDX-04`, `PERF-13`, `MON-05`).

---

## New-Page Checklist

1. Configure route rendering options: keep `ssr = true` (default) and set `prerender = true` (plus `entries()` for static parameterized routes) or `prerender = false` for dynamic SSR routes (`IDX-01`, `IDX-02`, `IDX-03`, `URL-01`).
2. Return `seo: { title, description, ... } satisfies SEOInput` from `+page.ts` or `+page.server.ts` with a unique title (15–60 chars rendered) and description (50–160 chars) (`META-01`, `META-02`, `META-10`..`META-13`).
3. Rely on automatic canonical normalization (`https://revolutiontradingpros.com` + lowercase path without trailing slash) or pass an explicit `canonical` path override (`URL-02`, `URL-03`, `URL-04`).
4. Ensure breadcrumb labels resolve cleanly via `buildBreadcrumbsForPath()` in `frontend/src/lib/seo/config.ts` so visible `<Breadcrumbs />` and `BreadcrumbList` JSON-LD match (`SD-05`).
5. Attach the appropriate JSON-LD builder from `frontend/src/lib/seo/schemas.ts` (`buildArticle`, `buildProduct`, `buildCourse`, `buildProfilePage`, `buildSoftwareApplication`, `buildWebPage`) with 100% visible-text parity (`SD-01`, `SD-03`, `SD-06`..`SD-13`).
6. For educational/financial (`YMYL`) pages, render a visible author byline linking to `/authors/[slug]`, visible publication/revision dates, and the financial risk disclaimer (`YMYL-01`, `YMYL-02`, `YMYL-06`).
7. Render images with explicit `width` and `height`, descriptive `alt` text, and `loading="eager"` + `fetchpriority="high"` on the LCP image (`PERF-02`, `PERF-03`, `PERF-04`, `HTML-03`).
8. Use crawlable `<a href="...">` elements with descriptive anchor text for internal navigation and `rel="noopener noreferrer"` (plus `rel="nofollow"` / `rel="sponsored"` / `rel="ugc"` where applicable) on external links (`IDX-10`, `IDX-11`, `HTML-04`, `SPAM-05`, `SPAM-06`).
9. Register dynamic route parameters in `paramValues` in `frontend/src/routes/sitemap.xml/+server.ts` (or add to `excludeRoutePatterns` if non-indexable) and link to the page from at least one hub/listing page (`MAP-01`, `MAP-02`, `IDX-11`).
10. Ensure the route's template is covered by `frontend/tests/seo/*.spec.ts` (`TOOL-03`, `MON-05`).
11. Run all gate commands below and verify exit code `0` with `0` warnings (`TOOL-01`, `TOOL-02`, `MON-05`).

---

## Gate Commands

Run from the repository root (`/Users/billyribeiro/dev/revolution-trading-pros`):

```bash
vp check
vp test
vp build
vp pack
pnpm --filter revolution-svelte check
pnpm --filter revolution-svelte lint
pnpm --filter revolution-svelte test:unit
pnpm --filter revolution-svelte build
pnpm test:seo
```
