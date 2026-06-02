// Measure layout shift caused by the hamburger open/close (interaction-driven,
// which the CLS metric excludes but users still see).
// Usage: node measure-hamburger.mjs <url> [width] [height]
import { chromium } from '@playwright/test';

const URL = process.argv[2] || 'http://localhost:5173/';
const W = Number(process.argv[3] || 390);
const H = Number(process.argv[4] || 844);

const b = await chromium.launch();
const ctx = await b.newContext({
	viewport: { width: W, height: H },
	userAgent: 'Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 Mobile/15E148',
	isMobile: true, hasTouch: true
});
const p = await ctx.newPage();
await p.goto(URL, { waitUntil: 'networkidle', timeout: 30000 });
await p.waitForTimeout(800);

// pick a stable hero/content reference inside <main>
const probe = async () => p.evaluate(() => {
	const main = document.querySelector('main#main-content') || document.querySelector('main');
	const hero = main?.querySelector('section, h1, .hero, [class*="hero"]') || main?.firstElementChild;
	const r = hero?.getBoundingClientRect();
	const html = document.documentElement;
	return {
		heroTag: hero ? hero.tagName.toLowerCase() + '.' + (typeof hero.className === 'string' ? hero.className.trim().split(/\s+/).slice(0,2).join('.') : '') : null,
		heroX: r ? +r.x.toFixed(2) : null, heroY: r ? +r.y.toFixed(2) : null, heroW: r ? +r.width.toFixed(2) : null,
		htmlPaddingRight: html.style.paddingRight || '0px',
		htmlOverflow: html.style.overflow || '',
		scrollbarW: window.innerWidth - html.clientWidth,
		scrollY: window.scrollY
	};
});

// scroll a little so the sticky nav is engaged like a real reader
await p.evaluate(() => window.scrollTo(0, 200));
await p.waitForTimeout(300);

const before = await probe();
const ham = await p.$('.hamburger, button[aria-controls="mobile-nav"]');
if (!ham) { console.log(JSON.stringify({ error: 'hamburger not found', before })); await b.close(); process.exit(0); }

await ham.click();           // OPEN
await p.waitForTimeout(500);
const opened = await probe();

// close via the hamburger again (it toggles to X) or the close button
const closeBtn = await p.$('.mobile-close') || ham;
await closeBtn.click();      // CLOSE
await p.waitForTimeout(600);
const closed = await probe();

console.log(JSON.stringify({
	url: URL, viewport: `${W}x${H}`,
	before, opened, closed,
	heroShift_open_x: opened.heroX !== null && before.heroX !== null ? +(opened.heroX - before.heroX).toFixed(2) : null,
	heroShift_open_y: opened.heroY !== null && before.heroY !== null ? +(opened.heroY - before.heroY).toFixed(2) : null,
	heroShift_afterClose_x: closed.heroX !== null && before.heroX !== null ? +(closed.heroX - before.heroX).toFixed(2) : null,
	heroShift_afterClose_y: closed.heroY !== null && before.heroY !== null ? +(closed.heroY - before.heroY).toFixed(2) : null
}, null, 2));
await b.close();
