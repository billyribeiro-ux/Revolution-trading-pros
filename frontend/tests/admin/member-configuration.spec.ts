import { expect, test, request } from '@playwright/test';
function responseId(value: unknown): number {
	if (
		!value ||
		typeof value !== 'object' ||
		!('id' in value) ||
		typeof value.id !== 'number' ||
		!Number.isSafeInteger(value.id) ||
		value.id <= 0
	)
		throw new Error('Disposable fixture identifier was not confirmed');
	return value.id;
}

test('member tags and saved filters persist, paginate, apply and report actual usage', async ({
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
		tagName = `Browser config tag ${stamp}`,
		filterName = `Browser config filter ${stamp}`;
	const users: number[] = [],
		tags: number[] = [],
		filters: number[] = [];
	let mainTag: number | undefined, mainFilter: number | undefined;
	const tagsTab = () => page.getByRole('button', { name: /^Tags \(/ });
	const filtersTab = () => page.getByRole('button', { name: /^Saved Filters \(/ });
	try {
		for (const suffix of ['A', 'B']) {
			const response = await page.request.post('/api/admin/users', {
				data: {
					name: `Config browser ${stamp} ${suffix}`,
					email: `config-browser-${stamp}-${suffix}@example.test`,
					password: 'OnlyForDisposableTest123!',
					role: 'user'
				}
			});
			expect(response.status()).toBe(200);
			users.push(responseId(await response.json()));
		}
		await page.goto('/admin/members/segments', { waitUntil: 'networkidle' });
		await tagsTab().click();
		await page.getByRole('button', { name: 'New Tag', exact: true }).click();
		await page.getByLabel('Tag Name', { exact: true }).fill(tagName);
		await page.getByRole('button', { name: 'Select color #3b82f6', exact: true }).click();
		const creatingTag = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === '/api/admin/members/tags' &&
				response.request().method() === 'POST'
		);
		await page.getByRole('button', { name: 'Create Tag', exact: true }).click();
		const createdTag = await creatingTag;
		expect(createdTag.status()).toBe(200);
		mainTag = responseId(await createdTag.json());
		tags.push(mainTag);
		let tagCard = page.locator('.tag-card').filter({ hasText: tagName });
		await expect(tagCard).toContainText('0 members');
		await tagCard.getByTitle('Edit', { exact: true }).click();
		await page.getByLabel('Tag Name', { exact: true }).fill(`${tagName} edited`);
		await page.getByRole('button', { name: 'Save Tag', exact: true }).click();
		tagCard = page.locator('.tag-card').filter({ hasText: `${tagName} edited` });
		await expect(tagCard).toBeVisible();
		for (const id of users)
			expect(
				(
					await page.request.post('/api/admin/members/tags/assign', {
						data: { user_id: id, tag_id: mainTag }
					})
				).status()
			).toBe(200);
		await page.reload({ waitUntil: 'networkidle' });
		await tagsTab().click();
		await expect(tagCard).toContainText('2 members');
		await tagCard.getByRole('button', { name: 'View Members', exact: true }).click();
		await expect(page).toHaveURL(`${baseURL}/admin/members?tag=${mainTag}`);
		await expect(page.locator('tbody tr')).toHaveCount(2);
		await page.reload({ waitUntil: 'networkidle' });
		await expect(page.locator('tbody tr')).toHaveCount(2);
		await page.goto('/admin/members/segments', { waitUntil: 'networkidle' });
		await page.getByRole('button', { name: 'Save Filter', exact: true }).click();
		const modal = page.getByRole('dialog');
		await modal.getByLabel('Filter Name', { exact: true }).fill(filterName);
		await modal.getByLabel('Filter settings (JSON)', { exact: true }).fill('{');
		let filterWrites = 0;
		page.on('request', (request) => {
			if (
				new URL(request.url()).pathname === '/api/admin/members/filters' &&
				request.method() === 'POST'
			)
				filterWrites++;
		});
		await modal.getByRole('button', { name: 'Save Filter', exact: true }).click();
		await expect(modal.getByRole('alert')).toBeVisible();
		expect(filterWrites).toBe(0);
		await expect(modal.getByRole('button', { name: 'Save Filter', exact: true })).toBeEnabled();
		await modal
			.getByLabel('Filter settings (JSON)', { exact: true })
			.fill(
				JSON.stringify({ tag: mainTag, search: `Config browser ${stamp}`, spending_tier: 'low' })
			);
		const creatingFilter = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === '/api/admin/members/filters' &&
				response.request().method() === 'POST'
		);
		await modal.getByRole('button', { name: 'Save Filter', exact: true }).click();
		const createdFilter = await creatingFilter;
		expect(createdFilter.status()).toBe(200);
		mainFilter = responseId(await createdFilter.json());
		filters.push(mainFilter);
		await filtersTab().click();
		let filterCard = page
			.locator('.filter-card')
			.filter({ has: page.getByRole('heading', { name: filterName, exact: true }) });
		await expect(filterCard).toContainText('Recorded applies: 0');
		await filterCard.getByTitle('Edit Filter', { exact: true }).click();
		await modal
			.getByLabel('Filter settings (JSON)', { exact: true })
			.fill(
				JSON.stringify({ tag: mainTag, search: `Config browser ${stamp} A`, spending_tier: 'low' })
			);
		await modal.getByRole('button', { name: 'Save Filter', exact: true }).click();
		await expect(filterCard).toBeVisible();
		const duplicating = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === `/api/admin/members/filters/${mainFilter}/duplicate`
		);
		await filterCard.getByTitle('Duplicate', { exact: true }).click();
		const duplicated = await duplicating;
		expect(duplicated.status()).toBe(200);
		filters.push(responseId(await duplicated.json()));
		await expect(
			page
				.locator('.filter-card')
				.filter({ has: page.getByRole('heading', { name: `${filterName} (Copy)`, exact: true }) })
		).toContainText('Recorded applies: 0');
		await filterCard.getByRole('button', { name: 'Apply Filter', exact: true }).click();
		await expect(page).toHaveURL(/\/admin\/members\?/);
		await expect(page.locator('tbody tr')).toHaveCount(1);
		await expect(page.locator('tbody')).toContainText(`config-browser-${stamp}-A@example.test`);
		await page.reload({ waitUntil: 'networkidle' });
		await expect(page.locator('tbody tr')).toHaveCount(1);
		await page.goto('/admin/members/segments', { waitUntil: 'networkidle' });
		await filtersTab().click();
		await expect(filterCard).toContainText('Recorded applies: 1');
		for (let index = 0; index < 21; index++) {
			const tag = await page.request.post('/api/admin/members/tags', {
				data: { name: `Pager tag ${stamp} ${String(index).padStart(2, '0')}`, color: '#123456' }
			});
			expect(tag.status()).toBe(200);
			tags.push(responseId(await tag.json()));
			const filter = await page.request.post('/api/admin/members/filters', {
				data: {
					name: `Pager filter ${stamp} ${String(index).padStart(2, '0')}`,
					filters: { search: `nonmatching-${stamp}` }
				}
			});
			expect(filter.status()).toBe(200);
			filters.push(responseId(await filter.json()));
		}
		await tagsTab().click();
		await page.getByRole('searchbox').fill(`Pager tag ${stamp}`);
		await page.getByRole('button', { name: 'Search', exact: true }).click();
		await expect(page.locator('.tag-card')).toHaveCount(20);
		let pagination = page.getByRole('navigation', { name: 'Member configuration pages' });
		await expect(pagination).toContainText('Page 1 · 21 records');
		await pagination.getByRole('button', { name: 'Next', exact: true }).click();
		await expect(page.locator('.tag-card')).toHaveCount(1);
		await expect(pagination).toContainText('Page 2 · 21 records');
		await filtersTab().click();
		await page.getByRole('searchbox').fill(`Pager filter ${stamp}`);
		await page.getByRole('button', { name: 'Search', exact: true }).click();
		await expect(page.locator('.filter-card')).toHaveCount(20);
		await pagination.getByRole('button', { name: 'Next', exact: true }).click();
		await expect(page.locator('.filter-card')).toHaveCount(1);
		await expect(pagination).toContainText('Page 2 · 21 records');
		await page.getByRole('searchbox').fill(filterName);
		await page.getByRole('button', { name: 'Search', exact: true }).click();
		await expect(filterCard).toBeVisible();
		await filterCard.getByTitle('Delete', { exact: true }).click();
		await page.getByRole('button', { name: 'Delete Filter', exact: true }).click();
		await expect(filterCard).toHaveCount(0);
		filters.splice(filters.indexOf(mainFilter), 1);
		mainFilter = undefined;
		await tagsTab().click();
		await page.getByRole('searchbox').fill(`${tagName} edited`);
		await page.getByRole('button', { name: 'Search', exact: true }).click();
		await expect(tagCard).toBeVisible();
		await tagCard.getByTitle('Delete', { exact: true }).click();
		await page.getByRole('button', { name: 'Delete Tag', exact: true }).click();
		await expect(tagCard).toHaveCount(0);
		tags.splice(tags.indexOf(mainTag), 1);
		mainTag = undefined;
		await page.reload({ waitUntil: 'networkidle' });
		await tagsTab().click();
		await expect(tagCard).toHaveCount(0);
		expect(errors).toEqual([]);
	} finally {
		for (const id of filters)
			expect
				.soft((await page.request.delete(`/api/admin/members/filters/${id}`)).status())
				.toBe(200);
		for (const id of tags)
			expect.soft((await page.request.delete(`/api/admin/members/tags/${id}`)).status()).toBe(200);
		for (const id of users)
			expect.soft((await page.request.delete(`/api/admin/users/${id}`)).status()).toBe(200);
	}
});

