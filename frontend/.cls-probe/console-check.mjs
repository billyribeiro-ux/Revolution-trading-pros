import { chromium } from '@playwright/test';
const b = await chromium.launch();
const c = await b.newContext();
await c.addCookies([{ name: 'rtp_access_token', value: 'x', domain: 'localhost', path: '/' }]);
const p = await c.newPage();
const errs = [];
p.on('console', (m) => {
	if (m.type() === 'error' || m.type() === 'warning') errs.push(m.text().slice(0, 160));
});
p.on('pageerror', (e) => errs.push('PAGEERR: ' + e.message.slice(0, 160)));
await p.goto('http://localhost:5173/dashboard', { waitUntil: 'networkidle' });
await p.waitForTimeout(2000);
console.log(
	'hydration/mismatch hits:',
	JSON.stringify(
		errs.filter((e) => /hydrat|mismatch|did not match/i.test(e)),
		null,
		2
	)
);
console.log('TOTAL console errors/warnings:', errs.length);
console.log('sample:', JSON.stringify(errs.slice(0, 6), null, 2));
await b.close();
