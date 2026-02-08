# SEO Plugin Layer — Revolution Trading Pros

Production-grade, unified SEO ownership layer for SvelteKit 2 + Svelte 5.

## Architecture

```
src/lib/seo/
├── types.ts          # All TypeScript interfaces (SEOInput, SEOResolved, etc.)
├── defaults.ts       # Site-wide default values
├── merge.ts          # Deterministic deep merge engine
├── canonical.ts      # URL normalization engine
├── robots.ts         # Environment-aware robots directive builder
├── jsonld.ts         # Type-safe JSON-LD schema builders
├── dedupe.ts         # Head tag deduplication engine
├── resolve.ts        # Full resolution pipeline
├── Seo.svelte        # Single <svelte:head> renderer
├── index.ts          # Barrel exports
└── README.md         # This file

__tests__/
├── seo.merge.test.ts
├── seo.canonical.test.ts
├── seo.robots.test.ts
├── seo.dedupe.test.ts
└── seo.resolve.test.ts

scripts/
└── seo-audit.ts      # Post-build HTML audit
```

### Ownership Model

**One layer owns all `<head>` SEO output.** The `<Seo>` component renders once in the root `+layout.svelte`. Per-route overrides flow through `page.data.seo` from load functions. No other component should emit SEO-related `<svelte:head>` tags.

```
┌─────────────────────────────────────────────────┐
│  +layout.server.ts  →  seoContext + seoDefaults │
│  +page.server.ts    →  page-level SEOInput      │
│  +layout.svelte     →  resolveSEO() → <Seo />  │
└─────────────────────────────────────────────────┘
```

**Data flow:** `defaults → layout overrides → page overrides → resolveSEO() → <Seo />`

## Quick Start

### 1. No additional install needed

All dependencies are already in the project:
- `super-sitemap` (sitemap generation)
- `vitest` + `@vitest/coverage-v8` (testing)
- `zod` (available for runtime validation if needed)

### 2. Root layout already integrated

The root `+layout.server.ts` provides `seoContext` and `seoDefaults` to all pages.
The root `+layout.svelte` renders `<Seo seo={resolvedSeo} />`.

### 3. Add per-route SEO overrides

In any `+page.server.ts` or `+page.ts`:

```typescript
import type { SEOInput } from '$lib/seo/types';

export const load = async () => {
  const seo: SEOInput = {
    title: 'About Us',
    description: 'Learn about Revolution Trading Pros.',
  };
  return { seo };
};
```

That's it. The layout picks up `page.data.seo` and resolves everything.

## API Reference

### Types

| Type | Purpose |
|------|---------|
| `SEOInput` | What routes provide (partial, nullable) |
| `SEOResolved` | Final output ready for rendering |
| `SEODefaults` | Site-wide configuration |
| `OpenGraph` | OG meta fields |
| `TwitterCard` | Twitter Card fields |
| `RobotsDirectives` | Robots meta directives |
| `AlternateLink` | hreflang alternate link |
| `VerificationTags` | Search engine verification codes |
| `JsonLdNode` | Union of all JSON-LD schema types |
| `RouteSEOContext` | Route context (pathname, env, flags) |
| `CanonicalConfig` | Canonical URL normalization config |

### Nullable Semantics

- `undefined` = inherit from parent layer
- `null` = explicitly remove inherited value
- any value = override

### Functions

#### `resolveSEO(context, defaults, ...overrides)`
Full resolution pipeline. Returns `SEOResolved`.

#### `createSEOContext(opts)`
Create a `RouteSEOContext` from common SvelteKit data.

#### `mergeSEO(...layers)`
Deterministic deep merge of SEOInput layers.

#### `normalizeCanonical(input, config)`
Normalize a URL: force HTTPS, lowercase host, strip tracking params, enforce trailing slash.

#### `buildRobots(context, overrides, defaults)`
Environment-aware robots directive builder. Returns `{ directives, content }`.

