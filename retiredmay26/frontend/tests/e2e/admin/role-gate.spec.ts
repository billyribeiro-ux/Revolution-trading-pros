/// <reference types="node" />
/**
 * Admin role-gate E2E
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Regression tests for the /admin role gate. Built in response to the
 * 2026-04-25 distinguished-engineer audit finding that /admin had no
 * role gate, and the 2026-05-10 audit finding that the gate is now
 * present in /admin/+layout.svelte.
 *
 * What this verifies:
 *   1. Visiting /admin while logged out kicks the user to /login with
 *      a redirect= query string.
 *   2. Visiting /admin while logged in as a non-admin user (a regular
 *      'user'-role account) bounces to / with ?error=admin_required.
 *   3. Visiting /admin as an admin renders the admin shell.
 *   4. Every admin API proxy returns 401/403 (NOT 200) when called by
 *      a non-admin session — defence in depth, in case the layout guard
 *      is ever bypassed by a route that doesn't render the layout.
 *
 * Run:
 *   pnpm exec playwright test tests/e2e/admin/role-gate.spec.ts \
 *     --project=chromium --headed
 *
 * Local stack required: Docker `db` + `redis` + `api` on :8080 plus
 * SvelteKit dev server on :5173. Seed the local admin via
 * `api/scripts/seed-local-admin.sh` and a non-admin via the
 * /api/auth/register endpoint (or the regular /signup form).
 */

import { test, expect, type Page } from '@playwright/test';
import { SUPERADMIN_EMAIL, SUPERADMIN_PASSWORD } from '../_creds';

const ADMIN_EMAIL = SUPERADMIN_EMAIL;
const ADMIN_PASSWORD = SUPERADMIN_PASSWORD;

const NONADMIN_EMAIL = process.env.E2E_NONADMIN_EMAIL || 'e2e-nonadmin@revolutiontradingpros.test';
const NONADMIN_PASSWORD = process.env.E2E_NONADMIN_PASSWORD || 'NonAdminPass1!';

async function loginAs(page: Page, email: string, password: string): Promise<boolean> {
	const resp = await page.request.post('/api/auth/login', {
		data: { email, password },
		failOnStatusCode: false,
		headers: { 'content-type': 'application/json' }
	});
	return resp.ok();
}

async function ensureNonAdmin(page: Page): Promise<void> {
	// Try login first; if the account doesn't exist, register it.
	const loggedIn = await loginAs(page, NONADMIN_EMAIL, NONADMIN_PASSWORD);
	if (loggedIn) return;

	const reg = await page.request.post('/api/auth/register', {
		data: {
			email: NONADMIN_EMAIL,
			password: NONADMIN_PASSWORD,
			name: 'E2E Non-Admin'
		},
		failOnStatusCode: false,
		headers: { 'content-type': 'application/json' }
	});
	expect(reg.ok(), `register failed: ${reg.status()} ${await reg.text()}`).toBeTruthy();

	const ok = await loginAs(page, NONADMIN_EMAIL, NONADMIN_PASSWORD);
	expect(ok, 'login as non-admin failed after registration').toBeTruthy();
}

test.describe('Admin role gate', () => {
	test('logged-out visit to /admin redirects to /login with redirect= query', async ({
		page
	}) => {
		// Make sure no auth cookie is set
		await page.context().clearCookies();

		await page.goto('/admin');
		await page.waitForURL((url) => url.pathname.startsWith('/login'), { timeout: 15_000 });

		const url = new URL(page.url());
		expect(url.pathname).toBe('/login');
		expect(url.searchParams.get('redirect')).toBe('/admin');
	});

	test('non-admin visit to /admin bounces to / with ?error=admin_required', async ({ page }) => {
		await ensureNonAdmin(page);

		await page.goto('/admin');
		// The layout guard fires from onMount; give it a moment to dispatch the goto.
		await page
			.waitForURL(
				(url) => url.pathname === '/' && url.searchParams.get('error') === 'admin_required',
				{ timeout: 15_000 }
			)
			.catch(async () => {
				const current = page.url();
				throw new Error(
					`expected redirect to /?error=admin_required, got ${current}. ` +
						`Layout guard at /admin/+layout.svelte may be skipping the role check.`
				);
			});

		expect(new URL(page.url()).pathname).toBe('/');
	});

	test('admin visit to /admin renders the admin shell', async ({ page }) => {
		const ok = await loginAs(page, ADMIN_EMAIL, ADMIN_PASSWORD);
		expect(ok, 'admin login failed; check credentials & local DB seed').toBeTruthy();

		await page.goto('/admin');
		await page.waitForLoadState('domcontentloaded');

		// We don't redirect away
		expect(new URL(page.url()).pathname).toBe('/admin');
		// Sidebar mounts (defensive locator — match the Overview entry)
		await expect(
			page.locator('a[href="/admin"]:has-text("Overview"), a[href="/admin"]')
		).toBeVisible({ timeout: 10_000 });
	});

	test('admin API proxies reject non-admin sessions (401/403)', async ({ page }) => {
		await ensureNonAdmin(page);

		// Spot-check a representative slice of the admin API surface. If any of
		// these returns 200 to a non-admin session, the gate is broken on that
		// endpoint regardless of what the layout guard does.
		const proxies = [
			'/api/admin/members',
			'/api/admin/orders',
			'/api/admin/products',
			'/api/admin/users',
			'/api/admin/settings',
			'/api/admin/coupons',
			'/api/admin/email/campaigns',
			'/api/admin/seo/keywords',
			'/api/admin/site-health'
		];

		const failures: string[] = [];
		for (const path of proxies) {
			const resp = await page.request.get(path, { failOnStatusCode: false });
			const status = resp.status();
			if (status !== 401 && status !== 403) {
				failures.push(`${path} → ${status} (expected 401/403)`);
			}
		}

		expect(
			failures,
			`admin API proxies leaked to non-admin session:\n${failures.join('\n')}`
		).toEqual([]);
	});
});
