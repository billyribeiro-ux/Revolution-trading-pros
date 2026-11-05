import { expect, test, type Page, type APIRequestContext } from '@playwright/test';
const root = '/api/admin/crm/contacts',
	password = 'OnlyForDisposableTest123!';
function object(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Expected actual saved contact record');
	return value as Record<string, unknown>;
}
function id(value: unknown): string {
	const raw = object(value).id;
	const key =
		typeof raw === 'number' && Number.isSafeInteger(raw) && raw > 0
			? raw.toString()
			: typeof raw === 'string'
				? raw
				: '';
	if (!/^[1-9]\d*$/.test(key)) throw new Error('Saved contact identity is missing');
	return key;
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
async function create(
	client: APIRequestContext,
	email: string,
	extra: Record<string, unknown> = {}
) {
	const response = await client.post(root, { data: { email, ...extra } });
	expect(response.status()).toBe(200);
	return object(await response.json());
}
async function read(client: APIRequestContext, key: string) {
	const response = await client.get(`${root}/${key}`);
	expect(response.status()).toBe(200);
	const value = object(await response.json());
	expect(id(value)).toBe(key);
	return value;
}
async function cleanup(client: APIRequestContext, ids: Set<string>) {
	for (const key of ids) expect((await client.delete(`${root}/${key}`)).status()).toBe(200);
}
async function diagnostics(page: Page, expected = new Set<string>()) {
	const errors: string[] = [],
		blocked = new Set<string>();
	page.on('pageerror', (cause) => errors.push(cause.message));
	page.on('console', (message) => {
		const status = message
				.text()
				.match(/^Failed to load resource: the server responded with a status of (\d+)/)?.[1],
			source = message.location().url;
		if (status && source && expected.has(`${status} ${new URL(source).pathname}`)) return;
		if (
			message.type() === 'error' &&
			!(
				blocked.has(message.location().url) &&
				message.text() === 'Failed to load resource: net::ERR_FAILED'
			)
		)
			errors.push(message.text());
	});
	page.on('response', (response) => {
		if (
			new URL(response.url()).pathname.startsWith('/api/') &&
			response.status() >= 400 &&
			!expected.has(`${response.status()} ${new URL(response.url()).pathname}`)
		)
			errors.push(`${response.status()} ${response.url()}`);
	});
	await page.route('**/*', (route) => {
		if (['127.0.0.1', 'localhost'].includes(new URL(route.request().url()).hostname))
			return route.continue();
		blocked.add(route.request().url());
		return route.abort();
	});
	return errors;
}

test('contact create/edit persists actual lifecycle facts across all native directories and owner assignment remains distinct from member identity', async ({
	page,
	baseURL
}) => {
	await login(page, baseURL);
	const errors = await diagnostics(page),
		stamp = Date.now(),
		email = `contact-browser-${stamp}@example.test`,
		ids = new Set<string>();
	try {
		await page.goto('/admin/crm', { waitUntil: 'networkidle' });
		await page
			.locator('.directory')
			.getByRole('link', { name: 'Add Contact', exact: true })
			.click();
		for (const [label, value] of [
			['Email address', email],
			['First name', 'Contact'],
			['Last name', `Browser${stamp}`],
			['Phone', '+1 555 010 1111'],
			['Company', 'Owned browser fixture'],
			['Job title', 'Fixture analyst']
		])
			await page.getByLabel(label, { exact: true }).fill(value);
		await page.getByLabel('Status', { exact: true }).selectOption('prospect');
		await page.getByLabel('Lifecycle stage', { exact: true }).selectOption('mql');
		const saving = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === root && response.request().method() === 'POST'
		);
		await page.getByRole('button', { name: 'Save Contact', exact: true }).click();
		const response = await saving;
		expect(response.status()).toBe(200);
		const key = id(await response.json());
		ids.add(key);
		await expect(page).toHaveURL(new RegExp(`/admin/crm/contacts/${key}$`));
		const actual = await read(page.request, key);
		expect(actual.email).toBe(email);
		expect(actual.lifecycle_stage).toBe('mql');
		expect(actual.company).toBe('Owned browser fixture');
		expect(actual.lead_score).toBeNull();
		const measurements = page.locator('section[aria-label="Recorded contact measurements"]');
		for (const name of ['Health score', 'Engagement score', 'Lifetime value'])
			await expect(
				measurements
					.locator('.stats > div')
					.filter({ has: page.getByText(name, { exact: true }) })
					.locator('strong')
			).toHaveText('Not measured');
		expect(object(object(actual.facts).unmeasured)).toMatchObject({
			health_score: null,
			engagement_score: null,
			lifetime_value: null
		});
		await page.getByRole('link', { name: 'Edit Contact', exact: true }).click();
		await expect(page.getByLabel('Company', { exact: true })).toHaveValue('Owned browser fixture');
		await page.getByLabel('Company', { exact: true }).fill('Edited actual browser fixture');
		await page.getByLabel('Lifecycle stage', { exact: true }).selectOption('sql');
		await page.getByRole('button', { name: 'Save Contact', exact: true }).click();
		await expect(page).toHaveURL(new RegExp(`/admin/crm/contacts/${key}$`));
		await page.reload({ waitUntil: 'networkidle' });
		await expect(page.getByText('Edited actual browser fixture', { exact: true })).toBeVisible();
		expect((await read(page.request, key)).lifecycle_stage).toBe('sql');
		for (const path of ['/admin/crm', '/admin/contacts', '/crm/contacts']) {
			await page.goto(path, { waitUntil: 'networkidle' });
			const directory = page.locator('.directory');
			await directory.getByLabel('Search contacts', { exact: true }).fill(email);
			await directory
				.getByRole('combobox', { name: 'Contact status', exact: true })
				.selectOption('prospect');
			await directory
				.getByRole('combobox', { name: 'Lifecycle stage', exact: true })
				.selectOption('sql');
			await directory.getByRole('button', { name: 'Apply filters', exact: true }).click();
			await expect(directory.locator('tbody tr')).toHaveCount(1);
			await expect(directory.locator('tbody tr')).toContainText(email);
			await page.reload({ waitUntil: 'networkidle' });
			await expect(directory.getByLabel('Search contacts', { exact: true })).toHaveValue(email);
			await expect(directory.locator('tbody tr')).toHaveCount(1);
		}
		const me = object(await (await page.request.get('/api/auth/me')).json());
		if (typeof me.id !== 'number') throw new Error('Actual active owner identity is missing');
		const linked = (await read(page.request, key)).user_id;
		expect((await page.request.put(`${root}/${key}`, { data: { owner_id: me.id } })).status()).toBe(
			200
		);
		const owned = await read(page.request, key);
		expect(owned.owner_id).toBe(me.id);
		expect(owned.user_id).toBe(linked);
		const ownedPage = object(
			await (
				await page.request.get(`${root}?search=${encodeURIComponent(email)}&owner_id=${me.id}`)
			).json()
		);
		expect(object(ownedPage.meta).total).toBe(1);
		expect(id((ownedPage.data as unknown[])[0])).toBe(key);
		const unassigned = object(
			await (
				await page.request.get(`${root}?search=${encodeURIComponent(email)}&owner_id=unassigned`)
			).json()
		);
		expect(object(unassigned.meta).total).toBe(0);
		expect((await page.request.put(`${root}/${key}`, { data: { owner_id: null } })).status()).toBe(
			200
		);
		expect((await read(page.request, key)).owner_id).toBeNull();
		expect((await read(page.request, key)).user_id).toBe(linked);
		expect(errors).toEqual([]);
	} finally {
		await cleanup(page.request, ids);
	}
});

