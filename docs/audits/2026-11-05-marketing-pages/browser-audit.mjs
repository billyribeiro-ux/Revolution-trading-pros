import { chromium } from '../../../frontend/node_modules/playwright/index.mjs';
import AxeBuilder from '../../../frontend/node_modules/@axe-core/playwright/dist/index.mjs';
import { mkdir, writeFile } from 'node:fs/promises';

const origin = process.env.AUDIT_ORIGIN;
if (!origin)
	throw new Error('Set AUDIT_ORIGIN to the deployment or production preview being audited');
const label = process.env.AUDIT_LABEL || 'live';
const output = new URL('./artifacts/', import.meta.url);
await mkdir(output, { recursive: true });
const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1280, height: 800 } });
await context.tracing.start({ screenshots: true, snapshots: true });
const page = await context.newPage();
const result = {
	origin,
	label,
	console: [],
	exceptions: [],
	network: [],
	pages: [],
	controls: [],
	navigation: []
};
page.on('console', (m) => {
	if (['error', 'warning'].includes(m.type()))
		result.console.push({ type: m.type(), text: m.text() });
});
page.on('pageerror', (e) => result.exceptions.push(e.message));
page.on('requestfailed', (r) =>
	result.network.push({ url: r.url(), failure: r.failure()?.errorText })
);
page.on('response', (r) => {
	if (r.status() >= 400 || r.request().resourceType() === 'fetch')
		result.network.push({ url: r.url(), status: r.status() });
});
const ready = async () => {
	await page.locator('main h1').waitFor();
	await page.evaluate(() =>
		Promise.race([document.fonts.ready, new Promise((r) => setTimeout(r, 3000))])
	);
	await page.evaluate(
		() => new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))
	);
};
try {
	result.version = await (await context.request.get(`${origin}/_app/version.json`)).text();
	for (const path of ['/about', '/alerts/explosive-swings']) {
		const response = await page.goto(origin + path, { waitUntil: 'domcontentloaded' });
		await ready();
		result.pages.push({
			path,
			status: response.status(),
			headers: Object.fromEntries(
				Object.entries(response.headers()).filter(([k]) =>
					['etag', 'last-modified', 'x-robots-tag', 'cf-cache-status'].includes(k)
				)
			)
		});
		for (const width of [
			320, 390, 639, 640, 767, 768, 1023, 1024, 1151, 1152, 1279, 1280, 1439, 1440, 1535,
			1536, 1920
		]) {
			await page.setViewportSize({ width, height: 800 });
			await page.evaluate(() => window.scrollTo({ top: 0, behavior: 'instant' }));
			await ready();
			result.pages.push(
				await page.evaluate(
					({ path, width }) => {
						const h1 = document.querySelector('h1');
						const style = getComputedStyle(h1);
						const rect = (e) => {
							const r = e.getBoundingClientRect();
							return {
								x: r.x,
								y: r.y,
								width: r.width,
								height: r.height,
								right: r.right,
								bottom: r.bottom
							};
						};
						const clipped = [];
						for (const e of document.querySelectorAll(
							'main h1, main h2, main h3, main button, main a, main p'
						)) {
							if (
								!e.getClientRects().length ||
								getComputedStyle(e).visibility === 'hidden'
							)
								continue;
							const r = e.getBoundingClientRect();
							if (r.width && (r.left < -1 || r.right > innerWidth + 1))
								clipped.push({
									text: e.textContent.trim().slice(0, 90),
									class: e.className,
									rect: rect(e)
								});
						}
						return {
							path,
							width,
							documentWidth: document.documentElement.scrollWidth,
							h1: rect(h1),
							font: style.fontFamily,
							tokenSans: style.getPropertyValue('--rtp-font-sans'),
							tokenHeading: style.getPropertyValue('--rtp-font-heading'),
							clipped,
							ticker: document.querySelector('.ticker')
								? rect(document.querySelector('.ticker'))
								: null,
							nav: rect(document.querySelector('nav'))
						};
					},
					{ path, width }
				)
			);
			if ([320, 768, 1280, 1920].includes(width))
				await page.screenshot({
					path: new URL(
						`${label}-${path.includes('about') ? 'about' : 'swings'}-${width}.png`,
						output
					).pathname,
					fullPage: width === 1280
				});
		}
		await page.setViewportSize({ width: 390, height: 560 });
		const faq = page.locator('.faq__trigger').first();
		if (await faq.count()) {
			await faq.click();
			result.controls.push({
				path,
				type: 'faq-open',
				expanded: await faq.getAttribute('aria-expanded')
			});
			await faq.focus();
			await page.keyboard.press('Enter');
			result.controls.push({
				path,
				type: 'faq-keyboard-close',
				expanded: await faq.getAttribute('aria-expanded')
			});
		}
		if (path.includes('swings')) {
			for (const name of ['Monthly', 'Annual', 'Quarterly']) {
				const button = page.locator('#pricing').getByRole('button', { name, exact: true });
				await button.click();
				result.controls.push({
					path,
					type: 'plan',
					name,
					ariaPressed: await button.getAttribute('aria-pressed'),
					bounds: await button.boundingBox()
				});
			}
			await page
				.locator('#pricing')
				.screenshot({ path: new URL(`${label}-pricing-390.png`, output).pathname });
		}
		const axe = await new AxeBuilder({ page }).include('main').analyze();
		result.pages.push({
			path,
			axe: axe.violations.map((v) => ({
				id: v.id,
				impact: v.impact,
				help: v.help,
				nodes: v.nodes.map((n) => ({ html: n.html, summary: n.failureSummary }))
			}))
		});
		await page.emulateMedia({ reducedMotion: 'reduce' });
		await page.reload({ waitUntil: 'domcontentloaded' });
		await ready();
		result.pages.push(
			await page.evaluate(
				(path) => ({
					path,
					reducedMotionAnimations: document
						.getAnimations()
						.filter((a) => a.playState === 'running')
						.map((a) => ({
							target: a.effect?.target?.className,
							duration: a.effect?.getTiming().duration,
							iterations: a.effect?.getTiming().iterations
						}))
						.slice(0, 35)
				}),
				path
			)
		);
		await page.evaluate(() => (document.documentElement.style.fontSize = '32px'));
		result.pages.push(
			await page.evaluate(
				(path) => ({
					path,
					largerText: {
						viewport: innerWidth,
						documentWidth: document.documentElement.scrollWidth,
						h1Width: document.querySelector('h1').getBoundingClientRect().width,
						h1Scroll: document.querySelector('h1').scrollWidth
					}
				}),
				path
			)
		);
		await page.reload({ waitUntil: 'domcontentloaded' });
		await ready();
		await page.emulateMedia({ reducedMotion: 'no-preference' });
	}
	await page.setViewportSize({ width: 1280, height: 800 });
	for (let i = 0; i < 3; i++) {
		await page.getByRole('link', { name: 'About Us', exact: true }).last().click();
		await ready();
		result.navigation.push({
			action: 'footer-about',
			url: page.url(),
			h1: await page.locator('h1').innerText()
		});
		// Use the public alert-services hub to reach the service via actual links.
		await page.getByRole('link', { name: 'Trading Alerts', exact: true }).last().click();
		await ready();
		await page.locator('a[href="/alerts/explosive-swings"]').first().click();
		await ready();
		result.navigation.push({
			action: 'hub-swings',
			url: page.url(),
			h1: await page.locator('h1').innerText()
		});
		await page.goBack();
		await ready();
		await page.goForward();
		await ready();
	}
} catch (error) {
	result.auditError = String(error.stack);
} finally {
	await writeFile(new URL(`${label}-browser.json`, output), JSON.stringify(result, null, 2));
	await context.tracing.stop({ path: new URL(`${label}-trace.zip`, output).pathname });
	await browser.close();
	console.log(
		JSON.stringify({
			label,
			pages: result.pages.length,
			controls: result.controls,
			navigation: result.navigation.length,
			exceptions: result.exceptions,
			errors: result.console,
			auditError: result.auditError
		})
	);
}
