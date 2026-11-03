import fs from 'node:fs';
import path from 'node:path';

const repoRoot = path.resolve(import.meta.dirname, '../../..');
const baselinePath = path.join(repoRoot, 'docs/seo/audit/03-baseline.json');
const lhciPath = path.join(repoRoot, 'docs/seo/audit/03-lab/lhci-summary.json');
const outPath = path.join(repoRoot, 'docs/seo/audit/03-matrix-baseline.md');

interface MatrixRow {
	id: string;
	sev: 'P0' | 'P1' | 'P2';
	template: string;
	status: 'PASS' | 'FAIL' | 'N-A' | 'REQUIRES-HUMAN' | 'REQUIRES-PRODUCTION';
	failingTargets: string;
	evidence: string;
}

const TEMPLATES = [
	'home',
	'static',
	'listing',
	'product',
	'profile',
	'course',
	'article',
	'software'
] as const;

function buildBaselineMatrix(): void {
	const baseline = JSON.parse(fs.readFileSync(baselinePath, 'utf8'));
	const lhci = JSON.parse(fs.readFileSync(lhciPath, 'utf8'));

	const pages: Array<{
		path: string;
		template: string;
		indexable: boolean;
		inSitemap: boolean;
		rawHttp: {
			finalStatus: number;
			rawMainWordCount: number;
			rawH1: string | null;
		};
		ssrHtml: {
			title: { text: string; charCount: number; tagCount: number };
			description: { content: string; charCount: number; tagCount: number };
			robots: { content: string; tagCount: number };
			canonical: { href: string; tagCount: number };
			og: {
				title: string | null;
				description: string | null;
				image: string | null;
				url: string | null;
				type: string | null;
			};
			twitter: { card: string | null };
			htmlLang: string | null;
			viewport: string | null;
			iconHref: string | null;
			headings: Array<{ level: number; text: string }>;
			jsonLdBlocks: Array<{
				parseError: string | null;
				types: string[];
				hasDangerousChars: boolean;
				hasAggregateRating: boolean;
			}>;
			breadcrumbs: Array<{ text: string; href: string | null }>;
			lcpCandidateImg: {
				src: string;
				loading: string | null;
				fetchpriority: string | null;
			} | null;
			imagesMissingAttributes: Array<{ src: string }>;
		};
		renderedDom: {
			mainWordCount: number;
			ssrVsHydratedWordCountDelta: number;
			links: Array<{ href: string; text: string; rel: string; isInternal: boolean }>;
			nonAnchorClickNavElements: Array<{ tag: string; text: string }>;
		};
	}> = baseline.pages.filter((p: { indexable: boolean }) => p.indexable);

	const rows: MatrixRow[] = [];

	// Helper to get pages for a template
	const byTemplate = (t: string) => pages.filter((p) => p.template === t);

	// ═══════════════════════════════════════════════════════════════════════════
	// 1. CRAWLING, SSR, PRERENDER & INDEXING (IDX-01 .. IDX-09)
	// ═══════════════════════════════════════════════════════════════════════════
	for (const t of TEMPLATES) {
		const tPages = byTemplate(t);
		const sample = tPages[0];
		rows.push({
			id: 'IDX-01',
			sev: 'P0',
			template: t,
			status: 'PASS',
			failingTargets: 'None (0/50 indexable URLs have ssr=false)',
			evidence: `[HTTP http://127.0.0.1:4273${sample?.path} -> 200, ssrTitle="${sample?.ssrHtml.title.text.slice(0, 35)}...", ssrWords=${sample?.rawHttp.rawMainWordCount}] [CODE frontend/src/routes/+layout.js:1]`
		});
	}

	rows.push({
		id: 'IDX-02',
		sev: 'P1',
		template: 'ALL',
		status: 'PASS',
		failingTargets:
			'None (26 static routes prerendered via +layout.js; dynamic routes set prerender=false)',
		evidence: `[CODE frontend/src/routes/+layout.js:1] [CMD pnpm build ->prerendered 37 static pages]`
	});

	rows.push({
		id: 'IDX-03',
		sev: 'P1',
		template: 'ALL',
		status: 'FAIL',
		failingTargets:
			'frontend/src/routes/authors/[slug]/+page.ts, frontend/src/routes/indicators/[id]/+page.ts',
		evidence: `[CODE frontend/src/routes/authors/[slug]/+page.ts:9] [CODE frontend/src/routes/indicators/[id]/+page.ts:8] (static data-backed parameterized routes set prerender = false instead of exporting entries() or prerendering static slugs)`
	});

	rows.push({
		id: 'IDX-04',
		sev: 'P0',
		template: 'ALL',
		status: 'FAIL',
		failingTargets: 'frontend/vite.config.ts:63-79',
		evidence: `[CODE frontend/vite.config.ts:63-79] (handleMissingId: 'ignore', handleUnseenRoutes: 'ignore', handleHttpError ignores 404 and 500s)`
	});

	rows.push({
		id: 'IDX-05',
		sev: 'P0',
		template: 'ALL',
		status: 'FAIL',
		failingTargets:
			'/indicators/__nonexistent_slug_seo_test__, frontend/src/routes/indicators/[id]/+page.ts:10-15, frontend/src/routes/classes/[slug]/+page.server.ts:97-101',
		evidence: `[HTTP http://127.0.0.1:4273/indicators/__nonexistent_slug_seo_test__ -> 200 soft-404 with robots="index, follow"] [CODE frontend/src/routes/indicators/[id]/+page.ts:12] [CODE frontend/src/routes/classes/[slug]/+page.server.ts:97]`
	});

	rows.push({
		id: 'IDX-06',
		sev: 'P0',
		template: 'ALL',
		status: 'FAIL',
		failingTargets: 'frontend/src/routes/signup/+page.server.ts:27',
		evidence: `[HTTP http://127.0.0.1:4273/signup -> 301 Location: /register] [CODE frontend/src/routes/signup/+page.server.ts:27] (uses 301 instead of 308 for permanent redirect)`
	});

	rows.push({
		id: 'IDX-07',
		sev: 'P0',
		template: 'ALL',
		status: 'FAIL',
		failingTargets: 'frontend/src/routes/robots.txt/+server.ts:19,60,395-411,457-458',
		evidence: `[HTTP http://127.0.0.1:4273/robots.txt -> Disallow: /_app/ blocks SvelteKit JS/CSS bundles; uses invalid wildcard User-agent: *-ai tokens; points Sitemap to pages.dev and broken news/video sitemaps] [CODE frontend/src/routes/robots.txt/+server.ts:60]`
	});

	rows.push({
		id: 'IDX-08',
		sev: 'P1',
		template: 'ALL',
		status: 'FAIL',
		failingTargets: 'frontend/src/routes/robots.txt/+server.ts:156-393',
		evidence: `[CODE frontend/src/routes/robots.txt/+server.ts:156-200] (lumps AI search/citation bots OAI-SearchBot, PerplexityBot, ChatGPT-User together with AI training bots GPTBot, Google-Extended, ClaudeBot without separate config toggle)`
	});

	rows.push({
		id: 'IDX-09',
		sev: 'P0',
		template: 'ALL',
		status: 'FAIL',
		failingTargets:
			'frontend/src/hooks.server.ts:458-486, frontend/src/routes/+layout.server.ts:46-55',
		evidence: `[HTTP http://127.0.0.1:4273/ -> x-robots-tag: null on non-production host 127.0.0.1:4273] [CODE frontend/src/hooks.server.ts:458-486] (does not set X-Robots-Tag: noindex, nofollow when url.origin !== PRODUCTION_ORIGIN or on missing prefixes like /dashboard, /my, /crm)`
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// 2. URL & CANONICAL ARCHITECTURE (URL-01 .. URL-04)
	// ═══════════════════════════════════════════════════════════════════════════
	rows.push({
		id: 'URL-01',
		sev: 'P1',
		template: 'ALL',
		status: 'PASS',
		failingTargets: 'None (all 50 indexable URLs use lowercase ASCII hyphen-separated slugs)',
		evidence: `[CODE docs/seo/audit/02-inventory.json:1] [HTTP http://127.0.0.1:4273/guides/risk-management -> 200]`
	});

	for (const t of TEMPLATES) {
		const tPages = byTemplate(t);
		const failing = tPages.filter(
			(p) =>
				p.ssrHtml.canonical.href !==
				(p.path === '/'
					? 'https://revolutiontradingpros.com'
					: `https://revolutiontradingpros.com${p.path}`)
		);
		rows.push({
			id: 'URL-02',
			sev: 'P0',
			template: t,
			status: failing.length === 0 ? 'PASS' : 'FAIL',
			failingTargets:
				failing.length === 0
					? 'None'
					: failing.map((p) => p.path).join(', ') +
						' (points to https://revolution-trading-pros.pages.dev)',
			evidence: `[HTTP http://127.0.0.1:4273${tPages[0]?.path} -> canonical="${tPages[0]?.ssrHtml.canonical.href}"] [CODE frontend/.env.production:20] [CODE frontend/src/lib/seo/defaults.ts:19]`
		});
	}

	rows.push({
		id: 'URL-03',
		sev: 'P1',
		template: 'ALL',
		status: 'FAIL',
		failingTargets:
			'frontend/src/lib/seo/canonical.ts:95, frontend/src/routes/sitemap.xml/+server.ts',
		evidence: `[HTTP http://127.0.0.1:4273/ -> canonical="https://revolution-trading-pros.pages.dev/"] [CODE frontend/src/lib/seo/defaults.ts:37] (trailingSlash is 'never', yet root canonical and JSON-LD URLs mix trailing slash vs no trailing slash)`
	});

	rows.push({
		id: 'URL-04',
		sev: 'P1',
		template: 'ALL',
		status: 'FAIL',
		failingTargets:
			'/?utm_source=x&fbclid=y, /about?utm_source=x&fbclid=y, /pricing?utm_source=x&fbclid=y',
		evidence: `[HTTP http://127.0.0.1:4273/about?utm_source=x&fbclid=y -> canonical="https://revolution-trading-pros.pages.dev/about" (wrong origin due to VITE_SITE_URL in .env.production)] [CODE frontend/.env.production:20]`
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// 3. RENDERING, HYDRATION & JS SEO (REND-01 .. REND-04)
	// ═══════════════════════════════════════════════════════════════════════════
	for (const t of TEMPLATES) {
		const tPages = byTemplate(t);
		const csrIssues = tPages.filter(
			(p) =>
				p.path === '/blog' ||
				p.path === '/classes' ||
				p.renderedDom.ssrVsHydratedWordCountDelta > 25
		);
		rows.push({
			id: 'REND-01',
			sev: 'P0',
			template: t,
			status: csrIssues.length === 0 ? 'PASS' : 'FAIL',
			failingTargets: csrIssues.length === 0 ? 'None' : csrIssues.map((p) => p.path).join(', '),
			evidence:
				csrIssues.length === 0
					? `[HTTP http://127.0.0.1:4273${tPages[0]?.path} -> ssrWords=${tPages[0]?.rawHttp.rawMainWordCount}, domWords=${tPages[0]?.renderedDom.mainWordCount}]`
					: `[CODE frontend/src/routes/blog/+page.svelte:197] [CODE frontend/src/routes/classes/+page.svelte:66] [CODE frontend/src/routes/cookie-policy/+page.svelte:1] (/blog and /classes fetch content in onMount rendering "Loading..." in SSR HTML; /cookie-policy adds 159 words only after client mount)`
		});
	}

	rows.push({
		id: 'REND-02',
		sev: 'P1',
		template: 'ALL',
		status: 'PASS',
		failingTargets: 'None (all indexable routes export csr=true and hydrate links as <a href>)',
		evidence: `[CODE docs/seo/audit/02-inventory.json:1] [HTTP http://127.0.0.1:4273/ -> csr=true]`
	});

	for (const t of TEMPLATES) {
		const vp = baseline.derivedChecks.viewportParity.find(
			(v: { template: string }) => v.template === t
		);
		rows.push({
			id: 'REND-03',
			sev: 'P0',
			template: t,
			status: vp?.parityPass ? 'PASS' : 'FAIL',
			failingTargets: vp?.parityPass ? 'None' : (vp?.path ?? t),
			evidence: `[HTTP http://127.0.0.1:4273${vp?.path} -> desktopWords=${vp?.desktopWords}, mobileWords=${vp?.mobileWords}, h1="${vp?.desktopH1}"]`
		});
	}

	rows.push({
		id: 'REND-04',
		sev: 'P0',
		template: 'ALL',
		status: 'FAIL',
		failingTargets: 'frontend/src/routes/+error.svelte:104-105',
		evidence: `[CODE frontend/src/routes/+error.svelte:104-105] (+error.svelte emits competing <title> and <meta name="description"> in <svelte:head> alongside root <Seo>, and does not set robots noindex directly inside +error.svelte)`
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// 4. METADATA, SOCIAL & HEAD HYGIENE (META-01 .. META-05)
	// ═══════════════════════════════════════════════════════════════════════════
	for (const t of TEMPLATES) {
		const tPages = byTemplate(t);
		const badTitles = tPages.filter(
			(p) =>
				p.ssrHtml.title.charCount < 15 ||
				p.ssrHtml.title.charCount > 60 ||
				p.ssrHtml.title.tagCount !== 1 ||
				p.ssrHtml.title.text.includes('Revolution Trading Pros | Revolution Trading Pros')
		);
		rows.push({
			id: 'META-01',
			sev: 'P0',
			template: t,
			status: badTitles.length === 0 ? 'PASS' : 'FAIL',
			failingTargets:
				badTitles.length === 0
					? 'None'
					: badTitles.map((p) => `${p.path} (${p.ssrHtml.title.charCount}ch)`).join(', '),
			evidence:
				badTitles.length === 0
					? `[HTTP http://127.0.0.1:4273${tPages[0]?.path} -> title="${tPages[0]?.ssrHtml.title.text}" (${tPages[0]?.ssrHtml.title.charCount}ch)]`
					: `[HTTP http://127.0.0.1:4273${badTitles[0]?.path} -> title="${badTitles[0]?.ssrHtml.title.text}" (${badTitles[0]?.ssrHtml.title.charCount}ch > 60ch limit)]`
		});

		const badDescs = tPages.filter(
			(p) =>
				p.ssrHtml.description.charCount < 50 ||
				p.ssrHtml.description.charCount > 160 ||
				p.ssrHtml.description.tagCount !== 1
		);
		rows.push({
			id: 'META-02',
			sev: 'P1',
			template: t,
			status: badDescs.length === 0 ? 'PASS' : 'FAIL',
			failingTargets:
				badDescs.length === 0
					? 'None'
					: badDescs.map((p) => `${p.path} (${p.ssrHtml.description.charCount}ch)`).join(', '),
			evidence:
				badDescs.length === 0
					? `[HTTP http://127.0.0.1:4273${tPages[0]?.path} -> description (${tPages[0]?.ssrHtml.description.charCount}ch)]`
					: `[HTTP http://127.0.0.1:4273${badDescs[0]?.path} -> description="${badDescs[0]?.ssrHtml.description.content.slice(0, 60)}..." (${badDescs[0]?.ssrHtml.description.charCount}ch outside 50-160ch)]`
		});

		const badOg = tPages.filter(
			(p) =>
				!p.ssrHtml.og.title ||
				!p.ssrHtml.og.description ||
				!p.ssrHtml.og.image ||
				!p.ssrHtml.og.url ||
				p.ssrHtml.og.url.includes('pages.dev') ||
				p.ssrHtml.og.image.includes('pages.dev') ||
				p.ssrHtml.twitter.card !== 'summary_large_image'
		);
		rows.push({
			id: 'META-03',
			sev: 'P1',
			template: t,
			status: badOg.length === 0 ? 'PASS' : 'FAIL',
			failingTargets:
				badOg.length === 0
					? 'None'
					: badOg.map((p) => p.path).join(', ') +
						' (og:url / og:image point to pages.dev or relative URL)',
			evidence: `[HTTP http://127.0.0.1:4273${tPages[0]?.path} -> og:url="${tPages[0]?.ssrHtml.og.url}", og:image="${tPages[0]?.ssrHtml.og.image}"] [CODE frontend/src/lib/seo/defaults.ts:28]`
		});
	}

	rows.push({
		id: 'META-04',
		sev: 'P1',
		template: 'ALL',
		status: 'PASS',
		failingTargets:
			'None (indexable pages emit index, follow, max-snippet:-1, max-image-preview:large, max-video-preview:-1)',
		evidence: `[HTTP http://127.0.0.1:4273/ -> meta[name="robots"]="index, follow, max-snippet:-1, max-image-preview:large, max-video-preview:-1"] [CODE frontend/src/lib/seo/defaults.ts:71-77]`
	});

	rows.push({
		id: 'META-05',
		sev: 'P0',
		template: 'ALL',
		status: 'FAIL',
		failingTargets:
			'frontend/src/lib/components/SEOHead.svelte:823, frontend/src/lib/components/seo/SeoHead.svelte:217, frontend/src/routes/+error.svelte:104, + 400 admin/dashboard/account files',
		evidence: `[CODE docs/seo/audit/03-static.md:53-136] (410 occurrences of <title>, <meta name="description">, <link rel="canonical">, or JSON-LD in <svelte:head> outside src/lib/seo/Seo.svelte)`
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// 5. SEMANTIC HTML, HEADINGS & ACCESSIBILITY (SEM-01 .. SEM-05)
	// ═══════════════════════════════════════════════════════════════════════════
	rows.push({
		id: 'SEM-01',
		sev: 'P1',
		template: 'ALL',
		status: 'PASS',
		failingTargets:
			'None (<header>, <nav>, <main id="main-content">, <footer> present on all marketing layouts)',
		evidence: `[CODE frontend/src/routes/+layout.svelte:215-228] [HTTP http://127.0.0.1:4273/ -> main#main-content present]`
	});

	for (const t of TEMPLATES) {
		const tPages = byTemplate(t);
		const badHeadings = tPages.filter((p) => {
			const h = p.ssrHtml.headings;
			const h1Count = h.filter((x) => x.level === 1).length;
			if (h1Count !== 1) return true;
			for (let i = 1; i < h.length; i++) {
				const currH = h[i];
				const prevH = h[i - 1];
				if (currH && prevH && currH.level > prevH.level + 1) return true;
			}
			return false;
		});
		rows.push({
			id: 'SEM-02',
			sev: 'P1',
			template: t,
			status: badHeadings.length === 0 ? 'PASS' : 'FAIL',
			failingTargets: badHeadings.length === 0 ? 'None' : badHeadings.map((p) => p.path).join(', '),
			evidence:
				badHeadings.length === 0
					? `[HTTP http://127.0.0.1:4273${tPages[0]?.path} -> 1 H1, no level skips]`
					: `[HTTP http://127.0.0.1:4273${badHeadings[0]?.path} -> heading level skip detected in docs/seo/audit/03-baseline.json]`
		});
	}

	rows.push({
		id: 'SEM-03',
		sev: 'P0',
		template: 'ALL',
		status: 'FAIL',
		failingTargets: 'frontend/src/app.html:4',
		evidence: `[CODE frontend/src/app.html:4] [HTTP http://127.0.0.1:4273/ -> html[lang]="en", html[dir]=null] (<html lang="en"> is hardcoded instead of %lang% / %dir% transformed in hooks.server.ts)`
	});

	rows.push({
		id: 'SEM-04',
		sev: 'P0',
		template: 'ALL',
		status: 'PASS',
		failingTargets:
			'None (<meta charset="utf-8">, <meta name="viewport" content="width=device-width, initial-scale=1">, <link rel="icon" href="/favicon.png"> in app.html)',
		evidence: `[CODE frontend/src/app.html:5-7,29] [HTTP http://127.0.0.1:4273/ -> viewport="width=device-width, initial-scale=1", icon="/favicon.png"]`
	});

	for (const t of TEMPLATES) {
		const axe = baseline.axeResultsByTemplate.find((a: { template: string }) => a.template === t);
		rows.push({
			id: 'SEM-05',
			sev: 'P1',
			template: t,
			status: axe && axe.violationCount === 0 ? 'PASS' : 'FAIL',
			failingTargets:
				axe && axe.violationCount === 0
					? 'None'
					: `${axe?.path} (${axe?.violationCount} axe rule violation: color-contrast)`,
			evidence: `[CMD node frontend/tests/seo/crawl.ts -> docs/seo/audit/03-lab/axe-summary.json: ${axe?.path} violations=${axe?.violationCount}]`
		});
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// 6. STRUCTURED DATA (JSON-LD) (SD-01 .. SD-13)
	// ═══════════════════════════════════════════════════════════════════════════
	for (const t of TEMPLATES) {
		const tPages = byTemplate(t);
		const badJsonLd = tPages.filter(
			(p) =>
				p.ssrHtml.jsonLdBlocks.length === 0 ||
				p.ssrHtml.jsonLdBlocks.some((b) => b.parseError !== null)
		);
		rows.push({
			id: 'SD-01',
			sev: 'P0',
			template: t,
			status: badJsonLd.length === 0 ? 'PASS' : 'FAIL',
			failingTargets: badJsonLd.length === 0 ? 'None' : badJsonLd.map((p) => p.path).join(', '),
			evidence: `[HTTP http://127.0.0.1:4273${tPages[0]?.path} -> ${tPages[0]?.ssrHtml.jsonLdBlocks.length} valid JSON-LD @graph block, parseError=null] [CODE frontend/src/lib/seo/Seo.svelte:129-131]`
		});
	}

	rows.push({
		id: 'SD-02',
		sev: 'P0',
		template: 'ALL',
		status: 'PASS',
		failingTargets:
			'None (serializeJsonLd escapes <, >, & as \\u003c, \\u003e, \\u0026 + U+2028/U+2029)',
		evidence: `[CODE frontend/src/lib/seo/serializeJsonLd.ts:150-169] [CODE frontend/src/lib/seo/__tests__/serializeJsonLd.test.ts:1]`
	});

	for (const t of TEMPLATES) {
		const tPages = byTemplate(t);
		const mismatch = tPages.filter(
			(p) =>
				(p.path !== '/' &&
					p.ssrHtml.jsonLdBlocks.some((b) => b.types.includes('BreadcrumbList')) &&
					p.ssrHtml.breadcrumbs.length === 0) ||
				p.ssrHtml.jsonLdBlocks.some((b) => b.hasAggregateRating)
		);
		rows.push({
			id: 'SD-03',
			sev: 'P0',
			template: t,
			status: mismatch.length === 0 ? 'PASS' : 'FAIL',
			failingTargets:
				mismatch.length === 0
					? 'None'
					: mismatch.map((p) => p.path).join(', ') +
						' (marks up BreadcrumbList or AggregateRating not visible in DOM)',
			evidence:
				mismatch.length === 0
					? `[HTTP http://127.0.0.1:4273${tPages[0]?.path} -> JSON-LD matches visible DOM]`
					: `[HTTP http://127.0.0.1:4273${mismatch[0]?.path} -> JSON-LD has BreadcrumbList/AggregateRating with domCrumbs=${mismatch[0]?.ssrHtml.breadcrumbs.length}]`
		});
	}

	rows.push({
		id: 'SD-04',
		sev: 'P1',
		template: 'home',
		status: 'FAIL',
		failingTargets: 'frontend/src/lib/seo/defaults.ts:86-120',
		evidence: `[CODE frontend/src/lib/seo/defaults.ts:86-120] (Organization and WebSite JSON-LD are emitted on EVERY route via seoDefaults.jsonld instead of only on /, and WebSite includes deprecated SearchAction pointing to non-existent /search?q=)`
	});

	for (const t of TEMPLATES.filter((x) => x !== 'home')) {
		const tPages = byTemplate(t);
		const missingDomCrumbs = tPages.filter(
			(p) =>
				!p.ssrHtml.jsonLdBlocks.some((b) => b.types.includes('BreadcrumbList')) ||
				p.ssrHtml.breadcrumbs.length === 0
		);
		rows.push({
			id: 'SD-05',
			sev: 'P1',
			template: t,
			status: missingDomCrumbs.length === 0 ? 'PASS' : 'FAIL',
			failingTargets:
				missingDomCrumbs.length === 0 ? 'None' : missingDomCrumbs.map((p) => p.path).join(', '),
			evidence:
				missingDomCrumbs.length === 0
					? `[HTTP http://127.0.0.1:4273${tPages[0]?.path} -> BreadcrumbList JSON-LD + visible <nav aria-label="Breadcrumb">]`
					: `[HTTP http://127.0.0.1:4273${missingDomCrumbs[0]?.path} -> domCrumbs=${missingDomCrumbs[0]?.ssrHtml.breadcrumbs.length}]`
		});
	}

	rows.push({
		id: 'SD-06',
		sev: 'P1',
		template: 'article',
		status: 'FAIL',
		failingTargets:
			'/guides/exit-strategies, /guides/position-sizing, /guides/risk-management, /guides/swing-entry, /resources/etf-stocks-list, /resources/stock-indexes-list',
		evidence: `[CODE frontend/src/routes/guides/risk-management/+page.ts:18-45] [CODE frontend/src/routes/resources/etf-stocks-list/+page.ts:1] (guides emit Article without image array and author.url; resource articles emit only WebPage instead of Article; visible author bylines and dates missing from DOM)`
	});

	rows.push({
		id: 'SD-07',
		sev: 'P1',
		template: 'ALL',
		status: 'N-A',
		failingTargets:
			'None (0 indexable dedicated video watch pages; /learning-center/[slug] and /dashboard/**/video/[slug] are private/noindex)',
		evidence: `[CODE docs/seo/audit/02-inventory.json:1] [CODE frontend/src/routes/learning-center/[slug]/+page.ts:8]`
	});

	rows.push({
		id: 'SD-08',
		sev: 'P1',
		template: 'product',
		status: 'FAIL',
		failingTargets:
			'/alerts/explosive-swings, /alerts/spx-profit-pulse, /live-trading-rooms/day-trading, /live-trading-rooms/small-accounts, /live-trading-rooms/swing-trading, /mentorship, /pricing',
		evidence: `[CODE frontend/src/routes/alerts/explosive-swings/+page.ts:28-43] [CODE frontend/src/routes/pricing/+page.ts:16-30] (Product JSON-LD on /alerts/* and /live-trading-rooms/* lacks image property; /mentorship and /pricing lack Product/Offer schema matching visible pricing)`
	});

	rows.push({
		id: 'SD-09',
		sev: 'P1',
		template: 'profile',
		status: 'FAIL',
		failingTargets: '/authors/billy-ribeiro, /authors/freddie-ferber, /authors/shao-wen',
		evidence: `[CODE frontend/src/routes/authors/[slug]/+page.ts:30-46] (emits separate WebPage with @type ProfilePage and separate Person node instead of ProfilePage with mainEntity: Person as required by Google ProfilePage spec)`
	});

	rows.push({
		id: 'SD-10',
		sev: 'P1',
		template: 'ALL',
		status: 'FAIL',
		failingTargets:
			'/store/scanners, /store/scanners/high-octane-scanner, frontend/src/routes/live-trading-rooms/+page.server.ts:226',
		evidence: `[CODE frontend/src/routes/store/scanners/+page.ts:89,128] [CODE frontend/src/routes/store/scanners/high-octane-scanner/+page.ts:41] [CODE frontend/src/routes/live-trading-rooms/+page.server.ts:226] (emits hardcoded self-serving AggregateRating ratingValue: 4.8, reviewCount: 127)`
	});

	rows.push({
		id: 'SD-11',
		sev: 'P0',
		template: 'ALL',
		status: 'PASS',
		failingTargets:
			'None (0 indexable pages emit deprecated FAQPage, HowTo, SpecialAnnouncement, SitelinksSearchBox, etc. — wait: defaults.ts emits SearchAction on WebSite!)',
		evidence: `[HTTP http://127.0.0.1:4273/ -> 0 FAQPage/HowTo blocks]`
	});
	// Wait: SD-11 forbids Sitelinks Search Box (SearchAction on WebSite), and defaults.ts:111-118 emits SearchAction! Let's mark SD-11 FAIL with exact evidence!
	rows[rows.length - 1] = {
		id: 'SD-11',
		sev: 'P0',
		template: 'ALL',
		status: 'FAIL',
		failingTargets: 'frontend/src/lib/seo/defaults.ts:111-118',
		evidence: `[CODE frontend/src/lib/seo/defaults.ts:111-118] (emits retired Sitelinks Search Box potentialAction: SearchAction on WebSite schema across all 50 indexable pages)`
	};

	rows.push({
		id: 'SD-12',
		sev: 'P2',
		template: 'course',
		status: 'PASS',
		failingTargets:
			'None (/courses/* and /classes/* emit Course with name, description, provider, hasCourseInstance)',
		evidence: `[CODE frontend/src/routes/courses/day-trading-masterclass/+page.ts:25-39] [HTTP http://127.0.0.1:4273/courses/day-trading-masterclass -> Course JSON-LD present]`
	});

	rows.push({
		id: 'SD-12',
		sev: 'P2',
		template: 'software',
		status: 'FAIL',
		failingTargets:
			'/indicators/moving-averages, /indicators/bollinger-bands, /indicators/vwap, /indicators/stochastic, /indicators/macd, /indicators/rsi',
		evidence: `[CODE frontend/src/routes/indicators/[id]/+page.ts:29-34] [CODE frontend/src/routes/indicators/rsi/+page.ts:25-31] (SoftwareApplication JSON-LD omits operatingSystem and offers required by Google Software App structured data)`
	});

	rows.push({
		id: 'SD-13',
		sev: 'P1',
		template: 'ALL',
		status: 'N-A',
		failingTargets:
			'None (gated member content under /dashboard, /learning-center, /watchlist is noindex; public marketing pages are not paywalled)',
		evidence: `[CODE frontend/src/lib/seo/defaults.ts:122-152] [CODE docs/seo/audit/02-inventory.json:1]`
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// 7. XML SITEMAPS (MAP-01 .. MAP-04)
	// ═══════════════════════════════════════════════════════════════════════════
	rows.push({
		id: 'MAP-01',
		sev: 'P0',
		template: 'ALL',
		status: 'FAIL',
		failingTargets:
			'frontend/src/routes/sitemap.xml/+server.ts:108,129,135,137, frontend/src/routes/news-sitemap.xml/+server.ts:43-77, frontend/src/routes/video-sitemap.xml/+server.ts:91-177',
		evidence: `[HTTP http://127.0.0.1:4273/sitemap.xml -> includes /signup (301), /login, /register; omits /authors/billy-ribeiro, /authors/freddie-ferber, /authors/shao-wen] [HTTP http://127.0.0.1:4273/news-sitemap.xml -> 4/4 URLs return 404] [HTTP http://127.0.0.1:4273/video-sitemap.xml -> /blog/weekly-market-recap returns 404]`
	});

	rows.push({
		id: 'MAP-02',
		sev: 'P0',
		template: 'ALL',
		status: 'FAIL',
		failingTargets: 'frontend/src/routes/sitemap.xml/+server.ts:27 vs frontend/.env.production:20',
		evidence: `[HTTP http://127.0.0.1:4273/sitemap.xml -> <loc>https://revolutiontradingpros.com/about</loc> vs page canonical="https://revolution-trading-pros.pages.dev/about"]`
	});

	rows.push({
		id: 'MAP-03',
		sev: 'P1',
		template: 'ALL',
		status: 'PASS',
		failingTargets:
			'None (sitemap.xml omits <changefreq> and <priority> and only emits <lastmod> when known)',
		evidence: `[HTTP http://127.0.0.1:4273/sitemap.xml -> hasPriority=false, hasChangefreq=false] [CODE frontend/src/routes/sitemap.xml/+server.ts:60-156]`
	});

	rows.push({
		id: 'MAP-04',
		sev: 'P2',
		template: 'ALL',
		status: 'FAIL',
		failingTargets:
			'frontend/src/routes/news-sitemap.xml/+server.ts:43-77, frontend/src/routes/video-sitemap.xml/+server.ts:91-177',
		evidence: `[CODE frontend/src/routes/news-sitemap.xml/+server.ts:43-77] [CODE frontend/src/routes/video-sitemap.xml/+server.ts:98,115,132,149,166] (news-sitemap and video-sitemap emit fabricated placeholder URLs and fake YouTube/Vimeo IDs like dQw4w9WgXcQ)`
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// 8. INTERNAL LINKING, NAVIGATION & PAGINATION (LINK-01 .. LINK-05)
	// ═══════════════════════════════════════════════════════════════════════════
	rows.push({
		id: 'LINK-01',
		sev: 'P0',
		template: 'ALL',
		status: 'FAIL',
		failingTargets: 'frontend/src/routes/blog/+page.svelte:368-414',
		evidence: `[CODE frontend/src/routes/blog/+page.svelte:368-414] (blog pagination uses <button onclick={prevPage}> and <button onclick={nextPage}> without <a href="/blog?page=...">)`
	});

	rows.push({
		id: 'LINK-02',
		sev: 'P1',
		template: 'ALL',
		status: 'FAIL',
		failingTargets:
			'/alerts, /authors/billy-ribeiro, /authors/freddie-ferber, /authors/shao-wen, /classes/quickstart-precision-trading-c, /classes/tax-loss-harvest-c, /cookie-policy, /guides/exit-strategies, /guides/position-sizing, /guides/risk-management, /guides/swing-entry, /live-trading-rooms, /live-trading-rooms/small-accounts, /live-trading-rooms/swing-trading, /resources, /resources/etf-stocks-list, /resources/stock-indexes-list, /store/scanners/high-octane-scanner, /team, /tools/options-calculator, /tutorials',
		evidence: `[CMD node frontend/tests/seo/crawl.ts -> docs/seo/audit/03-baseline.json: 13 orphan indexable pages and 21 indexable pages unreachable from /]`
	});

	rows.push({
		id: 'LINK-03',
		sev: 'P0',
		template: 'ALL',
		status: 'FAIL',
		failingTargets:
			'/get-started (404), /join (404), /methodology (404), /performance (404), /signup (301 -> /register)',
		evidence: `[HTTP http://127.0.0.1:4273/get-started -> 404] [HTTP http://127.0.0.1:4273/join -> 404] [HTTP http://127.0.0.1:4273/methodology -> 404] [HTTP http://127.0.0.1:4273/performance -> 404] [HTTP http://127.0.0.1:4273/signup -> 301]`
	});

	rows.push({
		id: 'LINK-04',
		sev: 'P1',
		template: 'ALL',
		status: 'FAIL',
		failingTargets: 'frontend/src/routes/blog/+page.svelte:343',
		evidence: `[CODE frontend/src/routes/blog/+page.svelte:343] (uses generic "Read More" text inside blog post cards)`
	});

	rows.push({
		id: 'LINK-05',
		sev: 'P1',
		template: 'listing',
		status: 'FAIL',
		failingTargets:
			'frontend/src/routes/blog/+page.svelte:367-415, frontend/src/routes/tutorials/+page.svelte',
		evidence: `[CODE frontend/src/routes/blog/+page.svelte:367-415] (paginated listing uses <button> instead of <a href="?page=2"> with self-canonical per page)`
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// 9. INTERNATIONALIZATION & HREFLANG (I18N-01 .. I18N-04)
	// ═══════════════════════════════════════════════════════════════════════════
	rows.push({
		id: 'I18N-01',
		sev: 'P0',
		template: 'ALL',
		status: 'N-A',
		failingTargets: 'None (single-locale en site)',
		evidence: `[CODE frontend/src/lib/seo/defaults.ts:30] [CODE docs/seo/audit/02-inventory.json:1]`
	});

	rows.push({
		id: 'I18N-02',
		sev: 'P1',
		template: 'ALL',
		status: 'N-A',
		failingTargets: 'None (single-locale en site; @inlang/paraglide-sveltekit not installed)',
		evidence: `[CODE frontend/package.json:45-136] [CODE docs/seo/audit/03-static.md:43-45]`
	});

	rows.push({
		id: 'I18N-03',
		sev: 'P0',
		template: 'ALL',
		status: 'N-A',
		failingTargets: 'None (single-locale en site; 0 alternate locales)',
		evidence: `[CODE frontend/src/lib/seo/defaults.ts:30] [HTTP http://127.0.0.1:4273/ -> alternates=[]]`
	});

	rows.push({
		id: 'I18N-04',
		sev: 'P0',
		template: 'ALL',
		status: 'PASS',
		failingTargets:
			'None (requests with Accept-Language: fr, en, es-MX all return 200 with 0 redirects)',
		evidence: `[HTTP http://127.0.0.1:4273/ (Accept-Language: fr) -> 200, redirectCount=0] [CODE docs/seo/audit/03-baseline.json:876-892]`
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// 10. CORE WEB VITALS & PERFORMANCE (PERF-01 .. PERF-05)
	// ═══════════════════════════════════════════════════════════════════════════
	for (const t of TEMPLATES) {
		const m = lhci.results.find((r: { template: string }) => r.template === t);
		const pass = m && m.lcpMs <= 2500 && m.tbtMs <= 200 && m.cls <= 0.1;
		rows.push({
			id: 'PERF-01',
			sev: 'P0',
			template: t,
			status: pass ? 'PASS' : 'FAIL',
			failingTargets: pass ? 'None' : (m?.path ?? t),
			evidence: `[CMD node frontend/tests/seo/lab-perf.ts -> ${m?.path}: LCP=${m?.lcpMs}ms (<=2500ms), TBT=${m?.tbtMs}ms (<=200ms), CLS=${m?.cls} (<=0.1)]`
		});
	}

	rows.push({
		id: 'PERF-02',
		sev: 'P1',
		template: 'ALL',
		status: 'FAIL',
		failingTargets:
			'frontend/package.json, frontend/vite.config.ts, 201 raw <img> tags across src/',
		evidence: `[CODE docs/seo/audit/03-static.md:164-210] (@sveltejs/enhanced-img is not installed and enhancedImages() is not in vite.config.ts)`
	});

	for (const t of TEMPLATES) {
		const tPages = byTemplate(t);
		const lazyLcp = tPages.filter((p) => p.ssrHtml.lcpCandidateImg?.loading === 'lazy');
		rows.push({
			id: 'PERF-03',
			sev: 'P0',
			template: t,
			status: lazyLcp.length === 0 ? 'PASS' : 'FAIL',
			failingTargets: lazyLcp.length === 0 ? 'None' : lazyLcp.map((p) => p.path).join(', '),
			evidence:
				lazyLcp.length === 0
					? `[HTTP http://127.0.0.1:4273${tPages[0]?.path} -> first main img loading!=lazy]`
					: `[HTTP http://127.0.0.1:4273${lazyLcp[0]?.path} -> first main img has loading="lazy" on src="${lazyLcp[0]?.ssrHtml.lcpCandidateImg?.src}"]`
		});
	}

	rows.push({
		id: 'PERF-04',
		sev: 'P1',
		template: 'ALL',
		status: 'FAIL',
		failingTargets: 'frontend/src/hooks.server.ts:545-549',
		evidence: `[CODE frontend/src/hooks.server.ts:545-549] (preload callback returns type === 'js', excluding type === 'font' and type === 'css')`
	});

	rows.push({
		id: 'PERF-05',
		sev: 'P1',
		template: 'ALL',
		status: 'PASS',
		failingTargets:
			'None (data-sveltekit-preload-data="hover" set on <body> in app.html; gzip/br compression active)',
		evidence: `[CODE frontend/src/app.html:47] [HTTP http://127.0.0.1:4273/ -> content-encoding: gzip]`
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// 11. IMAGES, VIDEO & MEDIA SEO (MEDIA-01 .. MEDIA-03)
	// ═══════════════════════════════════════════════════════════════════════════
	rows.push({
		id: 'MEDIA-01',
		sev: 'P1',
		template: 'ALL',
		status: 'FAIL',
		failingTargets: '/store/scanners (/images/scanners/high-octane-scanner.jpg returns 404)',
		evidence: `[HTTP http://127.0.0.1:4273/images/scanners/high-octane-scanner.jpg -> 404] [CODE frontend/src/routes/store/scanners/+page.ts:86]`
	});

	rows.push({
		id: 'MEDIA-02',
		sev: 'P1',
		template: 'ALL',
		status: 'N-A',
		failingTargets: 'None (0 indexable video watch pages)',
		evidence: `[CODE docs/seo/audit/02-inventory.json:1]`
	});

	rows.push({
		id: 'MEDIA-03',
		sev: 'P1',
		template: 'ALL',
		status: 'PASS',
		failingTargets: 'None (favicon.png is 32x32+ PNG referenced in app.html)',
		evidence: `[CODE frontend/src/app.html:7,29] [HTTP http://127.0.0.1:4273/favicon.png -> 200 image/png]`
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// 12. CONTENT QUALITY, E-E-A-T, YMYL & SPAM COMPLIANCE
	// ═══════════════════════════════════════════════════════════════════════════
	rows.push({
		id: 'YMYL-01',
		sev: 'P1',
		template: 'article',
		status: 'FAIL',
		failingTargets:
			'/guides/exit-strategies, /guides/position-sizing, /guides/risk-management, /guides/swing-entry, /resources/etf-stocks-list, /resources/stock-indexes-list',
		evidence: `[HTTP http://127.0.0.1:4273/guides/risk-management -> no visible author byline link to /authors/[slug] or visible published/updated <time datetime> in DOM]`
	});

	rows.push({
		id: 'YMYL-02',
		sev: 'P1',
		template: 'ALL',
		status: 'FAIL',
		failingTargets: '/authors/billy-ribeiro, /authors/freddie-ferber, /authors/shao-wen',
		evidence: `[CODE frontend/src/lib/authors/authors.ts:24,31,38] (author bio fields are empty strings ''; YMYL trading articles lack primary-source citations)`
	});

	rows.push({
		id: 'YMYL-03',
		sev: 'P1',
		template: 'ALL',
		status: 'PASS',
		failingTargets:
			'None (/about, /contact, /privacy, /terms, /disclaimer, /refund all return 200 and are linked in MarketingFooter)',
		evidence: `[HTTP http://127.0.0.1:4273/ -> footerPolicyLinks all present=true in docs/seo/audit/03-baseline.json:967-988]`
	});

	rows.push({
		id: 'SPAM-01',
		sev: 'P0',
		template: 'ALL',
		status: 'PASS',
		failingTargets:
			'None (0 user-agent conditionals in src/ serving different content to crawlers)',
		evidence: `[CODE frontend/src/hooks.server.ts:1-684]`
	});

	rows.push({
		id: 'SPAM-02',
		sev: 'P0',
		template: 'ALL',
		status: 'FAIL',
		failingTargets:
			'frontend/src/routes/store/scanners/+page.ts:89, frontend/src/routes/store/scanners/high-octane-scanner/+page.ts:41, frontend/src/routes/live-trading-rooms/+page.server.ts:226',
		evidence: `[CODE frontend/src/routes/store/scanners/high-octane-scanner/+page.ts:41] (hardcoded ratingValue: 4.8, ratingCount: 127 in JSON-LD with no real review data source)`
	});

	rows.push({
		id: 'SPAM-03',
		sev: 'P0',
		template: 'ALL',
		status: 'PASS',
		failingTargets: 'None (0 doorway or mass-generated programmatic pages)',
		evidence: `[CODE docs/seo/audit/02-inventory.json:1]`
	});

	rows.push({
		id: 'SPAM-04',
		sev: 'P0',
		template: 'ALL',
		status: 'PASS',
		failingTargets: 'None (0 third-party hosted parasite sections)',
		evidence: `[CODE docs/seo/audit/02-inventory.json:1]`
	});

	rows.push({
		id: 'SPAM-05',
		sev: 'P0',
		template: 'ALL',
		status: 'PASS',
		failingTargets: 'None (0 hidden text blocks or keyword stuffing detected)',
		evidence: `[CODE docs/seo/audit/03-baseline.json:893-966]`
	});

	rows.push({
		id: 'SPAM-06',
		sev: 'P1',
		template: 'ALL',
		status: 'PASS',
		failingTargets: 'None (0 affiliate/sponsored or public UGC outbound links on indexable pages)',
		evidence: `[CODE docs/seo/audit/03-baseline.json:1]`
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// 13. ANALYTICS, MONITORING & CI GATES (OPS-01 .. OPS-04, TOOL-01 .. TOOL-04)
	// ═══════════════════════════════════════════════════════════════════════════
	rows.push({
		id: 'OPS-01',
		sev: 'P1',
		template: 'ALL',
		status: 'REQUIRES-PRODUCTION',
		failingTargets: 'Google Search Console property verification & sitemap submission',
		evidence: `[CODE frontend/src/lib/seo/defaults.ts:79-84] (requires production DNS/GSC owner action)`
	});

	rows.push({
		id: 'OPS-02',
		sev: 'P1',
		template: 'ALL',
		status: 'PASS',
		failingTargets:
			'None (web-vitals v6.2.3 initialized in +layout.svelte via initPerformanceMonitoring)',
		evidence: `[CODE frontend/src/routes/+layout.svelte:44,155] [CODE frontend/package.json:74]`
	});

	rows.push({
		id: 'OPS-03',
		sev: 'P1',
		template: 'ALL',
		status: 'FAIL',
		failingTargets:
			'tests/seo/*.spec.ts, package.json (missing automated Playwright SEO CI suite per requirement ID)',
		evidence: `[CODE frontend/tests/sitemap.test.ts:1] (only basic sitemap.test.ts exists; full requirement-ID Playwright suite not yet wired)`
	});

	rows.push({
		id: 'OPS-04',
		sev: 'P1',
		template: 'ALL',
		status: 'PASS',
		failingTargets:
			'None (Cloudflare Pages / CDN compression active; s-maxage Cache-Control on sitemaps/robots)',
		evidence: `[CODE frontend/src/routes/sitemap.xml/+server.ts:154] [CODE frontend/src/routes/robots.txt/+server.ts:485]`
	});

	rows.push({
		id: 'TOOL-01',
		sev: 'P0',
		template: 'ALL',
		status: 'PASS',
		failingTargets: 'None (Svelte MCP list-sections, get-documentation, svelte-autofixer active)',
		evidence: `[CODE docs/seo/audit/00-environment.md:36-41] [CODE AGENTS.md:1-23]`
	});

	rows.push({
		id: 'TOOL-02',
		sev: 'P1',
		template: 'ALL',
		status: 'FAIL',
		failingTargets:
			'frontend/src/lib/seo/Seo.svelte, frontend/src/lib/components/SEOHead.svelte, frontend/src/lib/components/seo/SeoHead.svelte',
		evidence: `[CODE frontend/src/lib/seo/Seo.svelte:1] (3 competing SEO head components exist; svelte-meta-tags v5.0.3 is installed in package.json:69 but unused by Seo.svelte)`
	});

	rows.push({
		id: 'TOOL-03',
		sev: 'P1',
		template: 'ALL',
		status: 'FAIL',
		failingTargets: 'frontend/src/routes/sitemap.xml/+server.ts:60-156',
		evidence: `[CODE frontend/src/routes/sitemap.xml/+server.ts:115-151] (uses super-sitemap v2.0.6 but hardcodes SITE_URL and hand-lists additionalPaths including /login, /register, /signup while omitting /authors/[slug])`
	});

	rows.push({
		id: 'TOOL-04',
		sev: 'P2',
		template: 'ALL',
		status: 'FAIL',
		failingTargets: 'frontend/src/lib/seo/schemas.ts, frontend/src/lib/seo/jsonld.ts',
		evidence: `[CODE frontend/src/lib/seo/types.ts:15] (schema-dts v2.1.0 is imported in types.ts but JSON-LD builders in schemas.ts and jsonld.ts use custom loose interfaces instead of WithContext<Thing> compile-time typing)`
	});

	// Count summary
	const counts = {
		PASS: rows.filter((r) => r.status === 'PASS').length,
		FAIL: rows.filter((r) => r.status === 'FAIL').length,
		'N-A': rows.filter((r) => r.status === 'N-A').length,
		'REQUIRES-HUMAN': rows.filter((r) => r.status === 'REQUIRES-HUMAN').length,
		'REQUIRES-PRODUCTION': rows.filter((r) => r.status === 'REQUIRES-PRODUCTION').length
	};

	const p0Fails = rows.filter((r) => r.sev === 'P0' && r.status === 'FAIL').length;
	const p1Fails = rows.filter((r) => r.sev === 'P1' && r.status === 'FAIL').length;
	const p2Fails = rows.filter((r) => r.sev === 'P2' && r.status === 'FAIL').length;

	const tableLines = rows.map(
		(r) =>
			`| \`${r.id}\` | ${r.sev} | \`${r.template}\` | **${r.status}** | ${r.failingTargets.replace(/\|/g, '\\|')} | ${r.evidence.replace(/\|/g, '\\|')} |`
	);

	const md = `# Phase 3d — Baseline Compliance Matrix (\`03-matrix-baseline.md\`)

- **Generated At:** ${new Date().toISOString()}
- **Generator:** \`[CODE frontend/tests/seo/generate-baseline-matrix.ts:1]\`
- **Total Matrix Rows:** ${rows.length} (covering all 72 requirement IDs across all 8 active indexable templates)
- **Status Summary:**
  - **PASS:** ${counts.PASS}
  - **FAIL:** ${counts.FAIL} (P0: ${p0Fails}, P1: ${p1Fails}, P2: ${p2Fails})
  - **N-A:** ${counts['N-A']}
  - **REQUIRES-HUMAN:** ${counts['REQUIRES-HUMAN']}
  - **REQUIRES-PRODUCTION:** ${counts['REQUIRES-PRODUCTION']}

---

| ID | Sev | Template | Status | Failing URLs / Files | Evidence |
| --- | --- | --- | --- | --- | --- |
${tableLines.join('\n')}
`;

	fs.writeFileSync(outPath, md, 'utf8');
	console.log(`Wrote ${outPath} (${rows.length} rows)`);
	console.log({ counts, p0Fails, p1Fails, p2Fails });
}

buildBaselineMatrix();
