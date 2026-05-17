/// <reference types="node" />
/**
 * Subscription checkout E2E
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Regression test for the 2026-05-10 fix at api/src/routes/subscriptions.rs:446
 * — the create-subscription endpoint used to return a fake placeholder URL
 * (https://checkout.stripe.com/placeholder?price=...). It now calls
 * StripeService::create_subscription_checkout and returns a real session URL.
 *
 * What this verifies:
 *   1. POST /api/subscriptions returns a checkout_url that is *not* the
 *      placeholder pattern (no "placeholder" substring).
 *   2. The URL points at Stripe (`stripe.com` host) — implementation may
 *      hand back either checkout.stripe.com (live session) or a stripe.com
 *      domain in test mode.
 *   3. If the configured plan has no stripe_price_id (legacy data), the
 *      endpoint returns a non-Stripe response — that's the local-grant code
 *      path and is still valid; we accept either.
 *
 * Skipped by default (Stripe API call requires STRIPE_SECRET to be a real
 * test-mode key, not the local 'sk_test_placeholder' from .env.example).
 * Set RUN_STRIPE_E2E=1 in CI to enable.
 *
 * Run:
 *   RUN_STRIPE_E2E=1 pnpm exec playwright test \
 *     tests/e2e/admin/subscription-checkout.spec.ts --project=chromium
 */

import { test, expect, type APIRequestContext } from '@playwright/test';

const ADMIN_EMAIL = process.env.E2E_ADMIN_EMAIL || 'welberribeirodrums@gmail.com';
const ADMIN_PASSWORD = process.env.E2E_ADMIN_PASSWORD || 'Davedicenso01!';

const RUN_STRIPE = process.env.RUN_STRIPE_E2E === '1';

test.describe.configure({ mode: 'serial' });

test.describe('Subscription checkout returns a real Stripe URL', () => {
	test.skip(!RUN_STRIPE, 'set RUN_STRIPE_E2E=1 with a real Stripe test key to enable');

	let api: APIRequestContext;

	test.beforeAll(async ({ playwright, baseURL }) => {
		api = await playwright.request.newContext({ baseURL });
		const login = await api.post('/api/auth/login', {
			data: { email: ADMIN_EMAIL, password: ADMIN_PASSWORD },
			failOnStatusCode: false
		});
		expect(login.ok(), `admin login failed: ${login.status()}`).toBeTruthy();
	});

	test.afterAll(async () => {
		await api?.dispose();
	});

	test('GET /api/subscriptions/plans surfaces at least one plan', async () => {
		const resp = await api.get('/api/subscriptions/plans', { failOnStatusCode: false });
		expect(resp.ok(), `plans endpoint returned ${resp.status()}`).toBeTruthy();
		const body = await resp.json();
		// Response shape varies; accept either an array or { data: [...] }.
		const list = Array.isArray(body) ? body : body.plans ?? body.data ?? [];
		expect(Array.isArray(list)).toBeTruthy();
		expect(list.length, 'no membership plans seeded; run seed migrations').toBeGreaterThan(0);
	});

	test('POST /api/subscriptions returns a real Stripe checkout URL (no placeholder)', async () => {
		// Pick a plan that has a stripe_price_id set
		const plansResp = await api.get('/api/subscriptions/plans');
		const body = await plansResp.json();
		const list: Array<Record<string, unknown>> = Array.isArray(body)
			? body
			: body.plans ?? body.data ?? [];
		const stripePlan = list.find((p) => typeof p.stripe_price_id === 'string' && p.stripe_price_id);
		test.skip(!stripePlan, 'no plan has stripe_price_id; run cms_v2 seed or set one manually');

		const planId = stripePlan!.id as number;

		const resp = await api.post('/api/subscriptions', {
			data: { plan_id: planId },
			failOnStatusCode: false
		});

		// 409 means the test admin already has an active subscription —
		// not a regression in the fix; tell the operator how to clear it.
		if (resp.status() === 409) {
			test.skip(
				true,
				'admin user already has an active subscription; cancel via /admin/subscriptions and rerun'
			);
		}

		expect(resp.ok(), `create subscription returned ${resp.status()}: ${await resp.text()}`)
			.toBeTruthy();

		const json = (await resp.json()) as { checkout_url?: string };
		expect(json.checkout_url, 'no checkout_url in response').toBeTruthy();

		// THE regression assertion — the placeholder is gone.
		expect(json.checkout_url).not.toContain('placeholder');

		// Real Stripe URLs include `stripe.com` somewhere
		expect(json.checkout_url).toMatch(/stripe\.com/);
	});
});
