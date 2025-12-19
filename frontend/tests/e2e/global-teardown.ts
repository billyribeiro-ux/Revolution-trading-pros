/**
 * Revolution Trading Pros - Global E2E Test Teardown
 *
 * Runs once after all tests to:
 * - Clean up test data
 * - Generate summary reports
 * - Log test run statistics
 *
 * @see https://playwright.dev/docs/test-global-setup-teardown
 */

import type { FullConfig } from '@playwright/test';

async function globalTeardown(_config: FullConfig): Promise<void> {
	console.log('\n========================================');
	console.log('   Revolution Trading Pros E2E Teardown');
	console.log('========================================\n');

	const setupTimestamp = process.env.E2E_SETUP_TIMESTAMP;

	if (setupTimestamp) {
		const duration = Date.now() - new Date(setupTimestamp).getTime();
		console.log(`Total test run duration: ${Math.round(duration / 1000)}s`);
	}

	// Clean up any test data created during tests
	const apiUrl = process.env.E2E_API_URL || 'https://revolution-trading-pros-api.fly.dev/api';

	// Only attempt cleanup if we're in a dev/staging environment
	if (process.env.E2E_CLEANUP_TEST_DATA === 'true') {
		console.log('Cleaning up test data...');
		try {
			// This would call a test cleanup endpoint if configured
			const response = await fetch(`${apiUrl}/test/cleanup`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${process.env.E2E_CLEANUP_TOKEN || ''}`
				}
			});

			if (response.ok) {
				console.log('Test data cleanup complete');
			}
		} catch {
			// Cleanup endpoint may not exist - that's okay
			console.log('Cleanup endpoint not available');
		}
	}

	console.log('\nGlobal teardown complete\n');
}

export default globalTeardown;
