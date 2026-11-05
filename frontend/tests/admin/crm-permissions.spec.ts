import {
	expect,
	test,
	type APIRequestContext,
	type BrowserContext,
	type Page
} from '@playwright/test';
const prefix = '/api/admin/crm',
	password = 'OnlyForDisposableTest123!',
	member = 'user@admin-completion.test';
function object(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Invalid actual CRM fixture response');
	return value as Record<string, unknown>;
}
function identity(value: unknown): string {
	const row = object(value),
		valueId = row.id;
	const result =
		typeof valueId === 'string'
			? valueId
			: typeof valueId === 'number' && Number.isSafeInteger(valueId)
				? valueId.toString()
				: '';
	if (!/^[1-9]\d*$/.test(result)) throw new Error('Missing saved fixture identity');
	return result;
}
async function login(context: BrowserContext, email: string) {
	const response = await context.request.post('/api/auth/login', { data: { email, password } });
	expect(response.status(), await response.text()).toBe(200);
}
async function saved(client: APIRequestContext, path: string) {
	const response = await client.get(prefix + path);
	expect(response.status(), await response.text()).toBe(200);
	return object(await response.json());
}
async function capabilities(client: APIRequestContext) {
	const response = await client.get(prefix + '/permissions');
	expect(response.status(), await response.text()).toBe(200);
	expect(response.headers()['cache-control']).toBe('private, no-store');
	expect(response.headers().vary).toContain('Authorization');
	expect(response.headers().vary).toContain('Cookie');
	return object(await response.json());
}
async function diagnostics(page: Page, expected: Set<string>) {
	const errors: string[] = [],
		blocked = new Set<string>(),
		requests: string[] = [];
	page.on('pageerror', (error) => errors.push(error.message));
	page.on('request', (request) => {
		if (new URL(request.url()).pathname.startsWith('/api/'))
			requests.push(new URL(request.url()).pathname);
	});
	page.on('console', (message) => {
		if (!['error', 'warning'].includes(message.type())) return;
		const source = message.location().url,
			status = message
				.text()
				.match(/^Failed to load resource: the server responded with a status of (\d+)/)?.[1];
		if (status && source && expected.has(`${status} ${new URL(source).pathname}`)) return;
		if (blocked.has(source) && message.text() === 'Failed to load resource: net::ERR_FAILED')
			return;
		errors.push(`${message.text()} ${source}`);
	});
	page.on('response', (response) => {
		const path = new URL(response.url()).pathname;
		if (
			path.startsWith('/api/') &&
			response.status() >= 400 &&
			!expected.has(`${response.status()} ${path}`)
		)
			errors.push(`${response.status()} ${path}`);
	});
	await page.route('**/*', (route) => {
		const url = new URL(route.request().url());
		if (
			['localhost', '127.0.0.1'].includes(url.hostname) ||
			!['http:', 'https:'].includes(url.protocol)
		)
			return route.continue();
		blocked.add(route.request().url());
		return route.abort();
	});
	return { errors, requests };
}
const contactGrants = [
	{ module: 'contacts', action: 'manage', allowed: true },
	{ module: 'deals', action: 'view', allowed: true }
];
async function ownRole(client: APIRequestContext, name: string) {
	const response = await client.post(prefix + '/manager-roles', {
		data: { name, permissions: contactGrants }
	});
	expect(response.status(), await response.text()).toBe(200);
	return object(await response.json());
}
async function ownAssignment(client: APIRequestContext, user: string, role: string) {
	const response = await client.post(prefix + '/managers', {
		data: { user_id: user, role_id: role }
	});
	expect(response.status(), await response.text()).toBe(200);
	return object(await response.json());
}
async function cleanup(
	client: APIRequestContext,
	role: string | undefined,
	manager: string | undefined,
	contact: string | undefined,
	priorDefault: string | undefined
) {
	const failures: string[] = [];
	async function remove(path: string) {
		const response = await client.delete(prefix + path);
		if (response.status() !== 200)
			failures.push(`${path}: ${response.status()} ${await response.text()}`);
	}
	if (manager) {
		const response = await client.get(prefix + '/managers/' + manager);
		if (response.ok()) {
			const row = object(await response.json());
			await remove(`/managers/${manager}?expected_version=${row.version}`);
		} else if (response.status() !== 404)
			failures.push('Unable to read owned assignment for cleanup');
	}
	if (contact) await remove('/contacts/' + contact);
	if (priorDefault) {
		const row = await saved(client, '/manager-roles/' + priorDefault),
			response = await client.put(prefix + '/manager-roles/' + priorDefault, {
				data: { is_default: true, expected_version: row.version }
			});
		if (response.status() !== 200) failures.push('Unable to restore prior default selection');
	}
	if (role) {
		const row = await saved(client, '/manager-roles/' + role);
		if (row.is_default === true) failures.push('Owned role remains the current default');
		else await remove(`/manager-roles/${role}?expected_version=${row.version}`);
	}

	expect(failures).toEqual([]);
}