test('contact pagination preserves applied filters across reload and untouched nullable facts survive a small edit', async ({
	page,
	baseURL
}) => {
	await login(page, baseURL);
	const errors = await diagnostics(page),
		stamp = Date.now(),
		prefix = `contact-page-${stamp}`,
		ids = new Set<string>();
	try {
		for (let index = 0; index < 26; index++) {
			const saved = await create(
				page.request,
				`${prefix}-${String(index).padStart(2, '0')}@example.test`,
				{ first_name: 'Paged', last_name: `Fixture${index}`, lifecycle_stage: 'subscriber' }
			);
			ids.add(id(saved));
		}
		await page.goto(`/admin/contacts?search=${prefix}&lifecycle_stage=subscriber`, {
			waitUntil: 'networkidle'
		});
		const directory = page.locator('.directory'),
			pages = directory.getByRole('navigation', { name: 'Contact pages', exact: true });
		await expect(directory.locator('tbody tr')).toHaveCount(25);
		await expect(pages).toContainText('Page 1 of 2; 26 contacts');
		await pages.getByRole('button', { name: 'Next', exact: true }).click();
		await expect(directory.locator('tbody tr')).toHaveCount(1);
		await expect(page).toHaveURL(/page=2/);
		await page.reload({ waitUntil: 'networkidle' });
		await expect(pages).toContainText('Page 2 of 2; 26 contacts');
		await expect(directory.locator('tbody tr')).toHaveCount(1);
		const unknown = await create(page.request, `nullable-contact-${stamp}@example.test`),
			key = id(unknown);
		ids.add(key);
		expect(unknown.phone).toBeNull();
		expect(unknown.company).toBeNull();
		expect(unknown.lifecycle_stage).toBeNull();
		await page.goto(`/admin/crm/contacts/${key}/edit`, { waitUntil: 'networkidle' });
		await page.getByLabel('First name', { exact: true }).fill('Explicitly edited');
		const writing = page.waitForRequest(
			(request) =>
				new URL(request.url()).pathname === `${root}/${key}` && request.method() === 'PUT'
		);
		await page.getByRole('button', { name: 'Save Contact', exact: true }).click();
		expect((await writing).postDataJSON()).toEqual({ first_name: 'Explicitly edited' });
		await expect(page).toHaveURL(new RegExp(`/admin/crm/contacts/${key}$`));
		const actual = await read(page.request, key);
		for (const field of ['phone', 'company', 'job_title', 'lifecycle_stage', 'custom_fields'])
			expect(actual[field]).toBe(unknown[field]);
		expect(actual.first_name).toBe('Explicitly edited');
		await page.locator('.contact-header').getByTitle('Delete', { exact: true }).click();
		await page.getByRole('button', { name: 'Confirm delete contact', exact: true }).click();
		await expect(page).toHaveURL('/admin/crm');
		ids.delete(key);
		expect((await page.request.get(`${root}/${key}`)).status()).toBe(404);
		expect(errors).toEqual([]);
	} finally {
		await cleanup(page.request, ids);
	}
});

