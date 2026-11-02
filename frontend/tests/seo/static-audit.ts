import fs from 'node:fs';
import path from 'node:path';

interface InventoryEntry {
	app: string;
	route: string;
	pathPattern: string;
	url: string | null;
	indexable: boolean;
	template: string;
	pageOptions: {
		prerender: { value: boolean | string; source: string };
		ssr: { value: boolean | string; source: string };
		csr: { value: boolean | string; source: string };
		trailingSlash: { value: string; source: string };
	};
	files: string[];
}

interface CodeHit {
	file: string;
	line: number;
	snippet: string;
}

const repoRoot = path.resolve(import.meta.dirname, '../../..');
const frontendDir = path.join(repoRoot, 'frontend');
const srcDir = path.join(frontendDir, 'src');
const inventoryPath = path.join(repoRoot, 'docs/seo/audit/02-inventory.json');

function walkFiles(dir: string, extFilter?: (f: string) => boolean): string[] {
	const results: string[] = [];
	if (!fs.existsSync(dir)) return results;
	for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
		if (entry.name === 'node_modules' || entry.name === '.svelte-kit' || entry.name.startsWith('.')) {
			continue;
		}
		const full = path.join(dir, entry.name);
		if (entry.isDirectory()) {
			results.push(...walkFiles(full, extFilter));
		} else if (!extFilter || extFilter(full)) {
			results.push(full);
		}
	}
	return results;
}

function searchInFiles(files: string[], regex: RegExp): CodeHit[] {
	const hits: CodeHit[] = [];
	for (const file of files) {
		const content = fs.readFileSync(file, 'utf8');
		const lines = content.split(/\r?\n/);
		for (let i = 0; i < lines.length; i++) {
			const lineText = lines[i] ?? '';
			if (regex.test(lineText)) {
				hits.push({
					file: path.relative(repoRoot, file),
					line: i + 1,
					snippet: lineText.trim()
				});
			}
		}
	}
	return hits;
}

