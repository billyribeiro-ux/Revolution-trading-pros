import { expect, test, request, type Page, type APIRequestContext } from '@playwright/test';
import { fixtureRelay, mimeSubject, mimeText } from './fixtures/smtp-relay.js';

// This suite is confined to the disposable source-verification stack. It never
// uses external providers; the SMTP case accepts one explicitly allowed .test address.
test.describe.configure({ mode: 'default' });
const root = '/api/admin/crm/sequences';
const password = 'OnlyForDisposableTest123!';
function object(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Expected an actual fixture record');
	return value as Record<string, unknown>;
}
function sequenceId(value: unknown): string {
	const id = object(value).id;
	if (typeof id !== 'string' || !/^[1-9]\d*$/.test(id))
		throw new Error('The saved sequence identity was not confirmed');
	return id;
}
async function login(page: Page, baseURL: string | undefined) {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	expect(
		(
			await page.request.post('/api/auth/login', {
				data: { email: 'admin@admin-completion.test', password }
			})
		).status()
	).toBe(200);
}
async function isolate(page: Page) {
	const failures: string[] = [],
		blocked = new Set<string>();
	page.on('pageerror', (error) => failures.push(error.message));
	page.on('console', (message) => {
		if (message.type() !== 'error') return;
		if (
			blocked.has(message.location().url) &&
			message.text() === 'Failed to load resource: net::ERR_FAILED'
		)
			return;
		failures.push(`${message.text()} ${message.location().url}`);
	});
	page.on('response', (response) => {
		if (new URL(response.url()).pathname.startsWith('/api/') && response.status() >= 400)
			failures.push(`${response.status()} ${response.url()}`);
	});
	page.on('requestfailed', (failed) => {
		if (
			new URL(failed.url()).pathname.startsWith('/api/') &&
			!failed.failure()?.errorText.includes('ERR_ABORTED')
		)
			failures.push(`${failed.failure()?.errorText} ${failed.url()}`);
	});
	await page.route('**/*', (route) => {
		if (['127.0.0.1', 'localhost'].includes(new URL(route.request().url()).hostname))
			return route.continue();
		blocked.add(route.request().url());
		return route.abort();
	});
	return failures;
}
async function createDraft(client: APIRequestContext, title: string) {
	const response = await client.post(root, { data: { title } });
	expect(response.status()).toBe(200);
	return sequenceId(await response.json());
}
async function cleanup(client: APIRequestContext, ids: Set<string>) {
	for (const id of ids) {
		const response = await client.get(`${root}/${id}`);
		expect(response.status()).toBe(200);
		const saved = object(object(await response.json()).sequence);
		if (saved.status === 'active') {
			const paused = await client.put(`${root}/${id}`, {
				data: { status: 'paused', expected_version: saved.authoring_version }
			});
			expect(paused.status()).toBe(200);
		}
		await expect
			.poll(async () => object(await (await client.get(`${root}/${id}/stats`)).json()).processing)
			.toBe(0);
		expect((await client.delete(`${root}/${id}`)).status()).toBe(200);
	}
}
async function fillMail(page: Page, position: number, title: string, subject: string) {
	const step = page.locator(`[data-sequence-step="${position}"]`);
	await step.getByLabel('Email title', { exact: true }).fill(title);
	await step.getByLabel('Subject', { exact: true }).fill(subject);
	await step.getByLabel('Email HTML', { exact: true }).fill(`<p>Saved ${title} for {{email}}</p>`);
	await step.getByLabel('Plain text alternative', { exact: true }).fill(`Plain ${title}`);
}