test('contact notes and actual tag/list receipts persist, repeated assignment is a measured no-op, and a failed note write preserves the draft without a retry', async ({
	page,
	baseURL
}) => {
	await login(page, baseURL);
	const stamp = Date.now(),
		contact = await create(page.request, `contact-links-${stamp}@example.test`, {
			first_name: 'Actual',
			last_name: 'Links'
		}),
		key = id(contact),
		ids = new Set([key]);
	const expected = new Set([`503 ${root}/${key}/notes`]),
		errors = await diagnostics(page, expected);
	let tag: string | undefined, list: string | undefined;
	try {
		for (const kind of ['tag', 'list'] as const) {
			const path = kind === 'tag' ? '/api/admin/crm/contact-tags' : '/api/admin/crm/lists',
				response = await page.request.post(path, {
					data: {
						title: `Actual browser ${kind} ${stamp}`,
						...(kind === 'tag' ? { color: '#1673b1' } : { is_public: false })
					}
				});
			expect(response.status()).toBe(200);
			const actual = object(await response.json());
			const reference = String(actual.id);
			expect(reference).toMatch(/^[1-9]\d*$/);
			if (kind === 'tag') tag = reference;
			else list = reference;
		}
		await page.goto(`/admin/crm/contacts/${key}`, { waitUntil: 'networkidle' });
		let noteWrites = 0,
			failNote = true;
		await page.route(`**${root}/${key}/notes`, (route) => {
			if (route.request().method() !== 'POST') return route.continue();
			noteWrites += 1;
			if (failNote)
				return route.fulfill({
					status: 503,
					contentType: 'application/json',
					body: JSON.stringify({ error: 'Fixture note save unavailable' })
				});
			return route.continue();
		});
		await page
			.getByRole('navigation', { name: 'Contact sections', exact: true })
			.getByRole('button', { name: 'Notes', exact: true })
			.click();
		await page.getByRole('button', { name: 'Add Note', exact: true }).click();
		const dialog = page.getByRole('dialog', { name: 'Add Note', exact: true }),
			content = `Actual saved contact note ${stamp}`;
		await dialog.getByLabel('Note content', { exact: true }).fill(content);
		await dialog.getByRole('button', { name: 'Save Note', exact: true }).click();
		await expect(dialog.getByRole('alert')).toContainText('Fixture note save unavailable');
		await expect(dialog.getByLabel('Note content', { exact: true })).toHaveValue(content);
		expect(noteWrites).toBe(1);
		expect(
			object(object(await (await page.request.get(`${root}/${key}/notes`)).json()).meta).total
		).toBe(0);
		failNote = false;
		expected.clear();
		await dialog.getByRole('button', { name: 'Save Note', exact: true }).click();
		await expect(dialog).toHaveCount(0);
		await expect(page.locator('.note-content')).toHaveText(content);
		expect(noteWrites).toBe(2);
		await page.reload({ waitUntil: 'networkidle' });
		await page
			.getByRole('navigation', { name: 'Contact sections', exact: true })
			.getByRole('button', { name: 'Notes', exact: true })
			.click();
		await expect(page.locator('.note-content')).toHaveText(content);
		expect(
			object(object(await (await page.request.get(`${root}/${key}/notes`)).json()).meta).total
		).toBe(1);
		await page
			.getByRole('navigation', { name: 'Contact sections', exact: true })
			.getByRole('button', { name: 'Overview', exact: true })
			.click();
		for (const kind of ['tag', 'list'] as const) {
			for (let attempt = 0; attempt < 2; attempt++) {
				await page
					.getByRole('button', { name: kind === 'tag' ? 'Add tag' : 'Add to list', exact: true })
					.click();
				const picker = page.getByRole('dialog', {
					name: kind === 'tag' ? 'Add Tag' : 'Add to List',
					exact: true
				});
				await picker
					.getByLabel(`Search ${kind}s`, { exact: true })
					.fill(`Actual browser ${kind} ${stamp}`);
				await picker.getByRole('button', { name: 'Search', exact: true }).click();
				await picker
					.getByRole('button', { name: `Actual browser ${kind} ${stamp}`, exact: true })
					.click();
				await expect(picker).toHaveCount(0);
				await expect(
					page.getByRole('status').filter({
						hasText: `${kind === 'tag' ? 'Tag' : 'List'} assignment confirmed: ${attempt === 0 ? 1 : 0} changed.`
					})
				).toBeVisible();
			}
		}
		await page.reload({ waitUntil: 'networkidle' });
		await expect(
			page.getByRole('button', { name: `Remove tag Actual browser tag ${stamp}`, exact: true })
		).toBeVisible();
		await expect(
			page.getByRole('button', {
				name: `Remove from list Actual browser list ${stamp}`,
				exact: true
			})
		).toBeVisible();
		await page
			.getByRole('button', { name: `Remove tag Actual browser tag ${stamp}`, exact: true })
			.click();
		await expect(
			page.getByRole('button', { name: `Remove tag Actual browser tag ${stamp}`, exact: true })
		).toHaveCount(0);
		await page
			.getByRole('button', { name: `Remove from list Actual browser list ${stamp}`, exact: true })
			.click();
		await expect(
			page.getByRole('button', {
				name: `Remove from list Actual browser list ${stamp}`,
				exact: true
			})
		).toHaveCount(0);
		const notes = object(await (await page.request.get(`${root}/${key}/notes`)).json());
		expect(object(notes.meta).total).toBe(1);
		const timeline = object(await (await page.request.get(`${root}/${key}/timeline`)).json());
		expect(timeline.scope).toBe('recorded_contact_links');
		expect(object(timeline.coverage).human_email_engagement).toBeNull();
		expect(errors).toEqual([]);
	} finally {
		await cleanup(page.request, ids);
		if (tag)
			expect((await page.request.delete(`/api/admin/crm/contact-tags/${tag}`)).status()).toBe(200);
		if (list)
			expect((await page.request.delete(`/api/admin/crm/lists/${list}`)).status()).toBe(200);
	}
});

