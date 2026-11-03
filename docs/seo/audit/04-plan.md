# Phase 4 — Evidence-Backed Remediation Plan (`04-plan.md`)

- **Generated At:** 2026-11-01
- **Baseline Matrix Reference:** `[CODE docs/seo/audit/03-matrix-baseline.md:1-192]`
- **Total Baseline Failures Addressed:** 94 matrix rows (`41 P0`, `50 P1`, `3 P2`) across 35 distinct requirement IDs.
- **Ordering:** `P0 → P1 → P2`. Each task groups related requirement IDs, cites the exact baseline root cause (`[CODE path:line]`), lists the files to modify/create, and specifies the automated verification check.

---

## P0 Remediation Tasks (Indexing, Crawlability, Canonicals, Rendering, Structured Data Integrity, Spam Prevention)

### Task P0-1: Unify Canonical Origin & Trailing-Slash Normalization (`URL-02`, `URL-03`, `URL-04`, `MAP-02`, `META-03`)

- **Baseline Failing Rows:** `URL-02` (8 template rows — 49/50 indexable pages), `URL-03` (`ALL`), `URL-04` (`ALL`), `MAP-02` (`ALL`), `META-03` (8 template rows).
- **Root Cause:**
  1. `[CODE frontend/.env.production:20]` sets `VITE_SITE_URL=https://revolution-trading-pros.pages.dev`, which `[CODE frontend/src/lib/seo/defaults.ts:19]` reads into `SITE_URL`, causing 49/50 indexable pages to emit `<link rel="canonical">`, `og:url`, and `og:image` pointing to `https://revolution-trading-pros.pages.dev/...`.
  2. `[CODE frontend/src/routes/sitemap.xml/+server.ts:27]` and `[CODE frontend/src/routes/store/scanners/+page.ts:68]` hardcode `https://revolutiontradingpros.com`, creating a 49/50 mismatch between page canonicals and `<loc>` in `/sitemap.xml`.
  3. `[CODE frontend/src/lib/seo/canonical.ts:95]` and `[CODE frontend/src/lib/seo/resolve.ts:1]` append a trailing slash to `/` (`https://.../`) or allow mixed trailing slashes when `trailingSlash` policy is `'never'`.
- **Files to Change:**
  - `frontend/src/lib/seo/config.ts` (new single source of truth for `PRODUCTION_ORIGIN = 'https://revolutiontradingpros.com'`, `TRAILING_SLASH = 'never'`, `NOINDEX_PREFIXES`, `TRACKING_PARAM_ALLOWLIST`, `AI_TRAINING_BOTS`, `AI_SEARCH_BOTS`)
  - `frontend/.env.production` (set `VITE_SITE_URL=https://revolutiontradingpros.com` and `PUBLIC_SITE_URL=https://revolutiontradingpros.com`)
  - `frontend/src/lib/seo/defaults.ts` (derive `siteUrl` from `PRODUCTION_ORIGIN` in `config.ts`, never allowing `.pages.dev` or `localhost` into canonicals or JSON-LD `@id`s)
  - `frontend/src/lib/seo/canonical.ts` & `frontend/src/lib/seo/resolve.ts` (normalize root `/` and all paths identically across `<link rel="canonical">`, `og:url`, `BreadcrumbList`, and `/sitemap.xml` `<loc>`; strip tracking params `utm_*`, `gclid`, `fbclid`, `msclkid`, `mc_cid`, `mc_eid`, `_ga`, `_gl`, `ref`, `igshid`, `twclid`, `ttclid`, `yclid`)
- **Requirements Closed:** `URL-02`, `URL-03`, `URL-04`, `MAP-02`, `META-03`.
- **Verification:** `tests/seo/canonical.spec.ts` & `tests/seo/sitemap.spec.ts` assert 100% 1:1 exact string equality between every indexable page's `<link rel="canonical">`, `og:url`, and `<loc>` in `/sitemap.xml`.

---

### Task P0-2: Eliminate Prerender Error Masking, Fix Soft-404s, and Enforce `308` Permanent Redirects (`IDX-03`, `IDX-04`, `IDX-05`, `IDX-06`, `REND-04`)

