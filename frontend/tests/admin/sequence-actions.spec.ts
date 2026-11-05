import { expect, test, type Page, type APIRequestContext } from '@playwright/test';
import { fixtureRelay, mimeSubject, mimeText } from './fixtures/smtp-relay.js';

test.describe.configure({ mode: 'default' });
const root = '/api/admin/crm/sequences';
function record(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Expected a saved fixture record');
	return value as Record<string, unknown>;
}
async function login(page: Page, baseURL: string | undefined) {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	expect(
		(
			await page.request.post('/api/auth/login', {
				data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
			})
		).status()
	).toBe(200);
}
async function diagnostics(page: Page, expectedFailures = new Set<string>()) {
	const errors: string[] = [],
		blocked = new Set<string>();
	page.on('pageerror', (error) => errors.push(error.message));
	page.on('console', (message) => {
		if (message.type() !== 'error') return;
		const source = message.location().url;
		if (blocked.has(source) && message.text() === 'Failed to load resource: net::ERR_FAILED')
			return;
		if (
			source &&
			expectedFailures.has(new URL(source).pathname) &&
			/^Failed to load resource: the server responded with a status of 502/.test(message.text())
		)
			return;
		errors.push(`${message.text()} ${source}`);
	});
	page.on('response', (response) => {
		if (
			new URL(response.url()).pathname.startsWith('/api/') &&
			response.status() >= 400 &&
			!(expectedFailures.has(new URL(response.url()).pathname) && response.status() === 502)
		)
			errors.push(`${response.status()} ${response.url()}`);
	});
	page.on('requestfailed', (failed) => {
		if (
			new URL(failed.url()).pathname.startsWith('/api/') &&
			!failed.failure()?.errorText.includes('ERR_ABORTED')
		)
			errors.push(failed.url());
	});
	await page.route('**/*', (route) => {
		if (['127.0.0.1', 'localhost'].includes(new URL(route.request().url()).hostname))
			return route.continue();
		blocked.add(route.request().url());
		return route.abort();
	});
	return errors;
}
function email(title: string, subject: string) {
	return {
		id: crypto.randomUUID(),
		title,
		email_subject: subject,
		email_body: `<p>Saved ${title} for {{email}}</p>`,
		text_body: `Plain ${title}`,
		status: 'active',
		delay_value: 0,
		delay_unit: 'minutes'
	};
}
async function create(client: APIRequestContext, title: string) {
	const response = await client.post(root, {
		data: {
			title,
			emails: [
				email('First QA email', 'First QA subject'),
				email('Second QA email', 'Second QA for {{email}}')
			]
		}
	});
	expect(response.status()).toBe(200);
	const row = record(await response.json());
	if (typeof row.id !== 'string') throw new Error('Sequence identity was not confirmed');
	return { id: row.id, source: row };
}
async function remove(client: APIRequestContext, ids: Set<string>) {
	for (const id of ids) {
		const saved = record(record(await (await client.get(`${root}/${id}`)).json()).sequence);
		if (saved.status === 'active')
			expect(
				(
					await client.put(`${root}/${id}`, {
						data: { status: 'paused', expected_version: saved.authoring_version }
					})
				).status()
			).toBe(200);
		await expect
			.poll(async () => record(await (await client.get(`${root}/${id}/stats`)).json()).processing)
			.toBe(0);
		expect((await client.delete(`${root}/${id}`)).status()).toBe(200);
	}
}
async function openSecondary(page: Page, name: string) {
	await page.goto('/email', { waitUntil: 'networkidle' });
	await page.locator('.tabs').getByRole('button', { name: 'Sequences', exact: true }).click();
	await page.locator('.sequences-list').getByLabel('Search sequences', { exact: true }).fill(name);
	await page.getByRole('button', { name: 'Filter sequences', exact: true }).click();
}

