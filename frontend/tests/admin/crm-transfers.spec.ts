import { expect, request as playwrightRequest, test, type Page } from '@playwright/test';
import { readFile } from 'node:fs/promises';
const password = 'OnlyForDisposableTest123!';
const blockedResources = new WeakMap<Page, Set<string>>();
let cookies: Awaited<
	ReturnType<Awaited<ReturnType<typeof playwrightRequest.newContext>>['storageState']>
>['cookies'];
test.describe.configure({ mode: 'serial' });
test.beforeAll(async ({ baseURL }) => {
	expect(baseURL, 'Only the disposable verification frontend is permitted').toBe(
		'http://127.0.0.1:5274'
	);
	const api = await playwrightRequest.newContext({ baseURL });
	try {
		const response = await api.post('/api/auth/login', {
			data: { email: 'admin@admin-completion.test', password }
		});
		expect(response.status(), await response.text()).toBe(200);
		cookies = (await api.storageState()).cookies;
	} finally {
		await api.dispose();
	}
});
test.beforeEach(async ({ context, page }) => {
	await context.addCookies(cookies);
	const blocked = new Set<string>();
	blockedResources.set(page, blocked);
	await page.route('**/*', (route) => {
		const url = new URL(route.request().url());
		if (
			['127.0.0.1', 'localhost'].includes(url.hostname) ||
			!['http:', 'https:'].includes(url.protocol)
		)
			return route.continue();
		blocked.add(route.request().url());
		return route.abort();
	});
});
function watch(page: Page) {
	const errors: string[] = [];
	page.on('pageerror', (error) => errors.push(error.message));
	page.on('console', (message) => {
		if (message.type() !== 'error') return;
		if (
			blockedResources.get(page)?.has(message.location().url) &&
			message.text() === 'Failed to load resource: net::ERR_FAILED'
		)
			return;
		errors.push(`${message.text()} ${message.location().url}`);
	});
	page.on('requestfailed', (request) => {
		if (
			!blockedResources.get(page)?.has(request.url()) &&
			!request.failure()?.errorText.includes('ERR_ABORTED')
		)
			errors.push(`${request.failure()?.errorText} ${request.url()}`);
	});
	return errors;
}

