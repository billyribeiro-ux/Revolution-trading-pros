import { expect, test, request } from '@playwright/test';
function id(value: unknown): number {
	if (
		!value ||
		typeof value !== 'object' ||
		!('id' in value) ||
		typeof value.id !== 'number' ||
		!Number.isSafeInteger(value.id) ||
		value.id <= 0
	)
		throw new Error('Disposable record identity was not confirmed');
	return value.id;
}
test('campaign drafts use actual templates and segments, survive reloads and duplicate and delete independently', async ({
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
	const errors: string[] = [],
		blocked = new Set<string>();
	page.on('pageerror', (e) => errors.push(e.message));
	page.on('console', (m) => {
		if (
			m.type() === 'error' &&
			!(blocked.has(m.location().url) && m.text() === 'Failed to load resource: net::ERR_FAILED')
		)
			errors.push(m.text());
	});
	page.on('response', (r) => {
		if (new URL(r.url()).pathname.startsWith('/api/') && r.status() >= 400)
			errors.push(`${r.status()} ${r.url()}`);
	});
	page.on('requestfailed', (r) => {
		if (
			new URL(r.url()).pathname.startsWith('/api/') &&
			!r.failure()?.errorText.includes('ERR_ABORTED')
		)
			errors.push(r.url());
	});
	await page.route('**/*', (route) => {
		if (['localhost', '127.0.0.1'].includes(new URL(route.request().url()).hostname))
			return route.continue();
		blocked.add(route.request().url());
		return route.abort();
	});
	const stamp = Date.now(),
		name = `Campaign browser ${stamp}`,
		campaigns = new Set<number>();
	let template: number | undefined, segment: number | undefined;
	try {
		const t = await page.request.post('/api/admin/email/templates', {
			data: {
				name: `Template ${stamp}`,
				slug: `campaign-browser-${stamp}`,
				subject: 'Template subject',
				body_html: '<p>Actual persisted campaign browser source</p>',
				body_text: 'Actual alternative'
			}
		});
		expect(t.status()).toBe(200);
		template = id((await t.json()).data);
		const s = await page.request.post('/api/admin/members/segments', {
			data: {
				name: `Campaign segment ${stamp}`,
				rules: [{ field: 'days_since_signup', operator: 'gte', value: 0 }]
			}
		});
		expect(s.status()).toBe(200);
		segment = id(await s.json());
		const listing = await page.request.get(
			`/api/admin/members/segments?search=Campaign%20segment%20${stamp}`
		);
		expect(listing.status()).toBe(200);
		const memberCount = (await listing.json()).segments[0].memberCount;
		expect(Number.isSafeInteger(memberCount)).toBe(true);
		await page.goto('/admin/email/campaigns', { waitUntil: 'networkidle' });
		await page.getByRole('button', { name: 'New Campaign', exact: true }).click();
		await page.getByLabel('Campaign Name', { exact: true }).fill(name);
		await page.getByLabel('Subject Line', { exact: true }).fill('Actual browser campaign subject');
		await expect(
			page.getByLabel('Target Segment').locator(`option[value="${segment}"]`)
		).toHaveText(`Campaign segment ${stamp} (${memberCount.toLocaleString()})`);
		await page.getByLabel('Target Segment').selectOption(String(segment));
		await page.getByLabel('Email Template', { exact: true }).selectOption(String(template));
		const writing = page.waitForResponse(
			(r) =>
				new URL(r.url()).pathname === '/api/admin/email/campaigns' &&
				r.request().method() === 'POST'
		);
		await page
			.getByRole('dialog')
			.getByRole('button', { name: 'Save as Draft', exact: true })
			.click();
		const created = await writing;
		expect(created.status()).toBe(200);
		const draft = (await created.json()).data;
		const original = id(draft);
		campaigns.add(original);
		expect(draft.status).toBe('draft');
		expect(draft.html_content).toBe('<p>Actual persisted campaign browser source</p>');
		expect(draft.segment_id).toBe(segment);
		let card = page
			.locator('.campaign-card')
			.filter({ has: page.getByRole('heading', { name, exact: true }) });
		await expect(card).toBeVisible();
		await page.reload({ waitUntil: 'networkidle' });
		await expect(card).toContainText('Actual browser campaign subject');
		await expect(card).toContainText(`Campaign segment ${stamp}`);
		const cloning = page.waitForResponse(
			(r) =>
				new URL(r.url()).pathname === `/api/admin/email/campaigns/${original}/duplicate` &&
				r.request().method() === 'POST'
		);
		await card.getByRole('button', { name: 'Duplicate', exact: true }).click();
		const cloneResponse = await cloning;
		expect(cloneResponse.status()).toBe(200);
		const clone = id((await cloneResponse.json()).data);
		expect(clone).not.toBe(original);
		campaigns.add(clone);
		const copyName = `${name} (Copy)`;
		card = page
			.locator('.campaign-card')
			.filter({ has: page.getByRole('heading', { name: copyName, exact: true }) });
		await expect(card).toBeVisible();
		await page.reload({ waitUntil: 'networkidle' });
		await expect(card).toBeVisible();
		await card.getByRole('button', { name: `Delete ${copyName}`, exact: true }).click();
		const deleting = page.waitForResponse(
			(r) =>
				new URL(r.url()).pathname === `/api/admin/email/campaigns/${clone}` &&
				r.request().method() === 'DELETE'
		);
		await page
			.getByRole('dialog', { name: 'Delete Campaign', exact: true })
			.getByRole('button', { name: 'Delete', exact: true })
			.click();
		expect((await deleting).status()).toBe(200);
		campaigns.delete(clone);
		await expect(card).toHaveCount(0);
		expect((await page.request.get(`/api/admin/email/campaigns/${original}`)).status()).toBe(200);
		expect(errors).toEqual([]);
	} finally {
		for (const record of campaigns)
			expect((await page.request.delete(`/api/admin/email/campaigns/${record}`)).status()).toBe(
				200
			);
		if (segment !== undefined)
			expect((await page.request.delete(`/api/admin/members/segments/${segment}`)).status()).toBe(
				200
			);
		if (template !== undefined)
			expect((await page.request.delete(`/api/admin/email/templates/${template}`)).status()).toBe(
				200
			);
	}
});

test('campaign search and pagination use persisted records and recover after deleting the last page', async ({
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
	const prefix = `Campaign pages ${Date.now()}`,
		records = new Set<number>();
	const errors: string[] = [];
	page.on('pageerror', (e) => errors.push(e.message));
	await page.route('**/*', (route) =>
		['localhost', '127.0.0.1'].includes(new URL(route.request().url()).hostname)
			? route.continue()
			: route.abort()
	);
	try {
		for (let n = 0; n < 21; n++) {
			const response = await page.request.post('/api/admin/email/campaigns', {
				data: {
					name: `${prefix} ${n}`,
					subject: 'Pagination fixture',
					html_content: '<p>Local draft only</p>'
				}
			});
			expect(response.status()).toBe(200);
			records.add(id((await response.json()).data));
		}
		await page.goto('/admin/email/campaigns', { waitUntil: 'networkidle' });
		await page.getByLabel('Search campaigns', { exact: true }).fill(prefix);
		await page.getByRole('button', { name: 'Search', exact: true }).click();
		const pages = page.getByRole('navigation', { name: 'Campaign pagination' });
		await expect(pages).toContainText('Page 1 of 2');
		await expect(page.locator('.campaign-card')).toHaveCount(20);
		await pages.getByRole('button', { name: 'Next', exact: true }).click();
		await expect(pages).toContainText('Page 2 of 2');
		await expect(page.locator('.campaign-card')).toHaveCount(1);
		const card = page.locator('.campaign-card');
		await card.getByRole('button', { name: /^Delete / }).click();
		const removing = page.waitForResponse(
			(r) =>
				/\/api\/admin\/email\/campaigns\/\d+$/.test(new URL(r.url()).pathname) &&
				r.request().method() === 'DELETE'
		);
		await page
			.getByRole('dialog', { name: 'Delete Campaign', exact: true })
			.getByRole('button', { name: 'Delete', exact: true })
			.click();
		const response = await removing;
		expect(response.status()).toBe(200);
		records.delete((await response.json()).deleted_id);
		await expect(page.locator('.campaign-card')).toHaveCount(20);
		await expect(pages).toHaveCount(0);
		await page.reload({ waitUntil: 'networkidle' });
		await page.getByLabel('Search campaigns', { exact: true }).fill(prefix);
		await page.getByRole('button', { name: 'Search', exact: true }).click();
		await expect(page.locator('.campaign-card')).toHaveCount(20);
		expect(errors).toEqual([]);
	} finally {
		for (const record of records)
			expect((await page.request.delete(`/api/admin/email/campaigns/${record}`)).status()).toBe(
				200
			);
	}
});

test('campaign proxies reject anonymous and ordinary members across reads and mutations', async ({
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
							data: { email: 'user@admin-completion.test', password: 'OnlyForDisposableTest123!' }
						})
					).status()
				).toBe(200);
			for (const path of [
				'/api/admin/email/campaigns',
				'/api/admin/email/campaigns/stats',
				'/api/admin/email/campaigns/1'
			])
				expect((await client.get(path)).status()).toBe(expected);
			expect(
				(
					await client.post('/api/admin/email/campaigns', { data: { name: 'Must not persist' } })
				).status()
			).toBe(expected);
			expect((await client.delete('/api/admin/email/campaigns/1')).status()).toBe(expected);
			expect((await client.get('/admin/email/campaigns', { maxRedirects: 0 })).status()).toBe(303);
		}
		expect((await client.post('/api/auth/logout')).status()).toBe(200);
	} finally {
		await client.dispose();
	}
});