test('contact views reject malformed successful reads before writes and recover from service failures while admin-only mutations reject anonymous and member callers', async ({
	page,
	baseURL,
	browser
}) => {
	await login(page, baseURL);
	const created = await create(page.request, `contact-recovery-${Date.now()}@example.test`),
		key = id(created),
		ids = new Set([key]);
	const expected = new Set([`503 ${root}/${key}`]),
		errors = await diagnostics(page, expected);
	let mode: 'unavailable' | 'malformed' | 'live' = 'unavailable',
		writes = 0;
	try {
		await page.route(`**${root}/${key}`, (route) => {
			if (route.request().method() !== 'GET') {
				writes += 1;
				return route.continue();
			}
			return mode === 'live'
				? route.continue()
				: route.fulfill({
						status: mode === 'unavailable' ? 503 : 200,
						contentType: 'application/json',
						body: JSON.stringify(
							mode === 'unavailable'
								? { error: 'Fixture contact read unavailable' }
								: { id: key, email: created.email }
						)
					});
		});
		await page.goto(`/admin/crm/contacts/${key}/edit`, { waitUntil: 'networkidle' });
		await expect(page.getByRole('alert')).toContainText('Fixture contact read unavailable');
		await expect(page.getByRole('button', { name: 'Save Contact', exact: true })).toHaveCount(0);
		expect(writes).toBe(0);
		mode = 'malformed';
		expected.clear();
		await page.getByRole('button', { name: 'Retry loading', exact: true }).click();
		await expect(page.getByRole('alert')).toBeVisible();
		expect(writes).toBe(0);
		mode = 'live';
		await page.getByRole('button', { name: 'Retry loading', exact: true }).click();
		await expect(page.getByLabel('Email address', { exact: true })).toHaveValue(
			String(created.email)
		);
		expect(writes).toBe(0);
		let malformedList = true;
		await page.route(`**${root}?*`, (route) =>
			malformedList
				? route.fulfill({
						status: 200,
						contentType: 'application/json',
						body: JSON.stringify({
							data: [created],
							meta: { total: 999, current_page: 1, per_page: 25, total_pages: 1 }
						})
					})
				: route.continue()
		);
		await page.goto('/admin/contacts', { waitUntil: 'networkidle' });
		await expect(page.locator('.directory').getByRole('alert')).toBeVisible();
		await expect(page.locator('tbody tr')).toHaveCount(0);
		malformedList = false;
		await page.getByRole('button', { name: 'Retry contacts', exact: true }).click();
		await expect(page.locator('.directory').getByRole('table')).toBeVisible();
		for (const member of [false, true]) {
			const context = await browser.newContext({ baseURL });
			try {
				if (member)
					expect(
						(
							await context.request.post('/api/auth/login', {
								data: { email: 'user@admin-completion.test', password }
							})
						).status()
					).toBe(200);
				for (const [method, path, data] of [
					['GET', `${root}/${key}`, undefined],
					['PUT', `${root}/${key}`, { first_name: 'Forbidden' }],
					['POST', `${root}/${key}/notes`, { content: 'Forbidden' }],
					['POST', `${root}/${key}/tags`, { tag_id: 1 }],
					[
						'POST',
						`${root}/${key}/send-email`,
						{
							request_id: crypto.randomUUID(),
							expected_email: created.email,
							subject: 'Forbidden',
							body_html: '<p>Forbidden</p>'
						}
					]
				] as const) {
					const response = await context.request.fetch(path, { method, ...(data ? { data } : {}) });
					expect(response.status()).toBe(member ? 403 : 401);
				}
			} finally {
				await context.close();
			}
		}
		expect((await read(page.request, key)).first_name).toBe(created.first_name);
		expect(
			object(object(await (await page.request.get(`${root}/${key}/notes`)).json()).meta).total
		).toBe(0);
		expect(errors).toEqual([]);
	} finally {
		await page.unroute(`**${root}/${key}`);
		await cleanup(page.request, ids);
	}
});
