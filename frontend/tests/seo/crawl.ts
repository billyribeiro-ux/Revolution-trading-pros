import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import { chromium } from 'playwright';
import AxeBuilder from '@axe-core/playwright';
import { JSDOM } from 'jsdom';

interface InventoryEntry {
	app: string;
	route: string;
	pathPattern: string;
	url: string | null;
	indexable: boolean;
	template: string;
}

interface RedirectHop {
	url: string;
	status: number;
	location: string | null;
}

interface RawHttpResult {
	requestedUrl: string;
	finalUrl: string;
	redirectChain: RedirectHop[];
	finalStatus: number;
	contentType: string | null;
	xRobotsTag: string | null;
	linkHeader: string | null;
	cacheControl: string | null;
	contentEncoding: string | null;
	ttfbMs: number;
	bodyBytes: number;
	rawTitle: string | null;
	rawH1: string | null;
	rawMainWordCount: number;
	bodyHash: string;
}

interface JsonLdBlockInfo {
	raw: string;
	parsed: unknown;
	parseError: string | null;
	types: string[];
	hasDangerousChars: boolean;
	hasAggregateRating: boolean;
	hasFaqPage: boolean;
	hasHowTo: boolean;
}

interface ImgInfo {
	src: string;
	srcset: string | null;
	sizes: string | null;
	width: string | null;
	height: string | null;
	alt: string | null;
	loading: string | null;
	fetchpriority: string | null;
	decoding: string | null;
}

interface SsrHtmlResult {
	title: { text: string; charCount: number; tagCount: number };
	description: { content: string; charCount: number; tagCount: number };
	robots: { content: string; tagCount: number };
	googlebot: { content: string; tagCount: number };
	canonical: { href: string; tagCount: number };
	alternates: Array<{ hreflang: string; href: string }>;
	og: {
		title: string | null;
		description: string | null;
		image: string | null;
		url: string | null;
		type: string | null;
		siteName: string | null;
		locale: string | null;
	};
	twitter: {
		card: string | null;
		title: string | null;
		description: string | null;
		image: string | null;
	};
	htmlLang: string | null;
	htmlDir: string | null;
	viewport: string | null;
	iconHref: string | null;
	headings: Array<{ level: number; text: string }>;
	jsonLdBlocks: JsonLdBlockInfo[];
	breadcrumbs: Array<{ text: string; href: string | null }>;
	lcpCandidateImg: ImgInfo | null;
	imagesMissingAttributes: Array<{
		src: string;
		missingAlt: boolean;
		missingWidth: boolean;
		missingHeight: boolean;
	}>;
	totalImgCount: number;
}

interface LinkInfo {
	href: string;
	text: string;
	rel: string;
	target: string;
	isInternal: boolean;
}

interface RenderedDomResult {
	documentTitle: string;
	mainWordCount: number;
	consoleErrors: string[];
	pageErrors: string[];
	failedRequests: Array<{ url: string; errorText: string }>;
	links: LinkInfo[];
	nonAnchorClickNavElements: Array<{ tag: string; text: string }>;
	ssrVsHydratedWordCountDelta: number;
	csrOnlyContentRatio: number;
}

interface PageAuditRecord {
	path: string;
	template: string;
	indexable: boolean;
	inSitemap: boolean;
	rawHttp: RawHttpResult;
	ssrHtml: SsrHtmlResult;
	renderedDom: RenderedDomResult;
}

const isFinal = process.env.AUDIT_PHASE === 'final';
const repoRoot = path.resolve(import.meta.dirname, '../../..');
const inventoryPath = path.join(repoRoot, 'docs/seo/audit/02-inventory.json');
const baselineOutPath = path.join(
	repoRoot,
	isFinal ? 'docs/seo/audit/06-final.json' : 'docs/seo/audit/03-baseline.json'
);
const labDir = path.join(repoRoot, isFinal ? 'docs/seo/audit/06-lab' : 'docs/seo/audit/03-lab');

const PREVIEW_URL = (process.env.PREVIEW_URL || 'http://127.0.0.1:4273').replace(/\/$/, '');
const SITE_ORIGIN = 'https://revolutiontradingpros.com';

function countWords(text: string): number {
	const cleaned = text.replace(/\s+/g, ' ').trim();
	if (!cleaned) return 0;
	return cleaned.split(' ').length;
}