test('actual upload, mapping, saved reference selection, atomic apply and CSV/JSON snapshots survive reload', async ({
	page,
	baseURL
}) => {
	test.setTimeout(120000);
	const errors = watch(page),
		stamp = Date.now(),
		prefix = `browser-transfer-${stamp}`;
	let listId: string | undefined, tagId: string | undefined;
	const createdIds: string[] = [];
	try {
		const list = await page.request.post('/api/admin/crm/lists', {
			data: { title: `Browser transfer list ${stamp}` }
		});
		expect(list.status(), await list.text()).toBe(200);
		listId = String((await list.json()).id);
		const tag = await page.request.post('/api/admin/crm/contact-tags', {
			data: { title: `Browser transfer tag ${stamp}`, color: '#123456' }
		});
		expect(tag.status(), await tag.text()).toBe(200);
		tagId = String((await tag.json()).id);
		await page.goto('/admin/crm/import-export', { waitUntil: 'networkidle' });
		await page.getByRole('link', { name: /^Contacts Import contacts from CSV$/ }).click();
		await expect(page.getByRole('heading', { name: 'Import contacts', exact: true })).toBeVisible();
		const csv = `Mail,Given name\r\n${prefix}-a@example.test,"Line one\nLine two, quoted"\r\n${prefix.toUpperCase()}-A@example.test,Ignored duplicate\r\n${prefix}-b@example.test,=SUM(1+1)\r\n`;
		await page
			.getByLabel('CSV or JSON file')
			.setInputFiles({ name: `${prefix}.csv`, mimeType: 'text/csv', buffer: Buffer.from(csv) });
		const uploaded = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === '/api/admin/crm/imports' &&
				response.request().method() === 'POST'
		);
		await page.getByRole('button', { name: 'Upload and preview', exact: true }).click();
		const saved = await uploaded;
		expect(saved.status(), await saved.text()).toBe(200);
		const job = (await saved.json()).id;
		await expect(page).toHaveURL(new RegExp(`/admin/crm/import-export/new\\?job=${job}$`));
		await expect(page.getByRole('heading', { name: 'Field mapping', exact: true })).toBeVisible();
		await page.getByLabel('Map Mail', { exact: true }).selectOption('email');
		await page.getByLabel('Map Given name', { exact: true }).selectOption('first_name');
		await page
			.getByLabel('Search saved lists', { exact: true })
			.fill(`Browser transfer list ${stamp}`);
		await page.getByRole('button', { name: 'Search lists', exact: true }).click();
		await expect(
			page.getByRole('option', { name: `Browser transfer list ${stamp}`, exact: true })
		).toHaveCount(1);
		await page.getByRole('listbox', { name: 'Selected lists', exact: true }).selectOption(listId);
		await page
			.getByLabel('Search saved tags', { exact: true })
			.fill(`Browser transfer tag ${stamp}`);
		await page.getByRole('button', { name: 'Search tags', exact: true }).click();
		await expect(
			page.getByRole('option', { name: `Browser transfer tag ${stamp}`, exact: true })
		).toHaveCount(1);
		await page.getByRole('listbox', { name: 'Selected tags', exact: true }).selectOption(tagId);
		const applied = page.waitForResponse(
			(response) => new URL(response.url()).pathname === `/api/admin/crm/imports/${job}/start`
		);
		await page.getByRole('button', { name: 'Apply import', exact: true }).click();
		const receipt = await applied;
		expect(receipt.status(), await receipt.text()).toBe(200);
		expect(await receipt.json()).toMatchObject({
			id: job,
			status: 'completed',
			created_count: 2,
			updated_count: 0,
			skipped_count: 1,
			error_count: 0
		});
		await expect(page.getByRole('status').filter({ hasText: 'Import completed:' })).toContainText(
			'2 created, 0 updated, 1 skipped'
		);
		await page.reload({ waitUntil: 'networkidle' });
		await expect(page.getByLabel('Saved import summary')).toContainText('completed');
		await expect(page.getByLabel('Map Mail', { exact: true })).toHaveValue('email');
		await expect(page.getByRole('listbox', { name: 'Selected lists', exact: true })).toHaveValues([
			listId
		]);
		await expect(page.getByRole('listbox', { name: 'Selected tags', exact: true })).toHaveValues([
			tagId
		]);
		const contacts = await page.request.get(`/api/admin/crm/contacts?search=${prefix}`);
		expect(contacts.status()).toBe(200);
		const rows = (await contacts.json()).data;
		expect(rows).toHaveLength(2);
		createdIds.push(...rows.map((row: { id: string | number }) => String(row.id)));
		expect(
			rows.find((row: { email: string }) => row.email.endsWith('-a@example.test')).first_name
		).toBe('Line one\nLine two, quoted');
		const members = await page.request.get(`/api/admin/crm/lists/${listId}/contacts`);
		expect((await members.json()).meta.total).toBe(2);
		const tagDetail = await page.request.get(`/api/admin/crm/contact-tags/${tagId}`);
		expect((await tagDetail.json()).contacts_count).toBe(2);
		await page.goto('/admin/crm/import-export', { waitUntil: 'networkidle' });
		await page.getByRole('button', { name: 'Export', exact: true }).click();
		await page.getByLabel('Contact search', { exact: true }).fill(prefix);
		const exporting = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === '/api/admin/crm/exports' &&
				response.request().method() === 'POST'
		);
		await page.getByRole('button', { name: /^Contacts Export contacts$/ }).click();
		const output = await exporting;
		expect(output.status(), await output.text()).toBe(200);
		const exportJob = await output.json();
		expect(exportJob.total_records).toBe(2);
		const exportRow = page.locator(`[data-export-id="${exportJob.id}"]`);
		const downloading = page.waitForEvent('download');
		await exportRow.getByRole('button', { name: 'Download CSV', exact: true }).click();
		const download = await downloading;
		expect(download.suggestedFilename()).toBe(exportJob.file_name);
		const path = await download.path();
		if (!path) throw new Error('CSV download was not saved');
		const text = await readFile(path, 'utf8');
		expect(text).toContain("'=SUM(1+1)");
		expect(text).toContain('Line one\nLine two, quoted');
		expect(text).toContain(`${prefix}-a@example.test`);
		await page.reload({ waitUntil: 'networkidle' });
		await page.getByRole('button', { name: 'Export', exact: true }).click();
		await expect(page.locator(`[data-export-id="${exportJob.id}"]`)).toContainText('2');
		await page.getByLabel('Contact search', { exact: true }).fill(prefix);
		await page.getByRole('combobox', { name: 'File format', exact: true }).selectOption('json');
		const jsonReady = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === '/api/admin/crm/exports' &&
				response.request().method() === 'POST'
		);
		await page.getByRole('button', { name: /^Contacts Export contacts$/ }).click();
		const jsonResult = await jsonReady;
		expect(jsonResult.status()).toBe(200);
		const jsonJob = await jsonResult.json();
		const jsonDownloadReady = page.waitForEvent('download');
		await page
			.locator(`[data-export-id="${jsonJob.id}"]`)
			.getByRole('button', { name: 'Download JSON', exact: true })
			.click();
		const jsonDownload = await jsonDownloadReady;
		const jsonPath = await jsonDownload.path();
		if (!jsonPath) throw new Error('JSON download was not saved');
		const jsonRows = JSON.parse(await readFile(jsonPath, 'utf8')) as {
			email: string;
			first_name: string;
		}[];
		expect(jsonRows).toHaveLength(2);
		expect(jsonRows.find((row) => row.email.endsWith('-b@example.test'))?.first_name).toBe(
			'=SUM(1+1)'
		);
		expect(errors).toEqual([]);
	} finally {
		for (const id of createdIds)
			expect(
				(
					await page.request.delete(`/api/admin/crm/contacts/${id}`, {
						headers: { Origin: baseURL ?? '' }
					})
				).status()
			).toBe(200);
		if (tagId)
			expect(
				(
					await page.request.delete(`/api/admin/crm/contact-tags/${tagId}`, {
						headers: { Origin: baseURL ?? '' }
					})
				).status()
			).toBe(200);
		if (listId)
			expect(
				(
					await page.request.delete(`/api/admin/crm/lists/${listId}`, {
						headers: { Origin: baseURL ?? '' }
					})
				).status()
			).toBe(200);
	}
});

