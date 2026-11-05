import { expect, request as playwrightRequest, test, type Page } from '@playwright/test';
import { readFile, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
const password = 'OnlyForDisposableTest123!';
let cookies: Awaited<
	ReturnType<Awaited<ReturnType<typeof playwrightRequest.newContext>>['storageState']>
>['cookies'];
const stamp = Date.now(),
	prefix = `member-export-browser-${stamp}`,
	fixtureIds: string[] = [];
function object(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Invalid member fixture response');
	return value as Record<string, unknown>;
}
test.describe.configure({ mode: 'serial' });
test.beforeAll(async ({ baseURL }) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	const api = await playwrightRequest.newContext({ baseURL });
	try {
		expect(
			(
				await api.post('/api/auth/login', {
					data: { email: 'admin@admin-completion.test', password }
				})
			).status()
		).toBe(200);
		cookies = (await api.storageState()).cookies;
		for (const [name, email] of [
			['=SUM(1,2)', `${prefix}-a@example.test`],
			['D’Ávila, 李明\nquoted "member"', `${prefix}-b@example.test`],
			['Excluded export fixture', `excluded-export-${stamp}@example.test`]
		]) {
			const response = await api.post('/api/admin/member-management', {
				data: { name, email, password, role: 'user', send_welcome_email: false }
			});
			expect(response.status(), await response.text()).toBe(200);
			const value = object(await response.json()),
				row = value.member ? object(value.member) : value.data ? object(value.data) : value,
				id = String(row.id);
			expect(id).toMatch(/^[1-9]\d*$/);
			fixtureIds.push(id);
		}
	} finally {
		await api.dispose();
	}
});
test.afterAll(async ({ baseURL }) => {
	if (!cookies) return;
	const api = await playwrightRequest.newContext({
		baseURL,
		storageState: { cookies, origins: [] }
	});
	try {
		for (const id of fixtureIds)
			expect((await api.delete(`/api/admin/member-management/${id}`)).status()).toBe(200);
	} finally {
		await api.dispose();
	}
});
test.beforeEach(async ({ context }) => {
	await context.addCookies(cookies);
});
async function diagnostics(page: Page) {
	const errors: string[] = [],
		blocked = new Set<string>();
	page.on('pageerror', (cause) => errors.push(cause.message));
	page.on('console', (message) => {
		if (message.type() !== 'error') return;
		if (
			blocked.has(message.location().url) &&
			message.text() === 'Failed to load resource: net::ERR_FAILED'
		)
			return;
		errors.push(`${message.text()} ${message.location().url}`);
	});
	page.on('requestfailed', (request) => {
		if (!blocked.has(request.url()) && !request.failure()?.errorText.includes('ERR_ABORTED'))
			errors.push(`${request.failure()?.errorText} ${request.url()}`);
	});
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
	return errors;
}
function path() {
	return `/admin/members?search=${prefix}&status=never_subscribed&per_page=1&sort_by=created_at&sort_dir=desc`;
}