- **Baseline Failing Rows:** `IDX-03` (`ALL`), `IDX-04` (`ALL`), `IDX-05` (`ALL`), `IDX-06` (`ALL`), `REND-04` (`ALL`).
- **Root Cause:**
  1. `[CODE frontend/vite.config.ts:63-79]` sets `handleMissingId: 'ignore'`, `handleUnseenRoutes: 'ignore'`, and ignores `status === 404` and `status === 500` in `handleHttpError`.
  2. `[CODE frontend/src/routes/authors/[slug]/+page.ts:9]` and `[CODE frontend/src/routes/indicators/[id]/+page.ts:8]` export `prerender = false` despite reading from static TS arrays (`authors.ts` and `indicators/data.ts`), and `[CODE frontend/src/routes/indicators/[id]/+page.ts:12]` never throws `error(404)` when `indicator` is undefined — returning HTTP `200` with `robots="index, follow"` for `/indicators/__nonexistent_slug_seo_test__`.
  3. `[CODE frontend/src/routes/classes/[slug]/+page.server.ts:97-101]` catches missing course errors and returns `200` with a dummy fallback object when the slug is not one of the known classes.
  4. `[CODE frontend/src/routes/signup/+page.server.ts:27]` throws `redirect(301, ...)` instead of `redirect(308, ...)`.
  5. `[CODE frontend/src/routes/+error.svelte:104-105]` emits competing `<title>` and `<meta name="description">` in `<svelte:head>` alongside root `<Seo>` without `robots: noindex, nofollow`.
- **Files to Change:**
  - `frontend/vite.config.ts` (set `handleHttpError: 'fail'` and `handleMissingId: 'fail'`)
  - `frontend/src/routes/authors/[slug]/+page.ts` (export `prerender = true` and `entries()` from `ALL_AUTHORS`)
  - `frontend/src/routes/indicators/[id]/+page.ts` (export `prerender = true`, `entries()` from ` indicators`, and throw `error(404, 'Indicator not found')` when `!indicator`)
  - `frontend/src/routes/classes/[slug]/+page.server.ts` (only allow known public class slugs or real backend records; throw `error(404, 'Class not found')` for unknown slugs)
  - `frontend/src/routes/signup/+page.server.ts` (change `redirect(301, target)` to `redirect(308, target)`)
  - `frontend/src/routes/+error.svelte` & `frontend/src/routes/+layout.svelte` (remove competing `<svelte:head>` from `+error.svelte` and ensure root `<Seo>` emits `<meta name="robots" content="noindex, nofollow">` whenever `page.status >= 400` or `page.error` is non-null)
- **Requirements Closed:** `IDX-03`, `IDX-04`, `IDX-05`, `IDX-06`, `REND-04`.
- **Verification:** `tests/seo/status-and-redirects.spec.ts` asserts HTTP `404` + `noindex` on `/__nonexistent_seo_test_slug__`, `/indicators/__nonexistent_slug_seo_test__`, `/authors/__nonexistent_slug_seo_test__`, `/classes/__nonexistent_slug_seo_test__`, and HTTP `308` on `/signup`.

---

### Task P0-3: Fix `robots.txt` Asset Blocking & Enforce `X-Robots-Tag` in `hooks.server.ts` (`IDX-07`, `IDX-08`, `IDX-09`)

- **Baseline Failing Rows:** `IDX-07` (`ALL`), `IDX-08` (`ALL`), `IDX-09` (`ALL`).
- **Root Cause:**
  1. `[CODE frontend/src/routes/robots.txt/+server.ts:60]` contains `Disallow: /_app/`, blocking Googlebot from fetching SvelteKit JS/CSS bundles under `/_app/immutable/`.
  2. `[CODE frontend/src/routes/robots.txt/+server.ts:395-411]` emits invalid RFC 9309 wildcard `User-agent: *-ai` tokens and lumps AI search/citation bots (`OAI-SearchBot`, `PerplexityBot`, `ChatGPT-User`) together with AI training crawlers (`GPTBot`, `Google-Extended`, `ClaudeBot`, `CCBot`).
  3. `[CODE frontend/src/routes/robots.txt/+server.ts:19,457-458]` reads `VITE_SITE_URL` (`pages.dev`) and references broken `/news-sitemap.xml` and `/video-sitemap.xml`.
  4. `[CODE frontend/src/hooks.server.ts:458-486]` does not emit `X-Robots-Tag: noindex, nofollow` when `event.url.origin !== PRODUCTION_ORIGIN` (e.g. `*.pages.dev`, `localhost`, `127.0.0.1`) or on private route prefixes (`/dashboard`, `/my`, `/crm`, `/account`, `/settings`, `/verify-email`, `/forgot-password`, `/reset-password`).