test('secondary sequence actions use canonical records and navigate to real editors, subscribers and analytics', async ({
	page,
	baseURL
}) => {
	await login(page, baseURL);
	const errors = await diagnostics(page),
		name = `Secondary sequence ${Date.now()}`,
		saved = await create(page.request, name),
		ids = new Set([saved.id]);
	try {
		let writes = 0;
		page.on('request', (event) => {
			if (
				new URL(event.url()).pathname.startsWith('/api/admin/email/sequences') &&
				event.method() !== 'GET'
			)
				writes += 1;
		});
		await openSecondary(page, name);
		const card = page
			.locator('.sequence-card')
			.filter({ has: page.getByRole('heading', { name, exact: true }) });
		await expect(card).toBeVisible();
		expect(writes).toBe(0);
		await card.getByRole('link', { name: 'Edit', exact: true }).click();
		await expect(page).toHaveURL(new RegExp(`/admin/crm/sequences/${saved.id}/edit$`));
		await expect(page.getByLabel('Sequence title', { exact: true })).toHaveValue(name);
		await openSecondary(page, name);
		await card.getByRole('link', { name: 'Subscribers', exact: true }).click();
		await expect(
			page.getByRole('heading', { name: `${name} — Subscribers`, exact: true })
		).toBeVisible();
		await openSecondary(page, name);
		await card.getByRole('link', { name: 'View', exact: true }).click();
		await expect(
			page.getByRole('heading', { name: `${name} — Analytics`, exact: true })
		).toBeVisible();
		await openSecondary(page, name);
		await card.getByRole('button', { name: 'Activate', exact: true }).click();
		await expect(card.locator('.sequence-status')).toHaveText('active');
		await expect(card.getByRole('button', { name: 'Delete', exact: true })).toBeDisabled();
		await openSecondary(page, name);
		await expect(card.locator('.sequence-status')).toHaveText('active');
		await card.getByRole('button', { name: 'Pause', exact: true }).click();
		await expect(card.locator('.sequence-status')).toHaveText('paused');
		const duplicating = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === `/api/admin/email/sequences/${saved.id}/duplicate`
		);
		await card.getByRole('button', { name: 'Duplicate', exact: true }).click();
		const duplicated = await duplicating;
		expect(duplicated.status()).toBe(200);
		const clone = record(await duplicated.json()).id;
		if (typeof clone !== 'string')
			throw new Error('The copied sequence identity was not confirmed');
		ids.add(clone);
		const copy = page
			.locator('.sequence-card')
			.filter({ has: page.getByRole('heading', { name: `${name} (Copy)`, exact: true }) });
		await expect(copy).toBeVisible();
		await copy.getByRole('button', { name: 'Delete', exact: true }).click();
		const deleting = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === `/api/admin/email/sequences/${clone}` &&
				response.request().method() === 'DELETE'
		);
		await page
			.getByRole('dialog', { name: 'Delete Sequence', exact: true })
			.getByRole('button', { name: 'Delete', exact: true })
			.click();
		expect((await deleting).status()).toBe(200);
		ids.delete(clone);
		await expect(copy).toHaveCount(0);
		await expect(card).toBeVisible();
		expect(errors).toEqual([]);
	} finally {
		await remove(page.request, ids);
	}
});

