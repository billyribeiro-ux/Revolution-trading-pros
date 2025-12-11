/**
 * Revolution Trading Pros - Global E2E Test Setup
 *
 * Runs once before all tests to:
 * - Validate environment configuration
 * - Check service availability
 * - Set up global test state
 * - Warm up the test environment
 *
 * @see https://playwright.dev/docs/test-global-setup-teardown
 */

import { chromium, type FullConfig } from '@playwright/test';

async function globalSetup(config: FullConfig): Promise<void> {
	console.log('\n========================================');
	console.log('   Revolution Trading Pros E2E Setup');
	console.log('========================================\n');

	const { baseURL } = config.projects[0].use;
	const apiUrl = process.env.E2E_API_URL || 'http://localhost:8000/api';

	console.log(`Frontend URL: ${baseURL}`);
	console.log(`API URL: ${apiUrl}`);
	console.log(`Environment: ${process.env.CI ? 'CI' : 'Local'}`);
	console.log('');

	// Launch a browser to warm up and validate the environment
	const browser = await chromium.launch();
	const context = await browser.newContext();
	const page = await context.newPage();

	try {
		// Check frontend availability
		console.log('Checking frontend availability...');
		const frontendResponse = await page.goto(baseURL as string, {
			timeout: 30000,
			waitUntil: 'domcontentloaded'
		});

		if (frontendResponse?.ok()) {
			console.log('Frontend is ready');
		} else {
			console.warn(`Frontend returned status: ${frontendResponse?.status()}`);
		}

		// Check API availability (optional - backend may not be running)
		console.log('Checking API availability...');
		try {
			const apiResponse = await page.request.get(`${apiUrl}/health/live`, {
				timeout: 10000
			});
			if (apiResponse.ok()) {
				console.log('API is ready');
			} else {
				console.warn(`API returned status: ${apiResponse.status()}`);
			}
		} catch {
			console.warn('API not available - some tests may be skipped');
		}
	} catch (error) {
		console.error('Setup validation failed:', error);
		// Don't throw - let tests handle failures
	} finally {
		await browser.close();
	}

	// Store test metadata
	process.env.E2E_SETUP_TIMESTAMP = new Date().toISOString();

	console.log('\nGlobal setup complete\n');
}

export default globalSetup;
