import { expect, test } from '@playwright/test';

const routes = ['/about', '/alerts/explosive-swings'];

test('About decoration stays within the page and ticker clears shared navigation', async ({
	page
}) => {
	await page.goto('/about');
	const geometry = await page.evaluate(() => {
		const bounds = (selector: string) => {
			const element = document.querySelector(selector);
			if (!element) throw new Error(`Missing ${selector}`);
			const { top, bottom } = element.getBoundingClientRect();
			return { top, bottom };
		};
		return {
			about: bounds('.about'),
			atmosphere: bounds('.about__atmosphere'),
			breadcrumb: bounds('[aria-label="Breadcrumb"]'),
			ticker: bounds('.ticker'),
			navbar: bounds('.navbar')
		};
	});
	expect(geometry.atmosphere.top).toBeGreaterThanOrEqual(geometry.about.top);
	expect(geometry.atmosphere.bottom).toBeLessThanOrEqual(geometry.about.bottom);
	expect(geometry.ticker.top).toBeGreaterThanOrEqual(geometry.breadcrumb.bottom);
	expect(geometry.ticker.top).toBeGreaterThanOrEqual(geometry.navbar.bottom);
	const pause = page.getByRole('button', { name: 'Pause market ticker' });
	await pause.focus();
	await page.keyboard.press('Space');
	await expect(pause).toHaveAttribute('aria-pressed', 'true');
	await expect(page.locator('.ticker__track')).toHaveCSS('animation-play-state', 'paused');
	await page.keyboard.press('Space');
	await expect(pause).toHaveAttribute('aria-pressed', 'false');
});

for (const route of routes) {
	test(`${route} keeps heading text readable at 200% text size`, async ({ page }) => {
		await page.setViewportSize({ width: 390, height: 560 });
		await page.goto(route);
		await page.evaluate(() => {
			document.documentElement.style.fontSize = '32px';
		});
		const overflowing = await page.locator('main h1').evaluate((element) => {
			const range = document.createRange();
			range.selectNodeContents(element);
			return [...range.getClientRects()]
				.filter(
					(rect) =>
						rect.width > 0 && (rect.left < 0 || rect.right > document.documentElement.clientWidth)
				)
				.map((rect) => ({ left: rect.left, right: rect.right }));
		});
		expect(overflowing).toEqual([]);
	});

	test(`${route} respects reduced motion, including preference changes`, async ({ page }) => {
		await page.emulateMedia({ reducedMotion: 'reduce' });
		await page.goto(route);
		const animated = () =>
			page
				.locator('main')
				.evaluate(
					(element) =>
						element
							.getAnimations({ subtree: true })
							.filter(
								(animation) =>
									animation.playState === 'running' &&
									animation.effect?.getTiming().iterations === Infinity
							).length
				);
		await expect.poll(animated).toBe(0);
		const faq = page.locator('.faq__trigger').first();
		await faq.click();
		await expect(faq).toHaveAttribute('aria-expanded', 'true');
		await expect
			.poll(() =>
				page
					.locator('.faq__answer, .faq__panel')
					.evaluateAll(
						(elements) =>
							elements
								.flatMap((element) => element.getAnimations())
								.filter((animation) => animation.playState === 'running').length
					)
			)
			.toBe(0);
		await page.emulateMedia({ reducedMotion: 'no-preference' });
		await page.emulateMedia({ reducedMotion: 'reduce' });
		await expect.poll(animated).toBe(0);
	});
}

test('Swings plan selection fits a 320px screen and announces selected state', async ({ page }) => {
	await page.setViewportSize({ width: 320, height: 560 });
	await page.goto('/alerts/explosive-swings');
	for (const name of ['Monthly', 'Annual', 'Quarterly']) {
		const button = page.locator('#pricing').getByRole('button', { name, exact: true });
		await button.scrollIntoViewIfNeeded();
		const box = await button.boundingBox();
		if (!box) throw new Error(`${name} plan button has no visible bounds`);
		expect(box.x).toBeGreaterThanOrEqual(0);
		expect(box.x + box.width).toBeLessThanOrEqual(320);
		await button.focus();
		await page.keyboard.press('Space');
		await expect(button).toHaveAttribute('aria-pressed', 'true');
		await expect(page.locator('#pricing button[aria-pressed="true"]')).toHaveCount(1);
	}
});