test('campaigns recover from malformed success and unavailable statistics without invented measurements', async ({
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
	const errors: string[] = [];
	page.on('pageerror', (e) => errors.push(e.message));
	await page.route('**/*', (route) =>
		['localhost', '127.0.0.1'].includes(new URL(route.request().url()).hostname)
			? route.continue()
			: route.abort()
	);
	const listing = /\/api\/admin\/email\/campaigns\?/;
	const stats = '**/api/admin/email/campaigns/stats';
	const diagnostics = '**/api/admin/email/status';
	await page.route(diagnostics, (route) =>
		route.fulfill({ status: 503, json: { error: 'Fixture broadcast diagnostics unavailable' } })
	);
	await page.route(listing, (route) =>
		route.fulfill({
			json: { success: true, data: [{ id: 42 }], meta: { total: 1, current_page: 1, last_page: 1 } }
		})
	);
	await page.route(stats, (route) =>
		route.fulfill({ status: 503, json: { error: 'Fixture campaign statistics unavailable' } })
	);
	await page.goto('/admin/email/campaigns', { waitUntil: 'networkidle' });
	await expect(
		page.getByText('Invalid campaign identity or status', { exact: true })
	).toBeVisible();
	await expect(page.locator('.campaign-card')).toHaveCount(0);
	await expect(
		page.getByText('Broadcast configuration status is unavailable.', { exact: false })
	).toBeVisible();
	await page.getByRole('button', { name: 'New Campaign', exact: true }).click();
	await expect(
		page.getByRole('dialog').getByRole('heading', { name: 'Create New Campaign', exact: true })
	).toBeVisible();
	await page.getByRole('dialog').getByRole('button', { name: 'Cancel', exact: true }).click();
	await page.unroute(diagnostics);
	await page.getByRole('button', { name: 'Refresh delivery configuration', exact: true }).click();
	await expect(
		page.getByText('Fixture broadcast diagnostics unavailable', { exact: false })
	).toHaveCount(0);
	await page.unroute(listing);
	await page.getByRole('button', { name: 'Try Again', exact: true }).click();
	await expect(
		page.getByText('Fixture campaign statistics unavailable', { exact: false })
	).toBeVisible();
	await expect(
		page
			.locator('.stat-card')
			.filter({ hasText: 'Recorded Provider Acceptances' })
			.locator('.stat-value')
	).toHaveText('—');
	await page.unroute(stats);
	await page.getByRole('button', { name: 'Retry statistics', exact: true }).click();
	await expect(
		page.getByText('Fixture campaign statistics unavailable', { exact: false })
	).toHaveCount(0);
	expect(errors).toEqual([]);
});
