# Phase 3b — Static Code Audit (`03-static.md`)

- **Generated At:** 2026-11-01T15:15:26.101Z
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
- `[CODE frontend/src/lib/seo/defaults.ts:37]` — `trailingSlash: 'never',`


## 3. `IDX-04` — Prerender Error Handlers in `frontend/vite.config.ts`
- **Status:** FAIL — `handleMissingId: 'ignore'`, `handleUnseenRoutes: 'ignore'`, and custom `handleHttpError` ignoring 404s and 500s on `/dashboard` and `/tools/options-calculator`.
- `[CODE frontend/vite.config.ts:63]` — `handleHttpError: ({ status, path }) => {`
- `[CODE frontend/vite.config.ts:64]` — `if (status === 404) {`
- `[CODE frontend/vite.config.ts:68]` — `if (status === 500 && path.startsWith('/dashboard')) {`
- `[CODE frontend/vite.config.ts:72]` — `if (status === 500 && path.startsWith('/tools/options-calculator')) {`
- `[CODE frontend/vite.config.ts:78]` — `handleMissingId: 'ignore',`
- `[CODE frontend/vite.config.ts:79]` — `handleUnseenRoutes: 'ignore',`


## 4. `I18N-02` — Paraglide Package Check (`@inlang/paraglide-js` vs deprecated `@inlang/paraglide-sveltekit`)
- **Status:** PASS (Single-locale `en` site; deprecated `@inlang/paraglide-sveltekit` is not installed)
- None (`0` matches)


## 5. `META-05` / `TOOL-01` — Deprecated `$app/stores` Imports in `src/`
- **Status:** PASS (0 non-test files import `$app/stores`)
- None (`0` matches)