test('member configuration proxies reject anonymous and non-admin access', async ({ baseURL }) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	const anonymous = await request.newContext({ baseURL }),
		user = await request.newContext({ baseURL });
	try {
		expect(
			(
				await user.post('/api/auth/login', {
					data: { email: 'user@admin-completion.test', password: 'OnlyForDisposableTest123!' }
				})
			).status()
		).toBe(200);
		for (const [context, status] of [
			[anonymous, 401],
			[user, 403]
		] as const) {
			for (const path of [
				'/api/admin/members/tags',
				'/api/admin/members/filters',
				'/api/admin/members/segments'
			])
				expect((await context.get(path)).status()).toBe(status);
			for (const path of [
				'/api/admin/members/tags/assign',
				'/api/admin/members/filters/999999/apply',
				'/api/admin/members/filters/999999/duplicate'
			])
				expect(
					(await context.post(path, { data: { user_id: 999999, tag_id: 999999 } })).status()
				).toBe(status);
			for (const path of [
				'/api/admin/members/tags/999999',
				'/api/admin/members/filters/999999',
				'/api/admin/members/segments/999999'
			])
				expect((await context.delete(path)).status()).toBe(status);
			expect((await context.get('/admin/members/segments', { maxRedirects: 0 })).status()).toBe(
				303
			);
		}
		expect((await user.post('/api/auth/logout')).status()).toBe(200);
	} finally {
		await anonymous.dispose();
		await user.dispose();
	}
});