- **Files to Change:**
  - `frontend/src/routes/robots.txt/+server.ts` (rewrite using `src/lib/seo/config.ts`: `Allow: /_app/immutable/`, valid RFC 9309 `User-agent` tokens only, separate `AI_TRAINING_BOTS` disallow vs `AI_SEARCH_BOTS` allow controlled by `AI_SEARCH_OPT_OUT`, and `Sitemap: https://revolutiontradingpros.com/sitemap.xml`)
  - `frontend/src/hooks.server.ts` & `frontend/src/routes/+layout.server.ts` (emit `X-Robots-Tag: noindex, nofollow` when `event.url.origin !== PRODUCTION_ORIGIN` or pathname matches `NOINDEX_PREFIXES`)
- **Requirements Closed:** `IDX-07`, `IDX-08`, `IDX-09`.
- **Verification:** `tests/seo/robots.spec.ts` asserts `/robots.txt` allows `/_app/immutable/`, contains no wildcard `User-agent` tokens, separates AI training vs search bots, and verifies `X-Robots-Tag: noindex, nofollow` on non-production origins and `NOINDEX_PREFIXES`.

---

### Task P0-4: Fix CSR-Only Content on `/blog`, `/classes`, `/cookie-policy` and Crawlability of Links (`REND-01`, `LINK-01`, `LINK-03`, `LINK-05`)

- **Baseline Failing Rows:** `REND-01` (`static`, `listing`), `LINK-01` (`ALL`), `LINK-03` (`ALL`), `LINK-05` (`listing`).
- **Root Cause:**
  1. `[CODE frontend/src/routes/blog/+page.svelte:49,197]` ignores `data.posts` loaded by `+page.ts` and re-fetches in `onMount`, rendering `"Loading posts..."` (10 words) in SSR HTML.
  2. `[CODE frontend/src/routes/blog/+page.svelte:368-414]` uses `<button onclick={prevPage}>` and `<button onclick={nextPage}>` instead of `<a href="/blog?page=...">`.
  3. `[CODE frontend/src/routes/classes/+page.svelte:30,66]` fetches `/api/admin/courses` (which returns 401!) in `onMount`, rendering `"Loading courses..."` in SSR HTML.
  4. `[CODE frontend/src/routes/cookie-policy/+page.svelte:1]` renders 145 words in SSR and 304 words after hydration due to client-mounted sections.
  5. Broken internal links on indexable pages: `/get-started` (404), `/join` (404), `/methodology` (404), `/performance` (404), and `/signup` (redirect).
- **Files to Change:**
  - `frontend/src/routes/blog/+page.ts` & `frontend/src/routes/blog/+page.svelte` (render SSR `data.posts` immediately in initial HTML without waiting for `onMount`; convert pagination controls to `<a href="/blog?page={n}">` with self-canonical per page)
  - `frontend/src/routes/classes/+page.server.ts` (new or updated load function providing SSR classes list) & `frontend/src/routes/classes/+page.svelte` (render SSR classes immediately in initial HTML with links to `/classes/quickstart-precision-trading-c` and `/classes/tax-loss-harvest-c`)
  - `frontend/src/routes/cookie-policy/+page.svelte` (ensure all policy sections render during SSR so SSR word count equals hydrated DOM word count)
  - Update internal `<a href>` references pointing to `/get-started`, `/join`, `/signup` → `/register` or `/pricing`, and `/methodology`, `/performance` → `/our-mission` or `/about` across marketing components (`frontend/src/routes/our-mission/+page.svelte`, `frontend/src/routes/mentorship/+page.svelte`, `frontend/src/routes/pricing/+page.svelte`, `frontend/src/routes/live-trading-rooms/**`, `frontend/src/routes/alerts/**`, `frontend/src/lib/components/**`).