## 6. `META-05` — Duplicate SEO Head Tags in `<svelte:head>` Outside `src/lib/seo/Seo.svelte`
- **Total occurrences outside `Seo.svelte`:** 410
- **Status:** FAIL (routes/components emit competing `<title>`, `<meta>`, `<link rel="canonical">`, or `application/ld+json`)
- `[CODE frontend/src/lib/components/SEOHead.svelte:823]` — `<title>{fullTitle}</title>`
- `[CODE frontend/src/lib/components/SEOHead.svelte:825]` — `<meta name="description" content={fullDescription} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:834]` — `<meta name="robots" content={robots || robotsContent} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:842]` — `<link rel="canonical" href={fullCanonical} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:852]` — `<meta property="og:type" content={ogType} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:853]` — `<meta property="og:site_name" content={ogSiteName || siteName} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:854]` — `<meta property="og:title" content={fullTitle} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:855]` — `<meta property="og:description" content={fullDescription} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:856]` — `<meta property="og:url" content={fullCanonical} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:857]` — `<meta property="og:image" content={fullOgImage} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:858]` — `<meta property="og:image:secure_url" content={fullOgImage} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:859]` — `<meta property="og:image:type" content="image/png" />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:860]` — `<meta property="og:image:width" content={String(ogImageWidth)} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:861]` — `<meta property="og:image:height" content={String(ogImageHeight)} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:862]` — `<meta property="og:image:alt" content={ogImageAlt || title} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:863]` — `<meta property="og:locale" content={ogLocale} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:865]` — `<meta property="og:locale:alternate" content={locale} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:906]` — `<meta property="og:video" content={ogVideo} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:907]` — `<meta property="og:video:secure_url" content={ogVideo} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:908]` — `<meta property="og:video:type" content="video/mp4" />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:909]` — `<meta property="og:video:width" content="1280" />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:910]` — `<meta property="og:video:height" content="720" />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:915]` — `<meta property="og:audio" content={ogAudio} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:916]` — `<meta property="og:audio:secure_url" content={ogAudio} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:917]` — `<meta property="og:audio:type" content="audio/mpeg" />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:924]` — `<meta name="twitter:card" content={twitterCard} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:925]` — `<meta name="twitter:site" content={twitterSite || twitterHandle} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:926]` — `<meta name="twitter:creator" content={twitterCreator || twitterHandle} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:927]` — `<meta name="twitter:title" content={fullTitle} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:928]` — `<meta name="twitter:description" content={fullDescription} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:929]` — `<meta name="twitter:image" content={fullTwitterImage} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:930]` — `<meta name="twitter:image:alt" content={twitterImageAlt || ogImageAlt || title} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:932]` — `<meta name="twitter:player" content={twitterPlayer} />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:933]` — `<meta name="twitter:player:width" content="1280" />`
- `[CODE frontend/src/lib/components/SEOHead.svelte:934]` — `<meta name="twitter:player:height" content="720" />`
- `[CODE frontend/src/lib/components/admin/ComingSoonStub.svelte:43]` — `<title>{title ?? entity} — Coming Soon | Admin</title>`
- `[CODE frontend/src/lib/components/admin/TemplateForm.svelte:113]` — `<title>{isEdit ? 'Edit' : 'New'} Email Template | Admin | Revolution Trading Pros</title>`
- `[CODE frontend/src/lib/components/dashboard/Breadcrumbs.svelte:77]` — `{@html '<script type="application/ld+json">' + breadcrumbJsonLd + '</scr' + 'ipt>'}`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:217]` — `<title>{fullTitle}</title>`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:219]` — `<meta name="description" content={metaDescription} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:226]` — `<meta name="robots" content={robotsContent} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:230]` — `<link rel="canonical" href={canonicalUrl} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:234]` — `<meta property="og:type" content={ogType} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:235]` — `<meta property="og:site_name" content={ogSiteName} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:236]` — `<meta property="og:locale" content={ogLocale} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:237]` — `<meta property="og:title" content={finalOgTitle} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:239]` — `<meta property="og:description" content={finalOgDescription} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:242]` — `<meta property="og:url" content={finalOgUrl} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:245]` — `<meta property="og:image" content={finalOgImage} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:247]` — `<meta property="og:image:alt" content={ogImageAlt} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:249]` — `<meta property="og:image:width" content="1200" />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:250]` — `<meta property="og:image:height" content="630" />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:273]` — `<meta name="twitter:card" content={twitterCard} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:275]` — `<meta name="twitter:site" content={twitterSite} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:278]` — `<meta name="twitter:creator" content={twitterCreator} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:280]` — `<meta name="twitter:title" content={finalTwitterTitle} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:282]` — `<meta name="twitter:description" content={finalTwitterDescription} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:285]` — `<meta name="twitter:image" content={finalTwitterImage} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:287]` — `<meta name="twitter:image:alt" content={twitterImageAlt || ogImageAlt} />`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:297]` — `'ipt type="application/ld+json">' +`
- `[CODE frontend/src/routes/(dev)/workbench/+page.svelte:148]` — `<title>Component Workbench | {selectedComponent?.name ?? 'Select a component'}</title>`
- `[CODE frontend/src/routes/+error.svelte:104]` — `<title>{config.title} — Revolution Trading Pros</title>`
- `[CODE frontend/src/routes/+error.svelte:105]` — `<meta name="description" content={config.description} />`
- `[CODE frontend/src/routes/account/+error.svelte:6]` — `<title>{page.status} — Account: Revolution Trading Pros</title>`
- `[CODE frontend/src/routes/account/+page.svelte:46]` — `<title>My Account - Revolution Trading Pros</title>`
- `[CODE frontend/src/routes/account/+page.svelte:47]` — `<meta name="description" content="Manage your Revolution Trading Pros account" />`
- `[CODE frontend/src/routes/account/sessions/+page.svelte:169]` — `<title>Active Sessions - Revolution Trading Pros</title>`
- `[CODE frontend/src/routes/account/sessions/+page.svelte:170]` — `<meta name="description" content="Manage your active sessions and devices" />`
- `[CODE frontend/src/routes/admin/+error.svelte:133]` — `<title>Error {status} — Admin | Revolution Trading Pros</title>`
- `[CODE frontend/src/routes/admin/+error.svelte:134]` — `<meta name="description" content={config.description} />`
- `[CODE frontend/src/routes/admin/+layout.svelte:223]` — `<title>{formatPageTitle(page.url.pathname)} · Admin · Revolution Trading Pros</title>`
- `[CODE frontend/src/routes/admin/analytics/+page.svelte:184]` — `<title>Analytics Dashboard | Revolution Trading</title>`
- `[CODE frontend/src/routes/admin/analytics/attribution/+page.svelte:113]` — `<title>Attribution Report | Analytics</title>`
- `[CODE frontend/src/routes/admin/analytics/cohorts/+page.svelte:195]` — `<title>Cohort Analysis | Analytics</title>`
- `[CODE frontend/src/routes/admin/analytics/cohorts/create/+page.svelte:29]` — `<title>Create Cohort | Analytics</title>`
- `[CODE frontend/src/routes/admin/analytics/events/+page.svelte:131]` — `<title>Event Explorer | Analytics</title>`
- `[CODE frontend/src/routes/admin/analytics/funnels/+page.svelte:150]` — `<title>Conversion Funnels | Analytics</title>`
- `[CODE frontend/src/routes/admin/analytics/funnels/create/+page.svelte:29]` — `<title>Create Funnel | Analytics</title>`
- `[CODE frontend/src/routes/admin/analytics/goals/+page.svelte:186]` — `<title>Conversion Goals | Analytics</title>`
- `[CODE frontend/src/routes/admin/analytics/heatmaps/+page.svelte:121]` — `<title>Click Heatmaps | Analytics</title>`
- ... and 330 more occurrences in admin/dashboard routes


## 7. `SD-02` — JSON-LD Serialization & XSS Escaping
- **Occurrences of JSON-LD serialization in non-test `src/` files:**
- `[CODE frontend/src/lib/components/SEOHead.svelte:6]` — `import { serializeJsonLd } from '#lib/seo/serializeJsonLd.js';`
- `[CODE frontend/src/lib/components/SEOHead.svelte:263]` — `* canonical serializer in '#lib/seo/serializeJsonLd.js'. The script-tag`
- `[CODE frontend/src/lib/components/SEOHead.svelte:268]` — `return '<${tag} type="application/ld+json">${serializeJsonLd(schema)}</${tag}>';`
- `[CODE frontend/src/lib/components/dashboard/Breadcrumbs.svelte:77]` — `{@html '<script type="application/ld+json">' + breadcrumbJsonLd + '</scr' + 'ipt>'}`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:27]` — `import { serializeJsonLd } from '#lib/seo/serializeJsonLd.js';`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:297]` — `'ipt type="application/ld+json">' +`
- `[CODE frontend/src/lib/components/seo/SeoHead.svelte:298]` — `serializeJsonLd(jsonLd) +`
- `[CODE frontend/src/lib/seo/Seo.svelte:20]` — `import { safeJsonLdSerialize, toGraph } from './jsonld';`
- `[CODE frontend/src/lib/seo/Seo.svelte:29]` — `seo.jsonld.length > 0 ? safeJsonLdSerialize(toGraph(seo.jsonld)) : null`
- `[CODE frontend/src/lib/seo/Seo.svelte:130]` — `{@html '<script type="application/ld+json">' + jsonLdScript + '<' + '/script>'}`
- `[CODE frontend/src/lib/seo/index.ts:79]` — `safeJsonLdSerialize,`
- `[CODE frontend/src/lib/seo/jsonld.ts:22]` — `import { serializeJsonLd } from './serializeJsonLd';`
- `[CODE frontend/src/lib/seo/jsonld.ts:279]` — `export function safeJsonLdSerialize(`
- `[CODE frontend/src/lib/seo/jsonld.ts:282]` — `return serializeJsonLd(node);`
- `[CODE frontend/src/lib/seo/schemas.ts:12]` — `* - Undefined fields are omitted by serializeJsonLd downstream; callers may`
- `[CODE frontend/src/lib/seo/serializeJsonLd.ts:8]` — `* '<script type="application/ld+json">...</script>' element (via Svelte`
- `[CODE frontend/src/lib/seo/serializeJsonLd.ts:133]` — `* to embed verbatim inside an HTML '<script type="application/ld+json">'`
- `[CODE frontend/src/lib/seo/serializeJsonLd.ts:150]` — `export function serializeJsonLd(`
- `[CODE frontend/src/lib/seo/structured-data.ts:419]` — `return '<script type="application/ld+json">${toJsonLd(schema)}</script>';`
- `[CODE frontend/src/lib/seo/video-schema.ts:539]` — `return '<script type="application/ld+json">\n${JSON.stringify(schema, null, 2)}\n</script>';`
- `[CODE frontend/src/lib/utils/structured-data.ts:1951]` — `return '<script type="application/ld+json">${toJsonLdScript(data)}</script>';`


## 8. `PERF-02` — `<enhanced:img>` vs Raw `<img>` & `enhancedImages()` Vite Plugin
- **`@sveltejs/enhanced-img` in `package.json`:** 0
- **`enhancedImages()` in `frontend/vite.config.ts`:** 0
- **`<enhanced:img>` occurrences in `src/`:** 0
- **Raw `<img>` occurrences in `src/`:** 201
- **Status:** FAIL (`@sveltejs/enhanced-img` not installed; `enhancedImages()` missing from `vite.config.ts`; 201 raw `<img>` tags)
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
- `[CODE frontend/src/lib/components/courses/CourseCard.svelte:75]` — `<img`
- `[CODE frontend/src/lib/components/courses/CourseCard.svelte:125]` — `<img src={imageUrl} alt={props.course.title} loading="lazy" width="400" height="225" />`
- `[CODE frontend/src/lib/components/dashboard/LatestUpdates.svelte:115]` — `<img src={item.image} alt={item.title} loading="lazy" width="400" height="225" />`
- `[CODE frontend/src/lib/components/dashboard/VideoCard.svelte:69]` — `<img src={defaultThumbnail} alt={video.title} loading="lazy" width="320" height="180" />`
- `[CODE frontend/src/lib/components/dashboard/WatchHistory.svelte:154]` — `<img`
- `[CODE frontend/src/lib/components/dashboard/WeeklyWatchlist.svelte:167]` — `<img`
- `[CODE frontend/src/lib/components/dashboard/WeeklyWatchlist.svelte:187]` — `<img`
- ... and 161 more raw `<img>` occurrences


## 9. `PERF-04` — Asset Preload Filter in `src/hooks.server.ts`
- **Status:** FAIL (`preload` in `hooks.server.ts:545` returns `type === 'js'`, excluding `type === 'font'` and `type === 'css'`)
- `[CODE frontend/src/hooks.server.ts:545]` — `preload: ({ type }) => {`


## 10. `PERF-04` — Third-Party `<script src>` Tags Without `async`/`defer`
- **Occurrences in `app.html` and `.svelte` files:**
- `[CODE frontend/src/routes/dashboard/day-trading-room/+page.svelte:220]` — `<script src="https://apis.google.com/js/api.js"></script>`
- `[CODE frontend/src/routes/dashboard/small-account-mentorship/+page.svelte:220]` — `<script src="https://apis.google.com/js/api.js"></script>`
- `[CODE frontend/src/routes/dashboard/swing-trading-room/+page.svelte:196]` — `<script src="https://apis.google.com/js/api.js"></script>`


## 11. `MAP-03` — `<changefreq>` or `<priority>` in Sitemap Generators
- **Occurrences in `src/routes/*sitemap*`:** 0
- None (`0` matches)


## 12. `SD-11` — Deprecated `FAQPage` / `HowTo` Schema in Non-Admin Routes
- **Occurrences in public routes:** 1
- **Status:** FAIL (FAQPage restricted to government/health authority sites since Aug 2023 / May 2026)
- `[CODE frontend/src/routes/tools/options-calculator/+page.ts:25]` — `// FAQPage markup is deliberately left out: Google dropped FAQ rich results`


## 13. `SD-10` / `SPAM-02` — `AggregateRating` / `ratingValue` / `reviewCount` in Source Files
- **Occurrences in non-admin, non-test `src/` files:** 46
- `[CODE frontend/src/lib/components/SEOHead.svelte:494]` — `'@type': 'AggregateRating',`
- `[CODE frontend/src/lib/components/SEOHead.svelte:495]` — `ratingValue: productRating,`
- `[CODE frontend/src/lib/components/SEOHead.svelte:496]` — `reviewCount: productReviewCount || 1`
- `[CODE frontend/src/lib/components/forms/FormFieldRenderer.svelte:547]` — `{@const ratingValue = typeof props.value === 'number' ? props.value : 0}`
- `[CODE frontend/src/lib/components/forms/FormFieldRenderer.svelte:552]` — `class={['star-button', { active: ratingValue > i }]}`
- `[CODE frontend/src/lib/seo/schemas.ts:285]` — `aggregateRating?: { ratingValue: number; ratingCount: number };`
- `[CODE frontend/src/lib/seo/schemas.ts:308]` — `'@type': 'AggregateRating',`
- `[CODE frontend/src/lib/seo/schemas.ts:309]` — `ratingValue: input.aggregateRating.ratingValue,`
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
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:226]` — `ratingValue: 4.8,`
- `[CODE frontend/src/routes/store/scanners/+page.ts:45]` — `reviewCount: number;`
- `[CODE frontend/src/routes/store/scanners/+page.ts:89]` — `reviewCount: 127,`
- `[CODE frontend/src/routes/store/scanners/+page.ts:128]` — `ratingValue: scanner.rating,`
- `[CODE frontend/src/routes/store/scanners/+page.ts:129]` — `ratingCount: scanner.reviewCount`
- `[CODE frontend/src/routes/store/scanners/high-octane-scanner/+page.ts:41]` — `ratingValue: 4.8,`


## 14. `URL-02` — Hardcoded Production Origin Outside `src/lib/seo/`
- **Occurrences in `src/routes/`:** 83
- **Status:** FAIL (routes hardcode `https://revolutiontradingpros.com` instead of reading from single config)
- `[CODE frontend/src/routes/+page.server.ts:27]` — `'@id': 'https://revolutiontradingpros.com/#financialservice',`
- `[CODE frontend/src/routes/+page.server.ts:31]` — `url: 'https://revolutiontradingpros.com',`
- `[CODE frontend/src/routes/about/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/admin/seo/store-locator/+page.svelte:92]` — `website: 'https://revolutiontradingpros.com',`
- `[CODE frontend/src/routes/alerts/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/alerts/explosive-swings/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/alerts/spx-profit-pulse/+page.ts:5]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/api/maintenance/notify/+server.ts:209]` — `<a href="https://revolutiontradingpros.com" class="cta-button">Visit Our Website</a>`
- `[CODE frontend/src/routes/atom.xml/+server.ts:29]` — `const SITE_URL = import.meta.env.VITE_SITE_URL || 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/authors/[slug]/+page.ts:7]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/blog/+page.ts:7]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/blog/[slug]/+page.svelte:393]` — `url={'https://revolutiontradingpros.com/blog/${post.slug}'}`
- `[CODE frontend/src/routes/blog/[slug]/+page.ts:8]` — `const SITE_URL = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/careers/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/classes/+page.ts:14]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/classes/[slug]/+page.server.ts:20]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/classes/quickstart-precision-trading-c/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/classes/tax-loss-harvest-c/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/contact/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/cookie-policy/+page.ts:11]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/courses/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/courses/day-trading-masterclass/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/courses/options-trading/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/courses/risk-management/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/courses/swing-trading-pro/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/disclaimer/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/feed.xml/+server.ts:30]` — `const SITE_URL = import.meta.env.VITE_SITE_URL || 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/guides/exit-strategies/+page.ts:14]` — `{ name: 'Home', url: 'https://revolutiontradingpros.com' },`
- `[CODE frontend/src/routes/guides/exit-strategies/+page.ts:17]` — `url: 'https://revolutiontradingpros.com/guides/exit-strategies'`
- `[CODE frontend/src/routes/guides/exit-strategies/+page.ts:20]` — `'https://revolutiontradingpros.com/guides/exit-strategies#breadcrumb'`
- `[CODE frontend/src/routes/guides/position-sizing/+page.ts:14]` — `{ name: 'Home', url: 'https://revolutiontradingpros.com' },`
- `[CODE frontend/src/routes/guides/position-sizing/+page.ts:17]` — `url: 'https://revolutiontradingpros.com/guides/position-sizing'`
- `[CODE frontend/src/routes/guides/position-sizing/+page.ts:20]` — `'https://revolutiontradingpros.com/guides/position-sizing#breadcrumb'`
- `[CODE frontend/src/routes/guides/risk-management/+page.ts:14]` — `{ name: 'Home', url: 'https://revolutiontradingpros.com' },`
- `[CODE frontend/src/routes/guides/risk-management/+page.ts:17]` — `url: 'https://revolutiontradingpros.com/guides/risk-management'`
- `[CODE frontend/src/routes/guides/risk-management/+page.ts:20]` — `'https://revolutiontradingpros.com/guides/risk-management#breadcrumb'`
- `[CODE frontend/src/routes/guides/swing-entry/+page.ts:14]` — `{ name: 'Home', url: 'https://revolutiontradingpros.com' },`
- `[CODE frontend/src/routes/guides/swing-entry/+page.ts:17]` — `url: 'https://revolutiontradingpros.com/guides/swing-entry'`
- `[CODE frontend/src/routes/guides/swing-entry/+page.ts:20]` — `'https://revolutiontradingpros.com/guides/swing-entry#breadcrumb'`
- `[CODE frontend/src/routes/indicators/+page.ts:15]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/indicators/[id]/+page.ts:6]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/indicators/macd/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/indicators/rsi/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:18]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:164]` — `'@id': 'https://revolutiontradingpros.com/live-trading-rooms',`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:165]` — `url: 'https://revolutiontradingpros.com/live-trading-rooms',`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:172]` — `'@id': 'https://revolutiontradingpros.com',`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:176]` — `'@id': 'https://revolutiontradingpros.com#organization',`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:178]` — `url: 'https://revolutiontradingpros.com',`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:181]` — `url: 'https://revolutiontradingpros.com/logo.png',`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:193]` — `url: 'https://revolutiontradingpros.com/images/live-trading-rooms-hero.jpg',`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:202]` — `url: 'https://revolutiontradingpros.com'`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:211]` — `item: 'https://revolutiontradingpros.com'`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:217]` — `item: 'https://revolutiontradingpros.com/live-trading-rooms'`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:238]` — `'@id': 'https://revolutiontradingpros.com/live-trading-rooms#service',`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:244]` — `'@id': 'https://revolutiontradingpros.com#organization',`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:250]` — `serviceUrl: 'https://revolutiontradingpros.com/live-trading-rooms',`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:255]` — `'@id': 'https://revolutiontradingpros.com/live-trading-rooms#${room.id}',`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:265]` — `'@id': 'https://revolutiontradingpros.com#organization',`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:284]` — `'@id': 'https://revolutiontradingpros.com#organization',`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:288]` — `url: 'https://revolutiontradingpros.com',`
- `[CODE frontend/src/routes/live-trading-rooms/+page.server.ts:317]` — `'@id': 'https://revolutiontradingpros.com#organization',`
- `[CODE frontend/src/routes/live-trading-rooms/day-trading/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/live-trading-rooms/small-accounts/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/live-trading-rooms/swing-trading/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/mentorship/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/news-sitemap.xml/+server.ts:19]` — `const SITE_URL = import.meta.env.VITE_SITE_URL || 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/our-mission/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/pricing/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/privacy/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/refund/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/resources/+page.ts:14]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/resources/etf-stocks-list/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/resources/stock-indexes-list/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/robots.txt/+server.ts:19]` — `const SITE_URL = import.meta.env.VITE_SITE_URL || 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/sitemap.xml/+server.ts:27]` — `const SITE_URL = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/store/scanners/+page.ts:31]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/store/scanners/high-octane-scanner/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/team/+page.ts:5]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/terms/+page.ts:4]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/tools/options-calculator/embed/+page.svelte:51]` — `href="https://revolutiontradingpros.com/tools/options-calculator"`
- `[CODE frontend/src/routes/tutorials/+page.server.ts:19]` — `const SITE = 'https://revolutiontradingpros.com';`
- `[CODE frontend/src/routes/video-sitemap.xml/+server.ts:19]` — `const SITE_URL = import.meta.env.VITE_SITE_URL || 'https://revolutiontradingpros.com';`

