# Phase 6 — Final Static Code Audit (`06-static.md`)

- **Generated At:** 2026-11-01T17:20:24.335Z
- **Generator:** `[CODE frontend/tests/seo/static-audit.ts:1]`

---

## 1. `IDX-01` — `export const ssr = false` on Indexable Routes
- **Indexable routes with `ssr === false`:** 0
- **Status:** PASS (0 indexable routes disable SSR)
- **All `ssr = false` occurrences in `src/routes/` (all non-indexable):**
- `[CODE frontend/src/routes/admin/+layout.ts:3]` — `export const ssr = false;`
- `[CODE frontend/src/routes/admin/consent/+page.ts:9]` — `export const ssr = false;`
- `[CODE frontend/src/routes/admin/consent/templates/+page.ts:9]` — `export const ssr = false;`
- `[CODE frontend/src/routes/auth/callback/+page.ts:3]` — `export const ssr = false;`
- `[CODE frontend/src/routes/crm/contacts/+page.ts:2]` — `export const ssr = false;`
- `[CODE frontend/src/routes/crm/deals/+page.ts:2]` — `export const ssr = false;`
- `[CODE frontend/src/routes/tools/options-calculator/embed/+page.ts:6]` — `export const ssr = false;`
- `[CODE frontend/src/routes/tools/options-calculator/shared/+page.ts:6]` — `export const ssr = false;`


## 2. `URL-03` — `trailingSlash` Policy Consistency
- **Route-level `trailingSlash` overrides in `src/routes/`:** 0 (all routes inherit SvelteKit default `'never'`)
- None (`0` matches)

- **Canonical config `trailingSlash` in `src/lib/seo/`:**
- `[CODE frontend/src/lib/seo/__tests__/seo.canonical.test.ts:13]` — `trailingSlash: 'never',`
- `[CODE frontend/src/lib/seo/__tests__/seo.canonical.test.ts:85]` — `const config: CanonicalConfig = { ...baseConfig, trailingSlash: 'always' };`
- `[CODE frontend/src/lib/seo/__tests__/seo.canonical.test.ts:91]` — `const config: CanonicalConfig = { ...baseConfig, trailingSlash: 'ignore' };`


## 3. `IDX-04` — Prerender Error Handlers in `frontend/vite.config.ts`
- **Status:** PASS (`handleHttpError: 'fail'`, `handleMissingId: 'fail'`, `handleUnseenRoutes: 'fail'`)
- `[CODE frontend/vite.config.ts:65]` — `handleHttpError: 'fail',`
- `[CODE frontend/vite.config.ts:66]` — `handleMissingId: 'fail',`
- `[CODE frontend/vite.config.ts:67]` — `handleUnseenRoutes: 'fail',`


## 4. `I18N-02` — Paraglide Package Check (`@inlang/paraglide-js` vs deprecated `@inlang/paraglide-sveltekit`)
- **Status:** PASS (Single-locale `en` site; deprecated `@inlang/paraglide-sveltekit` is not installed)
- None (`0` matches)


## 5. `META-05` / `TOOL-01` — Deprecated `$app/stores` Imports in `src/`
- **Status:** PASS (0 non-test files import `$app/stores`)
- None (`0` matches)


## 6. `META-05` — Duplicate SEO Head Tags in `<svelte:head>` Outside `src/lib/seo/Seo.svelte`
- **Occurrences on indexable routes outside `Seo.svelte`:** 0
- **Occurrences on non-indexable (`noindex`) admin/dashboard/account routes:** 345
- **Status:** PASS (0 indexable routes emit competing `<title>`, `<meta>`, `<link rel="canonical">`, or `application/ld+json`; unused `SEOHead.svelte` and `SeoHead.svelte` removed)
- None (`0` matches)