- **Requirements Closed:** `REND-01`, `LINK-01`, `LINK-03`, `LINK-05`.
- **Verification:** `tests/seo/ssr-parity.spec.ts` and `tests/seo/links.spec.ts` assert SSR word count ≥ 90% of hydrated word count on all 50 indexable pages and 0 broken internal links (`status >= 400` or redirects).

---

### Task P0-5: Single-Source Head Ownership & `<title>` / `<meta name="description">` Compliance (`META-01`, `META-02`, `META-05`, `TOOL-02`)

- **Baseline Failing Rows:** `META-01` (7 template rows — 28 indexable pages), `META-02` (6 template rows — 11 indexable pages), `META-05` (`ALL`), `TOOL-02` (`ALL`).
- **Root Cause:**
  1. `[CODE frontend/src/lib/seo/defaults.ts:20]` sets `titleTemplate: '%s | Revolution Trading Pros'` (26 chars suffix), causing any page title > 34 chars to exceed the 60-char ceiling (28/50 pages fail, and `/authors/*` have a double brand suffix `"Billy Ribeiro — Revolution Trading Pros | Revolution Trading Pros"`).
  2. 11 indexable pages have `<meta name="description">` outside 50–160 chars (`/authors/*` are 32–44 chars; `/our-mission`, `/live-trading-rooms`, `/resources`, `/alerts/explosive-swings`, `/live-trading-rooms/day-trading`, `/mentorship`, `/resources/stock-indexes-list`, `/tools/options-calculator` are 161–207 chars).
  3. Legacy `SEOHead.svelte` and `SeoHead.svelte` and `<svelte:head>` blocks emit duplicate head tags outside `src/lib/seo/Seo.svelte`.
- **Files to Change:**
  - `frontend/src/lib/seo/defaults.ts` & `frontend/src/lib/seo/resolve.ts` (smart title formatting: use concise brand suffix `| RTP` when `${title} | Revolution Trading Pros` would exceed 60 chars, and strip duplicate brand suffixes; enforce description length clamping/validation or exact 50–160 char descriptions across all 50 indexable routes)
  - Route `+page.ts` / `+page.server.ts` files for the 28 title-failing and 11 description-failing pages (craft unique, accurate 15–60 char titles and 50–160 char descriptions)
  - `frontend/src/lib/components/SEOHead.svelte` & `frontend/src/lib/components/seo/SeoHead.svelte` (gut `<svelte:head>` emissions from legacy components or delete unused ones; remove competing `<title>` / `<meta name="description">` / `<link rel="canonical">` from child routes so `src/lib/seo/Seo.svelte` in root `+layout.svelte` is the single head owner)
- **Requirements Closed:** `META-01`, `META-02`, `META-05`, `TOOL-02`.
- **Verification:** `tests/seo/meta.spec.ts` asserts exactly 1 `<title>` (15–60 chars), 1 `<meta name="description">` (50–160 chars), 1 `<link rel="canonical">`, 1 `<meta name="robots">`, and complete OpenGraph tags on all 50 indexable pages, plus static check confirming 0 competing `<svelte:head>` SEO tags outside `Seo.svelte`.

---

### Task P0-6: Visible Breadcrumbs, JSON-LD Content Parity, and Removal of Retired/Fabricated Schema (`SD-03`, `SD-04`, `SD-05`, `SD-06`, `SD-08`, `SD-09`, `SD-10`, `SD-11`, `SD-12`, `SPAM-02`, `TOOL-04`)

