import { expect, test } from '@playwright/test';
import { createServer, type Server } from 'node:http';

// Requires the task API8281 with CONNECTION_TEST_BASE_URL=http://127.0.0.1:19021.
// This fixture performs read-only acknowledgments; it never sends an email.
let adapter: Server;
let submissions = 0;
test.describe.configure({ mode: 'serial' });
test.beforeAll(async () => {
	adapter = createServer((request, response) => {
		if (request.method !== 'GET' || request.url !== '/sendgrid/v3/scopes') {
			response.writeHead(404);
			response.end();
			return;
		}
		submissions++;
		const token = request.headers.authorization;
		response.setHeader('Content-Type', 'application/json');
		if (token === 'Bearer fixture-good') response.end('{"scopes":["mail.send"]}');
		else {
			response.writeHead(401);
			response.end('{"error":"fixture rejection"}');
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
	await page.goto('/admin/connections', { waitUntil: 'networkidle' });
	await expect(page.getByRole('heading', { name: 'Built-in Features', exact: true })).toBeVisible();
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
	await page.route('**/api/admin/connections?environment=production', (route) => {
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
