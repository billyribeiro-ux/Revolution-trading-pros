import { expect, test, type Page } from '@playwright/test';
import { createServer, type Server } from 'node:http';

// Requires the task API8281 with CONNECTION_TEST_BASE_URL=http://127.0.0.1:19021.
// This fixture performs read-only acknowledgments; it never sends an email.
let adapter: Server;
let submissions = 0;
let storageChecks = 0;
let videoChecks = 0;
test.describe.configure({ mode: 'serial' });
async function watchPage(page: Page): Promise<string[]> {
	const failures: string[] = [];
	page.on('pageerror', (error) => failures.push(error.message));
	page.on('console', (message) => {
		const url = message.location().url;
		if (
			message.type() === 'error' &&
			(!url || ['127.0.0.1', 'localhost'].includes(new URL(url).hostname))
		)
			failures.push(message.text());
	});
	page.on('response', (response) => {
		if (new URL(response.url()).pathname.startsWith('/api/') && response.status() >= 400)
			failures.push(`${response.status()} ${response.url()}`);
	});
	page.on('requestfailed', (request) => {
		if (
			new URL(request.url()).pathname.startsWith('/api/') &&
			!request.failure()?.errorText.includes('ERR_ABORTED')
		)
			failures.push(request.url());
	});
	await page.route('**/*', (route) => {
		const url = new URL(route.request().url());
		return ['127.0.0.1', 'localhost'].includes(url.hostname) ||
			!['https:', 'http:'].includes(url.protocol)
			? route.continue()
			: route.abort();
	});
	return failures;
}
test.beforeAll(async () => {
	adapter = createServer((request, response) => {
		response.setHeader('Content-Type', 'application/json');
		if (request.method === 'GET' && request.url === '/sendgrid/v3/scopes') {
			submissions++;
			if (request.headers.authorization === 'Bearer fixture-good')
				response.end('{"scopes":["mail.send"]}');
			else {
				response.writeHead(401);
				response.end('{"error":"fixture rejection"}');
			}
		} else if (request.method === 'GET' && request.url === '/bunny_cdn/fixture-zone/') {
			storageChecks++;
			if (request.headers.accesskey === 'fixture-storage') response.end('[]');
			else {
				response.writeHead(401);
				response.end('{}');
			}
		} else if (
			request.method === 'GET' &&
			request.url === '/bunny_stream/library/123/videos?page=1&itemsPerPage=10'
		) {
			videoChecks++;
			if (request.headers.accesskey === 'fixture-stream')
				response.end(
					'{"items":[{"guid":"fixture-video","videoLibraryId":123}],"totalItems":1,"currentPage":1,"itemsPerPage":10}'
				);
			else {
				response.writeHead(401);
				response.end('{}');
			}
		} else {
			response.writeHead(404);
			response.end('{}');
		}
	});
	await new Promise<void>((resolve, reject) => {
		adapter.once('error', reject);
		adapter.listen(19021, '127.0.0.1', resolve);
	});
});
test.afterAll(async () => {
	if (adapter)
		await new Promise<void>((resolve, reject) =>
			adapter.close((error) => (error ? reject(error) : resolve()))
		);
});
test('real connection checks, durable states, scoped configuration and failure recovery', async ({
	page,
	baseURL
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
	});
	expect(login.ok()).toBe(true);
	const failures = await watchPage(page);
	await page.goto('/admin/connections', { waitUntil: 'networkidle' });
	await expect(page.getByRole('heading', { name: 'Built-in Features', exact: true })).toBeVisible();
	await expect(page.getByLabel('Environment', { exact: true })).toHaveValue('sandbox');
	await expect(page.getByText('Application uses Sandbox', { exact: true })).toBeVisible();
	await page.getByLabel('Environment', { exact: true }).selectOption('production');
	const service = page
		.locator('.service-card')
		.filter({ has: page.getByRole('heading', { name: 'SendGrid', exact: true }) });
	await service.getByRole('button', { name: /Connect|Configure/, exact: true }).click();
	await page.getByLabel(/^API Key\s*\*?$/).fill('fixture-good');
	await page.getByLabel(/^From Email\s*\*?$/).fill('sender@example.test');
	await page.getByRole('button', { name: 'Test', exact: true }).click();
	await expect(page.getByText('Read-only provider check passed', { exact: true })).toBeVisible();
	expect(submissions).toBeGreaterThan(0);
	await page.getByRole('button', { name: 'Connect', exact: true }).last().click();
	await expect(service.getByRole('button', { name: 'Configure', exact: true })).toBeVisible();
	await expect(service.getByText('Passed', { exact: true })).toBeVisible();
	await page.reload({ waitUntil: 'networkidle' });
	await expect(service.getByRole('button', { name: 'Configure', exact: true })).toBeVisible();
	await service.getByRole('button', { name: 'Configure', exact: true }).click();
	await page.getByLabel(/^API Key\s*\*?$/).fill('fixture-reject');
	await page.getByLabel(/^From Email\s*\*?$/).fill('sender@example.test');
	await page.getByRole('button', { name: 'Test', exact: true }).click();
	await expect(page.getByText('Provider returned HTTP 401', { exact: true })).toBeVisible();
	await page.getByRole('button', { name: 'Connect', exact: true }).last().click();
	await expect(service.getByText('Provider returned HTTP 401', { exact: true })).toBeVisible();
	await page.reload({ waitUntil: 'networkidle' });
	await expect(service.getByText('Provider returned HTTP 401', { exact: true })).toBeVisible();
	await page.getByLabel('Environment', { exact: true }).selectOption('sandbox');
	await expect(service.getByText('Provider returned HTTP 401', { exact: true })).not.toBeVisible();
	await page.getByLabel('Environment', { exact: true }).selectOption('production');
	await expect(service.getByText('Provider returned HTTP 401', { exact: true })).toBeVisible();
	const statusResponse = await page.request.get(
		'/api/admin/connections/status?environment=production'
	);
	expect(statusResponse.ok()).toBe(true);
	expect(statusResponse.headers()['cache-control']).toContain('no-store');
	const status = await statusResponse.json();
	expect(status.connections.find((row: { key: string }) => row.key === 'sendgrid')).toMatchObject({
		is_connected: false,
		status: 'error',
		last_error: 'Provider returned HTTP 401'
	});
	expect(
		status.connections.find((row: { key: string }) => row.key === 'fluent_support')
	).toMatchObject({
		is_connected: false,
		status: 'pending',
		last_verified_at: null
	});
	expect(status.connections.some((row: { key: string }) => row.key === 'polygon')).toBe(true);
	expect(failures).toEqual([]);
});

