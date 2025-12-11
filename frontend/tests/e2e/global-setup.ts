/**
 * Revolution Trading Pros - Global E2E Test Setup
 *
 * Minimal setup that doesn't require browser launch.
 * The webServer in playwright.config.ts handles starting the dev server.
 *
 * @see https://playwright.dev/docs/test-global-setup-teardown
 */

async function globalSetup(): Promise<void> {
	console.log('\n========================================');
	console.log('   Revolution Trading Pros E2E Setup');
	console.log('========================================\n');

	const baseUrl = process.env.E2E_BASE_URL || 'http://localhost:5174';
	const apiUrl = process.env.E2E_API_URL || 'http://localhost:8000/api';

	console.log(`Frontend URL: ${baseUrl}`);
	console.log(`API URL: ${apiUrl}`);
	console.log(`Environment: ${process.env.CI ? 'CI' : 'Local'}`);

	// Store test metadata
	process.env.E2E_SETUP_TIMESTAMP = new Date().toISOString();

	console.log('\nGlobal setup complete\n');
}

export default globalSetup;