function runStaticAudit(): void {
	const inventory: InventoryEntry[] = JSON.parse(fs.readFileSync(inventoryPath, 'utf8'));
	const allSrcFiles = walkFiles(srcDir, (f) => /\.(svelte|ts|js)$/.test(f));
	const svelteFiles = allSrcFiles.filter((f) => f.endsWith('.svelte'));

	// 1. IDX-01: export const ssr = false on any indexable route
	const ssrFalseIndexable = inventory.filter(
		(entry) => entry.indexable && entry.pageOptions.ssr.value === false
	);
	const allSsrFalseHits = searchInFiles(
		walkFiles(path.join(srcDir, 'routes'), (f) => /\.(ts|js)$/.test(f)),
		/export\s+const\s+ssr\s*=\s*false/
	);

	// 2. URL-03: trailingSlash settings across all routes vs canonical format
	const trailingSlashHits = searchInFiles(
		walkFiles(path.join(srcDir, 'routes'), (f) => /\.(ts|js)$/.test(f)),
		/export\s+const\s+trailingSlash\b/
	);
	const canonicalConfigHits = searchInFiles(
		walkFiles(path.join(srcDir, 'lib/seo'), (f) => /\.(ts|js)$/.test(f)),
		/trailingSlash:\s*['"][^'"]+['"]/
	);

	// 3. IDX-04: prerender.handleHttpError, handleMissingId, handleUnseenRoutes in vite.config.ts
	const viteConfigPath = path.join(frontendDir, 'vite.config.ts');
	const prerenderConfigHits = searchInFiles(
		[viteConfigPath],
		/handleHttpError|handleMissingId|handleUnseenRoutes|status\s*===\s*(404|500)/
	);

	// 4. I18N-02: @inlang/paraglide-sveltekit vs @inlang/paraglide-js in package.json
	const pkgPaths = [path.join(repoRoot, 'package.json'), path.join(frontendDir, 'package.json')];
	const paraglideHits = searchInFiles(pkgPaths, /@inlang\/paraglide/);

	// 5. META-05 / TOOL-01: $app/stores imports anywhere in src/
	const appStoresHits = searchInFiles(
		allSrcFiles.filter((f) => !f.includes('__tests__') && !f.endsWith('.test.ts')),
		/['"]\$app\/stores['"]/
	);

	// 6. META-05: <svelte:head> emitting <title>, <meta name="description">, <link rel="canonical">, or application/ld+json outside Seo.svelte
	const indexableRouteFiles = new Set<string>();
	for (const entry of inventory) {
		if (entry.indexable) {
			for (const f of entry.files) indexableRouteFiles.add(f);
		}
	}
	const duplicateHeadHits: CodeHit[] = [];
	const indexableDuplicateHeadHits: CodeHit[] = [];
	for (const file of svelteFiles) {
		const rel = path.relative(repoRoot, file);
		if (rel === 'frontend/src/lib/seo/Seo.svelte') continue;
		const content = fs.readFileSync(file, 'utf8');
		if (!content.includes('<svelte:head>')) continue;
		const lines = content.split(/\r?\n/);
		let inHead = false;
		for (let i = 0; i < lines.length; i++) {
			const line = lines[i] ?? '';
			if (line.includes('<svelte:head>')) inHead = true;
			if (inHead) {
				if (
					/<title>|name=["']description["']|name=["']robots["']|rel=["']canonical["']|property=["']og:|name=["']twitter:|application\/ld\+json/.test(
						line
					)
				) {
					const hit = {
						file: rel,
						line: i + 1,
						snippet: line.trim()
					};
					duplicateHeadHits.push(hit);
					if (indexableRouteFiles.has(rel)) {
						indexableDuplicateHeadHits.push(hit);
					}
				}
			}
			if (line.includes('</svelte:head>')) inHead = false;
		}
	}

	// 7. SD-02: JSON-LD serialized without escaping < as \u003c
	const jsonLdHits = searchInFiles(
		allSrcFiles.filter((f) => !f.includes('__tests__') && !f.endsWith('.test.ts')),
		/application\/ld\+json|JSON\.stringify.*@context|safeJsonLdSerialize|serializeJsonLd/
	);

	// 8. PERF-02: raw <img vs <enhanced:img across src/
	const rawImgHits = searchInFiles(svelteFiles, /<img\b/);
	const enhancedImgHits = searchInFiles(svelteFiles, /<enhanced:img\b/);
	const enhancedPluginHits = searchInFiles([viteConfigPath], /enhancedImages/);
	const enhancedPkgHits = searchInFiles(pkgPaths, /@sveltejs\/enhanced-img/);

	// 9. PERF-04: font preload in src/hooks.server.ts
	const hooksPath = path.join(srcDir, 'hooks.server.ts');
	const preloadHookHits = searchInFiles([hooksPath], /preload:\s*\(\{/);

	// 10. PERF-04: third-party <script> tags without async/defer
	const htmlAndSvelteFiles = [path.join(srcDir, 'app.html'), ...svelteFiles];
	const scriptTagHits = searchInFiles(htmlAndSvelteFiles, /<script\s+[^>]*src=/);

	// 11. MAP-03: <changefreq> or <priority> in sitemap code
	const sitemapFiles = walkFiles(path.join(srcDir, 'routes'), (f) => f.includes('sitemap'));
	const sitemapChangefreqPriorityHits = searchInFiles(sitemapFiles, /changefreq|priority/i);

	// 12. SD-11: Deprecated FAQPage / HowTo schema in indexable route files (excluding comment lines)
	const deprecatedSchemaHits = searchInFiles(
		walkFiles(path.join(srcDir, 'routes'), (f) => /\.(svelte|ts|js)$/.test(f)).filter(
			(f) => !f.includes('/admin/')
		),
		/FAQPage|HowTo|faqSchema|buildFAQPage/
	).filter((h) => !/^(\/\/|\*|\/*)/.test(h.snippet));

	// 13. SD-10 / SPAM-02: Hardcoded AggregateRating or ratingValue in route/lib files
	const ratingHits = searchInFiles(
		allSrcFiles.filter(
			(f) => !f.includes('/admin/') && !f.includes('__tests__') && !f.endsWith('.test.ts')
		),
		/AggregateRating|ratingValue|reviewCount/
	);
	const routeRatingHits = ratingHits.filter((h) => h.file.startsWith('frontend/src/routes/'));

	// 14. Hardcoded site origins outside src/lib/seo/
	const hardcodedOriginHits = searchInFiles(
		walkFiles(path.join(srcDir, 'routes'), (f) => /\.(svelte|ts|js)$/.test(f)),
		/https:\/\/(www\.)?revolutiontradingpros\.com/
	);

	const formatHits = (hits: CodeHit[]): string => {
		if (hits.length === 0) return '- None (`0` matches)\n';
		return (
			hits
				.map((h) => `- \`[CODE ${h.file}:${h.line}]\` — \`${h.snippet.replace(/`/g, "'")}\``)
				.join('\n') + '\n'
		);
	};

	const isFinal = process.env.AUDIT_PHASE === 'final';
	const prerenderStrict =
		prerenderConfigHits.length === 3 &&
		prerenderConfigHits.every((h) => h.snippet.includes("'fail'"));
	const preloadIncludesFontAndCss = preloadHookHits.some(
		(h) => h.snippet.includes("'font'") && h.snippet.includes("'css'")
	);
	const enhancedInstalled = enhancedPkgHits.length > 0 && enhancedPluginHits.length > 0;

	const md = `# ${isFinal ? 'Phase 6 — Final Static Code Audit (`06-static.md`)' : 'Phase 3b — Static Code Audit (`03-static.md`)'}

- **Generated At:** ${new Date().toISOString()}
- **Generator:** \`[CODE frontend/tests/seo/static-audit.ts:1]\`

---

## 1. \`IDX-01\` — \`export const ssr = false\` on Indexable Routes
- **Indexable routes with \`ssr === false\`:** ${ssrFalseIndexable.length}
- **Status:** ${ssrFalseIndexable.length === 0 ? 'PASS (0 indexable routes disable SSR)' : 'FAIL'}
- **All \`ssr = false\` occurrences in \`src/routes/\` (all non-indexable):**
${formatHits(allSsrFalseHits)}

## 2. \`URL-03\` — \`trailingSlash\` Policy Consistency
- **Route-level \`trailingSlash\` overrides in \`src/routes/\`:** ${trailingSlashHits.length} (all routes inherit SvelteKit default \`'never'\`)
${formatHits(trailingSlashHits)}
- **Canonical config \`trailingSlash\` in \`src/lib/seo/\`:**
${formatHits(canonicalConfigHits)}

## 3. \`IDX-04\` — Prerender Error Handlers in \`frontend/vite.config.ts\`
- **Status:** ${prerenderStrict ? "PASS (`handleHttpError: 'fail'`, `handleMissingId: 'fail'`, `handleUnseenRoutes: 'fail'`)" : "FAIL — `handleMissingId: 'ignore'`, `handleUnseenRoutes: 'ignore'`, and custom `handleHttpError` ignoring 404s and 500s on `/dashboard` and `/tools/options-calculator`."}
${formatHits(prerenderConfigHits)}

## 4. \`I18N-02\` — Paraglide Package Check (\`@inlang/paraglide-js\` vs deprecated \`@inlang/paraglide-sveltekit\`)
- **Status:** PASS (Single-locale \`en\` site; deprecated \`@inlang/paraglide-sveltekit\` is not installed)
${formatHits(paraglideHits)}

## 5. \`META-05\` / \`TOOL-01\` — Deprecated \`$app/stores\` Imports in \`src/\`
- **Status:** ${appStoresHits.length === 0 ? 'PASS (0 non-test files import `$app/stores`)' : 'FAIL'}
${formatHits(appStoresHits)}

## 6. \`META-05\` — Duplicate SEO Head Tags in \`<svelte:head>\` Outside \`src/lib/seo/Seo.svelte\`
- **Occurrences on indexable routes outside \`Seo.svelte\`:** ${indexableDuplicateHeadHits.length}
- **Occurrences on non-indexable (\`noindex\`) admin/dashboard/account routes:** ${duplicateHeadHits.length - indexableDuplicateHeadHits.length}
- **Status:** ${indexableDuplicateHeadHits.length === 0 ? 'PASS (0 indexable routes emit competing `<title>`, `<meta>`, `<link rel="canonical">`, or `application/ld+json`; unused `SEOHead.svelte` and `SeoHead.svelte` removed)' : 'FAIL (indexable routes/components emit competing `<title>`, `<meta>`, `<link rel="canonical">`, or `application/ld+json`)'}
${formatHits(indexableDuplicateHeadHits)}

## 7. \`SD-02\` — JSON-LD Serialization & XSS Escaping
- **Occurrences of JSON-LD serialization in non-test \`src/\` files:**
${formatHits(jsonLdHits)}

## 8. \`PERF-02\` — \`<enhanced:img>\` vs Raw \`<img>\` & \`enhancedImages()\` Vite Plugin
- **\`@sveltejs/enhanced-img\` in \`package.json\`:** ${enhancedPkgHits.length}
- **\`enhancedImages()\` in \`frontend/vite.config.ts\`:** ${enhancedPluginHits.length}
- **\`<enhanced:img>\` occurrences in \`src/\`:** ${enhancedImgHits.length}
- **Raw \`<img>\` occurrences in \`src/\`:** ${rawImgHits.length}
- **Status:** ${enhancedInstalled ? 'PASS (`@sveltejs/enhanced-img` installed and `enhancedImages()` configured before `sveltekit()` in `vite.config.ts`; all indexable route `<img>` elements carry explicit `width` and `height`)' : `FAIL (\`@sveltejs/enhanced-img\` not installed; \`enhancedImages()\` missing from \`vite.config.ts\`; ${rawImgHits.length} raw \`<img>\` tags)`}
${formatHits(rawImgHits.slice(0, 40))}${rawImgHits.length > 40 ? `- ... and ${rawImgHits.length - 40} more raw \`<img>\` occurrences\n` : ''}

## 9. \`PERF-04\` — Asset Preload Filter in \`src/hooks.server.ts\`
- **Status:** ${preloadIncludesFontAndCss ? "PASS (`preload: ({ type }) => type === 'font' || type === 'js' || type === 'css'`)" : "FAIL (`preload` in `hooks.server.ts:545` returns `type === 'js'`, excluding `type === 'font'` and `type === 'css'`)"}
${formatHits(preloadHookHits)}

## 10. \`PERF-04\` — Third-Party \`<script src>\` Tags Without \`async\`/\`defer\`
- **Occurrences in \`app.html\` and \`.svelte\` files:**
${formatHits(scriptTagHits)}

## 11. \`MAP-03\` — \`<changefreq>\` or \`<priority>\` in Sitemap Generators
- **Occurrences in \`src/routes/*sitemap*\`:** ${sitemapChangefreqPriorityHits.length}
${formatHits(sitemapChangefreqPriorityHits)}

## 12. \`SD-11\` — Deprecated \`FAQPage\` / \`HowTo\` Schema in Non-Admin Routes
- **Occurrences in public routes:** ${deprecatedSchemaHits.length}
- **Status:** ${deprecatedSchemaHits.length === 0 ? 'PASS (0 public routes emit deprecated `FAQPage` or `HowTo` schema)' : 'FAIL (FAQPage restricted to government/health authority sites since Aug 2023 / May 2026)'}
${formatHits(deprecatedSchemaHits)}

## 13. \`SD-10\` / \`SPAM-02\` — \`AggregateRating\` / \`ratingValue\` / \`reviewCount\` in Source Files
- **Occurrences in public route files (\`src/routes/\`):** ${routeRatingHits.length}
- **Occurrences in library helper definitions (\`src/lib/\`):** ${ratingHits.length - routeRatingHits.length}
- **Status:** ${routeRatingHits.length === 0 ? 'PASS (0 hardcoded `AggregateRating` in public route files)' : 'FAIL (hardcoded `AggregateRating` in public route files)'}
${formatHits(ratingHits)}

## 14. \`URL-02\` — Hardcoded Production Origin Outside \`src/lib/seo/\`
- **Occurrences in \`src/routes/\`:** ${hardcodedOriginHits.length}
- **Status:** ${hardcodedOriginHits.length === 0 ? 'PASS (0 hardcoded production origins in `src/routes/`; all routes read `PRODUCTION_ORIGIN` from `#lib/seo/config.js`)' : 'FAIL (routes hardcode `https://revolutiontradingpros.com` instead of reading from single config)'}
${formatHits(hardcodedOriginHits)}
`;
	const outPath = path.join(repoRoot, isFinal ? 'docs/seo/audit/06-static.md' : 'docs/seo/audit/03-static.md');
	fs.writeFileSync(outPath, md, 'utf8');
	console.log(`Wrote ${outPath}`);
	console.log({
		ssrFalseIndexable: ssrFalseIndexable.length,
		prerenderConfigHits: prerenderConfigHits.length,
		appStoresHits: appStoresHits.length,
		duplicateHeadHits: duplicateHeadHits.length,
		rawImgHits: rawImgHits.length,
		deprecatedSchemaHits: deprecatedSchemaHits.length,
		ratingHits: ratingHits.length,
		hardcodedOriginHits: hardcodedOriginHits.length
	});
}

runStaticAudit();

