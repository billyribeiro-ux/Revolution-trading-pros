import { expect, test } from '@playwright/test';

test('performance storage failure is visible and never becomes an empty healthy report', async ({
	page,
	baseURL
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	const failures: string[] = [];
	let injectedFailures = 0;
	page.on('pageerror', (error) => failures.push(error.message));
	page.on('response', (response) => {
		const path = new URL(response.url()).pathname;
		if (
			path.startsWith('/api/') &&
			response.status() >= 400 &&
			!(path === '/api/performance/dashboard' && response.status() === 503)
		)
			failures.push(`${response.status()} ${path}`);
	});
	page.on('requestfailed', (request) => {
		const path = new URL(request.url()).pathname;
		if (path.startsWith('/api/') && !request.failure()?.errorText.includes('ERR_ABORTED'))
			failures.push(`${request.failure()?.errorText} ${path}`);
	});
	page.on('console', (message) => {
		if (message.type() !== 'error') return;
		const location = message.location().url;
		if (location && !['127.0.0.1', 'localhost'].includes(new URL(location).hostname)) return;
		if (
			message.text().startsWith('Failed to load resource') &&
			(!location || new URL(location).pathname === '/api/performance/dashboard')
		)
			return;
		failures.push(message.text());
	});
	await page.route('**/*', (route) => {
		const url = new URL(route.request().url());
		return ['127.0.0.1', 'localhost'].includes(url.hostname) ||
			!['http:', 'https:'].includes(url.protocol)
			? route.continue()
			: route.abort();
	});
	await page.route('**/api/performance/dashboard?**', async (route) => {
		injectedFailures++;
		await route.fulfill({
			status: 503,
			contentType: 'application/json',
			body: JSON.stringify({ error: { message: 'Controlled performance storage unavailable' } })
		});
	});
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
	});
	expect(login.ok()).toBe(true);
	await page.goto('/admin/performance', { waitUntil: 'networkidle' });
	await expect(
		page.getByRole('alert').filter({ hasText: 'Controlled performance storage unavailable' })
	).toBeVisible();
	await expect(page.getByRole('button', { name: 'Retry measurements', exact: true })).toBeVisible();
	await expect(page.locator('.vital-card')).toHaveCount(0);
	await expect(
		page.getByRole('heading', { name: 'No suggestions from recorded observations', exact: true })
	).toHaveCount(0);
	expect(injectedFailures).toBe(1);
	await page.getByRole('button', { name: 'Retry measurements', exact: true }).click();
	await expect.poll(() => injectedFailures).toBe(2);
	await expect(
		page.getByRole('alert').filter({ hasText: 'Controlled performance storage unavailable' })
	).toBeVisible();
	expect(failures, 'No unexpected application errors outside the explicit 503 fixture').toEqual([]);
});