function extractTypesFromJsonLd(node: unknown): string[] {
	const types: string[] = [];
	if (!node || typeof node !== 'object') return types;
	if (Array.isArray(node)) {
		for (const item of node) types.push(...extractTypesFromJsonLd(item));
		return types;
	}
	const obj = node as Record<string, unknown>;
	if (typeof obj['@type'] === 'string') {
		types.push(obj['@type']);
	} else if (Array.isArray(obj['@type'])) {
		for (const t of obj['@type']) {
			if (typeof t === 'string') types.push(t);
		}
	}
	for (const val of Object.values(obj)) {
		if (val && typeof val === 'object') {
			types.push(...extractTypesFromJsonLd(val));
		}
	}
	return types;
}

async function fetchWithManualRedirects(
	targetUrl: string,
	customHeaders?: Record<string, string>
): Promise<{
	rawHttp: RawHttpResult;
	htmlBody: string;
}> {
	const redirectChain: RedirectHop[] = [];
	let currentUrl = targetUrl;
	let finalRes: Response | null = null;
	let ttfbMs = 0;

	for (let hop = 0; hop < 10; hop++) {
		const start = performance.now();
		const res = await fetch(currentUrl, {
			redirect: 'manual',
			headers: {
				Accept: 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
				...customHeaders
			}
		});
		if (hop === 0) {
			ttfbMs = Math.round((performance.now() - start) * 100) / 100;
		}
		if (res.status >= 300 && res.status < 400) {
			const loc = res.headers.get('location');
			redirectChain.push({ url: currentUrl, status: res.status, location: loc });
			if (!loc) {
				finalRes = res;
				break;
			}
			currentUrl = new URL(loc, currentUrl).toString();
			continue;
		}
		finalRes = res;
		break;
	}

	if (!finalRes) {
		throw new Error(`Exceeded redirect limit for ${targetUrl}`);
	}

	const buf = Buffer.from(await finalRes.arrayBuffer());
	const htmlBody = buf.toString('utf8');
	const dom = new JSDOM(htmlBody);
	const doc = dom.window.document;
	const rawTitle = doc.querySelector('title')?.textContent?.trim() ?? null;
	const rawH1 = doc.querySelector('h1')?.textContent?.trim() ?? null;
	const mainEl = doc.querySelector('main') ?? doc.body;
	const rawMainText = mainEl ? (mainEl.textContent ?? '') : '';
	const rawMainWordCount = countWords(rawMainText);
	const bodyHash = crypto
		.createHash('sha256')
		.update(rawMainText.replace(/\s+/g, ' ').trim())
		.digest('hex');

	return {
		htmlBody,
		rawHttp: {
			requestedUrl: targetUrl,
			finalUrl: currentUrl,
			redirectChain,
			finalStatus: finalRes.status,
			contentType: finalRes.headers.get('content-type'),
			xRobotsTag: finalRes.headers.get('x-robots-tag'),
			linkHeader: finalRes.headers.get('link'),
			cacheControl: finalRes.headers.get('cache-control'),
			contentEncoding: finalRes.headers.get('content-encoding'),
			ttfbMs,
			bodyBytes: buf.byteLength,
			rawTitle,
			rawH1,
			rawMainWordCount,
			bodyHash
		}
	};
}

