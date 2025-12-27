/**
 * Revolution Trading Pros - Playwright Global Setup
 * Apple ICT 11+ Principal Engineer Grade
 *
 * Runs once before all tests to set up the test environment
 * Designed to work in CI without requiring browser launch
 */

import type { FullConfig } from '@playwright/test';

async function globalSetup(config: FullConfig): Promise<void> {
	const { baseURL } = config.projects[0].use;

	console.log(`\n🚀 Global Setup - Initializing E2E Test Environment`);
	console.log(`   Base URL: ${baseURL}`);
	console.log(`   CI Mode: ${!!process.env.CI}`);

	// Simple HTTP health check without browser (works in CI)
	if (baseURL) {
		const maxRetries = 30;
		let retries = 0;
		let ready = false;

		while (retries < maxRetries && !ready) {
			try {
				const response = await fetch(baseURL, {
					method: 'HEAD',
					signal: AbortSignal.timeout(5000)
				});
				if (response.ok || response.status < 500) {
					ready = true;
					console.log(`   ✅ Application is ready (status: ${response.status})`);
				}
			} catch (error) {
				retries++;
				if (retries < maxRetries) {
					console.log(`   ⏳ Waiting for server... (attempt ${retries}/${maxRetries})`);
					await new Promise((r) => setTimeout(r, 2000));
				}
			}
		}

		if (!ready) {
			console.log(`   ⚠️ Server check completed - tests will retry on their own`);
		}
	}

	console.log(`   ✅ Global Setup Complete\n`);
}

export default globalSetup;
