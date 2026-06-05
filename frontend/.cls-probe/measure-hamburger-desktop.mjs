// Hamburger open/close shift in a DESKTOP context (real scrollbars present).
import { chromium } from '@playwright/test';
const URL = process.argv[2] || 'http://localhost:5173/';
const W = Number(process.argv[3] || 900);
const H = Number(process.argv[4] || 800);

const b = await chromium.launch();
const ctx = await b.newContext({ viewport: { width: W, height: H } });
const p = await ctx.newPage();
await p.goto(URL, { waitUntil: 'networkidle', timeout: 30000 });
await p.waitForTimeout(800);

const probe = async () =>
	p.evaluate(() => {
		const main = document.querySelector('main#main-content') || document.querySelector('main');
		const hero =
			main?.querySelector('section, h1, .hero, [class*="hero"]') || main?.firstElementChild;
		const r = hero?.getBoundingClientRect();
		const html = document.documentElement;
		return {
			heroTag: hero ? hero.tagName.toLowerCase() : null,
			heroX: r ? +r.x.toFixed(2) : null,
			heroY: r ? +r.y.toFixed(2) : null,
			heroW: r ? +r.width.toFixed(2) : null,
			htmlPaddingRight: html.style.paddingRight || '0px',
			htmlOverflow: html.style.overflow || '',
			scrollbarW: window.innerWidth - html.clientWidth,
			scrollY: window.scrollY,
			bodyW: +document.body.getBoundingClientRect().width.toFixed(2)
		};
	});

await p.evaluate(() => window.scrollTo(0, 200));
await p.waitForTimeout(300);
const before = await probe();
const ham = await p.$('.hamburger, button[aria-controls="mobile-nav"]');
if (!ham) {
	console.log(JSON.stringify({ error: 'no hamburger at this width', before }, null, 2));
	await b.close();
	process.exit(0);
}
await ham.click();
await p.waitForTimeout(500);
const opened = await probe();
const closeBtn = (await p.$('.mobile-close')) || ham;
await closeBtn.click();
await p.waitForTimeout(600);
const closed = await probe();
console.log(
	JSON.stringify(
		{
			url: URL,
			viewport: `${W}x${H}`,
			before,
			opened,
			closed,
			heroDX_open: before.heroX != null ? +(opened.heroX - before.heroX).toFixed(2) : null,
			heroDY_open: before.heroY != null ? +(opened.heroY - before.heroY).toFixed(2) : null,
			heroDW_open: before.heroW != null ? +(opened.heroW - before.heroW).toFixed(2) : null,
			heroDX_close: before.heroX != null ? +(closed.heroX - before.heroX).toFixed(2) : null
		},
		null,
		2
	)
);
await b.close();
