import { expect, test } from '@playwright/test';
import { readFile } from 'node:fs/promises';

test('member segment creation, editing and matching members persist through reloads', async ({
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
	page.on('response', (response) => {
		if (new URL(response.url()).pathname.startsWith('/api/') && response.status() >= 400)
			errors.push(`${response.status()} ${response.url()}`);
	});
	page.on('requestfailed', (request) => {
		if (
			new URL(request.url()).pathname.startsWith('/api/') &&
			!request.failure()?.errorText.includes('ERR_ABORTED')
		)
			errors.push(request.url());
	});
	await page.route('**/*', (route) => {
		if (['localhost', '127.0.0.1'].includes(new URL(route.request().url()).hostname))
			return route.continue();
		blocked.add(route.request().url());
		return route.abort();
	});
	const stamp = Date.now(),
		name = `Browser segment ${stamp}`;
	const users: number[] = [];
	let tagId: number | undefined, segmentId: number | undefined;
	try {
		const tag = await page.request.post('/api/admin/members/tags', {
			data: { name: `Browser cohort ${stamp}`, color: '#123456' }
		});
		expect(tag.status()).toBe(200);
		tagId = (await tag.json()).id;
		for (let index = 0; index < 2; index++) {
			const response = await page.request.post('/api/admin/users', {
				data: {
					name: `Segment person ${stamp} ${index}`,
					email: `segment-browser-${stamp}-${index}@example.test`,
					password: 'OnlyForDisposableTest123!',
					role: 'user'
				}
			});
			expect(response.status()).toBe(200);
			const id: number = (await response.json()).id;
			users.push(id);
			expect(
				(
					await page.request.post('/api/admin/members/tags/assign', {
						data: { user_id: id, tag_id: tagId }
					})
				).status()
			).toBe(200);
		}
		await page.goto('/admin/members/segments', { waitUntil: 'networkidle' });
		await page.getByRole('button', { name: 'New Segment', exact: true }).click();
		await page.getByLabel('Segment Name', { exact: true }).fill(name);
		await page.getByLabel('Description', { exact: true }).fill('Actual tagged browser cohort');
		await page.getByRole('button', { name: 'Add Condition', exact: true }).click();
		await page.locator('.condition-row select').nth(0).selectOption('tag');
		await page.locator('.condition-row select').nth(1).selectOption('has');
		await page.getByPlaceholder('Value', { exact: true }).fill(String(tagId));
		const saved = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === '/api/admin/members/segments' &&
				response.request().method() === 'POST'
		);
		await page.getByRole('button', { name: 'Create Segment', exact: true }).click();
		const created = await saved;
		expect(created.status()).toBe(200);
		const record = await created.json();
		segmentId = record.id;
		expect(record.memberCount).toBe(2);
		let card = page
			.locator('.segment-card')
			.filter({ has: page.getByRole('heading', { name, exact: true }) });
		await expect(card).toBeVisible();
		await page.reload({ waitUntil: 'networkidle' });
		await expect(card).toContainText('Actual tagged browser cohort');
		await card.getByTitle('View analytics', { exact: true }).click();
		const drawer = page.getByRole('complementary', { name, exact: true });
		await drawer.getByRole('button', { name: 'Analytics', exact: true }).click();
		await expect(
			drawer.locator('.analytics-card').filter({ hasText: 'Avg. Spending' })
		).toContainText('$0');
		await expect(
			drawer.locator('.analytics-card').filter({ hasText: 'Email Open Rate' })
		).toContainText('—');
		await expect(drawer.locator('.stat').filter({ hasText: '30d Growth' })).toContainText('—');
		const downloading = page.waitForEvent('download');
		await drawer.getByRole('button', { name: 'Export', exact: true }).click();
		const download = await downloading;
		expect(download.suggestedFilename()).toBe(`member-segment-${segmentId}.csv`);
		const file = await download.path();
		if (!file) throw new Error('Actual segment CSV download was not created');
		const csv = await readFile(file, 'utf8');
		expect(csv.split(/\r?\n/).filter(Boolean)).toHaveLength(3);
		expect(csv).toContain(`segment-browser-${stamp}-0@example.test`);
		expect(csv).toContain(`segment-browser-${stamp}-1@example.test`);
		expect(csv).not.toContain('admin@admin-completion.test');
		await page.keyboard.press('Escape');
		await card.getByTitle('Edit', { exact: true }).click();
		await expect(page.getByLabel('Segment Name', { exact: true })).toHaveValue(name);
		await page.getByLabel('Segment Name', { exact: true }).fill(`${name} edited`);
		await page.getByRole('button', { name: 'Save Segment', exact: true }).click();
		card = page
			.locator('.segment-card')
			.filter({ has: page.getByRole('heading', { name: `${name} edited`, exact: true }) });
		await expect(card).toBeVisible();
		await card.getByTitle('View members', { exact: true }).click();
		await expect(page).toHaveURL(`${baseURL}/admin/members?segment=${segmentId}`);
		await expect(
			page
				.getByRole('row')
				.filter({ has: page.getByText(`segment-browser-${stamp}-0@example.test`, { exact: true }) })
		).toBeVisible();
		await expect(
			page
				.getByRole('row')
				.filter({ has: page.getByText(`segment-browser-${stamp}-1@example.test`, { exact: true }) })
		).toBeVisible();
		expect((await page.request.get(`/api/admin/members?segment=${segmentId}`)).status()).toBe(200);
		await page.reload({ waitUntil: 'networkidle' });
		await expect(page.locator('tbody tr')).toHaveCount(2);
		await page.goto('/admin/members/segments', { waitUntil: 'networkidle' });
		await card.getByTitle('Delete', { exact: true }).click();
		await page.getByRole('button', { name: 'Delete Segment', exact: true }).click();
		await expect(card).toHaveCount(0);
		segmentId = undefined;
		await page.reload({ waitUntil: 'networkidle' });
		await expect(card).toHaveCount(0);
		expect(errors).toEqual([]);
	} finally {
		if (segmentId !== undefined)
			expect
				.soft((await page.request.delete(`/api/admin/members/segments/${segmentId}`)).status())
				.toBe(200);
		for (const id of users) {
			if (tagId !== undefined)
				expect
					.soft(
						(
							await page.request.delete('/api/admin/members/tags/unassign', {
								data: { user_id: id, tag_id: tagId }
							})
						).status()
					)
					.toBe(200);
			expect.soft((await page.request.delete(`/api/admin/users/${id}`)).status()).toBe(200);
		}
		if (tagId !== undefined)
			expect
				.soft((await page.request.delete(`/api/admin/members/tags/${tagId}`)).status())
				.toBe(200);
	}
});