test('sequence QA records actual selected-email SMTP acceptance and rejection, and replays frozen history without another connection', async ({
	page,
	baseURL
}) => {
	await login(page, baseURL);
	const recipient = 'sequence-qa-browser@example.test',
		relay = await fixtureRelay(19027, new Set([recipient])),
		expectedFailures = new Set<string>(),
		errors = await diagnostics(page, expectedFailures),
		ids = new Set<string>();
	try {
		expect(
			record(await (await page.request.get('/api/admin/email/status')).json()).postmark_token_set
		).toBe(false);
		await page.goto('/admin/email/smtp', { waitUntil: 'networkidle' });
		for (const [label, value] of [
			['SMTP Host', '127.0.0.1'],
			['Port', '19027'],
			['Username', 'fixture-user'],
			['From Email', 'sequence-qa-sender@example.test'],
			['From Name', 'Sequence QA fixture']
		] as const)
			await page.getByLabel(label, { exact: true }).fill(value);
		await page.getByLabel('Encryption', { exact: true }).selectOption('null');
		await page.getByLabel(/Password \/ App Password/).fill('fixture-password');
		await page.getByRole('button', { name: 'Save Settings', exact: true }).click();
		await expect(
			page.getByText('SMTP settings saved. Test the connection to verify relay access.', {
				exact: true
			})
		).toBeVisible();
		await page.getByRole('button', { name: 'Test Connection', exact: true }).click();
		await expect(
			page.getByText('Authenticated SMTP connection and NOOP passed; no email was sent', {
				exact: true
			})
		).toBeVisible();
		const saved = await create(page.request, `Sequence QA ${Date.now()}`);
		ids.add(saved.id);
		const path = `${root}/${saved.id}`;
		await page.goto(`/admin/crm/sequences/${saved.id}/edit`, { waitUntil: 'networkidle' });
		await page
			.locator('.sequence-form')
			.getByRole('button', { name: 'Test saved email', exact: true })
			.click();
		const dialog = page.getByRole('dialog', { name: 'Sequence test message', exact: true });
		await expect(dialog.getByText('No recorded sequence tests.', { exact: true })).toBeVisible();
		expect(relay.messages).toHaveLength(0);
		await dialog
			.getByLabel('Saved email', { exact: true })
			.selectOption({ label: 'Email 2: Second QA email — Second QA for {{email}}' });
		await dialog.getByLabel('Test recipient', { exact: true }).fill(recipient);
		await dialog.getByRole('button', { name: 'Send test message', exact: true }).click();
		await expect(dialog.getByText('Provider accepted', { exact: true })).toBeVisible();
		expect(relay.messages).toHaveLength(1);
		const message = relay.messages[0];
		if (!message) throw new Error('No actual SMTP receipt');
		expect(mimeSubject(message.raw)).toBe(`[Test] Second QA for ${recipient}`);
		expect(mimeText(message.raw)).toContain(`Saved Second QA email for ${recipient}`);
		expect(mimeText(message.raw)).toContain('Plain Second QA email');
		const acceptedConnections = relay.connections;
		await dialog.getByRole('button', { name: 'Retry same request', exact: true }).click();
		await expect(
			dialog.getByRole('button', { name: 'Retry same request', exact: true })
		).toBeEnabled();
		expect(relay.connections).toBe(acceptedConnections);
		await dialog.getByRole('button', { name: 'Prepare another test', exact: true }).click();
		await dialog
			.getByLabel('Test recipient', { exact: true })
			.fill('sequence-rejected@example.test');
		expectedFailures.add(`${path}/test`);
		const rejecting = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === `${path}/test` &&
				response.request().method() === 'POST'
		);
		await dialog.getByRole('button', { name: 'Send test message', exact: true }).click();
		expect((await rejecting).status()).toBe(502);
		await expect(dialog.getByText('Provider rejected', { exact: true })).toBeVisible();
		const rejectedConnections = relay.connections;
		await dialog.getByRole('button', { name: 'Retry same request', exact: true }).click();
		await expect(
			dialog.getByRole('button', { name: 'Retry same request', exact: true })
		).toBeEnabled();
		expect(relay.connections).toBe(rejectedConnections);
		expect(relay.messages).toHaveLength(1);
		await dialog.getByRole('button', { name: 'Close sequence test message', exact: true }).click();
		await page.reload({ waitUntil: 'networkidle' });
		await page
			.locator('.sequence-form')
			.getByRole('button', { name: 'Test saved email', exact: true })
			.click();
		await expect(dialog.getByText('Provider accepted', { exact: true })).toBeVisible();
		await expect(dialog.getByText('Provider rejected', { exact: true })).toBeVisible();
		const accepted = dialog.locator('article').filter({ hasText: recipient });
		await accepted.getByRole('button', { name: 'Read this request again', exact: true }).click();
		await expect(
			accepted.getByRole('button', { name: 'Read this request again', exact: true })
		).toBeEnabled();
		expect(relay.connections).toBe(rejectedConnections);
		const stats = record(await (await page.request.get(`${path}/stats`)).json());
		expect(stats).toMatchObject({ total_sent: 0, subscribers: 0, queued: 0, processing: 0 });
		expect(record(await (await page.request.get(`${path}/subscribers`)).json()).data).toEqual([]);
		const malformed = `**${path}/tests`;
		await page.route(malformed, (route) =>
			route.fulfill({
				json: {
					success: true,
					data: {
						sequence_id: saved.id,
						metrics_scope: 'sequence_test',
						requests: [{ request_id: 'wrong' }]
					}
				}
			})
		);
		await dialog.getByRole('button', { name: 'Check receipts', exact: true }).click();
		await expect(dialog.getByRole('alert')).toContainText('Invalid sequence test');
		await page.unroute(malformed);
		await dialog.getByRole('button', { name: 'Check receipts', exact: true }).click();
		await expect(dialog.getByRole('alert')).toHaveCount(0);
		expect(errors).toEqual([]);
	} finally {
		try {
			await remove(page.request, ids);
		} finally {
			await relay.close();
		}
	}
});