test('sequence authoring preserves ordered emails, status and independent duplicates after reload', async ({
	page,
	baseURL
}) => {
	await login(page, baseURL);
	const failures = await isolate(page),
		ids = new Set<string>(),
		name = `Sequence authoring ${Date.now()}`;
	try {
		let writes = 0;
		page.on('request', (event) => {
			if (new URL(event.url()).pathname.startsWith(root) && event.method() !== 'GET') writes += 1;
		});
		await page.goto('/admin/crm/sequences/new', { waitUntil: 'networkidle' });
		await expect(page.getByRole('heading', { name: 'New Sequence', exact: true })).toBeVisible();
		expect(writes).toBe(0);
		await page.getByLabel('Sequence title', { exact: true }).fill(name);
		await page.getByLabel('Description', { exact: true }).fill('Saved sequence description');
		await page.getByRole('button', { name: 'Add email', exact: true }).click();
		await fillMail(page, 1, 'First email', 'First saved subject');
		await page.getByRole('button', { name: 'Add email', exact: true }).click();
		await fillMail(page, 2, 'Second email', 'Second saved subject');
		await page
			.locator('[data-sequence-step="2"]')
			.getByRole('button', { name: 'Move up', exact: true })
			.click();
		const saving = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === root && response.request().method() === 'POST'
		);
		await page.getByRole('button', { name: 'Create draft', exact: true }).click();
		const created = await saving;
		expect(created.status()).toBe(200);
		const id = sequenceId(await created.json());
		ids.add(id);
		await expect(page).toHaveURL(new RegExp(`/admin/crm/sequences/${id}/edit$`));
		await page.reload({ waitUntil: 'networkidle' });
		await expect(page.getByLabel('Sequence title', { exact: true })).toHaveValue(name);
		await expect(
			page.locator('[data-sequence-step="1"]').getByLabel('Email title', { exact: true })
		).toHaveValue('Second email');
		await expect(
			page.locator('[data-sequence-step="2"]').getByLabel('Subject', { exact: true })
		).toHaveValue('First saved subject');
		await expect(
			page.locator('[data-sequence-step="1"]').getByLabel('Plain text alternative', { exact: true })
		).toHaveValue('Plain Second email');
		const baselineWrites = writes;
		await page.getByRole('button', { name: 'Reload saved state', exact: true }).click();
		await expect(page.getByLabel('Sequence title', { exact: true })).toBeEnabled();
		expect(writes).toBe(baselineWrites);
		await page.getByRole('button', { name: 'Activate saved sequence', exact: true }).click();
		await expect(page.getByText('Saved sequence activated.', { exact: true })).toBeVisible();
		await expect(page.getByLabel('Sequence title', { exact: true })).toBeDisabled();
		await expect(page.getByRole('button', { name: 'Save sequence', exact: true })).toBeDisabled();
		await page.reload({ waitUntil: 'networkidle' });
		await expect(page.getByLabel('Sequence title', { exact: true })).toBeDisabled();
		await page.getByRole('button', { name: 'Pause sequence', exact: true }).click();
		await expect(page.getByLabel('Sequence title', { exact: true })).toBeEnabled();
		await page.getByLabel('Description', { exact: true }).fill('Updated while paused');
		await page.getByRole('button', { name: 'Save sequence', exact: true }).click();
		await expect(page.getByRole('status')).toContainText('Sequence saved.');
		await page.reload({ waitUntil: 'networkidle' });
		await expect(page.getByLabel('Description', { exact: true })).toHaveValue(
			'Updated while paused'
		);
		await page.goto('/admin/crm/sequences', { waitUntil: 'networkidle' });
		await page.getByLabel('Search sequences', { exact: true }).fill(name);
		const row = page.locator('.admin-crm-sequences .data-table tbody tr').filter({ hasText: name });
		await expect(row).toHaveCount(1);
		const duplicating = page.waitForResponse(
			(response) => new URL(response.url()).pathname === `${root}/${id}/duplicate`
		);
		await row.getByRole('button', { name: 'Duplicate', exact: true }).click();
		const duplicated = await duplicating;
		expect(duplicated.status()).toBe(200);
		const clone = sequenceId(await duplicated.json());
		ids.add(clone);
		expect(clone).not.toBe(id);
		const copy = page
			.locator('.admin-crm-sequences .data-table tbody tr')
			.filter({ hasText: `${name} (Copy)` });
		await expect(copy).toHaveCount(1);
		await copy.getByRole('button', { name: 'Delete', exact: true }).click();
		const deleting = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === `${root}/${clone}` &&
				response.request().method() === 'DELETE'
		);
		await page
			.getByRole('dialog', { name: 'Delete Sequence', exact: true })
			.getByRole('button', { name: 'Delete', exact: true })
			.click();
		expect((await deleting).status()).toBe(200);
		ids.delete(clone);
		await expect(copy).toHaveCount(0);
		await expect(
			page.locator('.admin-crm-sequences .data-table tbody tr').filter({ hasText: name })
		).toHaveCount(1);
		expect(failures).toEqual([]);
	} finally {
		await cleanup(page.request, ids);
	}
});