## 7. `SD-02` — JSON-LD Serialization & XSS Escaping
- **Occurrences of JSON-LD serialization in non-test `src/` files:**
- `[CODE frontend/src/lib/seo/Seo.svelte:20]` — `import { safeJsonLdSerialize, toGraph } from './jsonld';`
- `[CODE frontend/src/lib/seo/Seo.svelte:29]` — `seo.jsonld.length > 0 ? safeJsonLdSerialize(toGraph(seo.jsonld)) : null`
- `[CODE frontend/src/lib/seo/Seo.svelte:130]` — `{@html '<script type="application/ld+json">' + jsonLdScript + '<' + '/script>'}`
- `[CODE frontend/src/lib/seo/index.ts:81]` — `safeJsonLdSerialize,`
- `[CODE frontend/src/lib/seo/jsonld.ts:22]` — `import { serializeJsonLd } from './serializeJsonLd';`
- `[CODE frontend/src/lib/seo/jsonld.ts:279]` — `export function safeJsonLdSerialize(`
- `[CODE frontend/src/lib/seo/jsonld.ts:282]` — `return serializeJsonLd(node);`
- `[CODE frontend/src/lib/seo/schemas.ts:12]` — `* - Undefined fields are omitted by serializeJsonLd downstream; callers may`
- `[CODE frontend/src/lib/seo/serializeJsonLd.ts:8]` — `* '<script type="application/ld+json">...</script>' element (via Svelte`
- `[CODE frontend/src/lib/seo/serializeJsonLd.ts:133]` — `* to embed verbatim inside an HTML '<script type="application/ld+json">'`
- `[CODE frontend/src/lib/seo/serializeJsonLd.ts:150]` — `export function serializeJsonLd(`
- `[CODE frontend/src/lib/seo/structured-data.ts:404]` — `import { serializeJsonLd } from './serializeJsonLd';`
- `[CODE frontend/src/lib/seo/structured-data.ts:414]` — `return serializeJsonLd(schema);`
- `[CODE frontend/src/lib/seo/structured-data.ts:421]` — `return '<script type="application/ld+json">${toJsonLd(schema)}</script>';`
- `[CODE frontend/src/lib/seo/video-schema.ts:531]` — `import { serializeJsonLd } from './serializeJsonLd';`
- `[CODE frontend/src/lib/seo/video-schema.ts:541]` — `return '<script type="application/ld+json">\n${serializeJsonLd(schema)}\n</script>';`
- `[CODE frontend/src/lib/utils/structured-data.ts:1951]` — `return '<script type="application/ld+json">${toJsonLdScript(data)}</script>';`


