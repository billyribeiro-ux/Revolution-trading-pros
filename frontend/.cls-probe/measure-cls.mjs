// Measures real CLS on /dashboard against the production preview build.
// Usage: node .cls-probe/measure-cls.mjs <baseURL> <label>
import { chromium } from '@playwright/test';

const BASE = process.argv[2] || 'http://localhost:4173';
const LABEL = process.argv[3] || 'run';

const initScript = () => {
	// @ts-nocheck
	window.__clsValue = 0;
	window.__clsShifts = [];
	const obs = new PerformanceObserver((list) => {
		for (const entry of list.getEntries()) {
			// Only count shifts NOT caused by recent user input (per web-vitals CLS spec)
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

const resp = await page.goto(`${BASE}/dashboard`, { waitUntil: 'networkidle', timeout: 30000 });
// Let any late client fetches + their layout settle.
await page.waitForTimeout(2500);

const finalURL = page.url();
const cls = await page.evaluate(() => ({ value: window.__clsValue, shifts: window.__clsShifts }));

console.log(
	JSON.stringify(
		{
			label: LABEL,
			status: resp?.status(),
			finalURL,
			totalCLS: Number(cls.value.toFixed(4)),
			shiftCount: cls.shifts.length,
			shifts: cls.shifts.sort((a, b) => b.value - a.value).slice(0, 8)
		},
		null,
		2
	)
);

await browser.close();