#### JSON-LD Builders
- `organizationSchema(opts)` → `JsonLdOrganization`
- `websiteSchema(opts)` → `JsonLdWebSite`
- `breadcrumbSchema(items, id?)` → `JsonLdBreadcrumbList`
- `articleSchema(opts)` → `JsonLdArticle`
- `faqSchema(questions, id?)` → `JsonLdFAQPage`

#### Utilities
- `safeJsonLdSerialize(node)` — XSS-safe JSON-LD serialization
- `toGraph(nodes)` — Wrap nodes in `@graph` structure
- `dedupeJsonLd(nodes)` — Deduplicate by `@id` or stable hash
- `directivesToString(directives)` — Convert to meta content string
- `parseRobotsString(content)` — Parse content string back to directives

## Examples

### Marketing Page (Indexable)

```typescript
// src/routes/about/+page.server.ts
import type { SEOInput } from '$lib/seo/types';

export const load = async () => {
  const seo: SEOInput = {
    title: 'About Us',
    description: 'Revolution Trading Pros: 18,000+ traders trust our education.',
    og: { type: 'website' },
  };
  return { seo };
};
```

### Blog Article with JSON-LD

```typescript
// src/routes/blog/[slug]/+page.ts
import type { SEOInput } from '$lib/seo/types';
import { articleSchema, breadcrumbSchema } from '$lib/seo/jsonld';

export const load = async ({ params, url }) => {
  const post = await fetchPost(params.slug);
  const seo: SEOInput = {
    title: post.title,
    description: post.excerpt,
    canonical: `https://revolution-trading-pros.pages.dev${url.pathname}`,
    og: {
      type: 'article',
      image: post.image,
      article: {
        publishedTime: post.publishedAt,
        author: post.author,
      },
    },
    jsonld: [
      articleSchema({
        headline: post.title,
        url: `https://revolution-trading-pros.pages.dev${url.pathname}`,
        datePublished: post.publishedAt,
        authorName: post.author,
        publisherName: 'Revolution Trading Pros',
        type: 'BlogPosting',
      }),
      breadcrumbSchema([
        { name: 'Home', url: 'https://revolution-trading-pros.pages.dev' },
        { name: 'Blog', url: 'https://revolution-trading-pros.pages.dev/blog' },
        { name: post.title, url: `https://revolution-trading-pros.pages.dev${url.pathname}` },
      ]),
    ],
  };
  return { post, seo };
};
```

### Private Page (NoIndex)

```typescript
// src/routes/account/+page.server.ts
import type { SEOInput } from '$lib/seo/types';

export const load = async () => {
  const seo: SEOInput = {
    title: 'My Account',
    description: 'Manage your account.',
    robots: { index: false, follow: false },
  };
  return { seo };
};
```

Note: `/account` is in `privatePathPrefixes` so it's automatically noindex even without the explicit override. The explicit setting is defense-in-depth.

## Canonical Policy Tuning

Edit `src/lib/seo/defaults.ts` → `canonical`:

```typescript
canonical: {
  siteUrl: 'https://your-domain.com',
  forceHttps: true,
  trailingSlash: 'never',       // 'always' | 'never' | 'ignore'
  queryParamAllowlist: ['page'], // preserved in canonical
  queryParamDenylist: [          // stripped from canonical
    'utm_source', 'utm_medium', 'gclid', 'fbclid', ...
  ],
}
```

## Robots Policy Tuning

Edit `src/lib/seo/defaults.ts`:

```typescript
robots: {
  index: true,
  follow: true,
  'max-snippet': -1,
  'max-image-preview': 'large',
  'max-video-preview': -1,
}
```

**Automatic noindex rules:**
- Non-production environments (dev/staging)
- Paths matching `privatePathPrefixes`
- Paths matching `searchPathPrefixes`
- Pages with `isErrorPage: true`

**Safety guards:**
- `index: true` override is ignored on non-production environments
- `index: true` override is ignored on private/error pages

## JSON-LD Recipes

### FAQ Page

```typescript
import { faqSchema } from '$lib/seo/jsonld';

const seo: SEOInput = {
  jsonld: [
    faqSchema([
      { question: 'What is day trading?', answer: 'Day trading is...' },
      { question: 'How do I start?', answer: 'Start by...' },
    ]),
  ],
};
```

### Course Page

```typescript
import type { JsonLdNode } from '$lib/seo/types';