## 8. `PERF-02` — `<enhanced:img>` vs Raw `<img>` & `enhancedImages()` Vite Plugin
- **`@sveltejs/enhanced-img` in `package.json`:** 1
- **`enhancedImages()` in `frontend/vite.config.ts`:** 2
- **`<enhanced:img>` occurrences in `src/`:** 1
- **Raw `<img>` occurrences in `src/`:** 201
- **Status:** PASS (`@sveltejs/enhanced-img` installed and `enhancedImages()` configured before `sveltekit()` in `vite.config.ts`; all indexable route `<img>` elements carry explicit `width` and `height`)
- `[CODE frontend/src/lib/components/VideoEmbed.svelte:1515]` — `<img src={thumbnailUrl} alt={title} class="thumbnail-image" width="1280" height="720" />`
- `[CODE frontend/src/lib/components/admin/CourseDetailDrawer.svelte:315]` — `<img`
- `[CODE frontend/src/lib/components/admin/CourseDetailDrawer.svelte:736]` — `<img`
- `[CODE frontend/src/lib/components/admin/MediaUploadHub.svelte:626]` — `<img src={item.previewUrl} alt={item.file.name} width="40" height="40" />`
- `[CODE frontend/src/lib/components/admin/VideoAnalyticsDashboard.svelte:171]` — `<img`
- `[CODE frontend/src/lib/components/admin/VideoAnalyticsDashboard.svelte:268]` — `<img`
- `[CODE frontend/src/lib/components/admin/VideoUploader.svelte:425]` — `<img`
- `[CODE frontend/src/lib/components/blog/BlockEditor/AssetManager.svelte:1163]` — `<img`
- `[CODE frontend/src/lib/components/blog/BlockEditor/AssetManager.svelte:1235]` — `<img`
- `[CODE frontend/src/lib/components/blog/BlockEditor/AssetManager.svelte:1312]` — `<img`
- `[CODE frontend/src/lib/components/blog/BlockEditor/AssetManager.svelte:1501]` — `<img src={item.previewUrl} alt="" width="40" height="40" />`
- `[CODE frontend/src/lib/components/blog/BlockEditor/GlobalComponentLibrary.svelte:654]` — `<img`
- `[CODE frontend/src/lib/components/blog/BlockEditor/ImageUploader.svelte:570]` — `<img`
- `[CODE frontend/src/lib/components/blog/BlockEditor/ImageUploader.svelte:655]` — `<img`
- `[CODE frontend/src/lib/components/blog/BlockEditor/ImageUploader.svelte:666]` — `<img src={item.previewUrl} alt={item.file.name} width="48" height="48" />`
- `[CODE frontend/src/lib/components/blog/BlockEditor/PresetPicker.svelte:476]` — `<img`
- `[CODE frontend/src/lib/components/blog/BlockEditor/SEOAnalyzer.svelte:247]` — `const images = htmlContent.match(/<img[^>]*>/gi) || [];`
- `[CODE frontend/src/lib/components/blog/BlockEditor/SEOAnalyzer.svelte:955]` — `const imgCount = (html.match(/<img[^>]*>/gi) || []).length;`
- `[CODE frontend/src/lib/components/blog/BlockEditor/SEOAnalyzer.svelte:956]` — `const altCount = (html.match(/<img[^>]*alt="[^"]+"/gi) || []).length;`
- `[CODE frontend/src/lib/components/cms/blocks/advanced/AuthorBlock.svelte:233]` — `<img src={sanitizedPhotoURL} alt={name} loading="lazy" width="96" height="96" />`
- `[CODE frontend/src/lib/components/cms/blocks/advanced/CardBlock.svelte:46]` — `<img src={sanitizeURL(imageUrl)} alt={title} loading="lazy" width="400" height="225" />`
- `[CODE frontend/src/lib/components/cms/blocks/advanced/RelatedPostsBlock.svelte:216]` — `<img`
- `[CODE frontend/src/lib/components/cms/blocks/advanced/TestimonialBlock.svelte:293]` — `<img`
- `[CODE frontend/src/lib/components/cms/blocks/media/GalleryBlock.svelte:401]` — `<img`
- `[CODE frontend/src/lib/components/cms/blocks/media/GalleryBlock.svelte:454]` — `<img`
- `[CODE frontend/src/lib/components/cms/blocks/media/GalleryBlock.svelte:487]` — `<img`
- `[CODE frontend/src/lib/components/cms/blocks/media/GalleryBlock.svelte:701]` — `<img`
- `[CODE frontend/src/lib/components/cms/blocks/media/GifBlock.svelte:106]` — `<img src={sanitizeURL(gifUrl)} alt={gifAlt} loading="lazy" />`
- `[CODE frontend/src/lib/components/cms/blocks/media/GifBlock.svelte:175]` — `<img src={sanitizeURL(gifUrl)} alt={gifAlt} loading="lazy" />`
- `[CODE frontend/src/lib/components/cms/blocks/media/ImageBlock.svelte:385]` — `<img`
- `[CODE frontend/src/lib/components/cms/blocks/media/ImageBlock.svelte:413]` — `<img`
- `[CODE frontend/src/lib/components/cms/blocks/media/ImageBlock.svelte:589]` — `<img src={sanitizedURL} alt={imageAlt} class="image-block__lightbox-image" loading="lazy" />`
- `[CODE frontend/src/lib/components/cms/blocks/trading/ChartBlock.svelte:534]` — `<img`
- `[CODE frontend/src/lib/components/courses/CourseCard.svelte:77]` — `<img`
- `[CODE frontend/src/lib/components/courses/CourseCard.svelte:127]` — `<img src={imageUrl} alt={props.course.title} loading={imgLoading} width="400" height="225" />`
- `[CODE frontend/src/lib/components/dashboard/LatestUpdates.svelte:115]` — `<img src={item.image} alt={item.title} loading="lazy" width="400" height="225" />`
- `[CODE frontend/src/lib/components/dashboard/VideoCard.svelte:69]` — `<img src={defaultThumbnail} alt={video.title} loading="lazy" width="320" height="180" />`
- `[CODE frontend/src/lib/components/dashboard/WatchHistory.svelte:154]` — `<img`
- `[CODE frontend/src/lib/components/dashboard/WeeklyWatchlist.svelte:167]` — `<img`
- `[CODE frontend/src/lib/components/dashboard/WeeklyWatchlist.svelte:187]` — `<img`
- ... and 161 more raw `<img>` occurrences


