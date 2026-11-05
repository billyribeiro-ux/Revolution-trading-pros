import { expect, test } from '@playwright/test';

test('template and member-segment actions open actual draft references and require explicit saving', async ({
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
	const stamp = Date.now(),
		name = `Campaign reference browser ${stamp}`,
		owned = new Set<number>();
	const templateResponse = await page.request.post('/api/admin/email/templates', {
		data: {
			name,
			slug: `campaign-reference-browser-${stamp}`,
			subject: 'Actual template subject',
			body_html: '<p>Actual reference source</p>',
			body_text: 'Actual reference alternative'
		}
	});
	expect(templateResponse.status()).toBe(200);
	const template = (await templateResponse.json()).data.id;
	const segmentResponse = await page.request.post('/api/admin/members/segments', {
		data: { name, rules: [{ field: 'days_since_signup', operator: 'gte', value: 0 }] }
	});
	expect(segmentResponse.status()).toBe(200);
	const segment = (await segmentResponse.json()).id;
	expect(Number.isSafeInteger(template) && Number.isSafeInteger(segment)).toBe(true);
	const errors: string[] = [],
		blocked = new Set<string>();
	page.on('pageerror', (error) => errors.push(error.message));
	page.on('console', (message) => {
		if (
			message.type() === 'error' &&
			!(
				blocked.has(message.location().url) &&
				message.text() === 'Failed to load resource: net::ERR_FAILED'
			)
		)
			errors.push(message.text());
	});
	await page.route('**/*', (route) => {
		if (['localhost', '127.0.0.1'].includes(new URL(route.request().url()).hostname))
			return route.continue();
		blocked.add(route.request().url());
		return route.abort();
	});
	const writes: string[] = [];
	page.on('request', (request) => {
		if (
			request.method() === 'POST' &&
			new URL(request.url()).pathname.startsWith('/api/admin/email/campaigns')
		)
			writes.push(request.url());
	});
	async function save(draftName: string) {
		await page.getByLabel('Campaign Name', { exact: true }).fill(draftName);
		const pending = page.waitForResponse(
			(response) =>
				response.request().method() === 'POST' &&
				new URL(response.url()).pathname === '/api/admin/email/campaigns'
		);
		await page.getByRole('button', { name: 'Save as Draft', exact: true }).click();
		const saved = await pending;
		expect(saved.status()).toBe(200);
		const campaign = (await saved.json()).data;
		owned.add(campaign.id);
		expect(campaign).toMatchObject({
			name: draftName,
			status: 'draft',
			sent_count: 0,
			total_recipients: 0,
			template_id: template,
			html_content: '<p>Actual reference source</p>'
		});
		return campaign;
	}
	try {
		await page.goto('/email', { waitUntil: 'networkidle' });
		await page.getByRole('button', { name: 'Templates', exact: true }).last().click();
		await page.getByLabel('Search templates', { exact: true }).fill(name);
		await page.getByRole('button', { name: 'Filter templates', exact: true }).click();
		const card = page
			.locator('.template-card')
			.filter({ has: page.getByText(name, { exact: true }) });
		await card.getByRole('link', { name: 'Use Template', exact: true }).click();
		await expect(page).toHaveURL(
			new RegExp(`/admin/email/campaigns\\?new=1&template=${template}$`)
		);
		await expect(page.getByLabel('Email Template', { exact: true })).toHaveValue(String(template));
		await expect(page.getByLabel('Campaign Name', { exact: true })).toHaveValue(name);
		await expect(page.getByLabel('Subject Line', { exact: true })).toHaveValue(
			'Actual template subject'
		);
		expect(writes).toEqual([]);
		const first = await save(`${name} template`);
		expect(first.segment_id).toBeNull();
		await page.goto('/admin/members/segments', { waitUntil: 'networkidle' });
		await page.getByLabel('Search segments', { exact: true }).fill(name);
		await page.getByRole('button', { name: 'Search', exact: true }).click();
		const segmentCard = page
			.locator('.segment-card')
			.filter({ has: page.getByText(name, { exact: true }) });
		await segmentCard.getByRole('button', { name: 'Send campaign', exact: true }).click();
		await expect(page.getByLabel('Target Segment', { exact: true })).toHaveValue(String(segment));
		expect(writes).toHaveLength(1);
		await page.getByLabel('Subject Line', { exact: true }).fill('Actual segment campaign');
		await expect(
			page.getByLabel('Email Template', { exact: true }).locator(`option[value="${template}"]`)
		).toHaveCount(1);
		await page.getByLabel('Email Template', { exact: true }).selectOption(String(template));
		const second = await save(`${name} segment`);
		expect(second.segment_id).toBe(segment);
		await page.reload({ waitUntil: 'networkidle' });
		const actual = await page.request.get(`/api/admin/email/campaigns/${second.id}`);
		expect((await actual.json()).data).toMatchObject({
			template_id: template,
			segment_id: segment,
			status: 'draft',
			sent_count: 0
		});
		expect(writes).toHaveLength(2);
		expect(errors).toEqual([]);
	} finally {
		for (const id of owned)
			expect((await page.request.delete(`/api/admin/email/campaigns/${id}`)).status()).toBe(200);
		expect((await page.request.delete(`/api/admin/members/segments/${segment}`)).status()).toBe(
			200
		);
		expect((await page.request.delete(`/api/admin/email/templates/${template}`)).status()).toBe(
			200
		);
	}
});

test('draft reference failures block saving, retry actual reads and discard stale selections', async ({
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
	const name = `Campaign reference retry ${Date.now()}`;
	const created = await page.request.post('/api/admin/email/templates', {
		data: {
			name,
			slug: `reference-retry-${Date.now()}`,
			subject: 'Reference retry subject',
			body_html: '<p>Reference retry</p>'
		}
	});
	expect(created.status()).toBe(200);
	const template = (await created.json()).data.id;
	const source = `**/api/admin/email/templates/${template}`;
	const newPath = `/admin/email/campaigns?new=1&template=${template}`;
	try {
		await page.route(source, (route) =>
			route.fulfill({ status: 503, json: { error: 'Controlled reference outage' } })
		);
		await page.goto(newPath, { waitUntil: 'networkidle' });
		await expect(
			page.getByRole('alert').filter({ hasText: 'Controlled reference outage' })
		).toBeVisible();
		await expect(page.getByRole('button', { name: 'Save as Draft', exact: true })).toBeDisabled();
		await page.unroute(source);
		await page.getByRole('button', { name: 'Retry selected references', exact: true }).click();
		await expect(page.getByLabel('Email Template', { exact: true })).toHaveValue(String(template));
		await expect(page.getByRole('button', { name: 'Save as Draft', exact: true })).toBeEnabled();
		await page.goto('/admin/email/campaigns?new=1&template=../settings', {
			waitUntil: 'networkidle'
		});
		await expect(
			page.getByRole('alert').filter({ hasText: 'Choose a valid campaign template' })
		).toBeVisible();
		await expect(page.getByRole('button', { name: 'Save as Draft', exact: true })).toBeDisabled();
		let release = () => {};
		const held = new Promise<void>((resolve) => {
			release = resolve;
		});
		await page.route(source, async (route) => {
			await held;
			await route.continue();
		});
		await page.goto(newPath, { waitUntil: 'domcontentloaded' });
		await expect(
			page.getByText('Loading requested campaign references…', { exact: true })
		).toBeVisible();
		await page.getByRole('button', { name: 'Cancel', exact: true }).click();
		await page.getByRole('button', { name: 'New Campaign', exact: true }).click();
		release();
		await page.waitForResponse(
			(response) => new URL(response.url()).pathname === `/api/admin/email/templates/${template}`
		);
		await expect(page.getByLabel('Campaign Name', { exact: true })).toHaveValue('');
		await expect(page.getByLabel('Email Template', { exact: true })).toHaveValue('');
		await page.unroute(source);
	} finally {
		expect((await page.request.delete(`/api/admin/email/templates/${template}`)).status()).toBe(
			200
		);
	}
});