const courseNode: JsonLdNode = {
  '@context': 'https://schema.org',
  '@type': 'Course',
  '@id': 'https://example.com/courses/trading-101#course',
  name: 'Trading 101',
  description: 'Learn the fundamentals.',
  provider: { '@type': 'Organization', name: 'Revolution Trading Pros' },
};

const seo: SEOInput = { jsonld: [courseNode] };
```

## Sitemap Strategy

**super-sitemap** is the sole sitemap owner.

- **Location:** `src/routes/sitemap.xml/+server.ts`
- **Auto-discovers** routes from `src/routes/`
- **Excludes** all private/admin/auth/technical routes via `excludeRoutePatterns`
- **Supports** sitemap index for >50K URLs automatically
- **Constraints:** 50,000 URLs max OR 50MB uncompressed per file (Google limit)

### Adding dynamic routes to sitemap

For routes like `/blog/[slug]`, provide `paramValues`:

```typescript
paramValues: {
  '/blog/[slug]': slugs.map(s => s.slug),
}
```

### robots.txt reference

The `robots.txt` at `src/routes/robots.txt/+server.ts` already references:
```
Sitemap: https://revolution-trading-pros.pages.dev/sitemap.xml
```

## Google Search Console Verification

1. Get your verification code from [Google Search Console](https://search.google.com/search-console)
2. Update `src/lib/seo/defaults.ts`:
   ```typescript
   verification: {
     google: 'YOUR_VERIFICATION_CODE',
   }
   ```
3. Deploy and verify in GSC

## Deployment Checklist

- [ ] `npm run check` — 0 errors, 0 warnings
- [ ] `npm run test` — all tests pass
- [ ] `npm run build` — successful build
- [ ] `npx tsx scripts/seo-audit.ts` — 0 errors in audit
- [ ] Verify `robots.txt` at `/robots.txt`
- [ ] Verify `sitemap.xml` at `/sitemap.xml`
- [ ] Test OG tags with [Facebook Sharing Debugger](https://developers.facebook.com/tools/debug/)
- [ ] Test Twitter cards with [Twitter Card Validator](https://cards-dev.twitter.com/validator)
- [ ] Validate JSON-LD with [Google Rich Results Test](https://search.google.com/test/rich-results)
- [ ] Submit sitemap in Google Search Console
- [ ] Submit sitemap in Bing Webmaster Tools

## Google-Aligned Compliance

### Sitemap Constraints
- Max 50,000 URLs per sitemap file
- Max 50MB uncompressed per file
- super-sitemap auto-segments when exceeded

### Robots Meta / X-Robots-Tag
- Full support for all standard directives
- Environment-aware defaults prevent accidental indexing

### Structured Data Quality
- Type-safe builders prevent invalid schemas
- XSS-safe serialization for script injection context
- Deduplicated by `@id` or stable hash

### Snippet/Preview Controls
- `max-snippet`, `max-image-preview`, `max-video-preview` supported
- Defaults: `-1` (unlimited), `large`, `-1`

### Core Update Resilience
- Intent-match: per-route title/description customization
- Originality: unique canonical URLs, no duplicate content
- Trust signals: Organization + WebSite JSON-LD on every page
- Content quality: structured data validates required fields

### Google Discover Monitoring (February 2026)
- **Context:** February 2026 Discover update awareness
- **Action:** Monitor Discover traffic in GSC Performance → Discover tab
- **Note:** This is a Discover-specific volatility event, NOT a broad Search core update
- Track: impressions, CTR, and traffic changes weekly
- Document any significant drops with date + affected URLs

## Operations Checklist

- [ ] Weekly: Check GSC for crawl errors
- [ ] Weekly: Monitor Discover traffic (Feb 2026 update context)
- [ ] Monthly: Run `npx tsx scripts/seo-audit.ts` post-build
- [ ] Monthly: Validate JSON-LD with Rich Results Test
- [ ] Quarterly: Review and update sitemap exclusion patterns
- [ ] On deploy: Verify robots.txt and sitemap.xml are accessible
- [ ] On new routes: Add SEOInput to load function

## Troubleshooting Matrix

| Symptom | Cause | Fix |
|---------|-------|-----|
| Duplicate `<title>` tags | Multiple components emit `<svelte:head>` title | Remove all `<title>` from page-level `<svelte:head>` blocks; use `seo.title` in load function |
| Page indexed when it shouldn't be | Missing from `privatePathPrefixes` | Add path prefix to `defaults.ts` → `privatePathPrefixes` |
| OG image not showing | Missing `og.image` in SEOInput | Set `og.image` in page load or update `defaultImage` in defaults |
| JSON-LD validation errors | Missing required fields | Use type-safe builders (`articleSchema`, etc.) which enforce required fields |
| Canonical has tracking params | Param not in denylist | Add param to `canonical.queryParamDenylist` in defaults |
| Staging site indexed by Google | `env` not set correctly | Ensure `import.meta.env.MODE` returns correct value; check `detectEnv()` in layout server |
| Sitemap missing routes | Route matches exclusion pattern | Check `excludeRoutePatterns` in sitemap.xml server |
| Duplicate meta tags in audit | Legacy `<svelte:head>` in page components | Remove SEO tags from page `<svelte:head>` blocks; use load function `seo` instead |

## Migration Guide

### From Mixed Libraries to Unified Layer

**Before (scattered SEO):**
```svelte
<!-- In every page component -->
<svelte:head>
  <title>My Page | Brand</title>
  <meta name="description" content="..." />
  <meta property="og:title" content="..." />
  <link rel="canonical" href="..." />
  {@html `<script type="application/ld+json">...</script>`}