- **Baseline Failing Rows:** `SD-03` (7 template rows), `SD-04` (`home`), `SD-05` (7 template rows), `SD-06` (`article`), `SD-08` (`product`), `SD-09` (`profile`), `SD-10` (`ALL`), `SD-11` (`ALL`), `SD-12` (`software`), `SPAM-02` (`ALL`), `TOOL-04` (`ALL`).
- **Root Cause:**
  1. 43/49 non-home indexable pages emit `BreadcrumbList` in JSON-LD while rendering **zero** visible `<nav aria-label="Breadcrumb">` in the DOM (`domCrumbs=0`).
  2. `[CODE frontend/src/lib/seo/defaults.ts:86-120]` emits `Organization` and `WebSite` on every route instead of only `/`, and includes retired `potentialAction: SearchAction` (`SitelinksSearchBox` deprecated by Google).
  3. `[CODE frontend/src/routes/store/scanners/+page.ts:89,128]`, `[CODE frontend/src/routes/store/scanners/high-octane-scanner/+page.ts:41]`, and `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:226]` emit hardcoded self-serving `AggregateRating` (`ratingValue: 4.8`, `reviewCount: 127`).
  4. `[CODE frontend/src/routes/authors/[slug]/+page.ts:30-46]` emits separate `ProfilePage` and `Person` nodes instead of `ProfilePage` with `mainEntity: Person`.
  5. `[CODE frontend/src/routes/indicators/[id]/+page.ts:29-34]` and `[CODE frontend/src/routes/indicators/rsi/+page.ts:25-31]` emit `SoftwareApplication` without `operatingSystem` and `offers`.
  6. `/alerts/*`, `/live-trading-rooms/*`, `/mentorship`, and `/pricing` `Product` schemas lack `image` or `offers` matching visible DOM pricing; `/guides/*` and `/resources/*` `Article` schemas lack `image` array and `author.url`.
- **Files to Change:**
  - `frontend/src/lib/components/seo/Breadcrumbs.svelte` (new accessible `<nav aria-label="Breadcrumb"><ol>` component rendered automatically on all non-home marketing pages in `frontend/src/routes/+layout.svelte`, driven by the exact same breadcrumb items that generate `BreadcrumbList` JSON-LD)
  - `frontend/src/lib/seo/jsonld.ts`, `frontend/src/lib/seo/schemas.ts`, `frontend/src/lib/seo/defaults.ts` (type all builders with `WithContext<Thing>` from `schema-dts`; emit `Organization` + `WebSite` on `/` only; remove `SearchAction` from `WebSite`; auto-generate matching `BreadcrumbList` + visible `Breadcrumbs.svelte` for all 49 non-home indexable pages)
  - `frontend/src/routes/store/scanners/+page.ts`, `frontend/src/routes/store/scanners/high-octane-scanner/+page.ts`, `frontend/src/routes/live-trading-rooms/+page.server.ts` (remove hardcoded `AggregateRating` `4.8` / `127`)
  - `frontend/src/routes/authors/[slug]/+page.ts` (emit `ProfilePage` with `mainEntity: Person`)
  - `frontend/src/routes/indicators/[id]/+page.ts` & `frontend/src/routes/indicators/rsi/+page.ts` (add `operatingSystem: 'Web, ThinkorSwim, TradingView'` and `offers` to `SoftwareApplication`)
  - `frontend/src/routes/alerts/**`, `frontend/src/routes/live-trading-rooms/**`, `frontend/src/routes/mentorship/+page.ts`, `frontend/src/routes/pricing/+page.ts` (ensure `Product` includes `name`, `description`, `image`, and `offers` matching visible DOM price)
  - `frontend/src/routes/guides/**` & `frontend/src/routes/resources/etf-stocks-list/+page.ts`, `frontend/src/routes/resources/stock-indexes-list/+page.ts` (emit complete `Article` schema with `headline`, `image`, `datePublished`, `dateModified`, `author` linking to `/authors/billy-ribeiro`, and render visible author byline + `<time datetime>` in DOM)
- **Requirements Closed:** `SD-03`, `SD-04`, `SD-05`, `SD-06`, `SD-08`, `SD-09`, `SD-10`, `SD-11`, `SD-12`, `SPAM-02`, `TOOL-04`, `YMYL-01`.
- **Verification:** `tests/seo/jsonld.spec.ts` asserts valid JSON-LD on all 50 indexable pages, 0 `SearchAction`, 0 unbacked `AggregateRating`, `Organization`+`WebSite` on `/` only, and 1:1 parity between `BreadcrumbList` items and `<nav aria-label="Breadcrumb"> li` elements on all 49 non-home pages.

---

### Task P0-7: Clean Up `/sitemap.xml` and Unseeded News/Video Sitemaps (`MAP-01`, `MAP-04`, `TOOL-03`)

