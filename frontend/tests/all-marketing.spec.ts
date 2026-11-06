import { expect, test } from '@playwright/test';

const indicators = [
	['moving-averages', 'Moving Averages (SMA/EMA)'],
	['bollinger-bands', 'Bollinger Bands'],
	['vwap', 'VWAP - Volume Weighted Average Price'],
	['stochastic', 'Stochastic Oscillator']
];

for (const [slug, name] of indicators) {
	test(`public indicator ${slug} matches the catalog instead of another product`, async ({
		page
	}) => {
		await page.goto(`/indicators/${slug}`);
		await expect(page.locator('h1')).toHaveText(name);
		await expect(page.locator('a[href*="st-download-file"]')).toHaveCount(0);
		await expect(page.locator('[aria-label="Breadcrumb"]')).toHaveCount(1);
		await expect(page.locator('main')).not.toContainText('Volume Max Tool Kit');
	});
}

for (const route of [
	'/contact',
	'/courses/options-trading',
	'/live-trading-rooms',
	'/mentorship',
	'/our-mission'
]) {
	test(`${route} keeps enlarged heading text in its viewport`, async ({ page }) => {
		await page.setViewportSize({ width: 390, height: 560 });
		await page.goto(route);
		await page.evaluate(() => {
			document.documentElement.style.fontSize = '32px';
		});
		const overflow = await page.locator('h1').evaluateAll((elements) =>
			elements.flatMap((element) => {
				const range = document.createRange();
				range.selectNodeContents(element);
				return [...range.getClientRects()]
					.filter(
						(rect) =>
							rect.width > 0 && (rect.left < 0 || rect.right > document.documentElement.clientWidth)
					)
					.map((rect) => rect.right);
			})
		);
		expect(overflow).toEqual([]);
	});
}

test('SPX marketing page owns no second main landmark', async ({ page }) => {
	await page.goto('/alerts/spx-profit-pulse');
	await expect(page.locator('main')).toHaveCount(1);
});

test('stock index region filter has an accessible name and changes results', async ({ page }) => {
	await page.goto('/resources/stock-indexes-list');
	const region = page.getByRole('combobox', { name: /region/i });
	await expect(region).toBeVisible();
	await region.selectOption('Germany');
	await expect(page.locator('.index-card')).not.toContainText('S&P 500');
});

for (const route of [
	'/',
	'/alerts',
	'/courses',
	'/mentorship',
	'/our-mission',
	'/live-trading-rooms/day-trading'
]) {
	test(`${route} suppresses continuous animation for reduced motion`, async ({ page }) => {
		await page.emulateMedia({ reducedMotion: 'reduce' });
		await page.goto(route);
		await expect
			.poll(() =>
				page
					.locator('#main-content')
					.evaluate(
						(element) =>
							element
								.getAnimations({ subtree: true })
								.filter(
									(animation) =>
										animation.playState === 'running' &&
										animation.effect?.getTiming().iterations === Infinity
								).length
					)
			)
			.toBe(0);
	});
}

test('calculator chart tabs remain reachable with the keyboard on a narrow screen', async ({
	page
}) => {
	await page.setViewportSize({ width: 320, height: 700 });
	await page.goto('/tools/options-calculator');
	const first = page.getByRole('tab', { name: 'Payoff', exact: true });
	await first.focus();
	await page.keyboard.press('End');
	const last = page.getByRole('tab', { name: 'Options Chain', exact: true });
	await expect(last).toBeFocused();
	await expect(last).toHaveAttribute('aria-selected', 'true');
	const bounds = await last.boundingBox();
	expect(bounds).not.toBeNull();
	if (!bounds) throw new Error('Selected chart tab has no visible bounds');
	expect(bounds.x).toBeGreaterThanOrEqual(0);
	expect(bounds.x + bounds.width).toBeLessThanOrEqual(320);
	await page.keyboard.press('Home');
	await expect(first).toBeFocused();
});

test('homepage automatic slides can be paused without changing the toggle name', async ({
	page
}) => {
	await page.goto('/');
	const pause = page.getByRole('button', { name: 'Pause automatic slides', exact: true });
	await pause.click();
	await expect(pause).toHaveAttribute('aria-pressed', 'true');
	await pause.click();
	await expect(pause).toHaveAttribute('aria-pressed', 'false');
});

for (const route of ['/classes/quickstart-precision-trading-c', '/classes/tax-loss-harvest-c']) {
	test(`${route} directs anonymous visitors to sign in for member downloads`, async ({ page }) => {
		const memberReads: string[] = [];
		page.on('request', (request) => {
			if (request.url().includes('/api/my/courses/')) memberReads.push(request.url());
		});
		const response = await page.goto(route);
		await expect(
			page.getByRole('link', { name: 'Sign in to access your class downloads' })
		).toBeVisible();
		expect(memberReads).toEqual([]);
		const policy =
			response?.headers()['content-security-policy'] ??
			(await page.locator('meta[http-equiv="content-security-policy"]').getAttribute('content')) ??
			'';
		expect(policy).toContain('https://simpler-options.s3.amazonaws.com');
		expect(policy).toContain('https://cloud-streaming.s3.amazonaws.com');
	});
}

test('class recording failure is visible and retry issues a fresh media request', async ({
	page
}) => {
	let requests = 0;
	await page.route('https://cloud-streaming.s3.amazonaws.com/**', async (route) => {
		requests++;
		await route.fulfill({ status: 403, body: 'Unavailable test recording' });
	});
	await page.goto('/classes/tax-loss-harvest-c');
	const message = page
		.getByRole('status')
		.filter({ hasText: 'This recording is temporarily unavailable.' })
		.first();
	await expect(message).toBeVisible();
	const before = requests;
	await message.getByRole('button', { name: 'Retry recording' }).click();
	await expect.poll(() => requests).toBeGreaterThan(before);
	await expect(message).toBeVisible();
});
