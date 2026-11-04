import { expect, test } from '@playwright/test';
test.describe.configure({ mode: 'serial' });

test('dashboard template cards filter, paginate, preview and recover from unavailable responses', async ({
	page,
	baseURL
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	expect(
		(
			await page.request.post('/api/auth/login', {
				data: {
					email: 'admin@admin-completion.test',
					password: 'OnlyForDisposableTest123!'
				}
			})
		).status()
	).toBe(200);
	const category = `catalog-browser-${Date.now()}`;
	const ids: number[] = [];
	const expectedFailures = new Set<string>();
	const blocked = new Set<string>();
	const errors: string[] = [];
	page.on('pageerror', (error) => errors.push(error.message));
	page.on('console', (message) => {
		if (
			message.type() === 'error' &&
			!expectedFailures.has(message.location().url) &&
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
			!expectedFailures.has(response.url())
		)
			errors.push(`${response.status()} ${response.url()}`);
	});
	page.on('requestfailed', (request) => {
		if (
			new URL(request.url()).pathname.startsWith('/api/') &&
			!request.failure()?.errorText.includes('ERR_ABORTED')
		)
			errors.push(request.url());
	});
	// Unrelated unfinished dashboard modules are explicit failure fixtures here.
	// Template requests and authoring/navigation use the actual isolated API.
	await page.route('**/*', (route) => {
		const url = new URL(route.request().url());
		if (!['localhost', '127.0.0.1'].includes(url.hostname)) {
			blocked.add(url.href);
			return route.abort();
		}
		if (
			/^\/api\/admin\/email\/(campaigns|sequences|automations|analytics)(\/|$)/.test(url.pathname)
		) {
			expectedFailures.add(url.href);
			return route.fulfill({ status: 503, json: { error: 'Unrelated module failure fixture' } });
		}
		return route.continue();
	});
	try {
		for (let index = 0; index < 21; index++) {
			const response = await page.request.post('/api/admin/email/templates', {
				data: {
					name: `${category} ${index}`,
					slug: `${category}-${index}`,
					subject: 'Catalog browser subject',
					body_html: `<p>Catalog browser content ${index}</p>`,
					category
				}
			});
			expect(response.status()).toBe(200);
			ids.push((await response.json()).data.id);
		}
		await page.goto('/email', { waitUntil: 'networkidle' });
		await page.getByRole('button', { name: 'Templates', exact: true }).last().click();
		await page.getByLabel('Template category', { exact: true }).fill(category);
		await page.getByRole('button', { name: 'Filter templates', exact: true }).click();
		await expect(page.getByRole('navigation', { name: 'Template pages' })).toContainText(
			'Page 1 · 21 templates'
		);
		await expect(page.locator('.template-card')).toHaveCount(20);
		await page.getByRole('button', { name: 'Next templates', exact: true }).click();
		await expect(page.locator('.template-card')).toHaveCount(1);
		await expect(page.getByRole('navigation', { name: 'Template pages' })).toContainText(
			'Page 2 · 21 templates'
		);
		await page.getByLabel('Search templates', { exact: true }).fill(`${category}-20`);
		await page.getByRole('button', { name: 'Filter templates', exact: true }).click();
		await expect(page.locator('.template-card')).toHaveCount(1);
		await expect(page.getByRole('navigation', { name: 'Template pages' })).toContainText(
			'Page 1 · 1 templates'
		);
		await page.getByRole('link', { name: 'Preview', exact: true }).click();
		await expect(
			page.frameLocator('iframe[title="Email template HTML preview"]').locator('p')
		).toHaveText('Catalog browser content 20');
		await page.goto('/email', { waitUntil: 'networkidle' });
		await page.getByRole('button', { name: 'Templates', exact: true }).last().click();
		let fail: 'unavailable' | 'malformed' | false = 'unavailable';
		await page.route('**/api/admin/email/templates?*', (route) => {
			if (fail === 'unavailable') {
				expectedFailures.add(route.request().url());
				return route.fulfill({ status: 503, json: { error: 'Catalog temporarily unavailable' } });
			}
			if (fail === 'malformed') return route.fulfill({ status: 200, json: { data: [] } });
			return route.continue();
		});
		await page.getByRole('button', { name: 'Filter templates', exact: true }).click();
		await expect(page.getByRole('alert')).toContainText('Catalog temporarily unavailable');
		fail = 'malformed';
		await page.getByRole('button', { name: 'Retry templates', exact: true }).click();
		await expect(page.getByRole('alert')).toContainText('Invalid email template pagination');
		fail = false;
		await page.getByRole('button', { name: 'Retry templates', exact: true }).click();
		await expect(page.getByRole('alert')).toHaveCount(0);
		await expect(page.locator('.template-card').first()).toBeVisible();
		await page.getByRole('link', { name: 'Create Template', exact: true }).click();
		await expect(page.getByLabel('HTML Body', { exact: true })).toBeVisible();
		expect(errors).toEqual([]);
	} finally {
		for (const id of ids)
			expect((await page.request.delete(`/api/admin/email/templates/${id}`)).status()).toBe(200);
	}
});

test('secondary catalog client round-trips real metadata and duplicates without inheriting usage', async ({
	page,
	baseURL
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	expect(
		(
			await page.request.post('/api/auth/login', {
				data: {
					email: 'admin@admin-completion.test',
					password: 'OnlyForDisposableTest123!'
				}
			})
		).status()
	).toBe(200);
	await page.goto('/admin/email/templates', { waitUntil: 'networkidle' });
	const result = await page.evaluate(async (stamp) => {
		const modulePath = '/src/lib/api/email.ts';
		const { emailApi } = (await import(modulePath)) as typeof import('../../src/lib/api/email.js');
		const created = (
			await emailApi.createTemplate({
				name: `Browser catalog ${stamp}`,
				slug: `browser-catalog-${stamp}`,
				subject: 'Catalog subject',
				html_content: '<p>Catalog content</p>',
				plain_text_content: 'Catalog text',
				category: 'browser-catalog',
				description: 'Saved catalog description',
				design_json: { blocks: [{ text: 'Saved' }] },
				tags: ['catalog'],
				is_public: true
			})
		).template;
		let copyId: string | undefined;
		try {
			const reloaded = (await emailApi.getTemplate(created.id)).template;
			const listing = await emailApi.getTemplates({
				category: 'BROWSER-CATALOG',
				search: created.slug
			});
			const copy = (await emailApi.duplicateTemplate(created.id)).template;
			copyId = copy.id;
			const updated = (
				await emailApi.updateTemplate(copy.id, { plain_text_content: null, tags: [] })
			).template;
			return { created, reloaded, listing, copy, updated };
		} finally {
			if (copyId) await emailApi.deleteTemplate(copyId);
			await emailApi.deleteTemplate(created.id);
		}
	}, Date.now());
	expect(result.reloaded).toEqual(result.created);
	expect(result.listing.total).toBe(1);
	expect(result.listing.templates[0]?.id).toBe(result.created.id);
	expect(result.copy.id).not.toBe(result.created.id);
	expect(result.copy.slug).not.toBe(result.created.slug);
	expect(result.copy.design_json).toEqual({ blocks: [{ text: 'Saved' }] });
	expect(result.copy.description).toBe('Saved catalog description');
	expect(result.copy.usage_count).toBe(0);
	expect(result.updated.plain_text_content).toBeUndefined();
	expect(result.updated.tags).toEqual([]);
	expect((await page.request.get(`/api/admin/email/templates/${result.created.id}`)).status()).toBe(
		404
	);
});

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