test('saved custom-event configuration records a real event and consent-checked enrollment in trigger history', async ({
	page,
	baseURL
}) => {
	await login(page, baseURL);
	const errors = await diagnostics(page),
		ids = new Set<string>(),
		suffix = Date.now(),
		eventName = `browser_sequence_${suffix}`,
		title = `Trigger sequence ${suffix}`;
	try {
		const contacts = record(
			await (
				await page.request.get('/api/admin/crm/contacts?search=sequence-browser%40example.test')
			).json()
		).data;
		if (!Array.isArray(contacts)) throw new Error('Seed the disposable confirmed sequence contact');
		const contact = contacts
			.map(record)
			.find((row) => row.email === 'sequence-browser@example.test');
		if (!contact || typeof contact.id !== 'number')
			throw new Error('Missing actual sequence trigger fixture');
		await page.goto('/admin/crm/sequences/new', { waitUntil: 'networkidle' });
		await page.getByLabel('Sequence title', { exact: true }).fill(title);
		await page.getByLabel('Enrollment trigger', { exact: true }).selectOption('custom_event');
		await page
			.getByLabel('Trigger settings (JSON)', { exact: true })
			.fill(JSON.stringify({ event_name: eventName }));
		await page.getByRole('button', { name: 'Add email', exact: true }).click();
		const step = page.locator('[data-sequence-step="1"]');
		await step.getByLabel('Email title', { exact: true }).fill('Future trigger email');
		await step.getByLabel('Subject', { exact: true }).fill('Actual trigger fixture');
		await step.getByLabel('Email HTML', { exact: true }).fill('<p>Recorded trigger fixture</p>');
		await step
			.getByLabel('Delay after the previous accepted email (first email: after enrollment)', {
				exact: true
			})
			.fill('1');
		await step.getByLabel('Delay unit for email 1', { exact: true }).selectOption('days');
		const creating = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === root && response.request().method() === 'POST'
		);
		await page.getByRole('button', { name: 'Create draft', exact: true }).click();
		const created = await creating;
		expect(created.status()).toBe(200);
		const id = record(await created.json()).id;
		if (typeof id !== 'string') throw new Error('Missing trigger sequence ID');
		ids.add(id);
		await expect(page).toHaveURL(new RegExp(`/admin/crm/sequences/${id}/edit$`));
		await page.getByRole('button', { name: 'Activate saved sequence', exact: true }).click();
		await expect(page.getByText('Saved sequence activated.', { exact: true })).toBeVisible();
		const input = {
			event_id: crypto.randomUUID(),
			contact_id: contact.id,
			event_name: eventName,
			properties: { fixture: 'actual browser event' }
		};
		const recorded = await page.request.post(`${root}/events`, { data: input });
		expect(recorded.status()).toBe(200);
		const first = record(await recorded.json());
		expect(first).toMatchObject({
			event_recorded: true,
			event_id: input.event_id,
			metrics_scope: 'sequence_trigger'
		});
		const replayed = await page.request.post(`${root}/events`, { data: input });
		expect(replayed.status()).toBe(200);
		expect(record(await replayed.json()).record_id).toBe(first.record_id);
		await expect
			.poll(
				async () => {
					const rows = record(await (await page.request.get(`${root}/${id}/triggers`)).json()).data;
					return Array.isArray(rows) && rows.length === 1 ? record(rows[0]).status : 'missing';
				},
				{ timeout: 20000 }
			)
			.toBe('enrolled');
		await page
			.locator('.sequence-form')
			.getByRole('link', { name: 'Analytics', exact: true })
			.click();
		const history = page.getByRole('region', { name: 'Recorded trigger activity', exact: true });
		await expect(history.getByRole('row').filter({ hasText: 'custom_event' })).toContainText(
			'enrolled'
		);
		await page.reload({ waitUntil: 'networkidle' });
		await expect(history.getByRole('row').filter({ hasText: 'custom_event' })).toContainText(
			'enrolled'
		);
		const trackerRows = record(
			await (await page.request.get(`${root}/${id}/subscribers`)).json()
		).data;
		if (!Array.isArray(trackerRows)) throw new Error('Missing real enrollment');
		expect(trackerRows).toHaveLength(1);
		expect(record(trackerRows[0])).toMatchObject({
			contact_id: String(contact.id),
			emails_sent: 0,
			status: 'active'
		});
		expect(record(await (await page.request.get(`${root}/${id}/stats`)).json())).toMatchObject({
			subscribers: 1,
			total_sent: 0
		});
		expect(errors).toEqual([]);
	} finally {
		await remove(page.request, ids);
	}
});

