import { expect, test, type Page } from '@playwright/test';

// Run only against the isolated verification stack described in docs/admin.
// These are disposable fixture accounts, never production credentials.
const fixturePassword = 'OnlyForDisposableTest123!';
const browserFailures = new WeakMap<Page, string[]>();

test.beforeEach(async ({ page, baseURL }) => {
	expect(baseURL, 'Use the isolated verification frontend on port 5274').toBe(
		'http://127.0.0.1:5274'
	);
	const failures: string[] = [];
	browserFailures.set(page, failures);
	page.on('pageerror', (error) => failures.push(error.message));
	page.on('response', (response) => {
		if (new URL(response.url()).pathname.startsWith('/api/') && response.status() >= 400)
			failures.push(`${response.status()} ${new URL(response.url()).pathname}`);
	});
	page.on('requestfailed', (request) => {
		if (
			new URL(request.url()).pathname.startsWith('/api/') &&
			!request.failure()?.errorText.includes('ERR_ABORTED')
		)
			failures.push(`${request.failure()?.errorText} ${new URL(request.url()).pathname}`);
	});
	page.on('console', (message) => {
		if (message.type() !== 'error') return;
		const origin = message.location().url;
		// Third-party requests are intentionally blocked by this local-only harness.
		if (origin && !['127.0.0.1', 'localhost'].includes(new URL(origin).hostname)) return;
		if (!origin && message.text().startsWith('Failed to load resource')) return;
		failures.push(message.text());
	});
	// Keep this verification local, including third-party scripts/assets.
	await page.route('**/*', (route) => {
		const url = new URL(route.request().url());
		return ['127.0.0.1', 'localhost'].includes(url.hostname) ||
			!['http:', 'https:'].includes(url.protocol)
			? route.continue()
			: route.abort();
	});
});

test.afterEach(async ({ page }) => {
	expect(
		browserFailures.get(page) ?? [],
		'No unexpected application console or API failures'
	).toEqual([]);
});

test('login, CRM save/reload, durable behavior dashboard, and logout', async ({ page }) => {
	const errors: string[] = [];
	const failedApi: string[] = [];
	page.on('pageerror', (error) => errors.push(error.message));
	page.on('response', (response) => {
		if (response.url().includes('/api/') && response.status() >= 400) {
			failedApi.push(`${response.status()} ${new URL(response.url()).pathname}`);
		}
	});
	await page.goto('/login?redirect=/admin/crm/settings', { waitUntil: 'networkidle' });
	await page.locator('input[name="email"]').fill('admin@admin-completion.test');
	await page.locator('input[name="password"]').fill(fixturePassword);
	await page.getByRole('button', { name: 'Sign In to Trade' }).click();
	await expect(page).not.toHaveURL(/\/login/);
	await page.goto('/admin/crm/settings');
	const subject = page.getByLabel('Confirmation Email Subject');
	await expect(subject).toBeVisible();
	await subject.fill('Browser persistence fixture');
	await page.getByRole('button', { name: 'Save Settings' }).click();
	await expect(page.getByText('Double opt-in settings saved successfully')).toBeVisible();
	await page.reload();
	await expect(subject).toHaveValue('Browser persistence fixture');
	await page.goto('/admin/behavior');
	await expect(
		page.locator('.admin-behavior').getByRole('heading', { name: 'Behavior Tracking', exact: true })
	).toBeVisible();
	await expect(page.getByText('#subscribe', { exact: true })).toBeVisible();
	await expect(page.getByText('75%', { exact: true }).first()).toBeVisible();
	await page.screenshot({ path: 'test-results/admin-behavior.png', fullPage: true });
	const cookies = await page.context().cookies();
	const accessToken = cookies.find((cookie) => cookie.name === 'rtp_access_token')?.value;
	const refreshToken = cookies.find((cookie) => cookie.name === 'rtp_refresh_token')?.value;
	expect(accessToken).toBeTruthy();
	expect(refreshToken).toBeTruthy();
	const logout = await page.request.post('/api/logout');
	expect(logout.ok()).toBe(true);
	expect((await logout.json()).server_revoked).toBe(true);
	const revoked = await page.request.get('http://127.0.0.1:8281/api/auth/me', {
		headers: { Authorization: `Bearer ${accessToken}` }
	});
	expect(revoked.status()).toBe(401);
	const replay = await page.request.post('http://127.0.0.1:8281/api/auth/refresh', {
		data: { refresh_token: refreshToken }
	});
	expect(replay.status()).toBe(401);
	await page.goto('/admin/crm/settings');
	await expect(page).toHaveURL(/\/login/);
	expect(errors).toEqual([]);
	expect(failedApi).toEqual([]);
});

