// Mobile CLS measurement. Usage: node measure-cls-mobile.mjs <url> <label> [width] [height]
import { chromium } from '@playwright/test';

const URL = process.argv[2] || 'http://localhost:5173/';
const LABEL = process.argv[3] || 'run';
const W = Number(process.argv[4] || 390);
const H = Number(process.argv[5] || 844);

const initScript = () => {
	window.__cls = 0;
	window.__shifts = [];
	new PerformanceObserver((list) => {
		for (const e of list.getEntries()) {
			if (e.hadRecentInput) continue;
			window.__cls += e.value;
			const sources = (e.sources || []).map((s) => {
				const n = s.node;
				let d = 'unknown';
				if (n && n.nodeType === 1) {
					d = n.tagName.toLowerCase();
					if (n.id) d += '#' + n.id;
					if (n.className && typeof n.className === 'string')
						d += '.' + n.className.trim().split(/\s+/).slice(0, 3).join('.');
				}
				return {
					node: d,
					fromY: s.previousRect ? Math.round(s.previousRect.y) : null,
					toY: s.currentRect ? Math.round(s.currentRect.y) : null,
					fromH: s.previousRect ? Math.round(s.previousRect.height) : null,
					toH: s.currentRect ? Math.round(s.currentRect.height) : null
				};
			});
			window.__shifts.push({
				value: +e.value.toFixed(4),
				t: Math.round(e.startTime),
				sources
			});
		}
	}).observe({ type: 'layout-shift', buffered: true });
};

const b = await chromium.launch();
const ctx = await b.newContext({
	viewport: { width: W, height: H },
	userAgent:
		'Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 Mobile/15E148',
	isMobile: true,
	hasTouch: true
});
await ctx.addCookies([{ name: 'rtp_access_token', value: 'mock', domain: 'localhost', path: '/' }]);
const p = await ctx.newPage();
await p.addInitScript(initScript);
const resp = await p.goto(URL, { waitUntil: 'networkidle', timeout: 30000 });
await p.waitForTimeout(2500);
const r = await p.evaluate(() => ({ cls: window.__cls, shifts: window.__shifts }));
console.log(
	JSON.stringify(
		{
			label: LABEL,
			url: URL,
			viewport: `${W}x${H}`,
			status: resp?.status(),
			finalURL: p.url(),
			totalCLS: +r.cls.toFixed(4),
			shiftCount: r.shifts.length,
			shifts: r.shifts.sort((a, b2) => b2.value - a.value).slice(0, 8)
		},
		null,
		2
	)
);
await b.close();