test('unsupported legacy triggers stay visible with valid emails until an explicit supported repair is saved', async ({
	page,
	baseURL
}) => {
	await login(page, baseURL);
	const errors = await diagnostics(page),
		name = 'Legacy trigger browser fixture';
	const listing = await page.request.get(`${root}?search=${encodeURIComponent(name)}&per_page=100`);
	expect(listing.status()).toBe(200);
	const data = record(await listing.json()).data;
	if (!Array.isArray(data))
		throw new Error('Seed the controlled raw legacy-trigger browser fixture');
	const matching = data.map(record).filter((row) => row.title === name);
	expect(matching).toHaveLength(1);
	const fixture = matching[0];
	if (!fixture || typeof fixture.id !== 'string' || typeof fixture.authoring_version !== 'number')
		throw new Error('Legacy trigger fixture identity is unavailable');
	expect(fixture).toMatchObject({
		trigger_type: 'legacy',
		historical_trigger_type: 'historical-unsupported',
		emails_count: 1
	});
	const id = fixture.id,
		ids = new Set([id]);
	try {
		const writes: string[] = [];
		page.on('request', (event) => {
			if (new URL(event.url()).pathname === `${root}/${id}` && event.method() !== 'GET')
				writes.push(event.method());
		});
		await page.goto(`/admin/crm/sequences/${id}/edit`, { waitUntil: 'networkidle' });
		await expect(page.getByLabel('Enrollment trigger', { exact: true })).toHaveValue('legacy');
		await expect(page.getByRole('alert')).toContainText('historical-unsupported');
		await expect(page.locator('[data-sequence-step]')).toHaveCount(1);
		expect(writes).toEqual([]);
		await page.getByRole('button', { name: 'Save sequence', exact: true }).click();
		await expect(
			page.getByRole('alert').filter({
				hasText: 'Choose a supported enrollment trigger and review its settings before saving'
			})
		).toBeVisible();
		expect(writes).toEqual([]);
		await page.getByLabel('Enrollment trigger', { exact: true }).selectOption('manual');
		await page.getByLabel('Trigger settings (JSON)', { exact: true }).fill('{}');
		await page.getByRole('button', { name: 'Save sequence', exact: true }).click();
		await expect(page.getByRole('status')).toContainText('Sequence saved.');
		await page.reload({ waitUntil: 'networkidle' });
		await expect(page.getByLabel('Enrollment trigger', { exact: true })).toHaveValue('manual');
		await expect(page.getByRole('alert')).toHaveCount(0);
		await expect(page.locator('[data-sequence-step]')).toHaveCount(1);
		const repaired = record(
			record(await (await page.request.get(`${root}/${id}`)).json()).sequence
		);
		expect(repaired).toMatchObject({
			trigger_type: 'manual',
			trigger_config: {},
			trigger_error: null,
			historical_trigger_type: null,
			emails_count: 1
		});
		expect(repaired.authoring_version).toBeGreaterThan(fixture.authoring_version);
		expect(errors).toEqual([]);
	} finally {
		await remove(page.request, ids);
	}
});