test('sequence editor recovers from unavailable and malformed reads and rejects stale saved versions', async ({
	page,
	baseURL
}) => {
	await login(page, baseURL);
	const errors: string[] = [],
		writes: string[] = [],
		id = await createDraft(page.request, `Sequence recovery ${Date.now()}`),
		path = `${root}/${id}`;
	page.on('pageerror', (error) => errors.push(error.message));
	page.on('request', (event) => {
		if (new URL(event.url()).pathname === path && event.method() !== 'GET')
			writes.push(event.method());
	});
	await page.route('**/*', (route) =>
		['127.0.0.1', 'localhost'].includes(new URL(route.request().url()).hostname)
			? route.continue()
			: route.abort()
	);
	const resource = `**${path}`;
	try {
		await page.route(resource, (route) =>
			route.fulfill({ status: 503, json: { error: 'Fixture sequence read unavailable' } })
		);
		await page.goto(`/admin/crm/sequences/${id}/edit`, { waitUntil: 'networkidle' });
		await expect(page.getByRole('alert')).toHaveText('Service unavailable');
		await expect(page.getByRole('button', { name: 'Save sequence', exact: true })).toHaveCount(0);
		expect(writes).toEqual([]);
		await page.unroute(resource);
		await page.route(resource, (route) => route.fulfill({ json: { sequence: { id }, stats: {} } }));
		await page.getByRole('button', { name: 'Reload saved state', exact: true }).click();
		await expect(page.getByRole('alert')).toContainText(
			'Invalid sequence identity or measurement scope'
		);
		expect(writes).toEqual([]);
		await page.unroute(resource);
		await page.getByRole('button', { name: 'Reload saved state', exact: true }).click();
		await expect(page.getByLabel('Sequence title', { exact: true })).toBeEnabled();
		await expect(page.getByRole('alert')).toHaveCount(0);
		const saved = object(await (await page.request.get(path)).json()),
			sequence = object(saved.sequence),
			version = sequence.authoring_version;
		expect(typeof version).toBe('number');
		expect(
			(
				await page.request.put(path, {
					data: { description: 'Concurrent saved description', expected_version: version }
				})
			).status()
		).toBe(200);
		await page.getByLabel('Description', { exact: true }).fill('Stale editor overwrite');
		const stale = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === path && response.request().method() === 'PUT'
		);
		await page.getByRole('button', { name: 'Save sequence', exact: true }).click();
		expect((await stale).status()).toBe(409);
		await expect(page.getByRole('alert')).toContainText('Sequence changed; reload before saving');
		await page.getByRole('button', { name: 'Reload saved state', exact: true }).click();
		await expect(page.getByLabel('Description', { exact: true })).toHaveValue(
			'Concurrent saved description'
		);
		await page.route(resource, (route) =>
			route.request().method() === 'GET'
				? route.fulfill({ status: 503, json: { message: 'Fixture activity refresh unavailable' } })
				: route.continue()
		);
		await page.getByLabel('Description', { exact: true }).fill('Confirmed despite activity outage');
		await page.getByRole('button', { name: 'Save sequence', exact: true }).click();
		await expect(page.getByRole('status')).toContainText('Sequence saved.');
		await expect(page.getByRole('alert')).toContainText('Fixture activity refresh unavailable');
		await page.unroute(resource);
		await page.getByRole('button', { name: 'Reload saved state', exact: true }).click();
		await expect(page.getByLabel('Description', { exact: true })).toHaveValue(
			'Confirmed despite activity outage'
		);
		expect(errors).toEqual([]);
	} finally {
		await page.unroute(resource);
		await cleanup(page.request, new Set([id]));
	}
});