</svelte:head>
```

**After (unified layer):**
```typescript
// In +page.server.ts or +page.ts
export const load = async () => {
  const seo: SEOInput = {
    title: 'My Page',
    description: '...',
  };
  return { seo };
};
// No <svelte:head> SEO tags in the component!
```

### Migration Steps

1. **Remove SEO `<svelte:head>` blocks** from all page components
   - Remove `<title>`, `<meta name="description">`, `<meta property="og:*">`, `<meta name="twitter:*">`, `<link rel="canonical">`, JSON-LD scripts
   - Keep non-SEO tags (theme-color, RSS feeds, etc.)

2. **Add `seo` to load functions** for each page that had custom SEO
   - Import `SEOInput` from `$lib/seo/types`
   - Return `{ seo: { title, description, ... } }` from load

3. **Remove duplicate meta from `app.html`**
   - The `<Seo>` component now owns robots, OG, and Twitter meta
   - Remove any `<meta name="robots">`, `<meta property="og:*">`, `<meta name="twitter:*">` from `app.html`

4. **Update sitemap** to use super-sitemap (already done)

5. **Run verification:**
   ```bash
   npm run check          # 0 errors
   npm run test           # all pass
   npm run build          # success
   npx tsx scripts/seo-audit.ts  # 0 errors
   ```

### What to Keep in `app.html`

These tags stay in `app.html` because they're static and not route-specific:
- `<meta charset="utf-8" />`
- `<meta name="viewport" ...>`
- `<link rel="icon" ...>`
- `<link rel="manifest" ...>`
- `<meta name="theme-color" ...>`
- PWA/mobile meta tags
- Font loading
- `%sveltekit.head%`

### What Moves to the SEO Layer

These are now exclusively managed by `<Seo>`:
- `<title>`
- `<meta name="description">`
- `<meta name="robots">`
- `<link rel="canonical">`
- `<meta property="og:*">`
- `<meta name="twitter:*">`
- `<link rel="alternate" hreflang="...">`
- Verification meta tags
- JSON-LD `<script>` blocks

## Commands

```bash
# Typecheck
npm run check

# Run SEO tests
npx vitest run src/lib/seo/__tests__/

# Run all tests
npm run test

# Build
npm run build

# Post-build SEO audit
npx tsx scripts/seo-audit.ts

# Post-build SEO audit (custom dir)
npx tsx scripts/seo-audit.ts .svelte-kit/output/prerendered/pages
```
