/// <reference types="node" />
/**
 * Playwright Global Setup
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 *
 * Runs once before all tests to:
 * - Verify environment configuration
 * - Set up any global state
 * - Pre-authenticate if needed
 */

import { FullConfig } from '@playwright/test';

async function globalSetup(config: FullConfig) {
	const baseURL = config.projects[0]?.use?.baseURL || process.env.E2E_BASE_URL;

	console.log('╔═══════════════════════════════════════════════════════════╗');
	console.log('║          Revolution Trading Pros - E2E Test Suite         ║');
	console.log('╠═══════════════════════════════════════════════════════════╣');
	console.log(`║  Base URL: ${baseURL?.padEnd(46) || 'Not configured'.padEnd(46)} ║`);
	console.log(`║  Projects: ${config.projects.length.toString().padEnd(46)} ║`);
	console.log(`║  Workers:  ${(config.workers?.toString() || 'auto').padEnd(46)} ║`);
	console.log('╚═══════════════════════════════════════════════════════════╝');

	// Environment validation
	if (!baseURL) {
		console.warn('⚠️  Warning: E2E_BASE_URL not set, using default');
	}

	// Store start time for duration calculation
	process.env.E2E_START_TIME = Date.now().toString();
}

export default globalSetup;