test('global role/default/CAS and assignment controls persist without changing the delegated account global role', async ({
	page,
	context,
	browser,
	baseURL
}) => {
	test.setTimeout(120000);
	expect(baseURL).toBe('http://127.0.0.1:5274');
	await login(context, 'admin@admin-completion.test');
	const memberContext = await browser.newContext({ baseURL }),
		memberPage = await memberContext.newPage();
	await login(memberContext, member);
	const initial = await capabilities(memberContext.request);
	expect(initial).toMatchObject({ global_admin: false, manager_id: null, permissions: [] });
	const userId = initial.user_id;
	if (typeof userId !== 'string') throw new Error('Missing current fixture actor');
	const expected = new Set<string>(),
		adminDiag = await diagnostics(page, expected),
		memberDiag = await diagnostics(memberPage, new Set());
	const stamp = Date.now(),
		name = `CRM browser role ${stamp}`,
		renamed = name + ' revised';
	let role: string | undefined,
		manager: string | undefined,
		contact: string | undefined,
		priorDefault: string | undefined;
	try {
		const roles = await saved(context.request, '/manager-roles?per_page=100');
		if (!Array.isArray(roles.data)) throw new Error('Missing role catalog');
		const prior = roles.data.map(object).find((row) => row.is_default === true);
		priorDefault = prior ? identity(prior) : undefined;
		await page.goto('/admin/crm/managers/roles/new', { waitUntil: 'networkidle' });
		await page.getByLabel('Role name', { exact: true }).fill(name);
		await page
			.getByRole('group', { name: 'Contacts and leads', exact: true })
			.getByRole('checkbox', { name: 'manage', exact: true })
			.check();
		await page
			.getByRole('group', { name: 'Deals and deal activities', exact: true })
			.getByRole('checkbox', { name: 'view', exact: true })
			.check();
		await page.getByLabel('Default selection', { exact: true }).check();
		const created = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === prefix + '/manager-roles' &&
				response.request().method() === 'POST'
		);
		await page.getByRole('button', { name: 'Save CRM role', exact: true }).click();
		const createdResponse = await created;
		expect(createdResponse.status(), await createdResponse.text()).toBe(200);
		role = identity(await createdResponse.json());
		await expect(page).toHaveURL(/\/admin\/crm\/managers\?tab=roles$/);
		expect(await capabilities(memberContext.request)).toMatchObject({
			manager_id: null,
			permissions: []
		});
		await page.goto(`/admin/crm/managers/roles/${role}/edit`, { waitUntil: 'networkidle' });
		await expect(page.getByLabel('Role name', { exact: true })).toHaveValue(name);
		await expect(page.getByLabel('Default selection', { exact: true })).toBeChecked();
		const first = await saved(context.request, '/manager-roles/' + role);
		expect(
			(
				await context.request.put(prefix + '/manager-roles/' + role, {
					data: { description: 'Actual concurrent saved revision', expected_version: first.version }
				})
			).status()
		).toBe(200);
		expected.add(`409 ${prefix}/manager-roles/${role}`);
		await page.getByLabel('Role name', { exact: true }).fill(renamed);
		await page.getByRole('button', { name: 'Save CRM role', exact: true }).click();
		await expect(page.getByRole('alert')).toContainText('reload');
		expect((await saved(context.request, '/manager-roles/' + role)).name).toBe(name);
		await page.getByRole('button', { name: 'Reload saved role', exact: true }).click();
		await expect(page.getByLabel('Description', { exact: true })).toHaveValue(
			'Actual concurrent saved revision'
		);
		await page.getByLabel('Role name', { exact: true }).fill(renamed);
		await page.getByRole('button', { name: 'Save CRM role', exact: true }).click();
		await expect(page).toHaveURL(/\/admin\/crm\/managers\?tab=roles$/);
		expect((await saved(context.request, '/manager-roles/' + role)).name).toBe(renamed);
		const row = page.getByRole('row').filter({ hasText: renamed });
		await row.getByRole('button', { name: 'Delete role', exact: true }).click();
		await page
			.getByRole('dialog', { name: 'Delete CRM role', exact: true })
			.getByRole('button', { name: 'Confirm security change', exact: true })
			.click();
		await expect(page.getByRole('alert')).toContainText(/default/i);
		expect((await saved(context.request, '/manager-roles/' + role)).is_default).toBe(true);
		await page.goto('/admin/crm/managers/new', { waitUntil: 'networkidle' });
		await page.getByLabel('Search active users', { exact: true }).fill(member);
		await page.getByRole('button', { name: 'Search users', exact: true }).click();
		await expect(
			page
				.getByRole('combobox', { name: 'Active user', exact: true })
				.locator(`option[value="${userId}"]`)
				.filter({ hasText: member })
		).toHaveCount(1);
		await page.getByRole('combobox', { name: 'Active user', exact: true }).selectOption(userId);
		await page.getByLabel('Search CRM roles', { exact: true }).fill(renamed);
		await page.getByRole('button', { name: 'Search roles', exact: true }).click();
		await page.getByRole('combobox', { name: 'CRM role', exact: true }).selectOption(role);
		const assigned = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === prefix + '/managers' &&
				response.request().method() === 'POST'
		);
		await page.getByRole('button', { name: 'Save CRM assignment', exact: true }).click();
		const assignedResponse = await assigned;
		expect(assignedResponse.status(), await assignedResponse.text()).toBe(200);
		manager = identity(await assignedResponse.json());
		await expect(page).toHaveURL(/\/admin\/crm\/managers$/);
		await page.reload({ waitUntil: 'networkidle' });
		await expect(page.getByRole('row').filter({ hasText: member })).toContainText(renamed);
		const delegated = await capabilities(memberContext.request);
		expect(delegated).toMatchObject({ global_admin: false, manager_id: manager, role_id: role });
		expect(object(await (await memberContext.request.get('/api/auth/me')).json()).role).toBe(
			'user'
		);
		await page.goto(`/admin/crm/managers/${manager}/edit`, { waitUntil: 'networkidle' });
		const assignmentBefore = await saved(context.request, '/managers/' + manager);
		expect(
			(
				await context.request.put(prefix + '/managers/' + manager, {
					data: { role_id: role, expected_version: assignmentBefore.version }
				})
			).status()
		).toBe(200);
		expected.add(`409 ${prefix}/managers/${manager}`);
		await page.getByRole('button', { name: 'Save CRM assignment', exact: true }).click();
		await expect(page.getByRole('alert')).toContainText('reload');
		await page.getByRole('button', { name: 'Reload saved assignment', exact: true }).click();
		await expect(page.getByText(/Assignment\s+version\s+2/)).toBeVisible();
		await page.getByRole('button', { name: 'Save CRM assignment', exact: true }).click();
		await expect(page).toHaveURL(/\/admin\/crm\/managers$/);
		expect((await saved(context.request, '/managers/' + manager)).version).toBe(3);
		const savedContact = await context.request.post(prefix + '/contacts', {
			data: {
				email: `crm-permission-${stamp}@example.test`,
				first_name: 'Permission',
				last_name: 'Fixture'
			}
		});
		expect(savedContact.status()).toBe(200);
		contact = identity(await savedContact.json());
		await memberPage.goto('/admin/contacts', { waitUntil: 'networkidle' });
		await expect(
			memberPage.locator('.directory').getByRole('heading', { name: 'Contacts', exact: true })
		).toBeVisible();
		await expect(memberPage.locator('aside a[href="/admin/members"]')).toHaveCount(0);
		await expect(memberPage.locator('aside a[href="/admin/email/sequences"]')).toHaveCount(0);
		await memberPage.goto(`/admin/crm/contacts/${contact}`, { waitUntil: 'networkidle' });
		await memberPage.getByRole('button', { name: 'Send Email', exact: true }).click();
		const dialog = memberPage.getByRole('dialog', { name: 'Send Email', exact: true });
		await expect(dialog.getByLabel('Subject Line', { exact: false })).toBeEditable();
		await expect(dialog.getByText(/Native template picker access is not permitted/)).toBeVisible();
		await dialog.getByRole('button', { name: 'Cancel', exact: true }).click();
		await expect(memberPage.getByRole('button', { name: 'Delete', exact: true })).toBeDisabled();
		expect(
			memberDiag.requests.some(
				(path) =>
					path.startsWith('/api/admin/email') || path === prefix + '/contacts/native-templates'
			)
		).toBe(false);
		expect((await memberContext.request.get('/api/admin/email/templates')).status()).toBe(403);
		expect((await memberContext.request.get(prefix + '/managers/users')).status()).toBe(403);
		expect((await memberContext.request.get(prefix + '/deals')).status()).toBe(403);
		expect(
			(
				await memberContext.request.post(prefix + '/manager-roles', {
					data: { name: 'Never persisted', permissions: [] }
				})
			).status()
		).toBe(403);
		await page.goto('/admin/crm/managers', { waitUntil: 'networkidle' });
		await page
			.getByRole('row')
			.filter({ hasText: member })
			.getByRole('button', { name: 'Remove assignment', exact: true })
			.click();
		await page
			.getByRole('dialog', { name: 'Remove CRM assignment', exact: true })
			.getByRole('button', { name: 'Confirm security change', exact: true })
			.click();
		await expect(
			page.getByRole('status').filter({ hasText: 'CRM assignment removed' })
		).toContainText('CRM assignment removed');
		manager = undefined;
		expect(await capabilities(memberContext.request)).toMatchObject({
			manager_id: null,
			permissions: []
		});
		expect(adminDiag.errors).toEqual([]);
		expect(memberDiag.errors).toEqual([]);
	} finally {
		await cleanup(context.request, role, manager, contact, priorDefault);
		await memberContext.close();
	}
});