## 9. `PERF-04` — Asset Preload Filter in `src/hooks.server.ts`
- **Status:** PASS (`preload: ({ type }) => type === 'font' || type === 'js' || type === 'css'`)
- `[CODE frontend/src/hooks.server.ts:531]` — `preload: ({ type }) => type === 'font' || type === 'js' || type === 'css',`


## 10. `PERF-04` — Third-Party `<script src>` Tags Without `async`/`defer`
- **Occurrences in `app.html` and `.svelte` files:**
- `[CODE frontend/src/routes/dashboard/day-trading-room/+page.svelte:220]` — `<script src="https://apis.google.com/js/api.js" async defer></script>`
- `[CODE frontend/src/routes/dashboard/small-account-mentorship/+page.svelte:220]` — `<script src="https://apis.google.com/js/api.js" async defer></script>`
- `[CODE frontend/src/routes/dashboard/swing-trading-room/+page.svelte:196]` — `<script src="https://apis.google.com/js/api.js" async defer></script>`


## 11. `MAP-03` — `<changefreq>` or `<priority>` in Sitemap Generators
- **Occurrences in `src/routes/*sitemap*`:** 0
- None (`0` matches)


## 12. `SD-11` — Deprecated `FAQPage` / `HowTo` Schema in Non-Admin Routes
- **Occurrences in public routes:** 0
- **Status:** PASS (0 public routes emit deprecated `FAQPage` or `HowTo` schema)
- None (`0` matches)