test('invalid rows produce a durable failed import with no partial contacts, and pending cancellation persists', async ({
	page
}) => {
	const prefix = `browser-transfer-invalid-${Date.now()}`;
	await page.goto('/admin/crm/import-export/new', { waitUntil: 'networkidle' });
	await page.getByLabel('CSV or JSON file').setInputFiles({
		name: 'invalid-rows.json',
		mimeType: 'application/json',
		buffer: Buffer.from(
			JSON.stringify([
				{ email: `${prefix}@example.test`, first_name: 'Valid row' },
				{ email: 'not-an-email', first_name: 'Invalid row' }
			])
		)
	});
	await page.getByRole('button', { name: 'Upload and preview', exact: true }).click();
	await expect(page.getByRole('button', { name: 'Apply import', exact: true })).toBeVisible();
	await page.getByRole('button', { name: 'Apply import', exact: true }).click();
	await expect(page.getByRole('heading', { name: 'Validation errors', exact: true })).toBeVisible();
	await expect(page.getByLabel('Saved import summary')).toContainText('failed');
	const contacts = await page.request.get(`/api/admin/crm/contacts?search=${prefix}`);
	expect((await contacts.json()).meta.total).toBe(0);
	await page.reload({ waitUntil: 'networkidle' });
	await expect(page.getByRole('heading', { name: 'Validation errors', exact: true })).toBeVisible();
	await expect(page.getByRole('button', { name: 'Apply import', exact: true })).toHaveCount(0);
	await page.getByRole('link', { name: 'Upload another file', exact: true }).click();
	await page.getByLabel('CSV or JSON file').setInputFiles({
		name: 'cancelled.csv',
		mimeType: 'text/csv',
		buffer: Buffer.from(`email\n${prefix}-cancel@example.test\n`)
	});
	await page.getByRole('button', { name: 'Upload and preview', exact: true }).click();
	await expect(
		page.getByRole('button', { name: 'Cancel pending import', exact: true })
	).toBeVisible();
	await page.getByRole('button', { name: 'Cancel pending import', exact: true }).click();
	await expect(page.getByLabel('Saved import summary')).toContainText('cancelled');
	await page.reload({ waitUntil: 'networkidle' });
	await expect(page.getByLabel('Saved import summary')).toContainText('cancelled');
});

