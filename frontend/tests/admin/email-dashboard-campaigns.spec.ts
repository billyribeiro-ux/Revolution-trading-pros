import { expect, test } from '@playwright/test';

test('secondary dashboard reads actual campaign analytics and its create, view, edit and duplicate actions persist', async ({
	page,
	baseURL
}) => {
	test.setTimeout(90000);
	expect(baseURL).toBe('http://127.0.0.1:5274');
	expect(
		(
			await page.request.post('/api/auth/login', {
				data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
			})
		).status()
	).toBe(200);
	const prefix = `Secondary campaign browser ${Date.now()}`,
		owned = new Set<number>(),
		errors: string[] = [],
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
	try {
		for (let n = 0; n < 3; n++) {
			const created = await page.request.post('/api/admin/email/campaigns', {
				data: {
					name: `${prefix} ${n}`,
					subject: 'Secondary actual subject',
					html_content: '<p>Secondary saved source</p>'
				}
			});
			expect(created.status()).toBe(200);
			const row = (await created.json()).data;
			expect(row.name).toBe(`${prefix} ${n}`);
			expect(Number.isSafeInteger(row.id)).toBe(true);
			owned.add(row.id);
		}
		const analytics = await page.request.get('/api/admin/email/analytics?period=30d');
		expect(analytics.status()).toBe(200);
		const measured = (await analytics.json()).analytics;
		expect(measured.metrics_scope).toBe('native_campaign_outbox');
		expect(measured.emails_sent).toBeGreaterThan(0);
		expect(measured.total_opens).toBeGreaterThan(0);
		await page.goto('/email', { waitUntil: 'networkidle' });
		const stat = (label: string) =>
			page
				.locator('.stat-card')
				.filter({ has: page.getByText(label, { exact: true }) })
				.locator('.stat-value');
		await expect(stat('Campaign Provider Acceptances')).toHaveText(
			Number(measured.emails_sent).toLocaleString('en-US')
		);
		await expect(stat('Recipient Open Rate')).toHaveText(
			measured.average_open_rate === null
				? '—'
				: `${Number(measured.average_open_rate).toFixed(1)}%`
		);
		await expect(page.getByText('+12.5% vs last month', { exact: true })).toHaveCount(0);
		await page.getByLabel('Search campaigns', { exact: true }).fill(prefix);
		await page.getByRole('button', { name: 'Search campaigns', exact: true }).click();
		await expect(page.locator('.campaign-card')).toHaveCount(3);
		let card = page
			.locator('.campaign-card')
			.filter({ has: page.getByRole('heading', { name: `${prefix} 0`, exact: true }) });
		await expect(card).toContainText('—');
		await card.getByRole('link', { name: 'View', exact: true }).click();
		await expect(page.getByRole('heading', { name: 'Campaign Report', exact: true })).toBeVisible();
		await expect(
			page
				.locator('.metric-card')
				.filter({ has: page.getByText('Provider Accepted', { exact: true }) })
				.locator('.metric-value')
		).toHaveText('0');
		await page.goto('/email', { waitUntil: 'networkidle' });
		await page.getByLabel('Search campaigns', { exact: true }).fill(prefix);
		await page.getByRole('button', { name: 'Search campaigns', exact: true }).click();
		await expect(page.locator('.campaign-card')).toHaveCount(3);
		await card.getByRole('link', { name: 'Edit', exact: true }).click();
		await expect(
			page.getByRole('dialog').getByRole('heading', { name: 'Edit Campaign', exact: true })
		).toBeVisible();
		await page.getByLabel('Campaign Name', { exact: true }).fill(`${prefix} edited`);
		await page
			.getByRole('dialog')
			.getByRole('button', { name: 'Save Changes', exact: true })
			.click();
		await expect(page.getByRole('dialog')).toHaveCount(0);
		await page.goto('/email', { waitUntil: 'networkidle' });
		await page.getByLabel('Search campaigns', { exact: true }).fill(prefix);
		await page.getByRole('button', { name: 'Search campaigns', exact: true }).click();
		card = page
			.locator('.campaign-card')
			.filter({ has: page.getByRole('heading', { name: `${prefix} edited`, exact: true }) });
		await expect(card).toBeVisible();
		const duplicating = page.waitForResponse(
			(r) =>
				/\/api\/admin\/email\/campaigns\/\d+\/duplicate$/.test(new URL(r.url()).pathname) &&
				r.request().method() === 'POST'
		);
		await card.getByRole('button', { name: 'Duplicate', exact: true }).click();
		const copied = await duplicating;
		expect(copied.status()).toBe(200);
		const clone = (await copied.json()).data;
		expect(clone.name).toBe(`${prefix} edited (Copy)`);
		expect(clone.html_content).toBe('<p>Secondary saved source</p>');
		owned.add(clone.id);
		await page.reload({ waitUntil: 'networkidle' });
		await page.getByLabel('Search campaigns', { exact: true }).fill(prefix);
		await page.getByRole('button', { name: 'Search campaigns', exact: true }).click();
		await expect(page.locator('.campaign-card')).toHaveCount(4);
		await page.getByRole('button', { name: 'Analytics', exact: true }).click();
		await expect(
			page.getByRole('heading', { name: 'Recorded signals by UTC date', exact: true })
		).toBeVisible();
		await expect(page.getByText('Detailed analytics coming soon...', { exact: true })).toHaveCount(
			0
		);
		await expect(
			page.getByText(
				`Other or unspecified client observations: ${measured.unlisted_client_observations}`,
				{ exact: true }
			)
		).toBeVisible();
		const changing = page.waitForResponse(
			(r) =>
				new URL(r.url()).pathname === '/api/admin/email/analytics' &&
				new URL(r.url()).searchParams.get('period') === '7d'
		);
		await page.getByLabel('Reporting period', { exact: true }).selectOption('7d');
		expect((await changing).status()).toBe(200);
		await expect(
			page.getByRole('heading', { name: 'Recorded signals by UTC date', exact: true })
		).toBeVisible();
		await page.getByRole('link', { name: 'New Campaign', exact: true }).click();
		await expect(
			page.getByRole('dialog').getByRole('heading', { name: 'Create New Campaign', exact: true })
		).toBeVisible();
		expect(errors).toEqual([]);
	} finally {
		for (const id of owned)
			expect((await page.request.delete(`/api/admin/email/campaigns/${id}`)).status()).toBe(200);
	}
});