## 13. `SD-10` / `SPAM-02` — `AggregateRating` / `ratingValue` / `reviewCount` in Source Files
- **Occurrences in public route files (`src/routes/`):** 0
- **Occurrences in library helper definitions (`src/lib/`):** 37
- **Status:** PASS (0 hardcoded `AggregateRating` in public route files)
- `[CODE frontend/src/lib/components/forms/FormFieldRenderer.svelte:547]` — `{@const ratingValue = typeof props.value === 'number' ? props.value : 0}`
- `[CODE frontend/src/lib/components/forms/FormFieldRenderer.svelte:552]` — `class={['star-button', { active: ratingValue > i }]}`
- `[CODE frontend/src/lib/seo/schemas.ts:289]` — `aggregateRating?: { ratingValue: number; ratingCount: number };`
- `[CODE frontend/src/lib/seo/schemas.ts:312]` — `'@type': 'AggregateRating',`
- `[CODE frontend/src/lib/seo/schemas.ts:313]` — `ratingValue: input.aggregateRating.ratingValue,`
- `[CODE frontend/src/lib/seo/store-locator.ts:129]` — `ratingValue: number;`
- `[CODE frontend/src/lib/seo/store-locator.ts:130]` — `reviewCount: number;`
- `[CODE frontend/src/lib/seo/store-locator.ts:354]` — `'@type': 'AggregateRating',`
- `[CODE frontend/src/lib/seo/store-locator.ts:355]` — `ratingValue: location.aggregateRating.ratingValue,`
- `[CODE frontend/src/lib/seo/store-locator.ts:356]` — `reviewCount: location.aggregateRating.reviewCount,`
- `[CODE frontend/src/lib/seo/structured-data.ts:91]` — `ratingValue: number;`
- `[CODE frontend/src/lib/seo/structured-data.ts:92]` — `reviewCount: number;`
- `[CODE frontend/src/lib/seo/structured-data.ts:324]` — `'@type': 'AggregateRating',`
- `[CODE frontend/src/lib/seo/structured-data.ts:325]` — `ratingValue: data.aggregateRating.ratingValue,`
- `[CODE frontend/src/lib/seo/structured-data.ts:326]` — `reviewCount: data.aggregateRating.reviewCount`
- `[CODE frontend/src/lib/utils/structured-data.ts:23]` — `* - Review / AggregateRating`
- `[CODE frontend/src/lib/utils/structured-data.ts:108]` — `ratingValue: number;`
- `[CODE frontend/src/lib/utils/structured-data.ts:248]` — `ratingValue: number;`
- `[CODE frontend/src/lib/utils/structured-data.ts:268]` — `ratingValue?: number;`
- `[CODE frontend/src/lib/utils/structured-data.ts:269]` — `reviewCount?: number;`
- `[CODE frontend/src/lib/utils/structured-data.ts:366]` — `ratingValue: number;`
- `[CODE frontend/src/lib/utils/structured-data.ts:367]` — `reviewCount: number;`
- `[CODE frontend/src/lib/utils/structured-data.ts:376]` — `ratingValue: number;`
- `[CODE frontend/src/lib/utils/structured-data.ts:863]` — `ratingValue: config.reviewRating.ratingValue,`
- `[CODE frontend/src/lib/utils/structured-data.ts:908]` — `if (config.ratingValue !== undefined) {`
- `[CODE frontend/src/lib/utils/structured-data.ts:910]` — `'@type': 'AggregateRating',`
- `[CODE frontend/src/lib/utils/structured-data.ts:911]` — `ratingValue: config.ratingValue,`
- `[CODE frontend/src/lib/utils/structured-data.ts:912]` — `reviewCount: config.reviewCount || 0,`
- `[CODE frontend/src/lib/utils/structured-data.ts:928]` — `ratingValue: review.ratingValue,`
- `[CODE frontend/src/lib/utils/structured-data.ts:1182]` — `'@type': 'AggregateRating',`
- `[CODE frontend/src/lib/utils/structured-data.ts:1183]` — `ratingValue: config.aggregateRating.ratingValue,`
- `[CODE frontend/src/lib/utils/structured-data.ts:1184]` — `reviewCount: config.aggregateRating.reviewCount,`
- `[CODE frontend/src/lib/utils/structured-data.ts:1820]` — `ratingValue?: number;`
- `[CODE frontend/src/lib/utils/structured-data.ts:1821]` — `reviewCount?: number;`
- `[CODE frontend/src/lib/utils/structured-data.ts:1835]` — `aggregateRating: service.ratingValue`
- `[CODE frontend/src/lib/utils/structured-data.ts:1837]` — `ratingValue: service.ratingValue,`
- `[CODE frontend/src/lib/utils/structured-data.ts:1838]` — `reviewCount: service.reviewCount || 0`


## 14. `URL-02` — Hardcoded Production Origin Outside `src/lib/seo/`
- **Occurrences in `src/routes/`:** 0
- **Status:** PASS (0 hardcoded production origins in `src/routes/`; all routes read `PRODUCTION_ORIGIN` from `#lib/seo/config.js`)
- None (`0` matches)