- **Baseline Failing Rows:** `MAP-01` (`ALL`), `MAP-04` (`ALL`), `TOOL-03` (`ALL`).
- **Root Cause:**
  1. `[CODE frontend/src/routes/sitemap.xml/+server.ts:108,129,135,137]` includes `/login`, `/register`, and `/signup` (which redirects with 301/308), and omits `/authors/billy-ribeiro`, `/authors/freddie-ferber`, `/authors/shao-wen`.
  2. `[CODE frontend/src/routes/news-sitemap.xml/+server.ts:43-77]` hardcodes 4 non-existent blog URLs that all return 404.
  3. `[CODE frontend/src/routes/video-sitemap.xml/+server.ts:91-177]` hardcodes fake video embeds (`dQw4w9WgXcQ`, `123456789`) and `/blog/weekly-market-recap` (404).
- **Files to Change:**
  - `frontend/src/routes/sitemap.xml/+server.ts` (drive `/sitemap.xml` from `PRODUCTION_ORIGIN` and the exact 50 indexable routes, including `paramValues` for `/authors/[slug]`, `/indicators/[id]`, `/classes/[slug]`, and excluding `/login`, `/register`, `/signup`, `/cms/**`, `/admin/**`, `/dashboard/**`, `/account/**`, `/auth/**`, `/api/**`, `/__**`)
  - `frontend/src/routes/news-sitemap.xml/+server.ts` & `frontend/src/routes/video-sitemap.xml/+server.ts` (remove hardcoded fake URLs and Rickroll video IDs; return empty valid `<urlset>` when no real published news/video records exist, and remove them from `robots.txt` `Sitemap:` directives)
- **Requirements Closed:** `MAP-01`, `MAP-04`, `TOOL-03`.
- **Verification:** `tests/seo/sitemap.spec.ts` fetches every `<loc>` in `/sitemap.xml` and asserts HTTP `200`, `noindex=false`, and `canonical === loc` across all 50 URLs.

---

### Task P0-8: Root `<html>` `lang`/`dir` Transform & Above-the-Fold Image Eager Loading (`SEM-03`, `PERF-03`)

- **Baseline Failing Rows:** `SEM-03` (`ALL`), `PERF-03` (`listing`, `product`, `software`).
- **Root Cause:**
  1. `[CODE frontend/src/app.html:4]` hardcodes `<html lang="en">` without `dir` and without `%lang%` / `%dir%` placeholders transformed in `hooks.server.ts`.
  2. 7 indexable pages put `loading="lazy"` on the first `<main>` image: `/store/scanners`, `/tutorials`, `/live-trading-rooms/swing-trading`, `/indicators/bollinger-bands`, `/indicators/moving-averages`, `/indicators/stochastic`, `/indicators/vwap`.
- **Files to Change:**
  - `frontend/src/app.html` (set `<html lang="%lang%" dir="%dir%">`)
  - `frontend/src/hooks.server.ts` (replace `%lang%` with `'en'` and `%dir%` with `'ltr'` in `transformPageChunk`)
  - `frontend/src/routes/store/scanners/+page.svelte`, `frontend/src/routes/tutorials/+page.svelte`, `frontend/src/routes/live-trading-rooms/swing-trading/+page.svelte`, `frontend/src/routes/indicators/[id]/+page.svelte` (set `loading="eager"` and `fetchpriority="high"` on the primary above-the-fold image)
- **Requirements Closed:** `SEM-03`, `PERF-03`.
- **Verification:** `tests/seo/cwv-and-images.spec.ts` asserts `<html lang="en" dir="ltr">` on all pages and verifies no first `<main>` image has `loading="lazy"`.

---

## P1 & P2 Remediation Tasks (Internal Linking, Headings, Contrast, Preloading, Images, E-E-A-T, CI Suite)

### Task P1-1: Eliminate Orphan Pages & Improve Generic Anchor Text (`LINK-02`, `LINK-04`)

- **Baseline Failing Rows:** `LINK-02` (`ALL` — 13 orphan pages, 21 unreachable from `/`), `LINK-04` (`ALL`).
- **Root Cause:**
  1. Marketing navigation/footer and hub pages (`/resources`, `/courses`, `/alerts`, `/live-trading-rooms`, `/about`, `/team`) do not link to `/authors/*`, `/guides/*`, `/resources/etf-stocks-list`, `/resources/stock-indexes-list`, `/tools/options-calculator`, `/classes/*`, `/store/scanners/high-octane-scanner`, `/cookie-policy`, `/tutorials`, `/team`.
  2. `[CODE frontend/src/routes/blog/+page.svelte:343]` uses generic `"Read More"` anchor text.
