# SEO Plugin Layer — Revolution Trading Pros

Production-grade, unified SEO ownership layer for **SvelteKit 3.1.0 + Svelte 5.57.3** (as of **October 28, 2026**).

## Architecture

```
src/lib/seo/
├── types.ts                    # All TypeScript interfaces (SEOInput, SEOResolved, JsonLdVideoObject, etc.) + svelte-meta-tags / schema-dts re-exports
├── defaults.ts                 # Site-wide default values, canonical policy, private/search path prefixes
├── merge.ts                    # Deterministic deep merge engine with nullable override semantics
├── canonical.ts                # URL normalization engine (HTTPS, lowercase host, query param allow/denylist, trailingSlash: 'never')
├── robots.ts                   # Environment-aware robots directive builder + Google preview controls
├── jsonld.ts                   # Core JSON-LD schema builders, XSS-safe serializer, @graph wrapper
├── schemas.ts                  # Rich result schema builders (Article, Breadcrumb, Course, Product, VideoObject, SoftwareApplication, WebPage)
├── video-schema.ts             # VideoObject + ItemList schema generator for video library pages (with Sept 2026 creator support)
├── dedupe.ts                   # Head tag & JSON-LD @id deduplication engine
├── resolve.ts                  # Full resolution pipeline (resolveSEO, createSEOContext)
├── page-seo-context.svelte.ts  # Svelte 5.57 type-safe createContext() reactive SEO override channel
├── Seo.svelte                  # Single <svelte:head> renderer
├── index.ts                    # Barrel exports (unified layer + svelte-meta-tags@5.0.3 + schema-dts@2.1.0)
└── README.md                   # This file

src/lib/seo/__tests__/
├── seo.merge.test.ts
├── seo.canonical.test.ts
├── seo.robots.test.ts
├── seo.dedupe.test.ts
├── seo.resolve.test.ts
└── schemas.test.ts

scripts/
└── seo-audit.ts                # Post-build HTML + prerendered route SEO audit
```

### Best-in-Class Svelte SEO Stack (October 28, 2026)

| Package / Layer                             | Version  | Role                                                                                                                                                              |
| ------------------------------------------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `src/lib/seo/` (`<Seo />` + `resolveSEO()`) | Built-in | Primary unified `<svelte:head>` owner, canonical normalizer, environment-aware robots guard, and XSS-safe JSON-LD `@graph` serializer                             |
| `svelte-meta-tags`                          | `5.0.3`  | Official Svelte 5 runes meta/JSON-LD library (`MetaTags`, `JsonLd`, `defineBaseMetaTags`, `definePageMetaTags`, `deepMerge`) re-exported from `#lib/seo/index.js` |
| `super-sitemap`                             | `2.0.6`  | Automated SvelteKit route discovery and XML sitemap generation (`src/routes/sitemap.xml/+server.ts`)                                                              |
| `schema-dts`                                | `2.1.0`  | Google's official TypeScript definitions for Schema.org (`Thing`, `WithContext`) integrated with `#lib/seo/types.js`                                              |

### Ownership Model