function parseSsrHtml(html: string): SsrHtmlResult {
	const dom = new JSDOM(html);
	const doc = dom.window.document;

	const titleEls = [...doc.querySelectorAll('head title')];
	const titleText = titleEls[0]?.textContent?.trim() ?? '';

	const descEls = [...doc.querySelectorAll('head meta[name="description"]')];
	const descContent = descEls[0]?.getAttribute('content')?.trim() ?? '';

	const robotsEls = [...doc.querySelectorAll('head meta[name="robots"]')];
	const robotsContent = robotsEls[0]?.getAttribute('content')?.trim() ?? '';

	const googlebotEls = [...doc.querySelectorAll('head meta[name="googlebot"]')];
	const googlebotContent = googlebotEls[0]?.getAttribute('content')?.trim() ?? '';

	const canonicalEls = [...doc.querySelectorAll('head link[rel="canonical"]')];
	const canonicalHref = canonicalEls[0]?.getAttribute('href')?.trim() ?? '';

	const alternateEls = [...doc.querySelectorAll('head link[rel="alternate"][hreflang]')];
	const alternates = alternateEls.map((el) => ({
		hreflang: el.getAttribute('hreflang') ?? '',
		href: el.getAttribute('href') ?? ''
	}));

	const getMetaProp = (prop: string): string | null =>
		doc.querySelector(`head meta[property="${prop}"]`)?.getAttribute('content') ?? null;
	const getMetaName = (name: string): string | null =>
		doc.querySelector(`head meta[name="${name}"]`)?.getAttribute('content') ?? null;

	const headings = [...doc.querySelectorAll('h1, h2, h3, h4, h5, h6')].map((el) => ({
		level: Number.parseInt(el.tagName.slice(1), 10),
		text: (el.textContent ?? '').replace(/\s+/g, ' ').trim()
	}));

	const jsonLdScripts = [...doc.querySelectorAll('script[type="application/ld+json"]')];
	const jsonLdBlocks: JsonLdBlockInfo[] = jsonLdScripts.map((s) => {
		const raw = s.textContent ?? '';
		let parsed: unknown = null;
		let parseError: string | null = null;
		try {
			parsed = JSON.parse(raw);
		} catch (err) {
			parseError = err instanceof Error ? err.message : String(err);
		}
		const types = extractTypesFromJsonLd(parsed);
		// Check if raw JSON-LD contains unescaped </script or <!--
		const hasDangerousChars = /<\/script|<!--/i.test(raw);
		return {
			raw,
			parsed,
			parseError,
			types,
			hasDangerousChars,
			hasAggregateRating: types.includes('AggregateRating'),
			hasFaqPage: types.includes('FAQPage'),
			hasHowTo: types.includes('HowTo')
		};
	});

	const breadcrumbNav = doc.querySelector(
		'nav[aria-label*="Breadcrumb" i], [aria-label*="breadcrumb" i], .breadcrumbs'
	);
	const breadcrumbs: Array<{ text: string; href: string | null }> = breadcrumbNav
		? [...breadcrumbNav.querySelectorAll('a, li, span')]
				.filter(
					(el) =>
						el.tagName === 'A' ||
						el.classList.contains('current') ||
						el.getAttribute('aria-current') === 'page'
				)
				.map((el) => ({
					text: (el.textContent ?? '').trim(),
					href: el.tagName === 'A' ? el.getAttribute('href') : null
				}))
		: [];

	const allImgs = [...doc.querySelectorAll('img')];
	const firstMainImg = doc.querySelector('main img') ?? allImgs[0] ?? null;
	const lcpCandidateImg: ImgInfo | null = firstMainImg
		? {
				src: firstMainImg.getAttribute('src') ?? '',
				srcset: firstMainImg.getAttribute('srcset'),
				sizes: firstMainImg.getAttribute('sizes'),
				width: firstMainImg.getAttribute('width'),
				height: firstMainImg.getAttribute('height'),
				alt: firstMainImg.getAttribute('alt'),
				loading: firstMainImg.getAttribute('loading'),
				fetchpriority: firstMainImg.getAttribute('fetchpriority'),
				decoding: firstMainImg.getAttribute('decoding')
			}
		: null;

	const imagesMissingAttributes = allImgs
		.map((img) => {
			const role = img.getAttribute('role');
			const isDecorative = role === 'presentation' || role === 'none';
			const missingAlt = !isDecorative && img.getAttribute('alt') === null;
			const missingWidth = !img.getAttribute('width');
			const missingHeight = !img.getAttribute('height');
			return {
				src: img.getAttribute('src') ?? '',
				missingAlt,
				missingWidth,
				missingHeight
			};
		})
		.filter((i) => i.missingAlt || i.missingWidth || i.missingHeight);

	return {
		title: { text: titleText, charCount: titleText.length, tagCount: titleEls.length },
		description: { content: descContent, charCount: descContent.length, tagCount: descEls.length },
		robots: { content: robotsContent, tagCount: robotsEls.length },
		googlebot: { content: googlebotContent, tagCount: googlebotEls.length },
		canonical: { href: canonicalHref, tagCount: canonicalEls.length },
		alternates,
		og: {
			title: getMetaProp('og:title'),
			description: getMetaProp('og:description'),
			image: getMetaProp('og:image'),
			url: getMetaProp('og:url'),
			type: getMetaProp('og:type'),
			siteName: getMetaProp('og:site_name'),
			locale: getMetaProp('og:locale')
		},
		twitter: {
			card: getMetaName('twitter:card'),
			title: getMetaName('twitter:title'),
			description: getMetaName('twitter:description'),
			image: getMetaName('twitter:image')
		},
		htmlLang: doc.documentElement.getAttribute('lang'),
		htmlDir: doc.documentElement.getAttribute('dir'),
		viewport: getMetaName('viewport'),
		iconHref: doc.querySelector('head link[rel*="icon"]')?.getAttribute('href') ?? null,
		headings,
		jsonLdBlocks,
		breadcrumbs,
		lcpCandidateImg,
		imagesMissingAttributes,
		totalImgCount: allImgs.length
	};
}