test('sequence reads and mutations reject anonymous and ordinary members in both proxy namespaces', async ({
	baseURL
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	const client = await request.newContext({ baseURL });
	try {
		for (const expected of [401, 403]) {
			if (expected === 403)
				expect(
					(
						await client.post('/api/auth/login', {
							data: { email: 'user@admin-completion.test', password }
						})
					).status()
				).toBe(200);
			for (const namespace of [root, '/api/admin/email/sequences']) {
				for (const suffix of ['', '/1', '/1/stats', '/1/subscribers', '/1/emails'])
					expect((await client.get(`${namespace}${suffix}`)).status()).toBe(expected);
				expect(
					(await client.post(namespace, { data: { title: 'Must never persist' } })).status()
				).toBe(expected);
				expect((await client.put(`${namespace}/1`, { data: { status: 'active' } })).status()).toBe(
					expected
				);
				for (const action of ['duplicate', 'activate', 'pause', 'subscribe', 'unsubscribe', 'test'])
					expect((await client.post(`${namespace}/1/${action}`, { data: {} })).status()).toBe(
						expected
					);
				expect((await client.delete(`${namespace}/1`)).status()).toBe(expected);
			}
			expect((await client.get('/admin/crm/sequences', { maxRedirects: 0 })).status()).toBe(303);
		}
		expect((await client.post('/api/auth/logout')).status()).toBe(200);
	} finally {
		await client.dispose();
	}
});

test('running sequence timers submit one actual SMTP message and retain the receipt without repeated enrollment resends', async ({
	page,
	baseURL
}) => {
	await login(page, baseURL);
	const recipient = 'sequence-browser@example.test',
		relay = await fixtureRelay(19027, new Set([recipient])),
		ids = new Set<string>();
	const failures = await isolate(page),
		name = `Sequence timer ${Date.now()}`;
	try {
		expect(
			object(await (await page.request.get('/api/admin/email/status')).json()).postmark_token_set
		).toBe(false);
		const contacts = object(
			await (
				await page.request.get(`/api/admin/crm/contacts?search=${encodeURIComponent(recipient)}`)
			).json()
		).data;
		if (!Array.isArray(contacts)) throw new Error('Seed the disposable sequence contact');
		const matching = contacts.map(object).filter((contact) => contact.email === recipient);
		expect(matching).toHaveLength(1);
		const contactId = matching[0]?.id;
		if (typeof contactId !== 'number' || !Number.isSafeInteger(contactId))
			throw new Error('Missing actual fixture contact identity');
		await page.goto('/admin/email/smtp', { waitUntil: 'networkidle' });
		for (const [label, value] of [
			['SMTP Host', '127.0.0.1'],
			['Port', '19027'],
			['Username', 'fixture-user'],
			['From Email', 'sequence-sender@example.test'],
			['From Name', 'Sequence timer fixture']
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
		expect(relay.messages).toHaveLength(0);
		await page.goto('/admin/crm/sequences/new', { waitUntil: 'networkidle' });
		await page.getByLabel('Sequence title', { exact: true }).fill(name);
		await page.getByRole('button', { name: 'Add email', exact: true }).click();
		await fillMail(page, 1, 'Actual timer email', 'Sequence timer for {{email}}');
		const creating = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === root && response.request().method() === 'POST'
		);
		await page.getByRole('button', { name: 'Create draft', exact: true }).click();
		const created = await creating;
		expect(created.status()).toBe(200);
		const id = sequenceId(await created.json());
		ids.add(id);
		await expect(page).toHaveURL(new RegExp(`/admin/crm/sequences/${id}/edit$`));
		await page.getByRole('button', { name: 'Activate saved sequence', exact: true }).click();
		await expect(page.getByText('Saved sequence activated.', { exact: true })).toBeVisible();
		await page
			.locator('.sequence-form')
			.getByRole('link', { name: 'Subscribers', exact: true })
			.click();
		await page
			.getByLabel('Contact IDs (separated by commas or spaces)', { exact: true })
			.fill(String(contactId));
		await page.getByRole('button', { name: 'Enroll contacts', exact: true }).click();
		await expect(
			page
				.locator('.sequence-detail')
				.getByRole('status')
				.filter({ hasText: '1 contacts enrolled' })
		).toBeVisible();
		await expect.poll(() => relay.messages.length, { timeout: 30000 }).toBe(1);
		const message = relay.messages[0];
		if (!message) throw new Error('No actual SMTP message was acknowledged');
		expect(mimeSubject(message.raw)).toBe(`Sequence timer for ${recipient}`);
		expect(mimeText(message.raw)).toContain(`Saved Actual timer email for ${recipient}`);
		expect(mimeText(message.raw)).toContain('Plain Actual timer email');
		expect(message.raw).toContain('sequence-sender@example.test');
		await expect
			.poll(
				async () => object(await (await page.request.get(`${root}/${id}/stats`)).json()).completed,
				{ timeout: 20000 }
			)
			.toBe(1);
		await page.getByRole('button', { name: 'Refresh', exact: true }).click();
		await expect(
			page.locator('.sequence-detail tbody tr').filter({ hasText: recipient })
		).toContainText('completed');
		await page.getByRole('button', { name: 'Enroll contacts', exact: true }).click();
		await expect(
			page
				.locator('.sequence-detail')
				.getByRole('status')
				.filter({ hasText: '0 contacts enrolled; 1 already enrolled' })
		).toBeVisible();
		await page
			.locator('.sequence-detail nav')
			.getByRole('link', { name: 'Analytics', exact: true })
			.click();
		const metric = (label: string) =>
			page
				.locator('.sequence-detail dt')
				.filter({ hasText: new RegExp(`^${label}$`) })
				.locator('+ dd');
		await expect(metric('Accepted by provider')).toHaveText('1');
		await expect(metric('Delivered')).toHaveText('0');
		await page.reload({ waitUntil: 'networkidle' });
		await expect(metric('Accepted by provider')).toHaveText('1');
		await expect(metric('Completed enrollments')).toHaveText('1');
		const subscribers = object(
			await (await page.request.get(`${root}/${id}/subscribers`)).json()
		).data;
		if (!Array.isArray(subscribers)) throw new Error('Missing persisted sequence receipt');
		expect(subscribers).toHaveLength(1);
		expect(object(subscribers[0])).toMatchObject({
			contact_id: String(contactId),
			status: 'completed',
			emails_sent: 1,
			emails_opened: 0,
			emails_clicked: 0
		});
		expect(relay.messages).toHaveLength(1);
		expect(failures).toEqual([]);
	} finally {
		try {
			await cleanup(page.request, ids);
		} finally {
			await relay.close();
		}
	}
});
