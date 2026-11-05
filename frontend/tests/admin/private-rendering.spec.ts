import { expect, test } from '@playwright/test';

test('production private routes enforce authentication and do not become static login pages', async ({
	request,
	baseURL,
	page
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5275');
	for (const path of [
		'/admin',
		'/admin/email/subscribers',
		'/admin/email/campaigns',
		'/admin/email/campaigns/1/report',
		'/email',
		'/dashboard',
		'/dashboard/classes',
		'/my/courses',
		'/my/subscriptions',
		'/checkout/monthly-spx'
	]) {
		const response = await request.get(path, { maxRedirects: 0 });
		expect(response.status(), `${path} must use the runtime auth gate`).toBe(303);
		expect(response.headers().location).toContain('/login?redirect=');
		expect(response.headers()['cache-control']).toBe('no-store');
		expect(response.headers()['x-robots-tag']).toContain('noindex');
	}
	const anonymousApi = await request.get('/api/admin/connections');
	expect(anonymousApi.status()).toBe(401);
	expect(anonymousApi.headers()['cache-control']).toBe('no-store');
	const dataRedirect = await request.get('/admin/__data.json', { maxRedirects: 0 });
	expect(dataRedirect.status()).toBe(200);
	expect(await dataRedirect.json()).toMatchObject({ type: 'redirect', status: 303 });
	expect(dataRedirect.headers()['cache-control']).toBe('no-store');
	expect(dataRedirect.headers()['x-robots-tag']).toContain('noindex');
	const actionRedirect = await request.post('/checkout/monthly-spx', {
		headers: { Accept: 'application/json', 'x-sveltekit-action': 'true' },
		data: {}
	});
	expect(actionRedirect.status()).toBe(200);
	expect(await actionRedirect.json()).toMatchObject({ type: 'redirect', status: 303 });
	expect(actionRedirect.headers()['cache-control']).toBe('no-store');

	const ordinary = await request.post('/api/auth/login', {
		data: { email: 'user@admin-completion.test', password: 'OnlyForDisposableTest123!' }
	});
	expect(ordinary.ok()).toBe(true);
	const denied = await request.get('/admin', { maxRedirects: 0 });
	expect(denied.status()).toBe(303);
	expect(denied.headers().location).toBe('/?error=admin_required');
	expect((await request.get('/api/admin/connections')).status()).toBe(403);
	const campaignDenied = await request.get('/admin/email/campaigns', { maxRedirects: 0 });
	expect(campaignDenied.status()).toBe(303);
	expect(campaignDenied.headers().location).toBe('/?error=admin_required');
	const emailDenied = await request.get('/email', { maxRedirects: 0 });
	expect(emailDenied.status()).toBe(303);
	expect(emailDenied.headers().location).toBe('/?error=admin_required');
	expect(emailDenied.headers()['cache-control']).toBe('no-store');

	const login = await request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
	});
	expect(login.ok()).toBe(true);
	for (const path of ['/admin', '/admin/email/subscribers', '/admin/email/campaigns', '/email']) {
		const response = await request.get(path, { maxRedirects: 0 });
		expect(response.status(), path).toBe(200);
		expect(response.headers()['cache-control']).toBe('no-store');
		expect(response.headers()['x-robots-tag']).toContain('noindex');
		expect(await response.text()).not.toContain('<script>location.href="/login');
	}
	for (const [from, to] of [
		['/admin/popups/create', '/admin/popups/new'],
		['/dashboard/account/billing', '/my/subscriptions']
	]) {
		const response = await request.get(from, { maxRedirects: 0 });
		expect(response.status(), from).toBe(307);
		expect(response.headers().location).toBe(to);
	}
	const cookies = await request.storageState();
	await page.context().addCookies(cookies.cookies);
	const failures: string[] = [];
	page.on('pageerror', (e) => failures.push(e.message));

	page.on('requestfailed', (r) => {
		if (
			new URL(r.url()).pathname.startsWith('/api/') &&
			!r.failure()?.errorText.includes('ERR_ABORTED')
		)
			failures.push(`${r.failure()?.errorText} ${r.url()}`);
	});
	page.on('console', (m) => {
		if (m.type() !== 'error') return;
		const u = m.location().url;
		if (u && !['127.0.0.1', 'localhost'].includes(new URL(u).hostname)) return;
		failures.push(m.text());
	});
	page.on('response', (r) => {
		if (new URL(r.url()).pathname.startsWith('/api/') && r.status() >= 400)
			failures.push(`${r.status()} ${r.url()}`);
	});
	await page.route('**/*', (r) => {
		const u = new URL(r.request().url());
		return ['127.0.0.1', 'localhost'].includes(u.hostname) ||
			!['http:', 'https:'].includes(u.protocol)
			? r.continue()
			: r.abort();
	});
	await page.goto('/admin/email/subscribers', { waitUntil: 'networkidle' });
	await expect(page.getByRole('heading', { name: 'Email Subscribers', exact: true })).toBeVisible();
	await expect(page.getByRole('button', { name: 'Refresh subscribers' })).toBeVisible();
	await page.goto('/email', { waitUntil: 'networkidle' });
	await expect(page.getByRole('heading', { name: 'Email Marketing', exact: true })).toBeVisible();
	await expect(page.locator('.stats-grid')).toBeVisible();
	await page.getByRole('button', { name: 'Analytics', exact: true }).click();
	await expect(
		page.getByRole('heading', { name: 'Recorded signals by UTC date', exact: true })
	).toBeVisible();

	const reportFixture = await request.post('/api/admin/email/campaigns', {
		data: {
			name: `Production report fixture ${Date.now()}`,
			subject: 'Production report source',
			html_content: '<p>Actual report fixture</p>'
		}
	});
	expect(reportFixture.status()).toBe(200);
	const campaign = (await reportFixture.json()).data.id;
	expect(Number.isSafeInteger(campaign)).toBe(true);
	try {
		const html = await request.get(`/admin/email/campaigns/${campaign}/report`);
		expect(html.status()).toBe(200);
		expect(html.headers()['cache-control']).toBe('no-store');
		expect(html.headers()['x-robots-tag']).toContain('noindex');
		await page.goto(`/admin/email/campaigns/${campaign}/report`, { waitUntil: 'networkidle' });
		await expect(page.getByRole('heading', { name: 'Campaign Report', exact: true })).toBeVisible();
		await expect(
			page
				.locator('.metric-card')
				.filter({ has: page.getByText('Provider Accepted', { exact: true }) })
				.locator('.metric-value')
		).toHaveText('0');
		expect(failures).toEqual([]);
	} finally {
		const cleanup = await request.delete(`/api/admin/email/campaigns/${campaign}`, {
			headers: { Origin: baseURL ?? '', 'Content-Type': 'application/json' }
		});
		expect(cleanup.status(), await cleanup.text()).toBe(200);
	}
});