test('revocation and503 or malformed capabilities close cached panels until a verified refresh recovers', async ({
	context,
	browser,
	baseURL
}) => {
	test.setTimeout(90000);
	expect(baseURL).toBe('http://127.0.0.1:5274');
	await login(context, 'admin@admin-completion.test');
	const actor = await browser.newContext({ baseURL }),
		actorPage = await actor.newPage();
	await login(actor, member);
	const initial = await capabilities(actor.request);
	expect(initial).toMatchObject({ manager_id: null, permissions: [] });
	if (typeof initial.user_id !== 'string') throw new Error('Missing actor');
	let role: string | undefined, manager: string | undefined;
	const expected = new Set([`403 ${prefix}/contacts`, `503 ${prefix}/permissions`]),
		diag = await diagnostics(actorPage, expected);
	try {
		role = identity(await ownRole(context.request, `CRM revocation browser ${Date.now()}`));
		manager = identity(await ownAssignment(context.request, initial.user_id, role));
		await actorPage.goto('/admin/contacts', { waitUntil: 'networkidle' });
		await expect(
			actorPage.locator('.directory').getByRole('heading', { name: 'Contacts', exact: true })
		).toBeVisible();
		const current = await saved(context.request, '/manager-roles/' + role);
		expect(
			(
				await context.request.put(prefix + '/manager-roles/' + role, {
					data: { permissions: [], expected_version: current.version }
				})
			).status()
		).toBe(200);
		let mode: 'unavailable' | 'malformed' = 'unavailable',
			capCalls = 0;
		await actorPage.route('**/api/admin/crm/permissions', (route) => {
			capCalls++;
			return route.fulfill({
				status: mode === 'unavailable' ? 503 : 200,
				contentType: 'application/json',
				body: JSON.stringify(
					mode === 'unavailable'
						? { error: 'Owned capabilities outage' }
						: { user_id: initial.user_id, global_admin: true }
				)
			});
		});
		await actorPage.getByRole('button', { name: 'Refresh', exact: true }).click();
		await expect(
			actorPage.getByRole('button', { name: 'Refresh CRM permissions', exact: true })
		).toBeVisible();
		await expect(
			actorPage.locator('.directory').getByRole('heading', { name: 'Contacts', exact: true })
		).toHaveCount(0);
		expect(capCalls).toBe(1);
		const revoked = await saved(context.request, '/manager-roles/' + role);
		expect(
			(
				await context.request.put(prefix + '/manager-roles/' + role, {
					data: { permissions: contactGrants, expected_version: revoked.version }
				})
			).status()
		).toBe(200);
		mode = 'malformed';
		await actorPage.getByRole('button', { name: 'Refresh CRM permissions', exact: true }).click();
		await expect.poll(() => capCalls).toBe(2);
		await expect(
			actorPage.locator('.directory').getByRole('heading', { name: 'Contacts', exact: true })
		).toHaveCount(0);
		await actorPage.unroute('**/api/admin/crm/permissions');
		await actorPage.getByRole('button', { name: 'Refresh CRM permissions', exact: true }).click();
		await expect(
			actorPage.locator('.directory').getByRole('heading', { name: 'Contacts', exact: true })
		).toBeVisible();
		await expect(
			actorPage.getByRole('button', { name: 'Refresh CRM permissions', exact: true })
		).toHaveCount(0);
		expect(diag.errors).toEqual([]);
	} finally {
		await cleanup(context.request, role, manager, undefined, undefined);
		await actor.close();
	}
});

test('anonymous and unassigned identities retain401/403 and private noindex HTML without global expansion', async ({
	page,
	context,
	baseURL
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	const anonymous = await context.request.get(prefix + '/permissions');
	expect(anonymous.status()).toBe(401);
	expect((await context.request.get(prefix + '/contacts')).status()).toBe(401);
	await page.goto('/admin/crm/managers');
	await expect(page).toHaveURL(/\/login\?/);
	await login(context, member);
	expect(await capabilities(context.request)).toMatchObject({
		global_admin: false,
		manager_id: null,
		role_id: null,
		permissions: []
	});
	for (const path of [
		prefix + '/contacts',
		prefix + '/managers',
		'/api/admin/email/templates',
		'/api/admin/members'
	])
		expect((await context.request.get(path)).status(), path).toBe(403);
	const response = await page.goto('/admin/contacts');
	expect(response?.status()).toBe(403);
	expect(response?.headers()['cache-control']).toContain('no-store');
	expect(response?.headers()['x-robots-tag']).toContain('noindex');
	await expect(page.locator('.admin-sidebar')).toHaveCount(0);
});