- **Files to Change:**
  - `frontend/src/lib/components/layout/MarketingFooter.svelte` or `Footer.svelte` (add crawlable hub links to `/alerts`, `/live-trading-rooms`, `/resources`, `/tutorials`, `/team`, `/cookie-policy`, `/tools/options-calculator`, `/store/scanners`)
  - `frontend/src/routes/resources/+page.svelte` (link to all `/guides/*`, `/resources/etf-stocks-list`, `/resources/stock-indexes-list`, `/tools/options-calculator`)
  - `frontend/src/routes/team/+page.svelte` & `frontend/src/routes/about/+page.svelte` (link to `/authors/billy-ribeiro`, `/authors/freddie-ferber`, `/authors/shao-wen`)
  - `frontend/src/routes/store/scanners/+page.svelte` (link to `/store/scanners/high-octane-scanner`)
  - `frontend/src/routes/live-trading-rooms/+page.svelte` (link to `/live-trading-rooms/day-trading`, `/live-trading-rooms/swing-trading`, `/live-trading-rooms/small-accounts`)
  - `frontend/src/routes/blog/+page.svelte` (replace generic `"Read More"` with descriptive `"Read {post.title}"`)
- **Requirements Closed:** `LINK-02`, `LINK-04`.
- **Verification:** `tests/seo/links.spec.ts` asserts 0 orphan indexable pages and 100% reachability of all 50 indexable pages within ≤ 4 clicks from `/`.

---

### Task P1-2: Fix Heading Hierarchy Skips & WCAG 2 AA Color Contrast (`SEM-02`, `SEM-05`)

- **Baseline Failing Rows:** `SEM-02` (5 template rows — 13 pages), `SEM-05` (4 template rows — 4 pages).
- **Root Cause:**
  1. 13 indexable pages jump from `<h1>` to `<h3>` or `<h2>` to `<h4>`: `/our-mission`, `/resources`, `/alerts/explosive-swings`, `/live-trading-rooms/day-trading`, `/live-trading-rooms/swing-trading`, `/mentorship`, `/classes/quickstart-precision-trading-c`, `/courses/day-trading-masterclass`, `/courses/swing-trading-pro`, `/guides/exit-strategies`, `/guides/position-sizing`, `/guides/risk-management`, `/resources/etf-stocks-list`.
  2. `@axe-core/playwright` found `color-contrast` violations on `/about`, `/courses`, `/alerts/explosive-swings`, and `/guides/risk-management`.
- **Files to Change:**
  - Fix heading tag levels (`<h3>` → `<h2>`, `<h4>` → `<h3>`) in the 13 listed route `.svelte` files.
  - Darken/lighten low-contrast badge/muted text colors on `/about`, `/courses`, `/alerts/explosive-swings`, and `/guides/risk-management` to meet WCAG 2 AA (≥ 4.5:1 for normal text).
- **Requirements Closed:** `SEM-02`, `SEM-05`.
- **Verification:** `tests/seo/semantics.spec.ts` asserts 0 heading skips across all 50 indexable pages and 0 `@axe-core/playwright` violations across all 8 template representatives.

---

### Task P1-3: Fix Missing Scanner Image, Enable Font/CSS Preloading, and Configure `@sveltejs/enhanced-img` (`MEDIA-01`, `PERF-02`, `PERF-04`)

- **Baseline Failing Rows:** `MEDIA-01` (`ALL`), `PERF-02` (`ALL`), `PERF-04` (`ALL`).
- **Root Cause:**
  1. `[CODE frontend/src/routes/store/scanners/+page.ts:86]` and `+page.svelte` reference `/images/scanners/high-octane-scanner.jpg` which does not exist in `frontend/static/` (returns 404).
  2. `[CODE frontend/src/hooks.server.ts:545-549]` `preload` filter only returns `type === 'js'`, omitting `font` and `css`.
  3. `@sveltejs/enhanced-img` is not installed and `enhancedImages()` is missing from `frontend/vite.config.ts`.