async function parseSitemapXml(endpointPath: string): Promise<{
	status: number;
	contentType: string | null;
	rawLength: number;
	locs: string[];
	hasPriority: boolean;
	hasChangefreq: boolean;
	locStatuses: Array<{ loc: string; path: string; status: number }>;
}> {
	const res = await fetch(`${PREVIEW_URL}${endpointPath}`);
	const xml = await res.text();
	const locMatches = [...xml.matchAll(/<loc>([^<]+)<\/loc>/g)].map((m) => (m[1] ?? '').trim());
	const hasPriority = /<priority>/i.test(xml);
	const hasChangefreq = /<changefreq>/i.test(xml);

	const locStatuses: Array<{ loc: string; path: string; status: number }> = [];
	for (const loc of locMatches) {
		try {
			const u = new URL(loc);
			const checkRes = await fetch(`${PREVIEW_URL}${u.pathname}${u.search}`, {
				redirect: 'manual'
			});
			locStatuses.push({ loc, path: u.pathname, status: checkRes.status });
		} catch {
			locStatuses.push({ loc, path: loc, status: 0 });
		}
	}

	return {
		status: res.status,
		contentType: res.headers.get('content-type'),
		rawLength: xml.length,
		locs: locMatches,
		hasPriority,
		hasChangefreq,
		locStatuses
	};
}

