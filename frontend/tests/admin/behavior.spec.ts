import { expect, test } from '@playwright/test';

// Run against the disposable local API/DB fixture created by
// api/tests/behavior_integration_test.rs. Never run against production.
test.skip(!process.env.BEHAVIOR_E2E, 'Requires the disposable behavior verification stack');

test('ingested clicks and sessions appear on the admin website', async ({ page }) => {
	await page.goto('/');
	const origin = new URL(page.url()).origin;
	const sessionId = `browser-${crypto.randomUUID()}`;
	const now = Date.now();
	const ingest = await page.request.post('/api/behavior/events', {
		data: {
			session_id: sessionId,
			visitor_id: `anonymous-${crypto.randomUUID()}`,
			events: [
				{ event_type: 'page_view', timestamp: now, page_url: `${origin}/pricing` },
				{
					event_type: 'click',
					timestamp: now + 1,
					page_url: `${origin}/pricing`,
					element_selector: 'button',
					coordinates: { x: 42, y: 38 },
					coordinate_space: 'viewport_percent'
				},
				{
					event_type: 'scroll_depth',
					timestamp: now + 2,
					page_url: `${origin}/pricing`,
					event_value: 75
				}
			]
		}
	});
	expect(ingest.status()).toBe(200);
	expect((await ingest.json()).persisted).toBe(3);

	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@behavior.test', password: 'DisposableOnly123!' }
	});
	expect(login.status()).toBe(200);
	await page.goto('/behavior');
	await expect(page).toHaveURL(/\/admin\/behavior$/);
	await page.goto('/admin/analytics/heatmaps');
	await expect(page.getByRole('heading', { name: 'Click heatmaps' })).toBeVisible();
	await expect(page.locator('.point').first()).toBeVisible();
	await expect(page.getByText('75%', { exact: true }).first()).toBeVisible();

	await page.goto('/admin/analytics/recordings');
	await expect(page.getByRole('heading', { name: 'Session activity' })).toBeVisible();
	await page.locator(`[data-session-id="${sessionId}"]`).click();
	await expect(page.getByRole('heading', { name: 'Activity timeline' })).toBeVisible();
	await expect(page.locator('.detail li')).toHaveCount(3);
	await expect(page.locator('.detail li').first()).toContainText('/pricing');
});

test('browser tracking starts on analytics consent and stops on withdrawal', async ({ page }) => {
	await page.goto('/pricing');
	const activeBefore = await page.evaluate(() =>
		Boolean((window as Window & { __behaviorTracker?: unknown }).__behaviorTracker)
	);
	expect(activeBefore).toBe(false);
	await page.evaluate(async () => {
		const modulePath = '/src/lib/consent/store.svelte.ts';
		const { consentStore } = await import(modulePath);
		consentStore.setCategory('analytics', true);
	});
	await page.waitForFunction(() =>
		Boolean((window as Window & { __behaviorTracker?: unknown }).__behaviorTracker)
	);
	const receipt = page.waitForResponse(
		(response) => new URL(response.url()).pathname === '/api/behavior/events'
	);
	await page.evaluate(async () => {
		await (
			window as Window & { __behaviorTracker?: { flush(): Promise<void> } }
		).__behaviorTracker?.flush();
	});
	expect((await receipt).status()).toBe(200);
	await page.evaluate(async () => {
		const modulePath = '/src/lib/consent/store.svelte.ts';
		const { consentStore } = await import(modulePath);
		consentStore.setCategory('analytics', false);
	});
	await expect
		.poll(() =>
			page.evaluate(
				() => (window as Window & { __behaviorTracker?: unknown }).__behaviorTracker ?? null
			)
		)
		.toBeNull();
	expect(await page.evaluate(() => localStorage.getItem('behavior_failed_batches'))).toBeNull();
});
