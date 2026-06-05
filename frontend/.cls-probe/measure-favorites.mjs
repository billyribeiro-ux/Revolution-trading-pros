// Measures real CLS on a favorites route. Usage:
//   node .cls-probe/measure-favorites.mjs <baseURL> <label> [path]
// Default path is the explosive-swings favorites page.
import { chromium } from '@playwright/test';

const BASE = process.argv[2] || 'http://localhost:4173';
const LABEL = process.argv[3] || 'run';
const PATH = process.argv[4] || '/dashboard/explosive-swings/favorites';

const initScript = () => {
	// @ts-nocheck
	window.__clsValue = 0;
	window.__clsShifts = [];
	const obs = new PerformanceObserver((list) => {
		for (const entry of list.getEntries()) {
			if (entry.hadRecentInput) continue;
			window.__clsValue += entry.value;
			const sources = (entry.sources || []).map((s) => {
				const n = s.node;
				let desc = 'unknown';
				if (n && n.nodeType === 1) {
					desc = n.tagName.toLowerCase();
					if (n.id) desc += '#' + n.id;
					if (n.className && typeof n.className === 'string')
						desc += '.' + n.className.trim().split(/\s+/).slice(0, 3).join('.');
				}
				return {
					node: desc,
					from: s.previousRect ? `${Math.round(s.previousRect.y)}` : '?',
					to: s.currentRect ? `${Math.round(s.currentRect.y)}` : '?'
				};
			});
			window.__clsShifts.push({
				value: Number(entry.value.toFixed(4)),
				t: Math.round(entry.startTime),
				sources
			});
		}
	});
	obs.observe({ type: 'layout-shift', buffered: true });
};

const browser = await chromium.launch();
const ctx = await browser.newContext({ viewport: { width: 1280, height: 900 } });
await ctx.addCookies([
	{
		name: 'rtp_access_token',
		value: 'mock-token-for-measurement',
		domain: 'localhost',
		path: '/'
	},
	{ name: 'rtp_refresh_token', value: 'mock-refresh', domain: 'localhost', path: '/' }
]);
const page = await ctx.newPage();
await page.addInitScript(initScript);

const resp = await page.goto(`${BASE}${PATH}`, { waitUntil: 'networkidle', timeout: 30000 });
// Let any late client fetches + their layout settle.
await page.waitForTimeout(2500);

// Is the real favorites list present (vs. a skeleton)? Evidence the data is SSR'd.
const dom = await page.evaluate(() => ({
	cards: document.querySelectorAll('.favorite-card').length,
	skeletons: document.querySelectorAll('[class*="skeleton" i], [class*="loading" i]').length
}));
const cls = await page.evaluate(() => ({ value: window.__clsValue, shifts: window.__clsShifts }));

console.log(
	JSON.stringify(
		{
			label: LABEL,
			path: PATH,
			status: resp?.status(),
			finalURL: page.url(),
			favoriteCards: dom.cards,
			skeletonNodes: dom.skeletons,
			totalCLS: Number(cls.value.toFixed(4)),
			shiftCount: cls.shifts.length,
			shifts: cls.shifts.sort((a, b) => b.value - a.value).slice(0, 8)
		},
		null,
		2
	)
);

await browser.close();
