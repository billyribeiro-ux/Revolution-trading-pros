import { expect, test } from '@playwright/test';

test('campaign references beyond one hundred remain selected across searches and sparse edits preserve saved creative', async ({
	page,
	baseURL
}) => {
	test.setTimeout(90000);
	expect(baseURL).toBe('http://127.0.0.1:5274');
	const prefix = process.env.CAMPAIGN_PICKER_PREFIX;
	if (!prefix || !/^admin-move-picker-\d+$/.test(prefix))
		throw new Error(
			'Seed the guarded native_campaign_reference_picker.psql fixture and set CAMPAIGN_PICKER_PREFIX'
		);
	expect(
		(
			await page.request.post('/api/auth/login', {
				data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
			})
		).status()
	).toBe(200);
	const name = `Campaign picker browser ${Date.now()}`,
		campaigns: number[] = [];
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
	try {
		const templates = await page.request.get(
			`/api/admin/email/templates?search=${encodeURIComponent(prefix + ' 001')}`
		);
		expect(templates.status()).toBe(200);
		const template = (await templates.json()).data[0];
		const segments = await page.request.get(
			`/api/admin/members/segments?search=${encodeURIComponent(prefix + ' 001')}`
		);
		expect(segments.status()).toBe(200);
		const segment = (await segments.json()).segments[0];
		expect(Number.isSafeInteger(template.id)).toBe(true);
		expect(Number.isSafeInteger(segment.id)).toBe(true);
		await page.goto('/admin/email/campaigns?new=1', { waitUntil: 'networkidle' });
		const dialog = page.getByRole('dialog');
		await expect(
			dialog.getByRole('heading', { name: 'Create New Campaign', exact: true })
		).toBeVisible();
		await page.getByLabel('Campaign Name', { exact: true }).fill(name);
		await page.getByLabel('Subject Line', { exact: true }).fill('Saved picker subject');
		await page.getByLabel('Find email templates', { exact: true }).fill(prefix);
		await page.getByRole('button', { name: 'Search templates', exact: true }).click();
		await expect(page.getByRole('navigation', { name: 'Template options pages' })).toContainText(
			'Page 1 of 6'
		);
		await expect(page.locator(`#campaign-template option[value="${template.id}"]`)).toHaveCount(0);
		for (let n = 2; n <= 6; n++) {
			await page.getByRole('button', { name: 'Next templates', exact: true }).click();
			await expect(page.getByRole('navigation', { name: 'Template options pages' })).toContainText(
				`Page ${n} of 6`
			);
		}
		await page.getByLabel('Email Template', { exact: true }).selectOption(String(template.id));
		await page.getByLabel('Find email templates', { exact: true }).fill(prefix + ' 105');
		await page.getByRole('button', { name: 'Search templates', exact: true }).click();
		await expect(page.getByRole('navigation', { name: 'Template options pages' })).toContainText(
			'Page 1 of 1'
		);
		await expect(page.getByLabel('Email Template', { exact: true })).toHaveValue(
			String(template.id)
		);
		await page.getByLabel('Find member segments', { exact: true }).fill(prefix);
		await page.getByRole('button', { name: 'Search segments', exact: true }).click();
		await expect(page.getByRole('navigation', { name: 'Segment options pages' })).toContainText(
			'Page 1 of 6'
		);
		await expect(page.locator(`#campaign-segment option[value="${segment.id}"]`)).toHaveCount(0);
		for (let n = 2; n <= 6; n++) {
			await page.getByRole('button', { name: 'Next segments', exact: true }).click();
			await expect(page.getByRole('navigation', { name: 'Segment options pages' })).toContainText(
				`Page ${n} of 6`
			);
		}
		await page.getByLabel('Target Segment', { exact: true }).selectOption(String(segment.id));
		await page.getByLabel('Find member segments', { exact: true }).fill(prefix + ' 105');
		await page.getByRole('button', { name: 'Search segments', exact: true }).click();
		await expect(page.getByRole('navigation', { name: 'Segment options pages' })).toContainText(
			'Page 1 of 1'
		);
		await expect(page.getByLabel('Target Segment', { exact: true })).toHaveValue(
			String(segment.id)
		);
		const saving = page.waitForResponse(
			(r) =>
				new URL(r.url()).pathname === '/api/admin/email/campaigns' &&
				r.request().method() === 'POST'
		);
		await dialog.getByRole('button', { name: 'Save as Draft', exact: true }).click();
		const saved = await saving;
		expect(saved.status()).toBe(200);
		const actual = (await saved.json()).data;
		campaigns.push(actual.id);
		expect(actual.template_id).toBe(template.id);
		expect(actual.segment_id).toBe(segment.id);
		expect(actual.html_content).toBe(template.body_html);
		expect(
			(
				await page.request.put(`/api/admin/email/templates/${template.id}`, {
					data: { body_html: `<p>Later picker template revision ${name}</p>` }
				})
			).status()
		).toBe(200);
		await page.goto(`/admin/email/campaigns?edit=${actual.id}`, { waitUntil: 'networkidle' });
		await expect(dialog.getByRole('heading', { name: 'Edit Campaign', exact: true })).toBeVisible();
		await expect(page.getByLabel('Email Template', { exact: true })).toHaveValue(
			String(template.id)
		);
		await expect(page.getByLabel('Target Segment', { exact: true })).toHaveValue(
			String(segment.id)
		);
		await page.getByLabel('Campaign Name', { exact: true }).fill(name + ' renamed');
		const editing = page.waitForResponse(
			(r) =>
				new URL(r.url()).pathname === `/api/admin/email/campaigns/${actual.id}` &&
				r.request().method() === 'PUT'
		);
		await dialog.getByRole('button', { name: 'Save Changes', exact: true }).click();
		const edited = await editing;
		expect(edited.status()).toBe(200);
		expect(edited.request().postDataJSON()).toEqual({ name: name + ' renamed' });
		expect((await edited.json()).data.html_content).toBe(template.body_html);
		await page.reload({ waitUntil: 'networkidle' });
		await expect(dialog.getByRole('heading', { name: 'Edit Campaign', exact: true })).toBeVisible();
		await expect(page.getByLabel('Campaign Name', { exact: true })).toHaveValue(name + ' renamed');
		expect(errors).toEqual([]);
	} finally {
		for (const id of campaigns)
			expect((await page.request.delete(`/api/admin/email/campaigns/${id}`)).status()).toBe(200);
	}
});
