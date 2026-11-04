import { expect, test } from '@playwright/test';
import { createServer, type Server } from 'node:net';

let adapter: Server;
let checks = 0;
let mailAttempts = 0;
test.describe.configure({ mode: 'serial' });
test.beforeAll(async () => {
	adapter = createServer((socket) => {
		let buffer = '';
		let authenticated = false;
		socket.on('error', () => socket.destroy());
		socket.write('220 fixture.example.test ESMTP\r\n');
		socket.on('data', (chunk) => {
			buffer += chunk.toString();
			while (buffer.includes('\r\n')) {
				const position = buffer.indexOf('\r\n');
				const line = buffer.slice(0, position);
				buffer = buffer.slice(position + 2);
				if (line.startsWith('EHLO '))
					socket.write('250-fixture.example.test\r\n250-AUTH PLAIN\r\n250 SMTPUTF8\r\n');
				else if (line.startsWith('AUTH PLAIN ')) {
					authenticated =
						Buffer.from(line.slice(11), 'base64').toString() === '\0fixture-user\0fixture-password';
					socket.write(authenticated ? '235 2.7.0 Accepted\r\n' : '535 5.7.8 Rejected\r\n');
				} else if (line === 'NOOP') {
					if (authenticated) checks++;
					socket.write(authenticated ? '250 2.0.0 OK\r\n' : '530 Authentication required\r\n');
				} else if (line === 'QUIT') socket.end('221 Bye\r\n');
				else {
					if (line.startsWith('MAIL ') || line === 'DATA') mailAttempts++;
					socket.write('500 Unsupported\r\n');
				}
			}
		});
	});
	await new Promise<void>((resolve, reject) => {
		adapter.once('error', reject);
		adapter.listen(19025, '127.0.0.1', resolve);
	});
});
test.afterAll(async () => {
	if (adapter)
		await new Promise<void>((resolve, reject) =>
			adapter.close((error) => (error ? reject(error) : resolve()))
		);
});
test('both SMTP forms save encrypted state, preserve omitted passwords, and check actual relay access', async ({
	page,
	baseURL
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
	});
	expect(login.status()).toBe(200);
	const failures: string[] = [];
	page.on('pageerror', (error) => failures.push(error.message));
	page.on('console', (message) => {
		if (
			message.type() === 'error' &&
			(!message.location().url ||
				['127.0.0.1', 'localhost'].includes(new URL(message.location().url).hostname))
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
	await page.route('**/*', (route) =>
		['localhost', '127.0.0.1'].includes(new URL(route.request().url()).hostname)
			? route.continue()
			: route.abort()
	);
	for (const path of ['/admin/email/smtp', '/admin/email/settings']) {
		await page.goto(path, { waitUntil: 'networkidle' });
		await page.getByLabel('SMTP Host', { exact: true }).fill('127.0.0.1');
		await page.getByLabel('Port', { exact: true }).fill('19025');
		await page.getByLabel('Encryption', { exact: true }).selectOption('null');
		await page.getByLabel('Username', { exact: true }).fill('fixture-user');
		await page.getByLabel(/Password \/ App Password/).fill('fixture-password');
		await page.getByLabel('From Email', { exact: true }).fill('smtp-sender@example.test');
		await page.getByLabel('From Name', { exact: true }).fill('SMTP browser fixture');
		await page.getByRole('button', { name: 'Save Settings', exact: true }).click();
		await expect(
			page.getByText('SMTP settings saved. Test the connection to verify relay access.', {
				exact: true
			})
		).toBeVisible();
		await expect(page.getByLabel(/Password \/ App Password/)).toHaveValue('');
		await page.reload({ waitUntil: 'networkidle' });
		await expect(page.getByLabel('SMTP Host', { exact: true })).toHaveValue('127.0.0.1');
		await expect(page.getByLabel('From Name', { exact: true })).toHaveValue('SMTP browser fixture');
		await expect(page.getByLabel(/Password \/ App Password/)).toHaveValue('');
		await page.getByRole('button', { name: 'Test Connection', exact: true }).click();
		await expect(
			page.getByText('Authenticated SMTP connection and NOOP passed; no email was sent', {
				exact: true
			})
		).toBeVisible();
		await page.getByLabel(/Password \/ App Password/).fill('fixture-reject');
		await page.getByRole('button', { name: 'Test Connection', exact: true }).click();
		await expect(page.locator('.alert-error')).toContainText(/SMTP connection.*(rejected|failed)/);
	}
	expect(checks).toBeGreaterThanOrEqual(2);
	expect(mailAttempts).toBe(0);
	expect(failures).toEqual([]);
});

test('a settings load failure remains visible and Retry retrieves the saved configuration', async ({
	page,
	baseURL
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	expect(
		(
			await page.request.post('/api/auth/login', {
				data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
			})
		).status()
	).toBe(200);
	let injected = 0;
	await page.route('**/api/admin/email/settings', (route) => {
		const attempt = route.request().method() === 'GET' ? injected++ : -1;
		if (attempt === 0)
			return route.fulfill({
				status: 503,
				contentType: 'application/json',
				body: '{"error":"Fixture SMTP storage unavailable"}'
			});
		if (attempt === 1)
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: '{"provider":"smtp"}'
			});
		return route.continue();
	});
	await page.goto('/admin/email/settings', { waitUntil: 'networkidle' });
	const loadError = page.getByRole('alert').filter({ hasText: 'Fixture SMTP storage unavailable' });
	await expect(loadError).toBeVisible();
	await expect(page.getByRole('button', { name: 'Save Settings', exact: true })).toBeDisabled();
	await page.getByRole('button', { name: 'Retry SMTP settings', exact: true }).click();
	await expect(loadError).not.toBeVisible();
	const malformed = page.getByRole('alert').filter({ hasText: 'Invalid SMTP settings response' });
	await expect(malformed).toBeVisible();
	await expect(page.getByRole('button', { name: 'Save Settings', exact: true })).toBeDisabled();
	await page.getByRole('button', { name: 'Retry SMTP settings', exact: true }).click();
	await expect(malformed).not.toBeVisible();
	await expect(page.getByLabel('SMTP Host', { exact: true })).toHaveValue('127.0.0.1');
	await expect(page.getByRole('button', { name: 'Save Settings', exact: true })).toBeEnabled();
	expect(injected).toBeGreaterThanOrEqual(3);
});