async function main(): Promise<void> {
	fs.mkdirSync(labDir, { recursive: true });
	const inventory: InventoryEntry[] = JSON.parse(fs.readFileSync(inventoryPath, 'utf8'));

	// 1. Inspect robots.txt and sitemaps
	const robotsRes = await fetch(`${PREVIEW_URL}/robots.txt`);
	const robotsBody = await robotsRes.text();
	const robotsRules = robotsBody
		.split(/\r?\n/)
		.map((l) => l.trim())
		.filter((l) => l && !l.startsWith('#'));

	const mainSitemap = await parseSitemapXml('/sitemap.xml');
	const newsSitemap = await parseSitemapXml('/news-sitemap.xml');
	const videoSitemap = await parseSitemapXml('/video-sitemap.xml');

	const sitemapPaths = new Set(
		mainSitemap.locs.map((loc) => {
			const u = new URL(loc);
			return u.pathname;
		})
	);

	// Build crawl target list: all indexable URLs from inventory + all sitemap paths
	const targetMap = new Map<string, { template: string; indexable: boolean }>();
	for (const item of inventory) {
		if (item.indexable && item.url) {
			targetMap.set(item.url, { template: item.template, indexable: true });
		}
	}
	for (const p of sitemapPaths) {
		if (!targetMap.has(p)) {
			const invMatch = inventory.find((i) => i.url === p);
			targetMap.set(p, {
				template: invMatch?.template ?? 'static',
				indexable: invMatch?.indexable ?? false
			});
		}
	}

	const browser = await chromium.launch({ headless: true });
	const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });

	const pages: PageAuditRecord[] = [];
	const sortedPaths = [...targetMap.keys()].sort();

	console.log(`Crawling ${sortedPaths.length} URLs on ${PREVIEW_URL}...`);

	for (const urlPath of sortedPaths) {
		const meta = targetMap.get(urlPath) ?? { template: 'static', indexable: true };
		const fullUrl = `${PREVIEW_URL}${urlPath}`;
		const { rawHttp, htmlBody } = await fetchWithManualRedirects(fullUrl);
		const ssrHtml = parseSsrHtml(htmlBody);

		const page = await context.newPage();
		const consoleErrors: string[] = [];
		const pageErrors: string[] = [];
		const failedRequests: Array<{ url: string; errorText: string }> = [];

		page.on('console', (msg) => {
			if (msg.type() === 'error') {
				consoleErrors.push(msg.text());
			}
		});
		page.on('pageerror', (err) => {
			pageErrors.push(err.message);
		});
		page.on('requestfailed', (req) => {
			const failure = req.failure();
			failedRequests.push({
				url: req.url(),
				errorText: failure?.errorText ?? 'unknown'
			});
		});

		await page.goto(fullUrl, { waitUntil: 'domcontentloaded', timeout: 30000 });
		await page.waitForTimeout(250);

		const domInfo = await page.evaluate((originPrefix) => {
			const documentTitle = document.title;
			const mainEl = document.querySelector('main') ?? document.body;
			const mainText = (mainEl?.textContent ?? '').replace(/\s+/g, ' ').trim();
			const mainWordCount = mainText ? mainText.split(' ').length : 0;

			const links = [...document.querySelectorAll('a[href]')].map((a) => {
				const rawHref = a.getAttribute('href') ?? '';
				const isInternal =
					rawHref.startsWith('/') ||
					rawHref.startsWith(originPrefix) ||
					rawHref.startsWith('https://revolutiontradingpros.com');
				return {
					href: rawHref,
					text: (a.textContent ?? '').replace(/\s+/g, ' ').trim(),
					rel: a.getAttribute('rel') ?? '',
					target: a.getAttribute('target') ?? '',
					isInternal
				};
			});

			const nonAnchorClickNavElements = [
				...document.querySelectorAll('button[onclick], div[onclick], span[onclick], [role="link"]')
			].map((el) => ({
				tag: el.tagName.toLowerCase(),
				text: (el.textContent ?? '').replace(/\s+/g, ' ').trim().slice(0, 80)
			}));

			return { documentTitle, mainWordCount, links, nonAnchorClickNavElements };
		}, PREVIEW_URL);

		await page.close();

		const ssrVsHydratedWordCountDelta = domInfo.mainWordCount - rawHttp.rawMainWordCount;
		const csrOnlyContentRatio =
			domInfo.mainWordCount > 0
				? Math.max(0, ssrVsHydratedWordCountDelta) / domInfo.mainWordCount
				: 0;

		pages.push({
			path: urlPath,
			template: meta.template,
			indexable: meta.indexable,
			inSitemap: sitemapPaths.has(urlPath),
			rawHttp,
			ssrHtml,
			renderedDom: {
				...domInfo,
				consoleErrors,
				pageErrors,
				failedRequests,
				ssrVsHydratedWordCountDelta,
				csrOnlyContentRatio: Math.round(csrOnlyContentRatio * 1000) / 1000
			}
		});
	}

	// Derived checks
	// 1. Canonical mismatches
	const canonicalMismatches = pages
		.filter((p) => p.indexable)
		.filter((p) => {
			const expected = `${SITE_ORIGIN}${p.path}`;
			return p.ssrHtml.canonical.href !== expected || p.ssrHtml.canonical.tagCount !== 1;
		})
		.map((p) => ({
			path: p.path,
			expected: `${SITE_ORIGIN}${p.path}`,
			actual: p.ssrHtml.canonical.href,
			tagCount: p.ssrHtml.canonical.tagCount
		}));

	// 2. Duplicate titles, descriptions, body hashes across indexable pages
	const groupDuplicates = (items: Array<{ path: string; key: string }>) => {
		const map = new Map<string, string[]>();
		for (const item of items) {
			if (!item.key) continue;
			const list = map.get(item.key) ?? [];
			list.push(item.path);
			map.set(item.key, list);
		}
		return [...map.entries()]
			.filter(([, paths]) => paths.length > 1)
			.map(([value, paths]) => ({ value, paths }));
	};

	const indexablePages = pages.filter((p) => p.indexable);
	const duplicateTitles = groupDuplicates(
		indexablePages.map((p) => ({ path: p.path, key: p.ssrHtml.title.text }))
	);
	const duplicateDescriptions = groupDuplicates(
		indexablePages.map((p) => ({ path: p.path, key: p.ssrHtml.description.content }))
	);
	const duplicateBodyHashes = groupDuplicates(
		indexablePages.map((p) => ({ path: p.path, key: p.rawHttp.bodyHash }))
	);

	// 3. Link graph, click depth from /, orphan pages, broken internal links
	const adjacency = new Map<string, Set<string>>();
	const incomingCount = new Map<string, number>();
	for (const p of pages) {
		incomingCount.set(p.path, 0);
	}

	const allDiscoveredInternalPaths = new Set<string>();
	for (const p of pages) {
		const outSet = new Set<string>();
		for (const link of p.renderedDom.links) {
			if (!link.isInternal) continue;
			if (
				link.href.startsWith('#') ||
				link.href.startsWith('mailto:') ||
				link.href.startsWith('tel:')
			) {
				continue;
			}
			try {
				const u = new URL(link.href, PREVIEW_URL);
				const normPath = u.pathname === '/' ? '/' : u.pathname.replace(/\/$/, '');
				outSet.add(normPath);
				allDiscoveredInternalPaths.add(normPath);
				if (normPath !== p.path) {
					incomingCount.set(normPath, (incomingCount.get(normPath) ?? 0) + 1);
				}
			} catch {
				// ignore malformed
			}
		}
		adjacency.set(p.path, outSet);
	}

	// BFS click depth from /
	const clickDepthByUrl: Record<string, number | null> = {};
	for (const p of pages) clickDepthByUrl[p.path] = null;
	clickDepthByUrl['/'] = 0;
	const queue: string[] = ['/'];
	while (queue.length > 0) {
		const curr = queue.shift() ?? '/';
		const currDepth = clickDepthByUrl[curr] ?? 0;
		const neighbors = adjacency.get(curr) ?? new Set<string>();
		for (const next of neighbors) {
			if (next in clickDepthByUrl && clickDepthByUrl[next] === null) {
				clickDepthByUrl[next] = currDepth + 1;
				queue.push(next);
			}
		}
	}

	const orphanIndexablePages = indexablePages
		.filter((p) => p.path !== '/' && (incomingCount.get(p.path) ?? 0) === 0)
		.map((p) => p.path);

	// Check HTTP status of all internal links discovered
	const brokenInternalLinks: Array<{ path: string; status: number }> = [];
	const redirectInternalLinks: Array<{ path: string; status: number; location: string | null }> =
		[];
	for (const intPath of [...allDiscoveredInternalPaths].sort()) {
		const existing = pages.find((p) => p.path === intPath);
		if (existing) {
			if (existing.rawHttp.finalStatus >= 400) {
				brokenInternalLinks.push({ path: intPath, status: existing.rawHttp.finalStatus });
			}
			if (existing.rawHttp.redirectChain.length > 0) {
				const first = existing.rawHttp.redirectChain[0];
				if (first) {
					redirectInternalLinks.push({
						path: intPath,
						status: first.status,
						location: first.location
					});
				}
			}
		} else {
			const res = await fetch(`${PREVIEW_URL}${intPath}`, { redirect: 'manual' });
			if (res.status >= 400) {
				brokenInternalLinks.push({ path: intPath, status: res.status });
			} else if (res.status >= 300 && res.status < 400) {
				redirectInternalLinks.push({
					path: intPath,
					status: res.status,
					location: res.headers.get('location')
				});
			}
		}
	}

	// 4. Soft-404 probes
	const soft404Targets = [
		'/__nonexistent_seo_test_path__',
		'/authors/__nonexistent_slug_seo_test__',
		'/blog/__nonexistent_slug_seo_test__',
		'/indicators/__nonexistent_slug_seo_test__',
		'/classes/__nonexistent_slug_seo_test__'
	];
	const soft404Probes: Array<{
		path: string;
		status: number;
		robotsMeta: string;
		hasNoindex: boolean;
		isSoft404: boolean;
	}> = [];
	for (const probePath of soft404Targets) {
		const { rawHttp, htmlBody } = await fetchWithManualRedirects(`${PREVIEW_URL}${probePath}`);
		const parsed = parseSsrHtml(htmlBody);
		const hasNoindex = parsed.robots.content.toLowerCase().includes('noindex');
		soft404Probes.push({
			path: probePath,
			status: rawHttp.finalStatus,
			robotsMeta: parsed.robots.content,
			hasNoindex,
			isSoft404: rawHttp.finalStatus === 200
		});
	}

	// 5. Query-param variant probes
	const queryParamTargets = [
		'/?utm_source=x&fbclid=y',
		'/about?utm_source=x&fbclid=y',
		'/pricing?utm_source=x&fbclid=y'
	];
	const queryParamProbes: Array<{
		requested: string;
		status: number;
		canonical: string;
		strippedParamsCorrectly: boolean;
	}> = [];
	for (const qp of queryParamTargets) {
		const { rawHttp, htmlBody } = await fetchWithManualRedirects(`${PREVIEW_URL}${qp}`);
		const parsed = parseSsrHtml(htmlBody);
		const cleanPath = qp.split('?')[0] ?? '/';
		const expectedCanonical = `${SITE_ORIGIN}${cleanPath}`;
		queryParamProbes.push({
			requested: qp,
			status: rawHttp.finalStatus,
			canonical: parsed.canonical.href,
			strippedParamsCorrectly: parsed.canonical.href === expectedCanonical
		});
	}

	// 6. Accept-Language probes
	const acceptLanguageProbes: Array<{
		acceptLanguage: string;
		status: number;
		redirectCount: number;
	}> = [];
	for (const lang of ['fr', 'en', 'es-MX']) {
		const { rawHttp } = await fetchWithManualRedirects(`${PREVIEW_URL}/`, {
			'Accept-Language': lang
		});
		acceptLanguageProbes.push({
			acceptLanguage: lang,
			status: rawHttp.finalStatus,
			redirectCount: rawHttp.redirectChain.length
		});
	}

	// 7. Viewport parity & Axe accessibility on 1 representative URL per template
	const templateRepresentatives: Array<{ template: string; path: string }> = [
		{ template: 'home', path: '/' },
		{ template: 'static', path: '/about' },
		{ template: 'listing', path: '/courses' },
		{ template: 'product', path: '/alerts/explosive-swings' },
		{ template: 'profile', path: '/authors/billy-ribeiro' },
		{ template: 'course', path: '/courses/day-trading-masterclass' },
		{ template: 'article', path: '/guides/risk-management' },
		{ template: 'software', path: '/indicators/rsi' }
	];

	const viewportParity: Array<{
		template: string;
		path: string;
		desktopWords: number;
		mobileWords: number;
		desktopH1: string | null;
		mobileH1: string | null;
		parityPass: boolean;
	}> = [];

	const axeResultsByTemplate: Array<{
		template: string;
		path: string;
		violationCount: number;
		violations: Array<{
			id: string;
			impact: string | null | undefined;
			description: string;
			nodes: number;
		}>;
	}> = [];

	const mobileContext = await browser.newContext({ viewport: { width: 390, height: 844 } });

	for (const rep of templateRepresentatives) {
		const fullUrl = `${PREVIEW_URL}${rep.path}`;
		const desktopPage = await context.newPage();
		await desktopPage.goto(fullUrl, { waitUntil: 'domcontentloaded' });
		await desktopPage.waitForTimeout(200);
		const desktopData = await desktopPage.evaluate(() => {
			const mainEl = document.querySelector('main') ?? document.body;
			const words = (mainEl?.textContent ?? '')
				.replace(/\s+/g, ' ')
				.trim()
				.split(' ')
				.filter(Boolean).length;
			const h1 = document.querySelector('h1')?.textContent?.trim() ?? null;
			return { words, h1 };
		});

		const axeScan = await new AxeBuilder({ page: desktopPage })
			.withTags(['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa'])
			.analyze();

		axeResultsByTemplate.push({
			template: rep.template,
			path: rep.path,
			violationCount: axeScan.violations.length,
			violations: axeScan.violations.map((v) => ({
				id: v.id,
				impact: v.impact,
				description: v.description,
				nodes: v.nodes.length
			}))
		});
		await desktopPage.close();

		const mobilePage = await mobileContext.newPage();
		await mobilePage.goto(fullUrl, { waitUntil: 'domcontentloaded' });
		await mobilePage.waitForTimeout(200);
		const mobileData = await mobilePage.evaluate(() => {
			const mainEl = document.querySelector('main') ?? document.body;
			const words = (mainEl?.textContent ?? '')
				.replace(/\s+/g, ' ')
				.trim()
				.split(' ')
				.filter(Boolean).length;
			const h1 = document.querySelector('h1')?.textContent?.trim() ?? null;
			return { words, h1 };
		});
		await mobilePage.close();

		viewportParity.push({
			template: rep.template,
			path: rep.path,
			desktopWords: desktopData.words,
			mobileWords: mobileData.words,
			desktopH1: desktopData.h1,
			mobileH1: mobileData.h1,
			parityPass: desktopData.words === mobileData.words && desktopData.h1 === mobileData.h1
		});
	}

	await mobileContext.close();
	await context.close();
	await browser.close();

	// Save Axe lab report
	fs.writeFileSync(
		path.join(labDir, 'axe-summary.json'),
		JSON.stringify(axeResultsByTemplate, null, 2) + '\n',
		'utf8'
	);

	// 8. YMYL & SPAM signals summary
	const homePage = pages.find((p) => p.path === '/');
	const footerPolicyLinks = ['privacy', 'terms', 'disclaimer', 'refund', 'contact'].map((slug) => ({
		slug,
		presentOnHome: homePage?.renderedDom.links.some((l) => l.href.includes(`/${slug}`)) ?? false
	}));

	const pagesWithSelfServingAggregateRating = pages
		.filter((p) => p.ssrHtml.jsonLdBlocks.some((b) => b.hasAggregateRating))
		.map((p) => p.path);

	const baselineReport = {
		generatedAt: new Date().toISOString(),
		previewUrl: PREVIEW_URL,
		summary: {
			totalCrawledPages: pages.length,
			indexablePagesCount: indexablePages.length,
			canonicalMismatchCount: canonicalMismatches.length,
			duplicateTitleGroups: duplicateTitles.length,
			duplicateDescriptionGroups: duplicateDescriptions.length,
			duplicateBodyHashGroups: duplicateBodyHashes.length,
			orphanIndexablePageCount: orphanIndexablePages.length,
			brokenInternalLinkCount: brokenInternalLinks.length,
			soft404FailureCount: soft404Probes.filter((p) => p.isSoft404 || !p.hasNoindex).length,
			pagesWithMultipleTitles: indexablePages
				.filter((p) => p.ssrHtml.title.tagCount !== 1)
				.map((p) => p.path),
			pagesWithMultipleDescriptions: indexablePages
				.filter((p) => p.ssrHtml.description.tagCount !== 1)
				.map((p) => p.path),
			pagesWithMultipleCanonicals: indexablePages
				.filter((p) => p.ssrHtml.canonical.tagCount !== 1)
				.map((p) => p.path),
			pagesMissingH1: indexablePages
				.filter((p) => p.ssrHtml.headings.filter((h) => h.level === 1).length !== 1)
				.map((p) => ({
					path: p.path,
					h1Count: p.ssrHtml.headings.filter((h) => h.level === 1).length
				})),
			pagesWithSelfServingAggregateRating
		},
		robotsTxt: {
			status: robotsRes.status,
			contentType: robotsRes.headers.get('content-type'),
			disallowsAppDir: robotsRules.some((r) => /^Disallow:\s*\/_app\//i.test(r)),
			hasWildcardUserAgentPatterns: robotsRules.some((r) => /^User-agent:\s*\*-[a-z]+/i.test(r)),
			sitemapsDeclared: robotsRules.filter((r) => /^Sitemap:/i.test(r)),
			rawBody: robotsBody
		},
		sitemaps: {
			main: mainSitemap,
			news: newsSitemap,
			video: videoSitemap
		},
		derivedChecks: {
			canonicalMismatches,
			duplicateTitles,
			duplicateDescriptions,
			duplicateBodyHashes,
			orphanIndexablePages,
			clickDepthByUrl,
			brokenInternalLinks,
			redirectInternalLinks,
			soft404Probes,
			queryParamProbes,
			acceptLanguageProbes,
			viewportParity,
			footerPolicyLinks,
			pagesWithSelfServingAggregateRating
		},
		axeResultsByTemplate,
		pages
	};

	fs.writeFileSync(baselineOutPath, JSON.stringify(baselineReport, null, 2) + '\n', 'utf8');
	console.log(`Wrote ${baselineOutPath}`);
	console.log('=== BASELINE SUMMARY ===');
	console.log(JSON.stringify(baselineReport.summary, null, 2));
}

main().catch((err) => {
	console.error(err);
	process.exit(1);
});