test('controlled list failure stays visible and Retry loads real saved state', async ({
	page,
	baseURL
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	expect(
		(
			await page.request.post('/api/auth/login', {
				data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
			})
		).ok()
	).toBe(true);
	let injected = 0;
	await page.route('**/api/admin/connections?environment=runtime', (route) => {
		if (injected++ === 0)
			return route.fulfill({
				status: 503,
				contentType: 'application/json',
				body: '{"error":"Fixture connection listing unavailable"}'
			});
		return route.continue();
	});
	await page.goto('/admin/connections', { waitUntil: 'networkidle' });
	await expect(
		page.getByRole('alert').filter({ hasText: 'Fixture connection listing unavailable' })
	).toBeVisible();
	await page.getByRole('button', { name: 'Retry connections', exact: true }).click();
	await expect(page.getByRole('heading', { name: 'Built-in Features', exact: true })).toBeVisible();
	await expect(
		page.getByRole('alert').filter({ hasText: 'Fixture connection listing unavailable' })
	).not.toBeVisible();
	expect(injected).toBeGreaterThanOrEqual(2);
});

test('Bunny regional storage and optional video configuration use separate real checks', async ({
	page,
	baseURL
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	expect(
		(
			await page.request.post('/api/auth/login', {
				data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
			})
		).ok()
	).toBe(true);
	const failures = await watchPage(page);
	await page.goto('/admin/connections', { waitUntil: 'networkidle' });
	const service = page
		.locator('.service-card')
		.filter({ has: page.getByRole('heading', { name: 'Bunny.net CDN', exact: true }) });
	await service.getByRole('button', { name: /Connect|Configure/, exact: true }).click();
	const fillStorage = async () => {
		await page.getByLabel(/^Storage Zone Password\s*\*?$/).fill('fixture-storage');
		await page.getByLabel(/^Storage Zone\s*\*?$/).fill('fixture-zone');
		await page
			.getByLabel('Storage Endpoint Hostname', { exact: true })
			.fill('ny.storage.bunnycdn.com');
	};
	await fillStorage();
	await page.getByLabel('Video Library ID', { exact: true }).fill('123');
	await page.getByLabel('Stream API Key', { exact: true }).fill('fixture-stream');
	await page.getByRole('button', { name: 'Test', exact: true }).click();
	await expect(
		page.getByText(
			'Storage directory (ny.storage.bunnycdn.com) and video library access verified',
			{ exact: true }
		)
	).toBeVisible();
	expect(storageChecks).toBeGreaterThan(0);
	expect(videoChecks).toBeGreaterThan(0);
	await page.getByRole('button', { name: 'Connect', exact: true }).last().click();
	await expect(service.getByText('Passed', { exact: true })).toBeVisible();
	await page.reload({ waitUntil: 'networkidle' });
	await expect(service.getByText('Passed', { exact: true })).toBeVisible();
	await service.getByRole('button', { name: 'Configure', exact: true }).click();
	await fillStorage();
	await page.getByLabel('Video Library ID', { exact: true }).fill('123');
	await page.getByLabel('Stream API Key', { exact: true }).fill('fixture-reject');
	await page.getByRole('button', { name: 'Test', exact: true }).click();
	await expect(page.getByText('Bunny Stream returned HTTP 401', { exact: true })).toBeVisible();
	await page.getByRole('button', { name: 'Connect', exact: true }).last().click();
	await expect(service.getByText('Bunny Stream returned HTTP 401', { exact: true })).toBeVisible();
	await page.reload({ waitUntil: 'networkidle' });
	await expect(service.getByText('Bunny Stream returned HTTP 401', { exact: true })).toBeVisible();
	await service.getByRole('button', { name: 'Connect', exact: true }).click();
	await fillStorage();
	const before = videoChecks;
	await page.getByRole('button', { name: 'Test', exact: true }).click();
	await expect(
		page.getByText(
			'Storage directory access verified (ny.storage.bunnycdn.com); no video library configured',
			{ exact: true }
		)
	).toBeVisible();
	expect(videoChecks).toBe(before);
	await page.getByRole('button', { name: 'Connect', exact: true }).last().click();
	await expect(service.getByText('Passed', { exact: true })).toBeVisible();
	expect(failures).toEqual([]);
});
