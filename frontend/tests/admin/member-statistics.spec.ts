import { expect, request as playwrightRequest, test, type Page } from '@playwright/test';
import { readFile, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { parseMemberStats } from '../../src/lib/api/member-statistics';
const password = 'OnlyForDisposableTest123!',
	prefix = `member-stats-browser-${Date.now()}`;
let cookies: Awaited<
	ReturnType<Awaited<ReturnType<typeof playwrightRequest.newContext>>['storageState']>
>['cookies'];
const users: number[] = [],
	plans: number[] = [];
let targetPlan = 0;
function object(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Invalid fixture receipt');
	return value as Record<string, unknown>;
}
function identity(value: unknown): number {
	const result = Number(value);
	expect(Number.isSafeInteger(result) && result > 0).toBe(true);
	return result;
}
test.describe.configure({ mode: 'serial' });
test.beforeAll(async ({ baseURL }) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	const api = await playwrightRequest.newContext({ baseURL });
	try {
		const login = await api.post('/api/auth/login', {
			data: { email: 'admin@admin-completion.test', password }
		});
		expect(login.status(), await login.text()).toBe(200);
		cookies = (await api.storageState()).cookies;
		for (let n = 0; n < 3; n++) {
			const created = await api.post('/api/admin/member-management', {
				data: {
					name: `${prefix} member ${n}`,
					email: `${prefix}-${n}@example.test`,
					password,
					role: 'user',
					send_welcome_email: false
				}
			});
			expect(created.status(), await created.text()).toBe(200);
			const v = object(await created.json());
			users.push(identity(object(v.member || v.data || v).id));
		}
		for (let n = 0; n < 62; n++) {
			const created = await api.post('/api/admin/membership-plans', {
				data: {
					name: `${prefix} ${String(n).padStart(3, '0')}`,
					price_cents: 12000,
					billing_cycle: 'annual',
					is_active: n !== 61,
					trial_days: 0
				}
			});
			expect(created.status(), await created.text()).toBe(200);
			plans.push(identity(object(object(await created.json()).data).id));
		}
		targetPlan = plans[61];
		const past = new Date(Date.now() - 86_400_000 * 10).toISOString().slice(0, -1),
			future = new Date(Date.now() + 86_400_000 * 10).toISOString().slice(0, -1),
			expired = new Date(Date.now() - 86_400_000).toISOString().slice(0, -1);
		for (const [user, plan, end] of [
			[users[0], targetPlan, future],
			[users[1], targetPlan, future],
			[users[2], targetPlan, expired],
			[users[2], plans[0], future]
		] as const) {
			const granted = await api.post('/api/admin/user-memberships', {
				data: {
					user_id: user,
					plan_id: plan,
					status: user === users[2] && plan === targetPlan ? 'expired' : 'active',
					starts_at: past,
					expires_at: end
				}
			});
			expect(granted.status(), await granted.text()).toBe(200);
			const row = object(object(await granted.json()).membership);
			expect(identity(row.user_id)).toBe(user);
			expect(identity(row.plan_id)).toBe(plan);
		}
	} finally {
		await api.dispose();
	}
});
test.afterAll(async ({ baseURL }) => {
	if (!cookies) return;
	const api = await playwrightRequest.newContext({
		baseURL,
		storageState: { cookies, origins: [] }
	});
	try {
		for (const id of users)
			expect((await api.delete(`/api/admin/member-management/${id}`)).status()).toBe(200);
		for (const id of plans)
			expect((await api.delete(`/api/admin/membership-plans/${id}`)).status()).toBe(200);
	} finally {
		await api.dispose();
	}
});
interface BrowserDiagnostics {
	errors: string[];
	warnings: string[];
	failed: string[];
	responses: { url: string; method: string; status: number }[];
	blocked: Set<string>;
}
const diagnostics = new WeakMap<Page, BrowserDiagnostics>();
test.beforeEach(async ({ context, page }) => {
	await context.addCookies(cookies);
	const record: BrowserDiagnostics = {
		errors: [],
		warnings: [],
		failed: [],
		responses: [],
		blocked: new Set()
	};
	diagnostics.set(page, record);
	page.on('pageerror', (cause) => record.errors.push(cause.message));
	page.on('console', (message) => {
		const url = message.location().url;
		if (record.blocked.has(url) && message.text() === 'Failed to load resource: net::ERR_FAILED')
			return;
		if (message.type() === 'error') record.errors.push(`${message.text()} ${url}`);
		if (message.type() === 'warning') record.warnings.push(`${message.text()} ${url}`);
	});
	page.on('requestfailed', (request) => {
		if (!record.blocked.has(request.url()) && !request.failure()?.errorText.includes('ERR_ABORTED'))
			record.failed.push(`${request.failure()?.errorText} ${request.url()}`);
	});
	page.on('response', (response) =>
		record.responses.push({
			url: response.url(),
			method: response.request().method(),
			status: response.status()
		})
	);
	await page.route('**/*', (route) => {
		const url = new URL(route.request().url());
		if (
			['127.0.0.1', 'localhost'].includes(url.hostname) ||
			!['http:', 'https:'].includes(url.protocol)
		)
			return route.continue();
		record.blocked.add(route.request().url());
		return route.abort();
	});
});
test.afterEach(async ({ page }, info) => {
	const record = diagnostics.get(page);
	if (!record) return;
	await writeFile(
		info.outputPath('network-console.json'),
		JSON.stringify({ ...record, blocked: [...record.blocked] }, null, 2)
	);
	const expectedFailure = info.title.startsWith('malformed success');
	const unexpected = record.errors.filter(
		(message) =>
			!(
				expectedFailure &&
				message.startsWith('Failed to load resource:') &&
				(message.includes('/api/admin/members/services?') ||
					message.includes('/api/admin/members/service/2147483647'))
			)
	);
	expect(unexpected).toEqual([]);
	expect(record.failed).toEqual([]);
});
test('current facts use correct units and real plan identities; searched catalog preserves off-page selection and filtered export', async ({
	page
}, info) => {
	const statsReady = page.waitForResponse(
		(r) => new URL(r.url()).pathname === '/api/admin/members/stats'
	);
	await page.goto(`/admin/members?search=${prefix}&plan_id=${targetPlan}`);
	const response = await statsReady;
	expect(response.status()).toBe(200);
	const stats = parseMemberStats(await response.json());
	await expect(
		page
			.locator('.stats-grid .stat-card')
			.filter({ hasText: 'Total Members' })
			.locator('.stat-value')
	).toHaveText(stats.overview.total_members.toLocaleString());
	const usd = new Intl.NumberFormat('en-US', {
		style: 'currency',
		currency: 'USD',
		maximumFractionDigits: 2,
		minimumFractionDigits: 2
	}).format(stats.revenue.recorded_net_usd_order_value.net_cents / 100);
	await expect(
		page
			.locator('.stats-grid .stat-card')
			.filter({ hasText: 'Recorded net USD order value' })
			.locator('.stat-value')
	).toHaveText(usd);
	await page.getByText('Measurement coverage and catalogue estimate', { exact: true }).click();
	await expect(
		page.getByText('Monthly recurring revenue: unmeasured.', { exact: false })
	).toBeVisible();
	await expect(
		page.getByText('Average lifetime value: unmeasured.', { exact: false })
	).toBeVisible();
	await page.getByRole('button', { name: 'Toggle filters', exact: true }).click();
	const selection = page.getByLabel('Service (membership plan)', { exact: true });
	await expect(selection).toHaveValue(String(targetPlan));
	await page.getByRole('searchbox', { name: 'Search membership plans' }).fill(prefix);
	await page.getByRole('searchbox', { name: 'Search membership plans' }).press('Tab');
	await expect(page.getByText('Plan page 1 of 3', { exact: false })).toBeVisible();
	await expect(selection).toHaveValue(String(targetPlan));
	await expect(selection.locator('option', { hasText: `${prefix} 061 (inactive)` })).toHaveCount(1);
	await page.getByRole('button', { name: 'Next plans', exact: true }).click();
	await expect(page.getByText('Plan page 2 of 3', { exact: false })).toBeVisible();
	await expect(selection).toHaveValue(String(targetPlan));
	await expect(page.locator('.members-table tbody tr')).toHaveCount(3);
	const exported = page.waitForResponse(
		(r) => new URL(r.url()).pathname === '/api/admin/member-management/export'
	);
	const downloaded = page.waitForEvent('download');
	await page
		.locator('.page-header')
		.getByRole('button', { name: 'Export CSV', exact: true })
		.click();
	const receipt = await exported;
	expect(receipt.status()).toBe(200);
	expect(new URL(receipt.url()).searchParams.get('plan_id')).toBe(String(targetPlan));
	expect(new URL(receipt.url()).searchParams.has('product_id')).toBe(false);
	expect(receipt.headers()['x-export-row-count']).toBe('3');
	const file = info.outputPath('plan-filtered-members.csv');
	await (await downloaded).saveAs(file);
	const bytes = await readFile(file);
	expect(bytes.length).toBeGreaterThan(0);
	expect(createHash('sha256').update(bytes).digest('hex')).toBe(
		receipt.headers()['x-export-sha256']
	);
	const csv = bytes.toString('utf8');
	for (let n = 0; n < 3; n++) expect(csv).toContain(`${prefix}-${n}@example.test`);
	// Refresh must fetch actual changed rows twice inside the old one-minute cache window.
	const created = await page.request.post('/api/admin/member-management', {
		data: {
			name: `${prefix} transient`,
			email: `${prefix}-transient@example.test`,
			password,
			role: 'user',
			send_welcome_email: false
		}
	});
	expect(created.status(), await created.text()).toBe(200);
	const createdBody = object(await created.json()),
		transient = identity(object(createdBody.member || createdBody.data || createdBody).id);
	users.push(transient);
	try {
		const granted = await page.request.post('/api/admin/user-memberships', {
			data: {
				user_id: transient,
				plan_id: targetPlan,
				status: 'active',
				starts_at: new Date(Date.now() - 60_000).toISOString().slice(0, -1),
				expires_at: new Date(Date.now() + 86_400_000).toISOString().slice(0, -1)
			}
		});
		expect(granted.status(), await granted.text()).toBe(200);
		const refreshed = page.waitForResponse(
			(response) => new URL(response.url()).pathname === '/api/admin/members'
		);
		await page
			.locator('.page-header')
			.getByRole('button', { name: 'Refresh', exact: true })
			.click();
		expect((await refreshed).status()).toBe(200);
		await expect(page.locator('.members-table tbody tr')).toHaveCount(4);
	} finally {
		expect((await page.request.delete(`/api/admin/member-management/${transient}`)).status()).toBe(
			200
		);
		users.splice(users.indexOf(transient), 1);
	}
	const refreshedAfterDelete = page.waitForResponse(
		(response) => new URL(response.url()).pathname === '/api/admin/members'
	);
	await page.locator('.page-header').getByRole('button', { name: 'Refresh', exact: true }).click();
	expect((await refreshedAfterDelete).status()).toBe(200);
	await expect(page.locator('.members-table tbody tr')).toHaveCount(3);
	await page.reload();
	await page.getByRole('button', { name: 'Toggle filters', exact: true }).click();
	await expect(selection).toHaveValue(String(targetPlan));
	await expect(page.locator('.members-table tbody tr')).toHaveCount(3);
});
test('plan-local status, search and page size persist through paging and reload independently from global membership', async ({
	page
}) => {
	await page.goto(
		`/admin/members/service/${targetPlan}?search=${prefix}&status=active&per_page=1&page=1`
	);
	await expect(
		page
			.locator('.stats-grid .stat-card')
			.filter({ hasText: 'Total Members' })
			.locator('.stat-value')
	).toHaveText('2');
	await expect(page.locator('.members-table tbody tr')).toHaveCount(1);
	const first = await page.locator('.member-email').textContent();
	await page.locator('.pagination-controls button').last().click();
	await expect(page).toHaveURL(new RegExp('page=2'));
	expect(new URL(page.url()).searchParams.get('status')).toBe('active');
	expect(new URL(page.url()).searchParams.get('search')).toBe(prefix);
	expect(new URL(page.url()).searchParams.get('per_page')).toBe('1');
	await expect(page.locator('.member-email')).not.toHaveText(first || '');
	const second = await page.locator('.member-email').textContent();
	await page.reload();
	await expect(page.locator('.member-email')).toHaveText(second || '');
	await page.getByRole('combobox').first().selectOption('expired');
	await expect(
		page
			.locator('.stats-grid .stat-card')
			.filter({ hasText: 'Total Members' })
			.locator('.stat-value')
	).toHaveText('1');
	await expect(page.getByText(`${prefix}-2@example.test`, { exact: true })).toBeVisible();
	await expect(page.locator('.status-badge')).toContainText('expired (plan)');
	await expect(page.locator('.status-badge')).toContainText('Active (global)');
	expect(new URL(page.url()).searchParams.get('page')).toBe('1');
	await expect(page.getByText('Whole-member net USD order value', { exact: true })).toBeVisible();
	await expect(
		page.getByText('it is not revenue allocated to this service', { exact: false })
	).toBeVisible();
});
test('malformed success and outages show independent errors with explicit recovery; missing plans never fabricate a service', async ({
	page
}) => {
	let statsCalls = 0;
	await page.route('**/api/admin/members/stats', (route) => {
		statsCalls++;
		return route.fulfill({
			status: 200,
			json: { overview: { total_members: 0 }, revenue: { mrr: 0 } }
		});
	});
	await page.goto(`/admin/members?search=${prefix}&plan_id=${targetPlan}`);
	await expect(page.getByRole('alert').filter({ hasText: 'invalid member facts' })).toBeVisible();
	await expect(page.locator('.members-table tbody tr')).toHaveCount(3);
	expect(statsCalls).toBe(1);
	await expect(page.locator('.stats-slot .stat-value')).toHaveCount(0);
	await page.unroute('**/api/admin/members/stats');
	await page.getByRole('button', { name: 'Retry member statistics' }).click();
	await expect(page.locator('.stats-grid')).toBeVisible();
	await page.route('**/api/admin/members/services?*', (route) =>
		route.fulfill({ status: 503, json: { message: 'Actual catalog outage fixture' } })
	);
	await page.getByRole('button', { name: 'Toggle filters', exact: true }).click();
	await page.getByRole('searchbox', { name: 'Search membership plans' }).fill(prefix);
	await page.getByRole('searchbox', { name: 'Search membership plans' }).press('Tab');
	await expect(
		page.getByRole('alert').filter({ hasText: 'Actual catalog outage fixture' })
	).toBeVisible();
	await page.unroute('**/api/admin/members/services?*');
	await page.getByRole('button', { name: 'Retry membership plans' }).click();
	await expect(page.getByText('Plan page 1 of 3', { exact: false })).toBeVisible();
	await page.route(`**/api/admin/members/service/${targetPlan}?*`, (route) =>
		route.fulfill({ status: 200, json: { service: { id: targetPlan + 1 } } })
	);
	await page.goto(`/admin/members/service/${targetPlan}?search=${prefix}`);
	await expect(page.getByRole('alert').filter({ hasText: 'invalid member facts' })).toBeVisible();
	await expect(page.locator('.members-table tbody tr')).toHaveCount(0);
	await page.unroute(`**/api/admin/members/service/${targetPlan}?*`);
	await page.getByRole('button', { name: 'Retry service members' }).click();
	await expect(page.locator('.members-table tbody tr')).toHaveCount(3);
	await page.goto('/admin/members/service/922337203685477580');
	await expect(page.getByRole('alert')).toContainText('Invalid membership plan identifier');
	await expect(page.locator('.members-table tbody tr')).toHaveCount(0);
	await page.goto('/admin/members/service/2147483647');
	await expect(page.getByRole('alert')).toBeVisible();
	await expect(page.locator('.service-members-page h1')).toHaveText('Membership plan');
	await expect(page.locator('.members-table tbody tr')).toHaveCount(0);
});
test('statistics and plan data deny anonymous and ordinary member access', async ({ baseURL }) => {
	for (const ordinary of [false, true]) {
		const api = await playwrightRequest.newContext({ baseURL });
		try {
			if (ordinary)
				expect(
					(
						await api.post('/api/auth/login', {
							data: { email: 'user@admin-completion.test', password }
						})
					).status()
				).toBe(200);
			for (const path of [
				'/api/admin/members/stats',
				'/api/admin/members/services',
				`/api/admin/members/service/${targetPlan}`
			])
				expect((await api.get(path)).status()).toBe(ordinary ? 403 : 401);
		} finally {
			await api.dispose();
		}
	}
});
