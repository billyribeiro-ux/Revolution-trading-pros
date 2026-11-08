import { chromium } from '../../../frontend/node_modules/playwright/index.mjs';
import AxeBuilder from '../../../frontend/node_modules/@axe-core/playwright/dist/index.mjs';
import { readFile, writeFile, mkdir } from 'node:fs/promises';

const origin = process.env.AUDIT_ORIGIN;
if (!origin) throw new Error('Set AUDIT_ORIGIN');
const label = process.env.AUDIT_LABEL || 'baseline';
const out = new URL(`./artifacts/${label}/`, import.meta.url);
await mkdir(out, { recursive: true });
const routes = JSON.parse(await readFile(new URL('./routes.json', import.meta.url), 'utf8'));
const browser = await chromium.launch();
const result = { origin, label, routes: [], discovered: [] };
const selected = process.env.AUDIT_ROUTES?.split(',');
const queue = routes.filter((route) => !selected || selected.includes(route));
const slug = (path) => (path === '/' ? 'home' : path.slice(1).replaceAll('/', '--'));

async function geometry(page) {
	// Wait for entry transforms and loaded font metrics to settle, bounded at five seconds.
	await page.evaluate(
		() =>
			new Promise((resolve) => {
				const start = performance.now();
				let previous = '',
					stable = 0;
				const frame = () => {
					const current = [
						...document.querySelectorAll(
							'#main-content h1,#main-content h2,#main-content button,#main-content a'
						)
					]
						.filter((e) => e.getClientRects().length)
						.map((e) => {
							const r = e.getBoundingClientRect();
							return [
								r.x.toFixed(1),
								r.width.toFixed(1),
								getComputedStyle(e).opacity
							].join(',');
						})
						.join(';');
					stable = current === previous ? stable + 1 : 0;
					previous = current;
					if (stable >= 6 || performance.now() - start > 5000) resolve();
					else requestAnimationFrame(frame);
				};
				requestAnimationFrame(frame);
			})
	);
	return page.locator('#main-content').evaluate((main) => {
		const problems = [];
		const viewport = document.documentElement.clientWidth;
		for (const el of main.querySelectorAll('h1,h2,h3,p,a,button,input,select,textarea')) {
			if (!el.getClientRects().length || el.closest('[aria-hidden="true"], [hidden]'))
				continue;
			const style = getComputedStyle(el);
			if (style.visibility === 'hidden' || style.display === 'none') continue;
			const walker = document.createTreeWalker(el, NodeFilter.SHOW_TEXT);
			const rects = [];
			while (walker.nextNode()) {
				const text = walker.currentNode;
				if (
					!text.textContent.trim() ||
					text.parentElement?.closest('[aria-hidden="true"],svg,.sr-only')
				)
					continue;
				let ancestor = text.parentElement;
				let scrollable = false;
				while (ancestor && ancestor !== main) {
					const css = getComputedStyle(ancestor);
					if (
						/auto|scroll/.test(css.overflowX) &&
						ancestor.scrollWidth > ancestor.clientWidth
					) {
						scrollable = true;
						break;
					}
					ancestor = ancestor.parentElement;
				}
				if (scrollable) continue;
				const range = document.createRange();
				range.selectNodeContents(text);
				rects.push(...range.getClientRects());
			}
			const box = el.getBoundingClientRect();
			if (['INPUT', 'SELECT', 'TEXTAREA'].includes(el.tagName)) rects.push(box);
			const bad = rects.filter((r) => r.width > 0 && (r.left < -1 || r.right > viewport + 1));
			if (!bad.length) continue;
			let ancestor = el.parentElement;
			let scrollable = false;
			while (ancestor && ancestor !== main) {
				const css = getComputedStyle(ancestor);
				if (
					/auto|scroll/.test(css.overflowX) &&
					ancestor.scrollWidth > ancestor.clientWidth
				) {
					scrollable = true;
					break;
				}
				ancestor = ancestor.parentElement;
			}
			if (!scrollable)
				problems.push({
					tag: el.tagName,
					text: el.textContent.trim().slice(0, 90),
					class: typeof el.className === 'string' ? el.className : '',
					left: Math.min(...bad.map((r) => r.left)),
					right: Math.max(...bad.map((r) => r.right))
				});
		}
		return { viewport, documentWidth: document.documentElement.scrollWidth, problems };
	});
}
async function ready(page) {
	await page.locator('#main-content').waitFor({ timeout: 8000 });
	await page.evaluate(() =>
		Promise.race([document.fonts.ready, new Promise((r) => setTimeout(r, 2000))])
	);
	await page.evaluate(
		() => new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))
	);
}
await Promise.all(
	Array.from({ length: 4 }, async () => {
		const context = await browser.newContext({ viewport: { width: 1280, height: 800 } });
		const page = await context.newPage();
		let row;
		page.on('console', (m) => {
			if (row && ['error', 'warning'].includes(m.type()))
				row.console.push({ type: m.type(), text: m.text(), location: m.location() });
		});
		page.on('pageerror', (e) => row?.exceptions.push(e.message));
		page.on('requestfailed', (r) =>
			row?.network.push({ url: r.url(), failure: r.failure()?.errorText })
		);
		page.on('response', (r) => {
			if (row && r.status() >= 400) row.network.push({ url: r.url(), status: r.status() });
		});
		while (queue.length) {
			const path = queue.shift();
			row = {
				path,
				console: [],
				exceptions: [],
				network: [],
				viewports: [],
				controls: [],
				navigation: []
			};
			try {
				await page.emulateMedia({ reducedMotion: 'no-preference' });
				await page.setViewportSize({ width: 1280, height: 800 });
				const response = await page.goto(origin + path, {
					waitUntil: 'domcontentloaded',
					timeout: 20000
				});
				await ready(page);
				row.status = response.status();
				row.destination = new URL(page.url()).pathname;
				row.title = await page.title();
				row.h1 = await page.locator('main h1').allTextContents();
				row.mainCount = await page.locator('main').count();
				row.links = await page
					.locator('a[href]')
					.evaluateAll((es) => [...new Set(es.map((e) => e.getAttribute('href')))]);
				for (const href of row.links)
					if (/^\/(blog|learning-center)\/[^/?#]+$/.test(href))
						result.discovered.push(href);
				row.forms = await page.locator('main form').evaluateAll((es) =>
					es.map((e) => ({
						action: e.getAttribute('action'),
						method: e.method,
						inputs: [...e.querySelectorAll('input,select,textarea')].map((i) => ({
							name: i.name,
							type: i.type,
							required: i.required
						}))
					}))
				);
				await page.screenshot({ path: new URL(`${slug(path)}-1280.png`, out).pathname });
				for (const width of [
					320, 390, 639, 640, 767, 768, 1023, 1024, 1151, 1152, 1279, 1280, 1439, 1440,
					1535, 1536, 1920
				]) {
					await page.setViewportSize({ width, height: 800 });
					await page.evaluate(() => window.scrollTo({ top: 0, behavior: 'instant' }));
					row.viewports.push({ width, ...(await geometry(page)) });
					if (width === 390)
						await page.screenshot({
							path: new URL(`${slug(path)}-390.png`, out).pathname
						});
				}
				await page.setViewportSize({ width: 390, height: 560 });
				await page.evaluate(() =>
					window.scrollTo({ top: document.body.scrollHeight, behavior: 'instant' })
				);
				const expanded = page.locator('main button[aria-expanded]');
				for (let i = 0; i < Math.min(await expanded.count(), 20); i++) {
					const button = expanded.nth(i);
					if (!(await button.isVisible())) continue;
					const before = await button.getAttribute('aria-expanded');
					await button.focus();
					await page.keyboard.press('Enter');
					const after = await button.getAttribute('aria-expanded');
					row.controls.push({
						type: 'expanded',
						name: (await button.innerText()).slice(0, 70),
						before,
						after
					});
					if (before !== after) {
						await button.focus();
						await page.keyboard.press('Enter');
					}
				}
				await page.setViewportSize({ width: 1280, height: 800 });
				await page.emulateMedia({ reducedMotion: 'reduce' });
				await page.reload({ waitUntil: 'domcontentloaded', timeout: 20000 });
				await ready(page);
				row.motion = await page.locator('#main-content').evaluate((e) =>
					e
						.getAnimations({ subtree: true })
						.filter(
							(a) =>
								a.playState === 'running' &&
								a.effect?.getTiming().iterations === Infinity
						)
						.map((a) => ({
							name: a.animationName,
							target: a.effect?.target?.outerHTML?.slice(0, 200)
						}))
				);
				try {
					await page.waitForFunction(
						() =>
							[
								...document.querySelectorAll(
									'main [data-gsap], main [data-gsap-hero], main .course-card'
								)
							].every((e) => {
								const opacity = Number(getComputedStyle(e).opacity);
								return opacity === 0 || opacity >= 0.99;
							}),
						null,
						{ timeout: 5000 }
					);
				} catch {
					row.unsettledOpacity = true;
				}
				const axe = await new AxeBuilder({ page }).include('#main-content').analyze();
				row.axe = axe.violations.map((v) => ({
					id: v.id,
					impact: v.impact,
					nodes: v.nodes.map((n) => ({ html: n.html, summary: n.failureSummary }))
				}));
				await page.setViewportSize({ width: 390, height: 560 });
				await page.evaluate(() => {
					document.documentElement.style.fontSize = '32px';
					window.scrollTo({ top: 0, behavior: 'instant' });
				});
				row.largeText = await geometry(page);
				if (row.largeText.problems.length)
					await page.screenshot({
						path: new URL(`${slug(path)}-large.png`, out).pathname
					});
				await page.reload({ waitUntil: 'domcontentloaded', timeout: 20000 });
				await ready(page);
				await page.setViewportSize({ width: 1280, height: 800 });
				const home = page.locator('[aria-label="Breadcrumb"] a[href="/"]').first();
				if (path !== '/' && (await home.count())) {
					await home.click();
					await page.waitForURL(origin + '/', { timeout: 15000 });
					await ready(page);
					row.navigation.push(new URL(page.url()).pathname);
					await page.goBack({ waitUntil: 'domcontentloaded' });
					await page.waitForURL(origin + row.destination, { timeout: 15000 });
					await ready(page);
					row.navigation.push(new URL(page.url()).pathname);
					await page.goForward({ waitUntil: 'domcontentloaded' });
					await page.waitForURL(origin + '/', { timeout: 15000 });
					await ready(page);
					row.navigation.push(new URL(page.url()).pathname);
					await page.goBack({ waitUntil: 'domcontentloaded' });
					await page.waitForURL(origin + row.destination, { timeout: 15000 });
					await ready(page);
					row.navigation.push(new URL(page.url()).pathname);
				}
			} catch (error) {
				row.auditError = String(error.stack);
			}
			result.routes.push(row);
			await writeFile(new URL(`${slug(path)}.json`, out), JSON.stringify(row, null, 2));
			console.log(
				JSON.stringify({
					label,
					path,
					status: row.status,
					destination: row.destination,
					errors: row.exceptions,
					clipped: row.viewports.filter((v) => v.problems.length).map((v) => v.width),
					large: row.largeText?.problems.length,
					motion: row.motion?.length,
					axe: row.axe?.length,
					auditError: row.auditError
				})
			);
		}
		await context.close();
	})
);
result.discovered = [...new Set(result.discovered)].sort((a, b) => a.localeCompare(b));
await writeFile(new URL('./summary.json', out), JSON.stringify(result, null, 2));
await browser.close();