test('secondary sequence pagination recovers after deleting its final page and rejects a malformed catalog before retry', async ({
	page,
	baseURL
}) => {
	await login(page, baseURL);
	const errors = await diagnostics(page),
		ids = new Set<string>(),
		prefix = `Sequence pages ${Date.now()}`;
	const listing = /\/api\/admin\/email\/sequences(?:\?|$)/;
	try {
		for (let index = 0; index < 21; index++) {
			const response = await page.request.post(root, { data: { title: `${prefix} ${index}` } });
			expect(response.status()).toBe(200);
			const id = record(await response.json()).id;
			if (typeof id !== 'string') throw new Error('Pagination fixture identity unavailable');
			ids.add(id);
		}
		await openSecondary(page, prefix);
		const pages = page
			.locator('.sequences-list')
			.getByRole('navigation', { name: 'Sequence pages', exact: true });
		await expect(pages).toContainText('21 matching sequences · Page 1 of 2');
		await expect(page.locator('.sequence-card')).toHaveCount(20);
		await pages.getByRole('button', { name: 'Next page', exact: true }).click();
		await expect(page.locator('.sequence-card')).toHaveCount(1);
		await expect(pages).toContainText('Page 2 of 2');
		await page
			.locator('.sequence-card')
			.getByRole('button', { name: 'Delete', exact: true })
			.click();
		const deleting = page.waitForResponse(
			(response) =>
				/^\/api\/admin\/email\/sequences\/\d+$/.test(new URL(response.url()).pathname) &&
				response.request().method() === 'DELETE'
		);
		await page
			.getByRole('dialog', { name: 'Delete Sequence', exact: true })
			.getByRole('button', { name: 'Delete', exact: true })
			.click();
		const removed = await deleting;
		expect(removed.status()).toBe(200);
		const deleted = record(await removed.json()).id;
		if (typeof deleted !== 'string') throw new Error('Deletion identity unavailable');
		ids.delete(deleted);
		await expect(page.locator('.sequence-card')).toHaveCount(20);
		await expect(pages).toContainText('20 matching sequences · Page 1 of 1');
		await page.route(listing, (route) =>
			route.fulfill({
				json: {
					data: [{ id: '42' }],
					meta: { current_page: 1, per_page: 20, total: 1, last_page: 1 }
				}
			})
		);
		await page.getByRole('button', { name: 'Filter sequences', exact: true }).click();
		await expect(page.locator('.sequences-list').getByRole('alert')).toContainText(
			'Invalid sequence identity or measurement scope'
		);
		await expect(page.locator('.sequence-card')).toHaveCount(0);
		await page.unroute(listing);
		await page.getByRole('button', { name: 'Retry sequences', exact: true }).click();
		await expect(page.locator('.sequence-card')).toHaveCount(20);
		await expect(pages).toContainText('20 matching sequences · Page 1 of 1');
		expect(errors).toEqual([]);
	} finally {
		await page.unroute(listing);
		await remove(page.request, ids);
	}
});