test('segment details expose service failures and reject malformed analytics before Retry recovers', async ({
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
	const name = `Unavailable segment ${Date.now()}`;
	const created = await page.request.post('/api/admin/members/segments', {
		data: { name, rules: [{ field: 'tag', operator: 'has', value: name }] }
	});
	expect(created.status()).toBe(200);
	const id: number = (await created.json()).id;
	const errors: string[] = [],
		expected = new Set<string>(),
		blocked = new Set<string>();
	page.on('pageerror', (error) => errors.push(error.message));
	page.on('console', (message) => {
		if (
			message.type() === 'error' &&
			!expected.has(message.location().url) &&
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
			!expected.has(response.url())
		)
			errors.push(`${response.status()} ${response.url()}`);
	});
	await page.route('**/*', (route) => {
		if (['localhost', '127.0.0.1'].includes(new URL(route.request().url()).hostname))
			return route.continue();
		blocked.add(route.request().url());
		return route.abort();
	});
	let mode: 'unavailable' | 'malformed' | 'actual' = 'unavailable';
	await page.route(`**/api/admin/members/segments/${id}/analytics`, (route) => {
		if (mode === 'unavailable') {
			expected.add(route.request().url());
			return route.fulfill({ status: 503, json: { error: 'Analytics temporarily unavailable' } });
		}
		if (mode === 'malformed') return route.fulfill({ status: 200, json: { total_members: 0 } });
		return route.continue();
	});
	try {
		await page.goto('/admin/members/segments', { waitUntil: 'networkidle' });
		const card = page
			.locator('.segment-card')
			.filter({ has: page.getByRole('heading', { name, exact: true }) });
		await card.getByTitle('View analytics', { exact: true }).click();
		const drawer = page.getByRole('complementary', { name, exact: true });
		await expect(drawer.locator('.error-state')).toContainText('Analytics temporarily unavailable');
		mode = 'malformed';
		await drawer.getByRole('button', { name: 'Retry', exact: true }).click();
		await expect(drawer.locator('.error-state')).toContainText(
			'Invalid segment product measurements'
		);
		mode = 'actual';
		await drawer.getByRole('button', { name: 'Retry', exact: true }).click();
		await expect(drawer.locator('.error-state')).toHaveCount(0);
		await drawer.getByRole('button', { name: 'Members', exact: true }).click();
		await expect(drawer).toContainText('No members in this segment.');
		await drawer.getByRole('button', { name: 'Analytics', exact: true }).click();
		await expect(
			drawer.locator('.analytics-card').filter({ hasText: 'Avg. Spending' })
		).toContainText('—');
		expect(errors).toEqual([]);
	} finally {
		expect
			.soft((await page.request.delete(`/api/admin/members/segments/${id}`)).status())
			.toBe(200);
	}
});
