import { expect, request, test } from '@playwright/test';
import { readFileSync } from 'node:fs';

interface InventoryPage {
	route: string;
	files: string[];
}
const inventory: { pages: InventoryPage[] } = JSON.parse(
	readFileSync(new URL('../../../docs/admin/admin-inventory.json', import.meta.url), 'utf8')
);
const boundary: Record<string, string> = JSON.parse(
	readFileSync(new URL('../../../docs/admin/blog-editor-boundary.json', import.meta.url), 'utf8')
);
const aliases: Record<string, string> = {
	'/admin/dashboard': '/admin',
	'/admin/analytics/cohorts/create': '/admin/analytics/cohorts?create=1',
	'/admin/analytics/funnels/create': '/admin/analytics/funnels?create=1'
};
const pages = inventory.pages.filter(
	(page) => !page.route.includes('[') && !page.files.some((file) => file in boundary)
);
let storageState: Awaited<
	ReturnType<Awaited<ReturnType<typeof request.newContext>>['storageState']>
>;

test.use({ screenshot: 'off', video: 'off', trace: 'off' });
test.describe('source admin navigation inventory', () => {
	test.beforeAll(async ({ baseURL }) => {
		expect(baseURL, 'This inventory only visits the owned disposable stack').toBe(
			'http://127.0.0.1:5274'
		);
		const api = await request.newContext({ baseURL });
		try {
			const login = await api.post('/api/auth/login', {
				data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
			});
			expect(login.ok()).toBe(true);
			storageState = await api.storageState();
		} finally {
			await api.dispose();
		}
	});

	test('static in-scope pages load without broken contracts', async ({
		browser,
		baseURL
	}, testInfo) => {
		test.setTimeout(600000);
		const failures: { route: string; error: string }[] = [];
		for (const route of pages) {
			const context = await browser.newContext({ baseURL, storageState });
			const page = await context.newPage();
			const errors: string[] = [];
			const failedRequests: string[] = [];
			page.on('pageerror', (error) => errors.push(error.message.slice(0, 500)));
			page.on('console', (message) => {
				if (message.type() !== 'error') return;
				const origin = message.location().url;
				if (origin && !['127.0.0.1', 'localhost'].includes(new URL(origin).hostname)) return;
				if (!origin && message.text().startsWith('Failed to load resource')) return;
				errors.push(message.text().slice(0, 500));
			});
			page.on('response', (response) => {
				const url = new URL(response.url());
				if (url.pathname.startsWith('/api/') && response.status() >= 400)
					failedRequests.push(`${response.status()} ${url.pathname}`);
			});
			page.on('requestfailed', (request) => {
				const url = new URL(request.url());
				if (
					url.pathname.startsWith('/api/') &&
					!request.failure()?.errorText.includes('ERR_ABORTED')
				)
					failedRequests.push(`${request.failure()?.errorText} ${url.pathname}`);
			});
			await page.route('**/*', (route) => {
				const url = new URL(route.request().url());
				return ['127.0.0.1', 'localhost'].includes(url.hostname) ||
					!['http:', 'https:'].includes(url.protocol)
					? route.continue()
					: route.abort();
			});
			try {
				const response = await page.goto(route.route, { waitUntil: 'networkidle' });
				const evidence = {
					route: route.route,
					status: response?.status(),
					finalPath: new URL(page.url()).pathname,
					finalSearch: new URL(page.url()).search,
					headings: await page.getByRole('heading', { level: 1 }).allTextContents(),
					errors,
					failedRequests: [...new Set(failedRequests)]
				};
				await testInfo.attach('navigation-evidence', {
					body: JSON.stringify(evidence, null, 2),
					contentType: 'application/json'
				});
				expect(response?.status()).toBe(200);
				const expected = new URL(aliases[route.route] ?? route.route, baseURL);
				expect(evidence.finalPath).toBe(expected.pathname);
				if (expected.search) {
					expect(evidence.finalSearch).toBe(expected.search);
					await expect(page.getByRole('dialog')).toBeVisible();
				}
				expect(errors, 'Unhandled application errors').toEqual([]);
				expect(
					evidence.failedRequests,
					'Failed API contracts need implementation or verified unavailable-service handling'
				).toEqual([]);
				await expect(page.getByRole('complementary', { name: 'Admin sidebar' })).toBeVisible();
			} catch (error) {
				failures.push({
					route: route.route,
					error: error instanceof Error ? error.message : String(error)
				});
			} finally {
				await context.close();
			}
			console.info(`Visited ${route.route}; ${failures.length} page(s) need attention`);
		}
		await testInfo.attach('navigation-failures', {
			body: JSON.stringify(failures, null, 2),
			contentType: 'application/json'
		});
		expect(
			failures,
			'Navigation gaps remain; dynamic fixture routes and actual actions require separate workflow proofs'
		).toEqual([]);
	});
});
