/// <reference types="node" />
/**
 * SvelteKit route matrix — one navigation per `+page.svelte` route (see tests/fixtures/sveltekit-routes.ts).
 * Regenerate routes: node scripts/generate-sveltekit-e2e-routes.mjs
 *
 * Assertions: no 5xx from the server, document becomes visible. Dynamic segments use stable placeholders.
 * Protected or missing resources may yield 3xx/404; those are acceptable as long as the app does not 500.
 */

import { test, expect } from '@playwright/test';
import { SVELTEKIT_PAGE_ROUTES } from '../fixtures/sveltekit-routes';

function isBenignConsoleError(text: string): boolean {
	return (
		!text.includes('favicon') &&
		!text.includes('CORS') &&
		!text.includes('Access-Control-Allow-Origin') &&
		!text.includes('extension') &&
		!text.includes('third-party') &&
		!text.includes('ERR_FAILED') &&
		!text.includes('Failed to fetch') &&
		!text.includes('Failed to load resource') &&
		!text.includes('Origin http://localhost') &&
		// CSP warnings (third-party media/embed) are policy noise for route smoke, not 5xx regressions
		!text.includes('Content Security Policy') &&
		!/\[err_[a-z0-9_]+\]/.test(text) &&
		!/^Error:\s*Error$/.test(text)
	);
}

/** Serial: parallel hits to the dev server can exhaust Vite/Node and drop connections (ERR_CONNECTION_REFUSED). */
test.describe.serial('SvelteKit page routes @routes', () => {
	for (const path of SVELTEKIT_PAGE_ROUTES) {
		test(`GET ${path}`, async ({ page }) => {
			const errors: string[] = [];
			page.on('console', (msg) => {
				if (msg.type() === 'error') errors.push(msg.text());
			});

			const response = await page.goto(path, {
				waitUntil: 'domcontentloaded',
				timeout: 60_000
			});

			const status = response?.status() ?? 0;
			expect(status, `unexpected status for ${path}`).toBeLessThan(500);

			await expect(page.locator('body')).toBeVisible({ timeout: 15_000 });

			const critical = errors.filter(isBenignConsoleError);
			expect(critical, `console errors on ${path}`).toHaveLength(0);
		});
	}
});
