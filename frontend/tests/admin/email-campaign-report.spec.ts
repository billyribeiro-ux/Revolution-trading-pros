import { expect, test } from '@playwright/test';
import { readFile } from 'node:fs/promises';

test('recorded reports preserve unknown rates, export actual values safely, and recover without demo results', async ({
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
	const name = `=Report fixture ${Date.now()}`;
	const created = await page.request.post('/api/admin/email/campaigns', {
		data: { name, subject: '=1+1', html_content: '<p>Actual draft report</p>' }
	});
	expect(created.status()).toBe(200);
	const id = (await created.json()).data.id;
	expect(Number.isSafeInteger(id)).toBe(true);
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
	await page.route('**/*', (route) => {
		if (['localhost', '127.0.0.1'].includes(new URL(route.request().url()).hostname))
			return route.continue();
		blocked.add(route.request().url());
		return route.abort();
	});
	try {
		await page.goto(`/admin/email/campaigns/${id}/report`, { waitUntil: 'networkidle' });
		await expect(page.locator('.report-content')).toContainText(name);
		const metric = (label: string) =>
			page.locator('.metric-card').filter({ has: page.getByText(label, { exact: true }) });
		await expect(metric('Provider Accepted').locator('.metric-value')).toHaveText('0');
		await expect(metric('Unique Opens').locator('.metric-rate')).toHaveText('—');
		await expect(page.getByText('No recorded link clicks.', { exact: true })).toBeVisible();
		await page.getByRole('button', { name: 'Export Report', exact: true }).click();
		const downloading = page.waitForEvent('download');
		await page.getByRole('menuitem', { name: 'CSV Spreadsheet', exact: true }).click();
		const downloaded = await downloading;
		expect(downloaded.suggestedFilename()).toBe(`campaign-report-${id}.csv`);
		const file = await downloaded.path();
		if (!file) throw new Error('Report CSV download was not written');
		const csv = await readFile(file, 'utf8');
		expect(csv).toContain(`"'${name}"`);
		expect(csv).toContain('"\'=1+1"');
		expect(csv).toContain('"Emails Sent","0"');
		expect(csv).toContain('"Open Rate","—"');
		expect(errors).toEqual([]);
		const report = `**/api/admin/email/campaigns/${id}/report`;
		await page.route(report, (route) => route.fulfill({ json: { success: true, data: { id } } }));
		await page.reload({ waitUntil: 'networkidle' });
		await expect(
			page.getByRole('heading', { name: 'Unable to Load Report', exact: true })
		).toBeVisible();
		await expect(page.locator('.report-content')).toHaveCount(0);
		await expect(
			page.getByText('December Newsletter - Trading Insights', { exact: true })
		).toHaveCount(0);
		await page.unroute(report);
		await page.getByRole('button', { name: 'Try Again', exact: true }).click();
		await expect(page.locator('.report-content')).toContainText(name);
	} finally {
		const removed = await page.request.delete(`/api/admin/email/campaigns/${id}`);
		expect(removed.status()).toBe(200);
	}
});
