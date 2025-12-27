/**
 * Revolution Trading Pros - Playwright Global Teardown
 * Apple ICT 11+ Principal Engineer Grade
 *
 * Runs once after all tests complete to clean up
 */

import type { FullConfig } from '@playwright/test';

async function globalTeardown(_config: FullConfig): Promise<void> {
	console.log(`\n🧹 Global Teardown - Cleaning up E2E Test Environment`);

	// Add any cleanup logic here (clear test data, close connections, etc.)

	console.log(`   ✅ Global Teardown Complete\n`);
}

export default globalTeardown;