**One layer owns all `<head>` SEO output.** The `<Seo>` component renders once in the root `+layout.svelte`. Per-route overrides flow through `page.data.seo` from load functions (or via `setPageSeo` from `page-seo-context.svelte.ts`, which uses Svelte 5.57's type-safe `createContext()`). No page or child component should emit `<title>`, `<meta name="description">`, `<link rel="canonical">`, `<meta name="robots">`, `<meta property="og:*">`, or `<meta name="twitter:*">` inside its own `<svelte:head>`.

```
┌────────────────────────────────────────────────────────────────┐
│  +layout.server.ts           →  seoContext + seoDefaults       │
│  +page.ts / +page.server.ts  →  page-level SEOInput            │
│  page-seo-context.svelte.ts  →  createContext() reactive state │
│  +layout.svelte              →  resolveSEO() → <Seo />         │
└────────────────────────────────────────────────────────────────┘
```

**Data flow:** `defaults → layout overrides → page.data.seo → context overrides → resolveSEO() → <Seo />`

**Client-side navigation reactivity:** In `src/routes/+layout.svelte`, `seoContext` derives `pathname`, `isPrivate`, `isSearchPage`, and `isErrorPage` reactively from `page.url.pathname + page.url.search` and `page.status` via `$derived.by(...)`, ensuring SPA navigations always recompute canonical URLs and `noindex` guards accurately.

## Quick Start

### 1. Dependencies

All dependencies are installed in `frontend/package.json` under `pnpm`:

- `svelte-meta-tags@5.0.3` (Svelte 5 runes meta/JSON-LD components & utilities)
- `super-sitemap@2.0.6` (sitemap generation)
- `schema-dts@2.1.0` (Schema.org TypeScript definitions)
- `vitest@5.0.5` + `@vitest/coverage-v8@5.0.5` (unit testing)

### 2. Root layout integration

The root `+layout.server.ts` provides `seoContext` and `seoDefaults` to all pages.
The root `+layout.svelte` computes `resolvedSeo` via `$derived.by(...)` and renders `<Seo seo={resolvedSeo} />`.

### 3. Add per-route SEO overrides

In any `+page.ts` or `+page.server.ts`:

```typescript
import type { Load } from '@sveltejs/kit';
import type { SEOInput } from '#lib/seo/types.js';

export const load: Load = () => {
	const seo: SEOInput = {
		title: 'About Us',
		description: 'Learn about Revolution Trading Pros.',
		og: { type: 'website' }
	};
	return { seo };
};
```

That's it. The root layout picks up `page.data.seo` and resolves `<title>`, `<meta name="description">`, `<link rel="canonical">`, OpenGraph, Twitter Card, robots directives, and JSON-LD `@graph`.

## API Reference

### Types (`#lib/seo/types.js`)

| Type                           | Purpose                                                                                                                                                                         |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SEOInput`                     | What routes provide (partial, nullable)                                                                                                                                         |
| `SEOResolved`                  | Final output ready for rendering                                                                                                                                                |
| `SEODefaults`                  | Site-wide configuration                                                                                                                                                         |
| `OpenGraph`                    | OG meta fields                                                                                                                                                                  |
| `TwitterCard`                  | Twitter Card fields                                                                                                                                                             |
| `RobotsDirectives`             | Robots meta directives (`index`, `follow`, `max-snippet`, `max-image-preview`, `max-video-preview`)                                                                             |
| `AlternateLink`                | `hreflang` alternate link                                                                                                                                                       |
| `VerificationTags`             | Search engine verification codes (`google`, `bing`, `yandex`, `other`)                                                                                                          |
| `JsonLdNode`                   | Union of all JSON-LD schema types (Organization, WebSite, WebPage, Article, BreadcrumbList, FAQPage, Course, Product, SoftwareApplication, VideoObject, SpeakableSpecification) |
| `JsonLdVideoObject`            | Schema.org `VideoObject` with `creator` support (Google Sept 2026 update)                                                                                                       |
| `RouteSEOContext`              | Route context (`pathname`, `env`, `isPrivate`, `isSearchPage`, `isErrorPage`, `statusCode`)                                                                                     |
| `CanonicalConfig`              | Canonical URL normalization config                                                                                                                                              |
| `MetaTagsProps`, `JsonLdProps` | Re-exported from `svelte-meta-tags`                                                                                                                                             |
| `Thing`, `WithContext`         | Re-exported from `schema-dts`                                                                                                                                                   |

### Nullable Semantics

- `undefined` = inherit from parent layer
- `null` = explicitly remove inherited value
- any value = override

### Functions & Builders

#### Resolution & Context

- `resolveSEO(context, defaults, ...overrides)` — Full resolution pipeline returning `SEOResolved`.
- `createSEOContext(opts)` — Create a `RouteSEOContext` from SvelteKit route data.
- `mergeSEO(...layers)` — Deterministic deep merge of `SEOInput` layers.
- `normalizeCanonical(input, config)` — Normalize a URL: force HTTPS, lowercase host, strip tracking params, enforce `trailingSlash: 'never'`.
- `buildRobots(context, overrides, defaults)` — Environment-aware robots directive builder returning `{ directives, content }`.
- `initPageSeo()` / `setPageSeo(seo)` — Svelte 5.57 `createContext()` reactive page SEO state in `#lib/seo/page-seo-context.svelte.js`.

#### JSON-LD Builders (`#lib/seo/jsonld.js` & `#lib/seo/schemas.js`)

- `organizationSchema(opts)` → `JsonLdOrganization`
- `websiteSchema(opts)` → `JsonLdWebSite`
- `breadcrumbSchema(items, id?)` / `buildBreadcrumb(items, pageUrl)` → `JsonLdBreadcrumbList`
- `articleSchema(opts)` / `buildArticle(config)` → `JsonLdArticle`
- `buildCourse(config)` → `JsonLdCourse`
- `buildProduct(config)` → `JsonLdProduct` (defaults `hasAdultConsideration: 'no'` per Google May 2026 update)
- `buildVideoObject(config)` → `JsonLdVideoObject` (supports `creator`, `author`, `publisher`, `thumbnailUrl`, `uploadDate`, `contentUrl`, `embedUrl`, `duration` per Google Sept 2026 update)
- `generateVideoSchema(video)` / `generateVideoListSchema(videos, listMeta)` (`#lib/seo/video-schema.js`) → `VideoObject` / `ItemList` with `creator` support
- `buildSoftwareApplication(config)` → `JsonLdSoftwareApplication`
- `buildWebPage(config)` → `JsonLdWebPage`
- `faqSchema(questions, id?)` / `buildFAQ(items, pageUrl)` → `JsonLdFAQPage` _(deprecated for Google Search rich results — see §Google Search Central 2026 Updates)_
- `speakableSchema(opts)` / `buildSpeakable(cssSelectors, pageUrl)` → `JsonLdSpeakable` _(specialized voice/Assistant markup; not required for AI Overviews)_

#### Serialization & Deduplication Utilities

- `safeJsonLdSerialize(node)` — XSS-safe JSON-LD serialization (escapes `<`, `>`, `&`, `\u2028`, `\u2029`)
- `toGraph(nodes)` — Wrap nodes in a single `@graph` structure
- `dedupeJsonLd(nodes)` — Deduplicate by `@id` or stable hash
- `directivesToString(directives)` / `parseRobotsString(content)` — Robots directive string conversion

## Canonical Policy Tuning

Configured in `src/lib/seo/defaults.ts` → `canonical`:

```typescript
canonical: {
	siteUrl: 'https://revolutiontradingpros.com',
	forceHttps: true,
	trailingSlash: 'never', // 'always' | 'never' | 'ignore'
	queryParamAllowlist: ['page'], // preserved in canonical
	queryParamDenylist: [
		'utm_source', 'utm_medium', 'utm_campaign', 'utm_term', 'utm_content',
		'gclid', 'fbclid', 'msclkid', 'twclid', 'li_fat_id', 'mc_cid', 'mc_eid',
		'ref', 'source', 'affiliate', 'session_id', 'sid', '_ga', '_gl'
	]
}
```

> **Important:** Do NOT set `export const trailingSlash = 'always';` in individual route `+page.ts` files. All routes follow the global `trailingSlash: 'never'` canonical policy so prerendered URLs, `sitemap.xml`, and `<link rel="canonical">` never conflict.

## Robots Policy & Automatic NoIndex Guards

Configured in `src/lib/seo/defaults.ts`:

```typescript
robots: {
	index: true,
	follow: true,
	'max-snippet': -1,
	'max-image-preview': 'large',
	'max-video-preview': -1
}
```

**Automatic `noindex` rules (`privatePathPrefixes` & `searchPathPrefixes`):**

- Non-production environments (`dev`, `staging`, `preview`, `test`)
- Private/transactional/utility prefixes in `privatePathPrefixes`:
  `/account`, `/admin`, `/api`, `/APP`, `/auth`, `/cart`, `/checkout`, `/cms`, `/crm`, `/dashboard`, `/debug`, `/embed`, `/forgot-password`, `/login`, `/logout`, `/maintenance`, `/media`, `/members`, `/my`, `/onboarding`, `/order`, `/posts`, `/profile`, `/register`, `/reset-password`, `/ robots`, `/settings`, `/signup`, `/thank-you`, `/tools/options-calculator/embed`, `/tools/options-calculator/shared`, `/unauthorized`, `/user`, `/verify`, `/verify-email`, `/webhook`, `/watchlist`
- Search query routes matching `searchPathPrefixes` (`/search`, `/?s=`, `/?q=`, `/blog?search=`, `/courses?search=`) — matched against `url.pathname + url.search`
- Error pages (`isErrorPage: true` or HTTP status `>= 400`)

## Sitemap & `robots.txt` Strategy

### `src/routes/sitemap.xml/+server.ts`

- Powered by **super-sitemap 2.0.6**.
- Auto-discovers all public indexable routes from `src/routes/`.
- Excludes all private, auth, checkout, admin, API, embed, and internal routes via `excludeRoutePatterns`.
- Populates `paramValues` for dynamic public routes (`/alerts/[slug]`, `/blog/[slug]`, `/classes/[slug]`, `/courses/[slug]`, `/indicators/[slug]`, `/learn/[slug]`, `/live-trading-rooms/[slug]`, `/ scanner/[slug]`, `/ trading-rooms/[slug]`).

### `src/routes/robots.txt/+server.ts`

- Environment-aware: blocks all crawling (`Disallow: /`) on staging/preview/dev environments; allows public indexing on production while disallowing private/admin/checkout/API prefixes.
- Explicitly configures AI search and training user-agents (`GPTBot`, `OAI-SearchBot`, `ChatGPT-User`, `ClaudeBot`, `Claude-Web`, `PerplexityBot`, `Google-Extended`, `Applebot-Extended`, `CCBot`, `Bytespider`) to allow public marketing pages while blocking private/member/admin/API paths.
- Points crawlers to `Sitemap: https://revolutiontradingpros.com/sitemap.xml`.

## Google Search Central 2026 Updates (Verified through October 28, 2026)

All Google Search Central documentation and algorithm updates through **October 28, 2026** are reflected in this SEO layer:

| Date           | Google Search Central Update                                                                                                                                  | Implementation in Revolution Trading Pros                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **2026-02**    | **February 2026 Discover Core Update**                                                                                                                        | Large OG images (`1200×630`) + `max-image-preview: 'large'` enabled across all indexable marketing and blog routes.                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| **2026-03**    | **March 2026 Core Update & Spam Update**                                                                                                                      | Eliminated competitor brand leaks ("Simpler Trading") and hyphenated domain typos (`revolution-trading-pros.com`) across all routes; enforced single `<h1>` hierarchy and unique route metadata.                                                                                                                                                                                                                                                                                                                                                                     |
| **2026-05-07** | **FAQPage Rich Results Deprecation** (Rich Results Test support removed June 2026; GSC API removed Aug 2026; restricted to government/health authority sites) | `FAQPage` JSON-LD removed from commercial marketing routes; `faqSchema` / `buildFAQ` marked `@deprecated` in code; visible FAQ accordions preserved in semantic HTML for users.                                                                                                                                                                                                                                                                                                                                                                                      |
| **2026-05-15** | **AI Overviews & AI Mode Guidance** (`SpeakableSpecification` clarified as voice/Assistant-only; standard SSR HTML + preview controls drive AI Overviews)     | Full SSR HTML on all public pages (including `cookie-policy` fixed from `ssr = false` to `ssr = true; prerender = true`), `max-snippet: -1`, `max-image-preview: 'large'`, `max-video-preview: -1`, and explicit AI crawler rules in `robots.txt`.                                                                                                                                                                                                                                                                                                                   |
| **2026-05-20** | **Merchant Listing / Product `hasAdultConsideration`**                                                                                                        | `buildProduct()` in `src/lib/seo/schemas.ts` includes `hasAdultConsideration: 'no'` by default for all courses, indicators, and alert subscriptions.                                                                                                                                                                                                                                                                                                                                                                                                                 |
| **2026-05-21** | **May 2026 Core Update** (completed June 2, 2026)                                                                                                             | Every public marketing route (`about`, `blog`, `career`, `chatroom-rules`, `compare`, `contact`, `cookie-policy`, `courses`, `crm`, `disclaimer`, `faq`, `free-trading-room-archive`, `indicators`, `learning-center`, `live-trading-rooms`, `mentorship`, `my-story`, `our-services`, `preferred-partners`, `privacy-policy`, `refund-policy`, `resources/*`, `scanner`, `store/scanners`, `swing-trading-room-archive`, `terms-of-service`, `tools/options-calculator`, `tutorials`, `video-library`) has dedicated `SEOInput` via `+page.ts` / `+page.server.ts`. |
| **2026-08**    | **August 2026 Discover Core Update**                                                                                                                          | Enforced strict content-to-metadata alignment, clickbait-free titles (`50–60` chars), and high-resolution OpenGraph/Twitter summary_large_image cards.                                                                                                                                                                                                                                                                                                                                                                                                               |
| **2026-09**    | **Video Structured Data (`VideoObject.creator`) Update**                                                                                                      | Added `JsonLdVideoObject` with `creator` (`Person` or `Organization`) in `src/lib/seo/types.ts`, `buildVideoObject()` in `src/lib/seo/schemas.ts`, and `creator` support in `src/lib/seo/video-schema.ts`.                                                                                                                                                                                                                                                                                                                                                           |
| **2026-10**    | **Search Console API `search_type: 'ai_overview'` & Core Web Vitals INP ≤ 200ms**                                                                             | SSR + prerendering enabled on static marketing pages, zero duplicate `<svelte:head>` tags, and clean hydration on Svelte 5.57.3 / SvelteKit 3.1.0.                                                                                                                                                                                                                                                                                                                                                                                                                   |

## Verification & Audit Commands

Always use `pnpm` (never `npm`):

```bash
# Typecheck (0 errors, 0 warnings)
pnpm --filter revolution-svelte check
pnpm --filter revolution-svelte check:strict
pnpm --filter revolution-svelte check:a11y

# Run SEO unit tests
pnpm --filter revolution-svelte test:unit src/lib/seo/__tests__/

# Run full unit test suite
pnpm --filter revolution-svelte test:unit

# Build production bundle & prerender pages
pnpm --filter revolution-svelte build

# Run post-build HTML SEO audit (verifies <title>, description, canonical, robots, OG, Twitter, JSON-LD, single <h1>)
cd frontend && node scripts/seo-audit.ts
```
