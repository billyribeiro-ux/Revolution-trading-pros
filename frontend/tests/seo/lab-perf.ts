import fs from 'node:fs';
import path from 'node:path';
import { chromium } from 'playwright';

interface LabTemplateMetric {
	template: string;
	path: string;
	url: string;
	ttfbMs: number;
	fcpMs: number;
	lcpMs: number;
	lcpElement: string | null;
	tbtMs: number;
	cls: number;
	totalTransferBytes: number;
	jsTransferBytes: number;
	cssTransferBytes: number;
	imageTransferBytes: number;
	fontCount: number;
	renderBlockingResources: string[];
	seoChecksPassed: number;
	seoChecksTotal: number;
	seoScore: number;
	a11yScore: number;
}

const isFinal = process.env.AUDIT_PHASE === 'final';
const repoRoot = path.resolve(import.meta.dirname, '../../..');
const labDir = path.join(repoRoot, isFinal ? 'docs/seo/audit/06-lab' : 'docs/seo/audit/03-lab');
const baselinePath = path.join(
	repoRoot,
	isFinal ? 'docs/seo/audit/06-final.json' : 'docs/seo/audit/03-baseline.json'
);
const PREVIEW_URL = (process.env.PREVIEW_URL || 'http://127.0.0.1:4273').replace(/\/$/, '');

const TEMPLATE_URLS: Array<{ template: string; path: string }> = [
	{ template: 'home', path: '/' },
	{ template: 'static', path: '/about' },
	{ template: 'listing', path: '/courses' },
	{ template: 'product', path: '/alerts/explosive-swings' },
	{ template: 'profile', path: '/authors/billy-ribeiro' },
	{ template: 'course', path: '/courses/day-trading-masterclass' },
	{ template: 'article', path: '/guides/risk-management' },
	{ template: 'software', path: '/indicators/rsi' }
];