test('malformed history and an unavailable apply request show errors and recover without retrying a write', async ({
	page
}) => {
	await page.route('**/api/admin/crm/imports?*', (route) =>
		route.fulfill({
			status: 200,
			contentType: 'application/json',
			body: JSON.stringify({ data: [] })
		})
	);
	await page.goto('/admin/crm/import-export', { waitUntil: 'networkidle' });
	await expect(page.getByRole('alert')).toContainText('invalid or unconfirmed');
	await expect(page.getByText('No saved imports.', { exact: true })).toHaveCount(0);
	await page.unroute('**/api/admin/crm/imports?*');
	await page.getByRole('button', { name: 'Retry history', exact: true }).click();
	await expect(page.getByRole('alert')).toHaveCount(0);
	await page.goto('/admin/crm/import-export/new', { waitUntil: 'networkidle' });
	await page.getByLabel('CSV or JSON file').setInputFiles({
		name: 'retry.csv',
		mimeType: 'text/csv',
		buffer: Buffer.from(`email\nbrowser-transfer-retry-${Date.now()}@example.test\n`)
	});
	await page.getByRole('button', { name: 'Upload and preview', exact: true }).click();
	await expect(page.getByRole('button', { name: 'Apply import', exact: true })).toBeVisible();
	let writes = 0;
	await page.route('**/api/admin/crm/imports/*/start', (route) => {
		writes += 1;
		return route.fulfill({
			status: 503,
			contentType: 'application/json',
			body: JSON.stringify({ error: 'Controlled transfer unavailable' })
		});
	});
	await page.getByRole('button', { name: 'Apply import', exact: true }).click();
	await expect(page.getByRole('alert')).toContainText('Controlled transfer unavailable');
	expect(writes).toBe(1);
	await page.unroute('**/api/admin/crm/imports/*/start');
	await page
		.getByRole('alert')
		.getByRole('button', { name: 'Refresh saved status', exact: true })
		.click();
	await expect(page.getByLabel('Saved import summary')).toContainText('pending');
	await page.getByRole('button', { name: 'Cancel pending import', exact: true }).click();
	await expect(page.getByLabel('Saved import summary')).toContainText('cancelled');
});

test('transfer reads and writes reject anonymous and non-admin callers', async ({ request }) => {
	for (const endpoint of [
		'/api/admin/crm/imports',
		'/api/admin/crm/exports',
		'/api/admin/crm/imports/options?kind=tags'
	])
		expect((await request.get(endpoint)).status()).toBe(401);
	expect(
		(await request.post('/api/admin/crm/exports', { data: { type: 'contacts' } })).status()
	).toBe(401);
	const login = await request.post('/api/auth/login', {
		data: { email: 'user@admin-completion.test', password }
	});
	expect(login.status(), await login.text()).toBe(200);
	for (const endpoint of [
		'/api/admin/crm/imports',
		'/api/admin/crm/exports',
		'/api/admin/crm/imports/options?kind=lists'
	])
		expect((await request.get(endpoint)).status()).toBe(403);
	expect(
		(await request.post('/api/admin/crm/exports', { data: { type: 'contacts' } })).status()
	).toBe(403);
});