test('secondary dashboard rejects malformed campaigns and unavailable analytics, and retries actual reads', async ({
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
	const analytics = '**/api/admin/email/analytics?*';
	await page.route(listing, (route) =>
		route.fulfill({
			json: {
				success: true,
				data: [{ id: 9 }],
				meta: { total: 1, current_page: 1, last_page: 1, per_page: 20 }
			}
		})
	);
	await page.route(analytics, (route) =>
		route.fulfill({ status: 503, json: { error: 'Fixture recorded analytics unavailable' } })
	);
	await page.goto('/email', { waitUntil: 'networkidle' });
	await expect(
		page.getByText('Invalid campaign identity or status', { exact: true })
	).toBeVisible();
	await expect(
		page.getByText('Fixture recorded analytics unavailable', { exact: false })
	).toBeVisible();
	await expect(page.locator('.campaign-card')).toHaveCount(0);
	await expect(page.locator('.stats-grid')).toHaveCount(0);
	await page.unroute(listing);
	await page.getByRole('button', { name: 'Retry campaigns', exact: true }).click();
	await expect(page.getByText('Invalid campaign identity or status', { exact: true })).toHaveCount(
		0
	);
	await page.unroute(analytics);
	await page.getByRole('button', { name: 'Retry analytics', exact: true }).click();
	await expect(
		page.getByText('Fixture recorded analytics unavailable', { exact: false })
	).toHaveCount(0);
	await expect(page.locator('.stats-grid')).toBeVisible();
	expect(errors).toEqual([]);
});