test('CSV, real OOXML and multipage-capable PDF downloads preserve active filters across reload and ignore list pagination', async ({
	page
}, testInfo) => {
	const errors = await diagnostics(page);
	await page.goto(path(), { waitUntil: 'networkidle' });
	await expect(
		page.getByRole('heading', { name: 'Members Command Center', exact: true })
	).toBeVisible();
	for (const format of ['csv', 'xlsx', 'pdf'] as const) {
		const responseReady = page.waitForResponse(
			(response) => new URL(response.url()).pathname === '/api/admin/member-management/export'
		);
		const downloaded = page.waitForEvent('download');
		if (format === 'csv')
			await page
				.locator('.page-header')
				.getByRole('button', { name: 'Export CSV', exact: true })
				.click();
		else {
			const menu = page.locator('.page-header .export-dropdown');
			const trigger = menu.getByRole('button', { name: 'Choose export format', exact: true });
			const option = menu.getByRole('button', {
				name: format === 'xlsx' ? 'Excel' : 'PDF',
				exact: true
			});
			if (format === 'pdf') {
				await trigger.focus();
				await trigger.press('Enter');
				await expect(option).toBeVisible();
				await option.focus();
				await option.press('Enter');
			} else {
				await trigger.click();
				await option.click();
			}
		}
		const response = await responseReady;
		expect(response.status(), await response.text()).toBe(200);
		const query = new URL(response.url()).searchParams;
		expect(query.get('search')).toBe(prefix);
		expect(query.get('status')).toBe('never_subscribed');
		expect(query.get('format')).toBe(format);
		expect(query.get('sort_by')).toBe('created_at');
		expect(query.get('sort_dir')).toBe('desc');
		expect(query.has('page')).toBe(false);
		expect(query.has('per_page')).toBe(false);
		expect(response.headers()['x-export-row-count']).toBe('2');
		expect(response.headers()['x-export-sha256']).toMatch(/^[0-9a-f]{64}$/);
		const download = await downloaded;
		expect(download.suggestedFilename()).toMatch(
			new RegExp(`^members-export-\\d{4}-\\d{2}-\\d{2}\\.${format}$`)
		);
		const filename = testInfo.outputPath(`actual-members.${format}`);
		await download.saveAs(filename);
		const bytes = await readFile(filename);
		const networkBytes = await response.body();
		await writeFile(testInfo.outputPath(`network-members.${format}`), networkBytes);
		const receipt = {
			format,
			url: response.url(),
			headerSha256: response.headers()['x-export-sha256'],
			downloadBytes: bytes.length,
			networkBytes: networkBytes.length,
			downloadSha256: createHash('sha256').update(bytes).digest('hex'),
			networkSha256: createHash('sha256').update(networkBytes).digest('hex'),
			captureLimitation:
				networkBytes.length === 0 ? 'Playwright returned no body for the attachment response' : null
		};
		await writeFile(
			testInfo.outputPath(`download-receipt-${format}.json`),
			JSON.stringify(receipt, null, 2)
		);
		await testInfo.attach(`download-receipt-${format}`, {
			body: JSON.stringify(receipt, null, 2),
			contentType: 'application/json'
		});
		expect(bytes.length).toBeGreaterThan(0);
		expect(receipt.downloadSha256, JSON.stringify(receipt)).toBe(receipt.headerSha256);
		if (networkBytes.length > 0)
			expect(bytes.equals(networkBytes), JSON.stringify(receipt)).toBe(true);
		else expect(response.headers()['content-disposition']).toMatch(/^attachment\b/i);
		if (format === 'csv') {
			const value = bytes.toString('utf8');
			expect(value).toContain(`${prefix}-a@example.test`);
			expect(value).toContain(`${prefix}-b@example.test`);
			expect(value).not.toContain(`excluded-export-${stamp}@example.test`);
			expect(value).toContain("'=SUM(1,2)");
			expect(value).toContain('D’Ávila, 李明');
			expect(value).toContain('quoted ""member""');
		} else if (format === 'xlsx')
			expect([...bytes.subarray(0, 4)]).toEqual([0x50, 0x4b, 0x03, 0x04]);
		else expect(bytes.subarray(0, 5).toString()).toBe('%PDF-');
		await page.reload({ waitUntil: 'networkidle' });
		await expect(page.locator('#search-members')).toHaveValue(prefix);
	}
	const legacyReady = page.waitForResponse(
		(response) => new URL(response.url()).pathname === '/api/admin/members/export'
	);
	const legacyDownload = page.waitForEvent('download');
	await page.locator('.toolbar').getByRole('button', { name: 'Export CSV', exact: true }).click();
	const legacy = await legacyReady;
	expect(legacy.status()).toBe(200);
	expect(new URL(legacy.url()).searchParams.get('search')).toBe(prefix);
	expect(legacy.headers()['x-export-row-count']).toBe('2');
	const file = await legacyDownload;
	const saved = testInfo.outputPath('actual-members-legacy.csv');
	await file.saveAs(saved);
	const legacyBytes = await readFile(saved);
	expect(createHash('sha256').update(legacyBytes).digest('hex')).toBe(
		legacy.headers()['x-export-sha256']
	);
	const text = legacyBytes.toString('utf8');
	expect(text).toContain('id,name,email,created_at,updated_at');
	expect(text).toContain(`${prefix}-a@example.test`);
	expect(text).toContain(`${prefix}-b@example.test`);
	expect(text).not.toContain(`excluded-export-${stamp}@example.test`);
	expect(errors).toEqual([]);
});