- **Files to Change:**
  - `frontend/src/routes/store/scanners/+page.ts` & `+page.svelte` (update image reference to existing asset or add optimized image at `frontend/static/images/scanners/high-octane-scanner.jpg`)
  - `frontend/src/hooks.server.ts` (update `preload: ({ type }) => type === 'font' || type === 'js' || type === 'css'`)
  - `frontend/package.json` & `frontend/vite.config.ts` (install `@sveltejs/enhanced-img` and register `enhancedImages()` before `sveltekit()`)
- **Requirements Closed:** `MEDIA-01`, `PERF-02`, `PERF-04`.
- **Verification:** `tests/seo/cwv-and-images.spec.ts` asserts 0 broken images on all 50 indexable pages, `enhancedImages()` active in `vite.config.ts`, and `preload` callback enabling `font`, `js`, and `css`.

---

### Task P1-4: YMYL Author Bylines, Author Bios, and Primary-Source Citations (`YMYL-01`, `YMYL-02`)

- **Baseline Failing Rows:** `YMYL-01` (`article`), `YMYL-02` (`ALL`).
- **Root Cause:**
  1. `[CODE frontend/src/lib/authors/authors.ts:24,31,38]` leaves `bio: ''` empty on all 3 authors (`billy-ribeiro`, `freddie-ferber`, `shao-wen`).
  2. Trading guides (`/guides/*`) and resource articles (`/resources/etf-stocks-list`, `/resources/stock-indexes-list`) lack visible author bylines linking to `/authors/[slug]`, visible `<time datetime>` timestamps, and primary-source regulatory/exchange citations (SEC, FINRA, Cboe, CME).
- **Files to Change:**
  - `frontend/src/lib/authors/authors.ts` & `frontend/src/routes/authors/[slug]/+page.svelte` (populate factual editorial bios and role descriptions per owner confirmation)
  - `frontend/src/routes/guides/**/+page.svelte` & `frontend/src/routes/resources/etf-stocks-list/+page.svelte`, `frontend/src/routes/resources/stock-indexes-list/+page.svelte` (render visible author byline linking to `/authors/billy-ribeiro`, `<time datetime>` published/updated dates, and primary-source citations to SEC/FINRA/Cboe/CME documentation)
- **Requirements Closed:** `YMYL-01`, `YMYL-02`.
- **Verification:** `tests/seo/jsonld.spec.ts` and `tests/seo/semantics.spec.ts` assert visible author links, `<time datetime>`, non-empty author bios on `/authors/[slug]`, and primary-source outbound citations on YMYL guides.

---

### Task P1-5: Comprehensive Playwright SEO Regression Test Suite (`OPS-03`)

- **Baseline Failing Rows:** `OPS-03` (`ALL`).
- **Files to Create:**
  - `frontend/tests/seo/ssr-parity.spec.ts` (`IDX-01`, `REND-01`, `REND-02`, `REND-03`)
  - `frontend/tests/seo/status-and-redirects.spec.ts` (`IDX-05`, `IDX-06`, `REND-04`)
  - `frontend/tests/seo/robots.spec.ts` (`IDX-07`, `IDX-08`, `IDX-09`)
  - `frontend/tests/seo/canonical.spec.ts` (`URL-01`, `URL-02`, `URL-03`, `URL-04`)
  - `frontend/tests/seo/meta.spec.ts` (`META-01`..`META-05`)
  - `frontend/tests/seo/semantics.spec.ts` (`SEM-01`..`SEM-05`)
  - `frontend/tests/seo/jsonld.spec.ts` (`SD-01`..`SD-13`, `SPAM-02`, `YMYL-01`, `YMYL-02`)
  - `frontend/tests/seo/sitemap.spec.ts` (`MAP-01`..`MAP-04`)
  - `frontend/tests/seo/links.spec.ts` (`LINK-01`..`LINK-05`)
  - `frontend/tests/seo/i18n.spec.ts` (`I18N-01`..`I18N-04`)
  - `frontend/tests/seo/cwv-and-images.spec.ts` (`PERF-01`..`PERF-05`, `MEDIA-01`..`MEDIA-03`)
  - Wire `test:seo` script in `frontend/package.json` and root `package.json`.
- **Requirements Closed:** `OPS-03`.
- **Verification:** `pnpm test:seo` exits `0` with all spec files passing.