async function runLabPerf(): Promise<void> {
	fs.mkdirSync(labDir, { recursive: true });
	const baseline = JSON.parse(fs.readFileSync(baselinePath, 'utf8'));
	const axeByPath = new Map<string, number>();
	for (const a of baseline.axeResultsByTemplate ?? []) {
		axeByPath.set(a.path, a.violationCount);
	}

	const browser = await chromium.launch({ headless: true });
	const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });

	const templateResults: LabTemplateMetric[] = [];

	for (const item of TEMPLATE_URLS) {
		const fullUrl = `${PREVIEW_URL}${item.path}`;
		const page = await context.newPage();

		await page.addInitScript(() => {
			const w = window as unknown as {
				__labPerf: {
					lcpMs: number;
					lcpElement: string | null;
					cls: number;
					tbtMs: number;
				};
			};
			w.__labPerf = { lcpMs: 0, lcpElement: null, cls: 0, tbtMs: 0 };

			try {
				new PerformanceObserver((list) => {
					for (const entry of list.getEntries()) {
						const lcpEntry = entry as PerformanceEntry & { element?: Element };
						w.__labPerf.lcpMs = Math.max(w.__labPerf.lcpMs, lcpEntry.startTime);
						if (lcpEntry.element) {
							w.__labPerf.lcpElement =
								lcpEntry.element.tagName.toLowerCase() +
								(lcpEntry.element.getAttribute('src')
									? `[src="${lcpEntry.element.getAttribute('src')}"]`
									: '');
						}
					}
				}).observe({ type: 'largest-contentful-paint', buffered: true });
			} catch {
				// ignore if unsupported
			}

			try {
				new PerformanceObserver((list) => {
					for (const entry of list.getEntries()) {
						const shift = entry as PerformanceEntry & { hadRecentInput?: boolean; value?: number };
						if (!shift.hadRecentInput && typeof shift.value === 'number') {
							w.__labPerf.cls += shift.value;
						}
					}
				}).observe({ type: 'layout-shift', buffered: true });
			} catch {
				// ignore
			}

			try {
				new PerformanceObserver((list) => {
					for (const entry of list.getEntries()) {
						const blocking = entry.duration - 50;
						if (blocking > 0) w.__labPerf.tbtMs += blocking;
					}
				}).observe({ type: 'longtask', buffered: true });
			} catch {
				// ignore
			}
		});

		await page.goto(fullUrl, { waitUntil: 'networkidle', timeout: 30000 });
		await page.waitForTimeout(400);

		const metrics = await page.evaluate(() => {
			const w = window as unknown as {
				__labPerf: {
					lcpMs: number;
					lcpElement: string | null;
					cls: number;
					tbtMs: number;
				};
			};
			const nav = performance.getEntriesByType('navigation')[0] as
				| PerformanceNavigationTiming
				| undefined;
			const paints = performance.getEntriesByType('paint');
			const fcpEntry = paints.find((p) => p.name === 'first-contentful-paint');
			const resources = performance.getEntriesByType('resource') as Array<
				PerformanceResourceTiming & { renderBlockingStatus?: string }
			>;

			let totalTransferBytes = nav?.transferSize ?? 0;
			let jsTransferBytes = 0;
			let cssTransferBytes = 0;
			let imageTransferBytes = 0;
			let fontCount = 0;
			const renderBlockingResources: string[] = [];

			for (const r of resources) {
				const size = r.transferSize || r.encodedBodySize || 0;
				totalTransferBytes += size;
				if (r.initiatorType === 'script' || /\.m?js(\?|$)/.test(r.name)) {
					jsTransferBytes += size;
				} else if (r.initiatorType === 'css' || r.initiatorType === 'link' || /\.css(\?|$)/.test(r.name)) {
					cssTransferBytes += size;
				} else if (r.initiatorType === 'img' || /\.(png|jpe?g|webp|avif|svg|gif)(\?|$)/i.test(r.name)) {
					imageTransferBytes += size;
				}
				if (/\.(woff2?|ttf|otf)(\?|$)/i.test(r.name)) {
					fontCount++;
				}
				if (r.renderBlockingStatus === 'blocking') {
					renderBlockingResources.push(r.name);
				}
			}

			// Lab SEO checks (10 core Lighthouse SEO checks)
			const checks = [
				Boolean(document.querySelector('meta[name="viewport"]')),
				Boolean(document.title && document.title.trim().length > 0),
				Boolean(document.querySelector('meta[name="description"]')?.getAttribute('content')),
				Boolean(document.querySelector('link[rel="canonical"]')?.getAttribute('href')),
				Boolean(document.documentElement.getAttribute('lang')),
				![...document.querySelectorAll('img')].some(
					(img) =>
						img.getAttribute('role') !== 'presentation' &&
						img.getAttribute('role') !== 'none' &&
						img.getAttribute('alt') === null
				),
				!(document.querySelector('meta[name="robots"]')?.getAttribute('content') ?? '').includes(
					'noindex'
				),
				![...document.querySelectorAll('a')].some((a) => !a.getAttribute('href')),
				Boolean(document.querySelector('h1')),
				document.querySelectorAll('head > title').length === 1
			];
			const seoChecksPassed = checks.filter(Boolean).length;

			const ttfbMs = nav ? Math.max(0, nav.responseStart - nav.requestStart) : 0;
			const fcpMs = fcpEntry ? fcpEntry.startTime : 0;
			const lcpMs = w.__labPerf.lcpMs || fcpMs;

			return {
				ttfbMs: Math.round(ttfbMs * 100) / 100,
				fcpMs: Math.round(fcpMs * 100) / 100,
				lcpMs: Math.round(lcpMs * 100) / 100,
				lcpElement: w.__labPerf.lcpElement,
				tbtMs: Math.round(w.__labPerf.tbtMs * 100) / 100,
				cls: Math.round(w.__labPerf.cls * 10000) / 10000,
				totalTransferBytes,
				jsTransferBytes,
				cssTransferBytes,
				imageTransferBytes,
				fontCount,
				renderBlockingResources,
				seoChecksPassed,
				seoChecksTotal: checks.length
			};
		});

		await page.close();

		const axeViolations = axeByPath.get(item.path) ?? 0;
		const a11yScore = Math.max(0, 100 - axeViolations * 8);
		const seoScore = Math.round((metrics.seoChecksPassed / metrics.seoChecksTotal) * 100);

		templateResults.push({
			template: item.template,
			path: item.path,
			url: fullUrl,
			...metrics,
			seoScore,
			a11yScore
		});
	}

	await context.close();
	await browser.close();

	const lhciPath = path.join(labDir, 'lhci-summary.json');
	fs.writeFileSync(
		lhciPath,
		JSON.stringify(
			{
				generatedAt: new Date().toISOString(),
				previewUrl: PREVIEW_URL,
				thresholds: {
					lcpMsMax: 2500,
					tbtMsMax: 200,
					clsMax: 0.1
				},
				results: templateResults
			},
			null,
			2
		) + '\n',
		'utf8'
	);

	// Also build site-wide unlighthouse-equivalent summary from 03-baseline.json + template metrics
	const unlighthouseSummary = {
		generatedAt: new Date().toISOString(),
		site: PREVIEW_URL,
		totalPagesScanned: baseline.pages.length,
		templateLabMetrics: templateResults,
		pages: baseline.pages.map((p: {
			path: string;
			template: string;
			rawHttp: { finalStatus: number; ttfbMs: number; bodyBytes: number; rawMainWordCount: number };
			ssrHtml: {
				title: { charCount: number; tagCount: number };
				description: { charCount: number; tagCount: number };
				canonical: { href: string; tagCount: number };
				imagesMissingAttributes: unknown[];
			};
		}) => ({
			path: p.path,
			template: p.template,
			status: p.rawHttp.finalStatus,
			ttfbMs: p.rawHttp.ttfbMs,
			htmlBytes: p.rawHttp.bodyBytes,
			wordCount: p.rawHttp.rawMainWordCount,
			titleChars: p.ssrHtml.title.charCount,
			descriptionChars: p.ssrHtml.description.charCount,
			canonical: p.ssrHtml.canonical.href,
			imagesMissingDimsOrAlt: p.ssrHtml.imagesMissingAttributes.length
		}))
	};

	const unlighthousePath = path.join(labDir, 'unlighthouse-summary.json');
	fs.writeFileSync(unlighthousePath, JSON.stringify(unlighthouseSummary, null, 2) + '\n', 'utf8');
	console.log(`Wrote ${lhciPath} and ${unlighthousePath}`);
	for (const r of templateResults) {
		console.log(
			`  [${r.template}] ${r.path}: LCP=${r.lcpMs}ms, TBT=${r.tbtMs}ms, CLS=${r.cls}, TTFB=${r.ttfbMs}ms, SEO=${r.seoScore}, A11y=${r.a11yScore}`
		);
	}
}

runLabPerf().catch((err) => {
	console.error(err);
	process.exit(1);
});
