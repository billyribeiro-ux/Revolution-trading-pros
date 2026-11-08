import { chromium } from '../../../frontend/node_modules/playwright/index.mjs';
import AxeBuilder from '../../../frontend/node_modules/@axe-core/playwright/dist/index.mjs';
import { mkdir, writeFile } from 'node:fs/promises';
const origin = process.env.AUDIT_ORIGIN;
if (!origin) throw Error('AUDIT_ORIGIN required');
const label = process.env.AUDIT_LABEL || 'before';
const out = new URL(`./artifacts/${label}/`, import.meta.url);
await mkdir(out, { recursive: true });
const browser = await chromium.launch();
const context = await browser.newContext({ reducedMotion: 'reduce' });
const page = await context.newPage();
const result = { origin, label, routes: [] };
try {
	for (const path of [
		'/privacy',
		'/terms',
		'/refund',
		'/disclaimer',
		'/cookie-policy',
		'/contact'
	]) {
		const row = { path, exceptions: [], widths: [] };
		const error = (e) => row.exceptions.push(e.message);
		page.on('pageerror', error);
		await page.setViewportSize({ width: 1280, height: 800 });
		const response = await page.goto(origin + path, { waitUntil: 'domcontentloaded' });
		await page.locator('h1').waitFor();
		await page.evaluate(() =>
			Promise.race([
				document.fonts.ready,
				new Promise((resolve) => setTimeout(resolve, 2000))
			])
		);
		row.status = response.status();
		row.destination = page.url();
		row.typography = await page.locator('main').evaluate((main) => {
			const style = (sel) => {
				const e = main.querySelector(sel);
				if (!e) return null;
				const s = getComputedStyle(e),
					r = e.getBoundingClientRect();
				return {
					font: s.fontFamily,
					size: s.fontSize,
					weight: s.fontWeight,
					lineHeight: s.lineHeight,
					width: r.width,
					x: r.x
				};
			};
			return { h1: style('h1'), h2: style('h2'), body: style('p:not([class*="label"])') };
		});
		for (const width of [320, 390, 639, 640, 767, 768, 1023, 1024, 1280, 1920]) {
			await page.setViewportSize({ width, height: 800 });
			row.widths.push(
				await page.locator('main').evaluate((main) => {
					const width = document.documentElement.clientWidth;
					const problems = [];
					for (const e of main.querySelectorAll('h1,h2,h3,p,a,button,input,select')) {
						if (!e.getClientRects().length || e.closest('[aria-hidden=true]')) continue;
						let n = e;
						let scroll = false;
						while (n && n !== main) {
							const s = getComputedStyle(n);
							if (/auto|scroll/.test(s.overflowX) && n.scrollWidth > n.clientWidth) {
								scroll = true;
								break;
							}
							n = n.parentElement;
						}
						if (scroll) continue;
						const r = e.getBoundingClientRect();
						if (r.left < -1 || r.right > width + 1)
							problems.push({
								tag: e.tagName,
								text: e.textContent.trim().slice(0, 90),
								left: r.left,
								right: r.right
							});
					}
					return { width, problems };
				})
			);
			if ([390, 1280].includes(width))
				await page.screenshot({
					path: new URL(`${path.slice(1)}-${width}.png`, out).pathname
				});
		}
		await page.setViewportSize({ width: 390, height: 560 });
		await page.evaluate(() => (document.documentElement.style.fontSize = '32px'));
		row.enlarged = await page.locator('h1').evaluate((e) => {
			const range = document.createRange();
			range.selectNodeContents(e);
			return [...range.getClientRects()].map((r) => ({ left: r.left, right: r.right }));
		});
		await page.reload({ waitUntil: 'domcontentloaded' });
		await page.setViewportSize({ width: 1280, height: 800 });
		row.axe = (
			await new AxeBuilder({ page }).include('#main-content').analyze()
		).violations.map((v) => ({
			id: v.id,
			nodes: v.nodes.map((n) => ({ html: n.html, summary: n.failureSummary }))
		}));
		result.routes.push(row);
		page.off('pageerror', error);
		console.log(JSON.stringify(row));
	}
	await writeFile(new URL('summary.json', out), JSON.stringify(result, null, 2));
} finally {
	await browser.close();
}
