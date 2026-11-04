import { expect, test } from '@playwright/test';
test.describe.configure({ mode: 'serial' });

test('template authoring, pagination, strict previews and text clearing persist through the admin proxy', async ({
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
	const failures: string[] = [];
	const blocked = new Set<string>();
	const expectedPreview = new Set<string>();
	let missingPreviewResponses = 0;
	page.on('pageerror', (error) => failures.push(error.message));
	page.on('console', (message) => {
		if (
			message.type() === 'error' &&
			![...expectedPreview].some((url) =>
				message.text().startsWith(`[API] Error for ${new URL(url).pathname.replace(/^\/api/, '')}`)
			) &&
			!(
				expectedPreview.has(message.location().url) &&
				message.text().startsWith('Failed to load resource:') &&
				message.text().includes('status of 400')
			) &&
			!(
				blocked.has(message.location().url) &&
				message.text() === 'Failed to load resource: net::ERR_FAILED'
			)
		)
			failures.push(`${message.text()} ${message.location().url}`);
	});
	page.on('response', (response) => {
		if (new URL(response.url()).pathname.startsWith('/api/') && response.status() >= 400) {
			if (expectedPreview.has(response.url()) && response.status() === 400)
				missingPreviewResponses++;
			else failures.push(`${response.status()} ${response.url()}`);
		}
	});
	page.on('requestfailed', (request) => {
		if (
			new URL(request.url()).pathname.startsWith('/api/') &&
			!request.failure()?.errorText.includes('ERR_ABORTED')
		)
			failures.push(request.url());
	});
	await page.route('**/*', (route) => {
		if (['localhost', '127.0.0.1'].includes(new URL(route.request().url()).hostname))
			return route.continue();
		blocked.add(route.request().url());
		return route.abort();
	});
	let mutations = 0;
	page.on('request', (request) => {
		if (
			new URL(request.url()).pathname === '/api/admin/email/templates' &&
			request.method() === 'POST'
		)
			mutations++;
	});
	const title = `Browser template workflow ${Date.now()}`,
		slug = `browser-template-${Date.now()}`;
	await page.goto('/admin/email/templates/new', { waitUntil: 'networkidle' });
	await page.getByLabel('Name', { exact: true }).fill(title);
	await page.getByLabel('Slug', { exact: true }).fill(slug);
	await page.getByLabel('Subject', { exact: true }).fill('Hello {{name}}');
	await page.getByLabel('Email Type', { exact: true }).fill('marketing');
	await page.getByLabel('HTML Body', { exact: true }).fill('<p>Hello {{name}} &amp; fixture</p>');
	await page.getByLabel('Plain Text Body (optional)', { exact: true }).fill('Plain {{name}}');
	await page.getByLabel('Variables (JSON)', { exact: true }).fill('{');
	await page.getByRole('button', { name: 'Save', exact: true }).click();
	await expect(page.getByRole('alert')).toBeVisible();
	await expect(page.getByRole('button', { name: 'Save', exact: true })).toBeEnabled();
	expect(mutations).toBe(0);
	await page.getByLabel('Variables (JSON)', { exact: true }).fill('{}');
	await page.getByRole('button', { name: 'Save', exact: true }).click();
	await expect(page.getByRole('alert')).toContainText('Variables must be a JSON array');
	expect(mutations).toBe(0);
	await page
		.getByLabel('Variables (JSON)', { exact: true })
		.fill('[{"key":"name","type":"string"}]');
	const saving = page.waitForResponse(
		(response) =>
			new URL(response.url()).pathname === '/api/admin/email/templates' &&
			response.request().method() === 'POST'
	);
	await page.getByRole('button', { name: 'Save', exact: true }).click();
	const saved = await saving;
	expect(saved.status()).toBe(200);
	const id = (await saved.json()).data.id;
	await expect(page).toHaveURL(/\/admin\/email\/templates$/);
	await page.getByPlaceholder('Search templates...').fill(slug);
	let row = page
		.getByRole('row')
		.filter({ has: page.getByRole('cell', { name: title, exact: true }) });
	await expect(row).toContainText('marketing');
	await row.getByTitle('Edit', { exact: true }).click();
	await expect(page.getByLabel('HTML Body', { exact: true })).toHaveValue(
		'<p>Hello {{name}} &amp; fixture</p>'
	);
	await expect(page.getByLabel('Plain Text Body (optional)', { exact: true })).toHaveValue(
		'Plain {{name}}'
	);
	await expect(page.getByLabel('Email Type', { exact: true })).toHaveValue('marketing');
	await page.getByLabel('Name', { exact: true }).fill(`${title} renamed`);
	await page.getByLabel('Plain Text Body (optional)', { exact: true }).fill('');
	await page.getByRole('button', { name: 'Save', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/email\/templates$/);
	await page.goto(`/admin/email/templates/edit/${id}`, { waitUntil: 'networkidle' });
	await expect(page.getByLabel('Name', { exact: true })).toHaveValue(`${title} renamed`);
	await expect(page.getByLabel('Slug', { exact: true })).toHaveValue(slug);
	await expect(page.getByLabel('Plain Text Body (optional)', { exact: true })).toHaveValue('');
	expectedPreview.add(new URL(`/api/admin/email/templates/${id}/preview`, baseURL).href);
	await page.goto(`/admin/email/templates/preview/${id}`, { waitUntil: 'networkidle' });
	await expect(page.getByRole('alert')).toContainText('missing variables');
	await page
		.getByLabel('Preview data (JSON)', { exact: true })
		.fill('{"name":"<Customer & fixture>"}');
	await page.getByRole('button', { name: 'Preview template', exact: true }).click();
	await expect(
		page.getByRole('heading', { name: 'Hello <Customer & fixture>', exact: true })
	).toBeVisible();
	await expect(
		page.frameLocator('iframe[title="Email template HTML preview"]').locator('p')
	).toHaveText('Hello <Customer & fixture> & fixture');
	await expect(
		page.frameLocator('iframe[title="Email template HTML preview"]').locator('script')
	).toHaveCount(0);
	// Enough independent saved fixtures to cross the backend's default page size.
	for (let index = 0; index < 21; index++) {
		const seeded = await page.request.post('/api/admin/email/templates', {
			data: {
				name: `Pagination ${slug} ${index}`,
				slug: `${slug}-${index}`,
				subject: 'Fixture',
				body_html: '<p>Pagination fixture</p>',
				variables: []
			}
		});
		expect(seeded.status()).toBe(200);
	}
	await page.goto('/admin/email/templates', { waitUntil: 'networkidle' });
	await expect(page.locator('.templates-table tbody tr')).toHaveCount(20);
	await page.getByRole('button', { name: 'Next templates', exact: true }).click();
	await expect(page.getByText(/Page 2 of/).first()).toBeVisible();
	await page.getByPlaceholder('Search templates...').fill(`${title} renamed`);
	row = page
		.getByRole('row')
		.filter({ has: page.getByRole('cell', { name: `${title} renamed`, exact: true }) });
	await expect(row).toBeVisible();
	await expect(page.locator('.templates-table tbody tr')).toHaveCount(1);
	await row.getByTitle('Delete', { exact: true }).click();
	const deleting = page.waitForResponse(
		(response) =>
			new URL(response.url()).pathname === `/api/admin/email/templates/${id}` &&
			response.request().method() === 'DELETE'
	);
	await page.getByRole('dialog').getByRole('button', { name: 'Delete', exact: true }).click();
	expect((await deleting).status()).toBe(200);
	await expect(row).toHaveCount(0);
	await page.reload({ waitUntil: 'networkidle' });
	const removed = await page.request.get(`/api/admin/email/templates/${id}`);
	expect(removed.status()).toBe(404);
	expect(missingPreviewResponses).toBe(1);
	// Invalid JSON is an expected validation state, not an application exception.
	expect(
		failures.filter(
			(failure) =>
				!failure.startsWith('Failed to save template: Error: Variables must be a JSON array') &&
				!failure.startsWith('Failed to save template: SyntaxError:')
		)
	).toEqual([]);
});

test('template list failures stay visible and Retry rejects a malformed success before recovery', async ({
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
	let mode = 'failure';
	await page.route('**/api/admin/email/templates?*', (route) =>
		mode === 'failure'
			? route.fulfill({
					status: 503,
					contentType: 'application/json',
					body: '{"error":"Fixture template storage unavailable"}'
				})
			: mode === 'malformed'
				? route.fulfill({
						status: 200,
						contentType: 'application/json',
						body: '{"data":[],"meta":{}}'
					})
				: route.continue()
	);
	await page.goto('/admin/email/templates', { waitUntil: 'networkidle' });
	await expect(page.getByRole('alert')).toContainText('Fixture template storage unavailable');
	mode = 'malformed';
	await page.getByRole('button', { name: 'Retry', exact: true }).click();
	await expect(page.getByRole('alert')).toContainText('Invalid email template list response');
	mode = 'real';
	await page.getByRole('button', { name: 'Retry', exact: true }).click();
	await expect(page.getByRole('alert')).toHaveCount(0);
	await expect(page.locator('.templates-table tbody tr')).toHaveCount(20);
});
