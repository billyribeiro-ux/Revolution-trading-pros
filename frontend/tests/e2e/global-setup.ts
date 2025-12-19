/**
 * Revolution Trading Pros - Global E2E Test Setup
 *
 * Detects backend availability and configures test environment.
 * The webServer in playwright.config.ts handles starting the dev server.
 *
 * @see https://playwright.dev/docs/test-global-setup-teardown
 */

import { isBackendAvailable } from './helpers/backend-check.helper';

async function globalSetup(): Promise<void> {
	console.log('\n========================================');
	console.log('   Revolution Trading Pros E2E Setup');
	console.log('========================================\n');

	const baseUrl = process.env.E2E_BASE_URL || 'http://localhost:5174';
	const apiUrl = process.env.E2E_API_URL || 'https://revolution-trading-pros-api.fly.dev/api';
	const isCI = !!process.env.CI;

	console.log(`Frontend URL: ${baseUrl}`);
	console.log(`API URL: ${apiUrl}`);
	console.log(`Environment: ${isCI ? 'CI' : 'Local'}`);

	// Check if backend is available
	console.log('\nChecking backend availability...');
	const backendAvailable = await isBackendAvailable(apiUrl);

	if (backendAvailable) {
		console.log('✓ Backend API is available');
		process.env.BACKEND_AVAILABLE = 'true';
	} else {
		console.log('✗ Backend API is not available');
		console.log('  Tests requiring backend will be skipped');
		process.env.BACKEND_AVAILABLE = 'false';
	}

	// Store test metadata
	process.env.E2E_SETUP_TIMESTAMP = new Date().toISOString();

	console.log('\nGlobal setup complete\n');
}

export default globalSetup;
