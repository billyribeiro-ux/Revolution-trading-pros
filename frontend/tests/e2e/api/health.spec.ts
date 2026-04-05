import { test, expect } from '@playwright/test';

/**
 * Direct HTTP checks against the deployed API (Fly.io).
 * Uses the `api` project's baseURL (VITE_API_URL / API_BASE_URL), not the Svelte app.
 */
test.describe('Production API', () => {
	test('GET /health returns 200 with healthy status', async ({ request }) => {
		const res = await request.get('/health');
		expect(res.ok(), await res.text()).toBeTruthy();
		const body = (await res.json()) as { status?: string; version?: string };
		expect(body.status).toBe('healthy');
		expect(body.version).toBeTruthy();
	});
});
