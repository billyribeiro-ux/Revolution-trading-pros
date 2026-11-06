import { chromium } from '../../../frontend/node_modules/playwright/index.mjs';
import { writeFile } from 'node:fs/promises';

const origin = 'http://127.0.0.1:4174';
const artifacts = new URL('./artifacts/', import.meta.url);
const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 390, height: 560 } });
const page = await context.newPage();
const result = { keyboard: [], anchors: [], links: [], text: [], listeners: [], exceptions: [] };
page.on('pageerror', (e) => result.exceptions.push(e.message));
const ready = async () => {
	await page.locator('main h1').waitFor();
	await page.evaluate(() =>
		Promise.race([document.fonts.ready, new Promise((resolve) => setTimeout(resolve, 3000))])
	);
};
try {
	for (const path of ['/about', '/alerts/explosive-swings']) {
		await page.goto(origin + path, { waitUntil: 'domcontentloaded' });
		await ready();
		await page.keyboard.press('Tab');
		await page.keyboard.press('Enter');
		result.keyboard.push({
			path,
			skip: await page.evaluate(() => ({
				hash: location.hash,
				focused: document.activeElement?.id
			}))
		});
		const menu = page.getByRole('button', { name: 'Open menu', exact: true });
		await menu.click();
		await page.getByRole('dialog').waitFor();
		await page.waitForFunction(() => document.activeElement?.closest('[role="dialog"]'), {
			timeout: 5000
		});
		await page.keyboard.press('Escape');
		await page.getByRole('dialog').waitFor({ state: 'detached' });
		result.keyboard.push({
			path,
			menuClosed: true,
			scrollLock: await page.evaluate(() => ({
				html: getComputedStyle(document.documentElement).overflow,
				body: getComputedStyle(document.body).overflow
			}))
		});
		for (const button of await page.locator('.faq__trigger').all()) {
			await button.focus();
			await page.keyboard.press('Enter');
			if ((await button.getAttribute('aria-expanded')) !== 'true')
				throw new Error('FAQ did not open');
			await page.keyboard.press('Enter');
			if ((await button.getAttribute('aria-expanded')) !== 'false')
				throw new Error('FAQ did not close');
		}
		result.keyboard.push({
			path,
			faqs: await page.locator('.faq__trigger').count(),
			keyboardToggle: true
		});
		const selector = path === '/about' ? 'a[href="#how-it-works"]' : 'a[href="#methodology"]';
		await page.locator(selector).first().click();
		result.anchors.push({
			path,
			hash: new URL(page.url()).hash,
			targetExists: await page.evaluate(
				() => !!document.getElementById(location.hash.slice(1))
			)
		});
		const hrefs = await page
			.locator('a[href^="/"]')
			.evaluateAll((es) => [
				...new Set(es.map((e) => e.getAttribute('href')).filter((h) => !h.startsWith('//')))
			]);
		for (const href of hrefs) {
			try {
				const response = await context.request.get(origin + href, { timeout: 5000 });
				result.links.push({
					path,
					href,
					status: response.status(),
					destination: response.url()
				});
			} catch (error) {
				result.links.push({ path, href, error: String(error) });
			}
		}
		await page.evaluate(() => (document.documentElement.style.fontSize = '32px'));
		const overflow = await page
			.locator('main h1, main h2, main h3, main p, main a, main button')
			.evaluateAll((es) =>
				es.flatMap((e) => {
					if (!e.getClientRects().length) return [];
					// Element ranges also include offscreen decorative shimmer boxes.
					// Measure actual text nodes to avoid reporting those as clipped copy.
					const walker = document.createTreeWalker(e, NodeFilter.SHOW_TEXT);
					const textRects = [];
					while (walker.nextNode()) {
						const node = walker.currentNode;
						if (
							!node.textContent.trim() ||
							node.parentElement?.closest('[aria-hidden="true"]')
						)
							continue;
						const range = document.createRange();
						range.selectNodeContents(node);
						textRects.push(...range.getClientRects());
					}
					const rects = textRects.filter(
						(r) =>
							r.width > 0 &&
							(r.left < 0 || r.right > document.documentElement.clientWidth)
					);
					return rects.length
						? [
								{
									text: e.textContent.trim().slice(0, 90),
									class: e.className,
									left: Math.min(...rects.map((r) => r.left)),
									right: Math.max(...rects.map((r) => r.right))
								}
							]
						: [];
				})
			);
		result.text.push({ path, viewport: 390, fontSize: '32px', overflow });
		await page.evaluate(() => window.scrollTo({ top: 0, behavior: 'instant' }));
		await page.screenshot({
			path: new URL(
				`after-${path === '/about' ? 'about' : 'swings'}-large-text.png`,
				artifacts
			).pathname,
			fullPage: false
		});
	}
	await page.setViewportSize({ width: 1280, height: 800 });
	await page.goto(origin + '/alerts/explosive-swings');
	await ready();
	const cdp = await context.newCDPSession(page);
	async function listeners() {
		const counts = {};
		for (const target of ['window', 'document']) {
			const { result: obj } = await cdp.send('Runtime.evaluate', { expression: target });
			const response = await cdp.send('DOMDebugger.getEventListeners', {
				objectId: obj.objectId
			});
			counts[target] = response.listeners.reduce((map, l) => {
				map[l.type] = (map[l.type] || 0) + 1;
				return map;
			}, {});
			await cdp.send('Runtime.releaseObject', { objectId: obj.objectId });
		}
		return counts;
	}
	for (let i = 0; i < 6; i++) {
		await page.getByRole('link', { name: 'About Us', exact: true }).last().click();
		await ready();
		await page.getByRole('link', { name: 'Trading Alerts', exact: true }).last().click();
		await ready();
		await page.locator('a[href="/alerts/explosive-swings"]').first().click();
		await ready();
		result.listeners.push({ cycle: i, ...(await listeners()) });
	}
	result.serviceWorkers = context.serviceWorkers().map((w) => w.url());
} catch (error) {
	result.error = String(error.stack);
} finally {
	await writeFile(new URL('after-interactions.json', artifacts), JSON.stringify(result, null, 2));
	console.log(JSON.stringify(result));
	await browser.close();
}
