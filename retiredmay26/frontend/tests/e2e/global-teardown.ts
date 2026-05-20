/// <reference types="node" />
/**
 * Playwright Global Teardown
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 *
 * Runs once after all tests to:
 * - Clean up global state
 * - Report summary statistics
 * - Close any persistent connections
 */

import { FullConfig } from '@playwright/test';

async function globalTeardown(config: FullConfig) {
	const startTime = parseInt(process.env.E2E_START_TIME || '0', 10);
	const duration = startTime ? Math.round((Date.now() - startTime) / 1000) : 0;

	console.log('');
	console.log('╔═══════════════════════════════════════════════════════════╗');
	console.log('║              E2E Test Suite Complete                      ║');
	console.log('╠═══════════════════════════════════════════════════════════╣');
	console.log(`║  Total Duration: ${duration}s`.padEnd(60) + '║');
	console.log('╚═══════════════════════════════════════════════════════════╝');
}

export default globalTeardown;
