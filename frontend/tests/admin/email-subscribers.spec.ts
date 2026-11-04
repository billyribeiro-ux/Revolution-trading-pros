import { expect, test } from '@playwright/test';

test('newsletter subscriber profiles, pending consent, filtering, CSV and atomic bulk deletion persist', async ({
	page,
	baseURL,
	request: outsider
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	const failures: string[] = [];
	page.on('pageerror', (e) => failures.push(e.message));
	page.on('response', (r) => {
		const path = new URL(r.url()).pathname;
		if (path.startsWith('/api/') && r.status() >= 400) failures.push(`${r.status()} ${path}`);
	});
	page.on('requestfailed', (r) => {
		if (
			new URL(r.url()).pathname.startsWith('/api/') &&
			!r.failure()?.errorText.includes('ERR_ABORTED')
		)
			failures.push(`${r.failure()?.errorText} ${r.url()}`);
	});
	page.on('console', (m) => {
		if (m.type() !== 'error') return;
		const location = m.location().url;
		if (location && !['127.0.0.1', 'localhost'].includes(new URL(location).hostname)) return;
		failures.push(m.text());
	});
	await page.route('**/*', (r) => {
		const u = new URL(r.request().url());
		return ['127.0.0.1', 'localhost'].includes(u.hostname) ||
			!['http:', 'https:'].includes(u.protocol)
			? r.continue()
			: r.abort();
	});
	expect((await outsider.get('/api/admin/email/subscribers')).status()).toBe(401);
	expect(
		(
			await outsider.post('/api/auth/login', {
				data: { email: 'user@admin-completion.test', password: 'OnlyForDisposableTest123!' }
			})
		).ok()
	).toBe(true);
	expect((await outsider.get('/api/admin/email/subscribers')).status()).toBe(403);
	expect(
		(
			await outsider.post('/api/admin/email/subscribers', {
				data: { email: 'unauthorized-browser-subscriber@example.test' }
			})
		).status()
	).toBe(403);

	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
	});
	expect(login.ok()).toBe(true);
	const root = '/api/admin/email/subscribers';
	const prefix = `subscriber-browser-${Date.now()}`;
	for (let i = 0; i < 26; i++) {
		const r = await page.request.post(root, {
			data: {
				email: `${prefix}-${i}@example.test`,
				first_name: `Subscriber ${i}`,
				tags: ['browser-fixture']
			}
		});
		expect(r.status(), await r.text()).toBe(201);
	}
	await page.goto('/admin/email/subscribers', { waitUntil: 'networkidle' });
	await expect(page.getByRole('heading', { name: 'Email Subscribers', exact: true })).toBeVisible();
	await page.getByLabel('Search subscribers').fill(prefix);
	await page.getByLabel('Search subscribers').press('Enter');
	await expect(page.locator('tbody tr')).toHaveCount(25);
	await page.getByRole('button', { name: 'Next', exact: true }).click();
	await expect(page.locator('tbody tr')).toHaveCount(1);
	await page.getByRole('button', { name: 'Previous', exact: true }).click();
	await expect(page.locator('tbody tr')).toHaveCount(25);

	await page.getByRole('button', { name: 'Add Subscriber', exact: true }).click();
	await page.getByLabel('Email Address *').fill(`${prefix}-new@example.test`);
	await page.getByLabel('First Name', { exact: true }).fill('Browser created');
	await page.getByLabel('Tags (comma-separated)').fill('browser-fixture, vip');
	await page
		.getByRole('dialog')
		.getByRole('button', { name: 'Add Subscriber', exact: true })
		.click();
	await expect(page.getByRole('dialog')).toHaveCount(0);
	await page.getByLabel('Search subscribers').fill(`${prefix}-new@`);
	await page.getByLabel('Search subscribers').press('Enter');
	const row = page.locator('tbody tr').filter({ hasText: `${prefix}-new@example.test` });
	await expect(row).toHaveCount(1);
	await expect(row.locator('.status-badge')).toHaveText('pending');
	await expect(row).toContainText('Confirmation: unavailable');
	await expect(row).toContainText('Unavailable');
	await page.reload({ waitUntil: 'networkidle' });
	await page.getByLabel('Search subscribers').fill(`${prefix}-new@`);
	await page.getByLabel('Search subscribers').press('Enter');
	await expect(row).toContainText('Browser created');
	await page.getByLabel('Subscriber status').selectOption('subscribed');
	await expect(page.locator('tbody tr')).toHaveCount(0);
	await page.getByLabel('Subscriber status').selectOption('pending');
	await expect(row).toBeVisible();
	await page.getByLabel('Filter subscriber tags').fill('vip');
	await page.getByLabel('Filter subscriber tags').press('Enter');
	await expect(row).toBeVisible();
	const downloadPromise = page.waitForEvent('download');
	await page.getByRole('button', { name: 'Export', exact: true }).click();
	const download = await downloadPromise;
	const stream = await download.createReadStream();
	const chunks: Buffer[] = [];
	if (!stream) throw new Error('CSV download missing');
	for await (const chunk of stream) chunks.push(Buffer.from(chunk));
	const csv = Buffer.concat(chunks).toString('utf8');
	expect(csv).toContain(`${prefix}-new@example.test`);
	expect(csv).toContain('pending');
	expect(csv).not.toContain(`${prefix}-1@example.test`);
	await page.getByLabel('Filter subscriber tags').fill('');
	await page.getByLabel('Search subscribers').fill(prefix);
	await page.getByLabel('Search subscribers').press('Enter');
	await expect(page.locator('tbody tr')).toHaveCount(25);
	await page.getByRole('checkbox', { name: 'Select all visible subscribers' }).check();
	await page.getByRole('button', { name: 'Delete', exact: true }).click();
	await page
		.getByRole('dialog')
		.getByRole('button', { name: /Delete/ })
		.click();
	await expect(page.locator('tbody tr')).toHaveCount(2);
	await page.reload({ waitUntil: 'networkidle' });
	await page.getByLabel('Search subscribers').fill(prefix);
	await page.getByLabel('Search subscribers').press('Enter');
	await expect(page.locator('tbody tr')).toHaveCount(2);
	const remaining = await page.request.get(`${root}?search=${prefix}`);
	const records: { subscribers: { id: string }[]; total: number } = await remaining.json();
	expect(records.total).toBe(2);
	const deleted = await page.request.post(`${root}/bulk-delete`, {
		data: { ids: records.subscribers.map((s) => s.id) }
	});
	expect(deleted.ok()).toBe(true);
	await page.getByRole('button', { name: 'Refresh subscribers' }).click();
	await expect(page.locator('tbody tr')).toHaveCount(0);
	expect(failures).toEqual([]);
});
