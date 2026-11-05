import { expect, test } from '@playwright/test';
import { fixtureRelay, mimeSubject, mimeText } from './fixtures/smtp-relay.js';
let relay: Awaited<ReturnType<typeof fixtureRelay>>;
const allowed = new Set<string>();
test.beforeAll(async () => {
	relay = await fixtureRelay(19027, allowed);
});

test('actual SMTP rejection remains a recorded failure and replay does not reconnect', async ({
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
	const stamp = Date.now(),
		name = `Rejected campaign test ${stamp}`;
	const created = await page.request.post('/api/admin/email/campaigns', {
		data: { name, subject: 'Rejected sample', html_content: '<p>Fixture rejection</p>' }
	});
	expect(created.status()).toBe(200);
	const id = (await created.json()).data.id;
	const acceptedMessages = relay.messages.length;
	try {
		await page.goto('/admin/email/campaigns', { waitUntil: 'networkidle' });
		await page.getByLabel('Search campaigns', { exact: true }).fill(name);
		await page.getByRole('button', { name: 'Search', exact: true }).click();
		await page
			.locator('.campaign-card')
			.filter({ has: page.getByText(name, { exact: true }) })
			.getByRole('button', { name: 'Test message', exact: true })
			.click();
		const dialog = page.getByRole('dialog', { name: 'Campaign test message' });
		await expect(dialog.getByText('No recorded campaign tests.', { exact: true })).toBeVisible();
		// Deliberately absent from the relay allowlist: actual RCPT rejection, no DATA.
		await dialog
			.getByLabel('Test recipient', { exact: true })
			.fill(`rejected-sample-${stamp}@example.test`);
		await dialog.getByRole('button', { name: 'Send test message', exact: true }).click();
		await expect(dialog.getByText('Provider rejected', { exact: true })).toBeVisible();
		await expect(dialog.getByText('Provider accepted', { exact: true })).toHaveCount(0);
		expect(relay.messages).toHaveLength(acceptedMessages);
		const connections = relay.connections;
		await dialog.getByRole('button', { name: 'Retry same request', exact: true }).click();
		await expect(
			dialog.getByRole('button', { name: 'Retry same request', exact: true })
		).toBeEnabled();
		await expect(dialog.getByText('Provider rejected', { exact: true })).toBeVisible();
		expect(relay.connections).toBe(connections);
		expect(relay.messages).toHaveLength(acceptedMessages);
		const malformed = `**/api/admin/email/campaigns/${id}/tests`;
		await page.route(malformed, (route) =>
			route.fulfill({
				json: {
					success: true,
					data: {
						campaign_id: id,
						metrics_scope: 'native_campaign_test',
						requests: [{ request_id: 'wrong' }]
					}
				}
			})
		);
		await dialog.getByRole('button', { name: 'Check receipts', exact: true }).click();
		await expect(dialog.getByRole('alert')).toContainText('Invalid campaign test request');
		await page.unroute(malformed);
		await dialog.getByRole('button', { name: 'Check receipts', exact: true }).click();
		await expect(dialog.getByRole('alert')).toHaveCount(0);
		await expect(dialog.getByText('Provider rejected', { exact: true })).toBeVisible();
	} finally {
		expect((await page.request.delete(`/api/admin/email/campaigns/${id}`)).status()).toBe(200);
	}
});
test.afterAll(async () => {
	if (relay) await relay.close();
});

test('explicit campaign tests record real SMTP acceptance, replay without resending and survive reload outside production metrics', async ({
	page,
	baseURL
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	const stamp = Date.now(),
		name = `Native test browser ${stamp}`,
		email = `native-test-browser-${stamp}@example.test`;
	allowed.add(email);
	expect(
		(
			await page.request.post('/api/auth/login', {
				data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
			})
		).status()
	).toBe(200);
	const settingsResponse = await page.request.get('/api/admin/email/settings');
	expect(settingsResponse.status()).toBe(200);
	const raw = await settingsResponse.json(),
		settings = raw.data ?? raw.settings ?? raw;
	expect(settings.host).toBe('127.0.0.1');
	expect(Number(settings.port)).toBe(19027);
	const created = await page.request.post('/api/admin/email/campaigns', {
		data: {
			name,
			subject: 'Original sample subject',
			html_content:
				'<p>Saved content for {{email}}</p><a href="{{unsubscribe_url}}">Unsubscribe</a>',
			plain_text_content: 'Saved alternative for {{email}}',
			from_name: 'Actual sample sender',
			from_email: 'sample-sender@example.test',
			reply_to: 'sample-reply@example.test',
			ab_test_config: { enabled: true, subject_b: 'Actual B for {{email}}', split_percentage: 50 }
		}
	});
	expect(created.status()).toBe(200);
	const id = (await created.json()).data.id;
	expect(Number.isSafeInteger(id)).toBe(true);
	const errors: string[] = [],
		blocked = new Set<string>();
	page.on('pageerror', (error) => errors.push(error.message));
	page.on('console', (message) => {
		if (
			message.type() === 'error' &&
			!(
				blocked.has(message.location().url) &&
				message.text() === 'Failed to load resource: net::ERR_FAILED'
			)
		)
			errors.push(message.text());
	});
	page.on('requestfailed', (request) => {
		if (
			new URL(request.url()).pathname.startsWith('/api/') &&
			!request.failure()?.errorText.includes('ERR_ABORTED')
		)
			errors.push(request.url());
	});
	await page.route('**/*', (route) => {
		if (['127.0.0.1', 'localhost'].includes(new URL(route.request().url()).hostname))
			return route.continue();
		blocked.add(route.request().url());
		return route.abort();
	});
	try {
		await page.goto('/admin/email/campaigns', { waitUntil: 'networkidle' });
		await page.getByLabel('Search campaigns', { exact: true }).fill(name);
		await page.getByRole('button', { name: 'Search', exact: true }).click();
		const card = page
			.locator('.campaign-card')
			.filter({ has: page.getByText(name, { exact: true }) });
		await card.getByRole('button', { name: 'Test message', exact: true }).click();
		const dialog = page.getByRole('dialog', { name: 'Campaign test message' });
		await expect(dialog.getByText('No recorded campaign tests.', { exact: true })).toBeVisible();
		await dialog.getByLabel('Test recipient', { exact: true }).fill(email);
		await dialog.getByLabel('Subject variant', { exact: true }).selectOption('b');
		await dialog.getByRole('button', { name: 'Send test message', exact: true }).click();
		await expect(dialog.getByText('Provider accepted', { exact: true })).toBeVisible();
		await expect.poll(() => relay.messages.length).toBe(1);
		const message = relay.messages[0];
		if (!message) throw new Error('The controlled SMTP receipt has no actual message');
		expect(mimeSubject(message.raw)).toBe(`[Test] Actual B for ${email}`);
		expect(mimeText(message.raw)).toContain(`Saved content for ${email}`);
		expect(mimeText(message.raw)).toContain(`Saved alternative for ${email}`);
		expect(message.raw).toContain('sample-sender@example.test');
		expect(message.raw).toContain('sample-reply@example.test');
		await dialog.getByRole('button', { name: 'Retry same request', exact: true }).click();
		await expect(
			dialog.getByRole('button', { name: 'Retry same request', exact: true })
		).toBeEnabled();
		expect(relay.messages).toHaveLength(1);
		await dialog.getByRole('button', { name: 'Close test message', exact: true }).click();
		await page.reload({ waitUntil: 'networkidle' });
		await page.getByLabel('Search campaigns', { exact: true }).fill(name);
		await page.getByRole('button', { name: 'Search', exact: true }).click();
		await card.getByRole('button', { name: 'Test message', exact: true }).click();
		await expect(dialog.getByText('Provider accepted', { exact: true })).toBeVisible();
		const report = await page.request.get(`/api/admin/email/campaigns/${id}/report`);
		expect(report.status()).toBe(200);
		const campaign = await page.request.get(`/api/admin/email/campaigns/${id}`);
		expect((await campaign.json()).data).toMatchObject({
			status: 'draft',
			total_recipients: 0,
			sent_count: 0,
			opened_count: 0,
			clicked_count: 0
		});
		expect(relay.messages).toHaveLength(1);
		expect(errors).toEqual([]);
	} finally {
		expect((await page.request.delete(`/api/admin/email/campaigns/${id}`)).status()).toBe(200);
	}
});
