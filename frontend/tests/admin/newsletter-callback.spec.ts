import { expect, test } from '@playwright/test';
import { createHmac } from 'node:crypto';

// Seed api/tests/fixtures/newsletter_browser.sql in the disposable verification DB.
test('public newsletter callbacks persist changes without exposing tokens to page scripts', async ({
	page,
	baseURL,
	playwright
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	const admin = await playwright.request.newContext({ baseURL });
	try {
		const login = await admin.post('/api/auth/login', {
			data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
		});
		expect(login.ok()).toBe(true);
		const subscriber = async () => {
			const response = await admin.get(
				'/api/newsletter/subscribers?search=browser-newsletter%40example.test'
			);
			expect(response.ok()).toBe(true);
			const body = await response.json();
			expect(body.data).toHaveLength(1);
			return body.data[0] as {
				id: number;
				email: string;
				status: string;
				confirmed_at: string | null;
				unsubscribed_at: string | null;
			};
		};
		const record = await subscriber();
		expect(record.status).toBe('pending');
		const token = (action: string) =>
			`${Buffer.from(`${record.id}:${record.email}`).toString('base64url')}:${createHmac('sha256', 'local-disposable-test-secret-at-least-32-chars').update(`${record.id}:${record.email}:${action}`).digest('base64url')}`;
		const failures: string[] = [];
		page.on('pageerror', (error) => failures.push(error.message));
		page.on('requestfailed', (request) =>
			failures.push(`${request.failure()?.errorText} ${new URL(request.url()).pathname}`)
		);
		const requests: string[] = [];
		page.on('request', (request) => requests.push(request.url()));
		// The shared shell loads Google Fonts. Intercept its stylesheet locally and
		// verify capability URLs are never sent as a referrer to that dependency.
		await page.route('https://fonts.googleapis.com/**', async (route) => {
			expect(route.request().headers()['referer']).toBeUndefined();
			await route.fulfill({ status: 200, contentType: 'text/css', body: '' });
		});
		const verifyPage = async (path: string, status: number, heading: string) => {
			const response = await page.goto(path, { waitUntil: 'networkidle' });
			expect(response?.status()).toBe(status);
			expect(response?.headers()['cache-control']).toContain('no-store');
			expect(response?.headers()['referrer-policy']).toBe('no-referrer');
			expect(response?.headers()['x-robots-tag']).toContain('noindex');
			await expect(page.getByRole('heading', { name: heading, exact: true })).toBeVisible();
			await expect(page.locator('head title')).toHaveCount(1);
			await expect(page.locator('meta[name="robots"]')).toHaveAttribute('content', /noindex/);
			const canonical = await page.locator('link[rel="canonical"]').getAttribute('href');
			expect(canonical).not.toContain('token');
			expect(canonical).not.toContain('?');
			expect(await page.locator('script:not([type="application/ld+json"])').count()).toBe(0);
		};
		const confirmation = `/newsletter/confirm?${new URLSearchParams({ token: token('confirm') })}`;
		await verifyPage(confirmation, 200, 'Newsletter subscription confirmed');
		const confirmed = await subscriber();
		expect(confirmed.status).toBe('confirmed');
		expect(confirmed.confirmed_at).not.toBeNull();
		await verifyPage(confirmation, 200, 'Newsletter subscription confirmed');
		expect((await subscriber()).confirmed_at).toBe(confirmed.confirmed_at);
		const oneClick = await page.request.post(
			`/newsletter/unsubscribe?${new URLSearchParams({ token: token('unsubscribe') })}`,
			{ form: { 'List-Unsubscribe': 'One-Click' } }
		);
		expect(oneClick.status()).toBe(200);
		expect((await oneClick.json()).status).toBe('unsubscribed');
		const oneClickRecord = await subscriber();
		expect(oneClickRecord.status).toBe('unsubscribed');
		await verifyPage(
			`/newsletter/unsubscribe?${new URLSearchParams({ token: token('unsubscribe') })}`,
			200,
			'Newsletter subscription cancelled'
		);
		const unsubscribed = await subscriber();
		expect(unsubscribed.status).toBe('unsubscribed');
		expect(unsubscribed.unsubscribed_at).not.toBeNull();
		expect(unsubscribed.unsubscribed_at).toBe(oneClickRecord.unsubscribed_at);
		for (const path of ['/newsletter/confirm', '/newsletter/unsubscribe?token=invalid'])
			await verifyPage(path, 400, 'Unable to update your newsletter subscription');
		expect(failures).toEqual([]);
		expect(
			requests.every((url) =>
				['127.0.0.1', 'localhost', 'fonts.googleapis.com'].includes(new URL(url).hostname)
			)
		).toBe(true);
		expect(await page.context().cookies()).toEqual([]);
	} finally {
		await admin.dispose();
	}
});