test('malformed200 and real failure envelopes surface exact errors without a download or success toast, then recover', async ({
	page
}) => {
	await page.goto(path(), { waitUntil: 'networkidle' });
	let downloads = 0,
		attempts = 0;
	page.on('download', () => {
		downloads++;
	});
	const route = '**/api/admin/member-management/export?*';
	await page.route(route, (item) => {
		attempts++;
		return item.fulfill({ json: { success: true } });
	});
	await page
		.locator('.page-header')
		.getByRole('button', { name: 'Export CSV', exact: true })
		.click();
	await expect(
		page.getByText('The server did not return the requested member export format', { exact: true })
	).toBeVisible();
	expect(attempts).toBe(1);
	expect(downloads).toBe(0);
	await expect(page.getByText('Members exported as CSV', { exact: true })).toHaveCount(0);
	await page.unroute(route);
	const detail = 'Fixture date range is too large; no partial export was generated';
	await page.route(route, (item) => {
		attempts++;
		return item.fulfill({ status: 422, json: { message: detail } });
	});
	await page
		.locator('.page-header')
		.getByRole('button', { name: 'Export CSV', exact: true })
		.click();
	await expect(page.getByText(detail, { exact: true })).toBeVisible();
	expect(attempts).toBe(2);
	expect(downloads).toBe(0);
	await expect(page.getByText('Members exported as CSV', { exact: true })).toHaveCount(0);
	await page.unroute(route);
	const legacyRoute = '**/api/admin/members/export?*',
		legacyError = 'Fixture legacy export unavailable; no download was created';
	await page.route(legacyRoute, (item) =>
		item.fulfill({ status: 503, json: { error: legacyError } })
	);
	await page.locator('.toolbar').getByRole('button', { name: 'Export CSV', exact: true }).click();
	await expect(page.getByText(legacyError, { exact: true })).toBeVisible();
	expect(downloads).toBe(0);
	await expect(page.getByText('Members exported successfully', { exact: true })).toHaveCount(0);
	await page.unroute(legacyRoute);
	const recovering = page.waitForEvent('download');
	await page.locator('.toolbar').getByRole('button', { name: 'Export CSV', exact: true }).click();
	await recovering;
	await expect(page.getByText('Members exported successfully', { exact: true })).toBeVisible();
	expect(downloads).toBe(1);
});

test('member export reads reject anonymous and ordinary users for every actual format', async ({
	baseURL
}) => {
	const anonymous = await playwrightRequest.newContext({ baseURL });
	try {
		for (const route of [
			'/api/admin/members/export',
			...['csv', 'xlsx', 'pdf'].map(
				(format) => `/api/admin/member-management/export?format=${format}`
			)
		])
			expect((await anonymous.get(route)).status()).toBe(401);
	} finally {
		await anonymous.dispose();
	}
	const ordinary = await playwrightRequest.newContext({ baseURL });
	try {
		expect(
			(
				await ordinary.post('/api/auth/login', {
					data: { email: 'user@admin-completion.test', password }
				})
			).status()
		).toBe(200);
		for (const route of [
			'/api/admin/members/export',
			...['csv', 'xlsx', 'pdf'].map(
				(format) => `/api/admin/member-management/export?format=${format}`
			)
		])
			expect((await ordinary.get(route)).status()).toBe(403);
	} finally {
		await ordinary.dispose();
	}
});