test('anonymous and authenticated non-admin users cannot open admin pages', async ({ page }) => {
	await page.goto('/admin/crm/settings');
	await expect(page).toHaveURL(/\/login/);
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'user@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	await page.goto('/admin/crm/settings');
	await expect(page).toHaveURL(/admin_required/);
});

test('company create, edit, reload, view, duplicate and delete persist through the CRM UI', async ({
	page
}) => {
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	await page.goto('/admin/crm/companies/new', { waitUntil: 'networkidle' });
	const companyName = `Browser Company ${Date.now()}`;
	await page.getByLabel('Company name', { exact: true }).fill(companyName);
	await page.getByLabel('Website', { exact: true }).fill('https://example.test');
	await page.getByLabel('Industry', { exact: true }).selectOption('finance');
	await page.getByLabel('Company size', { exact: true }).selectOption('11-50');
	await page
		.getByLabel('Description', { exact: true })
		.fill('Created by the isolated browser fixture.');
	await page.getByRole('button', { name: 'Save Company', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/companies$/);
	let row = page.getByRole('row').filter({ hasText: companyName });
	await expect(row).toBeVisible();
	await row.getByRole('link', { name: 'Edit', exact: true }).click();
	await expect(page.getByLabel('Company name', { exact: true })).toHaveValue(companyName);
	await page
		.getByLabel('Description', { exact: true })
		.fill('Edited and persisted browser fixture.');
	await page.getByRole('button', { name: 'Save Company', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/companies$/);
	row = page.getByRole('row').filter({ hasText: companyName });
	await row.getByRole('link', { name: 'View', exact: true }).click();
	await expect(
		page.getByText('Edited and persisted browser fixture.', { exact: true })
	).toBeVisible();
	await page.reload();
	await expect(
		page.getByText('Edited and persisted browser fixture.', { exact: true })
	).toBeVisible();
	await page.getByRole('link', { name: '← Companies', exact: true }).click();
	row = page.getByRole('row').filter({ hasText: companyName });
	await row.getByRole('button', { name: 'Duplicate', exact: true }).click();
	const copy = page.getByRole('row').filter({ hasText: `${companyName} (copy)` });
	await expect(copy).toBeVisible();
	await copy.getByRole('button', { name: 'Delete', exact: true }).click();
	await page.getByRole('button', { name: 'Delete', exact: true }).last().click();
	await expect(copy).toHaveCount(0);
});

test('contact lists save edits and persist membership without deleting contacts', async ({
	page
}) => {
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	const email = `list-browser-${Date.now()}@completion.test`;
	const created = await page.request.post('/api/admin/crm/contacts', {
		data: { email, first_name: 'List browser fixture' }
	});
	expect(created.ok()).toBe(true);
	const contactResponse = (await created.json()) as {
		id?: string | number;
		data?: { id: string | number };
	};
	const contactId = contactResponse.data?.id ?? contactResponse.id;
	expect(contactId).toBeDefined();
	const title = `Browser List ${Date.now()}`;
	await page.goto('/admin/crm/lists/new', { waitUntil: 'networkidle' });
	await page.getByLabel('List title', { exact: true }).fill(title);
	await page.getByLabel('Description', { exact: true }).fill('List description fixture');
	await page.getByLabel('Public list', { exact: true }).check();
	await page.getByRole('button', { name: 'Save List', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/lists$/);
	let row = page.getByRole('row').filter({ hasText: title });
	await expect(row).toBeVisible();
	await row.getByRole('link', { name: 'Edit', exact: true }).click();
	await expect(page.getByLabel('Public list', { exact: true })).toBeChecked();
	await page.getByLabel('List title', { exact: true }).fill(`${title} Renamed`);
	await page.getByRole('button', { name: 'Save List', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/lists$/);
	row = page.getByRole('row').filter({ hasText: `${title} Renamed` });
	await row.getByRole('link', { name: 'View Contacts', exact: true }).click();
	await page.getByLabel('Contact IDs', { exact: true }).fill(String(contactId));
	await page.getByRole('button', { name: 'Add Contacts', exact: true }).click();
	await expect(page.getByRole('link', { name: email, exact: true })).toBeVisible();
	await page.reload();
	await expect(page.getByRole('link', { name: email, exact: true })).toBeVisible();
	await page.getByRole('checkbox', { name: `Select ${email}`, exact: true }).check();
	await page.getByRole('button', { name: 'Remove Selected From List', exact: true }).click();
	await expect(page.getByRole('link', { name: email, exact: true })).toHaveCount(0);
	const preserved = await page.request.get(`/api/admin/crm/contacts/${contactId}`);
	expect(preserved.ok()).toBe(true);
});

test('email templates save HTML and configuration, reload, preview, duplicate and delete', async ({
	page
}) => {
	const errors: string[] = [];
	page.on('pageerror', (error) => errors.push(error.message));
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	const title = `Browser Template ${Date.now()}`;
	await page.goto('/admin/crm/templates/new', { waitUntil: 'networkidle' });
	await page.getByLabel('Template title', { exact: true }).fill(title);
	await page.getByLabel('Email subject', { exact: true }).fill('Browser subject');
	await page.getByLabel('Category', { exact: true }).fill('Browser category');
	await page.getByLabel('Email HTML', { exact: true }).fill('<h2>Original fixture HTML</h2>');
	await page
		.getByLabel('Template configuration (JSON)', { exact: true })
		.fill('{"theme":"fixture"}');
	await page.getByRole('checkbox', { name: 'Default template', exact: true }).check();
	await page.getByRole('button', { name: 'Save Template', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/templates$/);
	let card = page.getByRole('listitem', { name: title, exact: true });
	await expect(card).toBeVisible();
	await card.getByRole('link', { name: `Edit ${title}`, exact: true }).click();
	await expect(page.getByLabel('Email HTML', { exact: true })).toHaveValue(
		'<h2>Original fixture HTML</h2>'
	);
	await expect(page.getByRole('checkbox', { name: 'Default template', exact: true })).toBeChecked();
	await page.getByLabel('Email HTML', { exact: true }).fill('<h2>Persisted browser HTML</h2>');
	await page.getByRole('button', { name: 'Save Template', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/templates$/);
	await page.reload();
	card = page.getByRole('listitem', { name: title, exact: true });
	await card.getByRole('button', { name: `Preview ${title}`, exact: true }).click();
	await expect(
		page.getByRole('heading', { name: 'Persisted browser HTML', exact: true })
	).toBeVisible();
	await page.keyboard.press('Escape');
	await card.getByRole('button', { name: `Duplicate ${title}`, exact: true }).click();
	const copy = page.getByRole('listitem', { name: `${title} (copy)`, exact: true });
	await expect(copy).toBeVisible();
	await copy.getByRole('button', { name: `Delete ${title} (copy)`, exact: true }).click();
	await page.getByRole('button', { name: 'Delete', exact: true }).last().click();
	await expect(copy).toHaveCount(0);
	expect(errors).toEqual([]);
});

test('contact create/edit and lead edit persist without an external CRM connection', async ({
	page
}) => {
	const errors: string[] = [];
	page.on('pageerror', (error) => errors.push(error.message));
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	const email = `contact-browser-${Date.now()}@completion.test`;
	await page.goto('/admin/crm/contacts/new', { waitUntil: 'networkidle' });
	await page.getByLabel('Email address', { exact: true }).fill(email);
	await page.getByLabel('First name', { exact: true }).fill('Browser contact');
	await page.getByLabel('Company', { exact: true }).fill('Original company');
	const saved = page.waitForResponse(
		(response) =>
			response.request().method() === 'POST' &&
			new URL(response.url()).pathname === '/api/admin/crm/contacts'
	);
	await page.getByRole('button', { name: 'Save Contact', exact: true }).click();
	const response = await saved;
	expect(response.ok()).toBe(true);
	const record = (await response.json()) as {
		id?: string | number;
		data?: { id: string | number };
	};
	const contactId = record.data?.id ?? record.id;
	expect(contactId).toBeDefined();
	await expect(page).toHaveURL(/\/admin\/crm$/);
	await expect(page.getByText(email, { exact: true })).toBeVisible();
	await page.goto(`/admin/crm/contacts/${contactId}/edit`, { waitUntil: 'networkidle' });
	await expect(page.getByLabel('Company', { exact: true })).toHaveValue('Original company');
	await page.getByLabel('Company', { exact: true }).fill('Persisted company');
	await page.getByLabel('Status', { exact: true }).selectOption('prospect');
	await page.getByRole('button', { name: 'Save Contact', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm$/);
	await page.goto(`/admin/crm/contacts/${contactId}/edit`);
	await page.reload();
	await expect(page.getByLabel('Company', { exact: true })).toHaveValue('Persisted company');
	await expect(page.getByLabel('Status', { exact: true })).toHaveValue('prospect');
	const lead = await page.request.post('/api/admin/crm/leads', {
		data: {
			email: `lead-browser-${Date.now()}@completion.test`,
			first_name: 'Browser lead',
			company_name: 'Lead company',
			status: 'new'
		}
	});
	expect(lead.ok()).toBe(true);
	const leadRecord = (await lead.json()) as { id: string };
	await page.goto(`/admin/crm/leads/${leadRecord.id}/edit`, { waitUntil: 'networkidle' });
	await expect(page.getByLabel('Company', { exact: true })).toHaveValue('Lead company');
	await page.getByLabel('Status', { exact: true }).selectOption('qualified');
	await page.getByRole('button', { name: 'Save Contact', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/leads$/);
	await page.goto(`/admin/crm/leads/${leadRecord.id}/edit`);
	await page.reload();
	await expect(page.getByLabel('Status', { exact: true })).toHaveValue('qualified');
	expect(errors).toEqual([]);
});

test('dynamic segments save rules, reload, and retrieve live matching contacts', async ({
	page
}) => {
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	const fixture = async (suffix: string) => {
		const email = `segment-browser-${suffix}-${Date.now()}@completion.test`;
		const response = await page.request.post('/api/admin/crm/contacts', {
			data: { email, first_name: 'Segment fixture' }
		});
		expect(response.ok()).toBe(true);
		const record = (await response.json()) as {
			id?: string | number;
			data?: { id: string | number };
		};
		return { email, id: record.data?.id ?? record.id };
	};
	const first = await fixture('a');
	const second = await fixture('b');
	expect(second.id).toBeDefined();
	const title = `Browser Segment ${Date.now()}`;
	await page.goto('/admin/crm/segments/new', { waitUntil: 'networkidle' });
	await page.getByLabel('Segment title', { exact: true }).fill(title);
	const rule = page.getByRole('group', { name: 'Rule 1', exact: true });
	await rule.getByLabel('Contact field', { exact: true }).selectOption('email');
	await rule.getByLabel('Value', { exact: true }).fill(first.email);
	await page.getByRole('button', { name: 'Save Segment', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/segments$/);
	let card = page
		.locator('.segment-card')
		.filter({ has: page.getByRole('heading', { name: title, exact: true }) });
	await expect(card).toBeVisible();
	await card.getByTitle('View Contacts', { exact: true }).click();
	await expect(page.getByRole('link', { name: first.email, exact: true })).toBeVisible();
	await expect(page.getByText('1 matching contacts', { exact: true })).toBeVisible();
	await page.getByRole('link', { name: 'Edit Segment', exact: true }).click();
	await expect(page.getByLabel('Value', { exact: true })).toHaveValue(first.email);
	await page.getByLabel('Value', { exact: true }).fill(second.email);
	await page.getByRole('button', { name: 'Save Segment', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/segments$/);
	card = page
		.locator('.segment-card')
		.filter({ has: page.getByRole('heading', { name: title, exact: true }) });
	await card.getByTitle('View Contacts', { exact: true }).click();
	await expect(page.getByRole('link', { name: second.email, exact: true })).toBeVisible();
	await page.reload();
	await expect(page.getByRole('link', { name: second.email, exact: true })).toBeVisible();
	const changed = await page.request.put(`/api/admin/crm/contacts/${second.id}`, {
		data: { email: `changed-${second.email}` }
	});
	expect(changed.ok()).toBe(true);
	await page.getByRole('button', { name: 'Refresh Matches', exact: true }).click();
	await expect(page.getByText('0 matching contacts', { exact: true })).toBeVisible();
	const restored = await page.request.put(`/api/admin/crm/contacts/${second.id}`, {
		data: { email: second.email }
	});
	expect(restored.ok()).toBe(true);
	await page.getByRole('button', { name: 'Refresh Matches', exact: true }).click();
	await expect(page.getByRole('link', { name: second.email, exact: true })).toBeVisible();
});
