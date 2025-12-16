/**
 * Revolution Trading Pros - Backend Availability Check Helper
 *
 * Provides utilities to check if backend API is available during tests.
 * This allows tests to gracefully handle missing backend in CI environments.
 *
 * Usage:
 * - Call isBackendAvailable() to check if backend is running
 * - Use shouldSkipBackendTests() in test.skip() conditions
 * - Check BACKEND_AVAILABLE env var set during global setup
 */

import { request } from '@playwright/test';

/**
 * Checks if the backend API is available and responding
 */
export async function isBackendAvailable(apiUrl?: string): Promise<boolean> {
	const url = apiUrl || process.env.E2E_API_URL || 'http://localhost:8000/api';

	try {
		const context = await request.newContext();

		// Try health endpoint first
		try {
			const response = await context.get(`${url}/health/live`, {
				timeout: 3000,
				ignoreHTTPSErrors: true
			});

			if (response.ok()) {
				await context.dispose();
				return true;
			}
		} catch {
			// Health endpoint may not exist, try root
		}

		// Try root API endpoint as fallback
		try {
			const response = await context.get(url, {
				timeout: 3000,
				ignoreHTTPSErrors: true
			});

			await context.dispose();
			// Even 404 means backend is responding
			return response.status() !== 0;
		} catch {
			await context.dispose();
			return false;
		}
	} catch (error) {
		console.log(`Backend check failed: ${error}`);
		return false;
	}
}

/**
 * Returns true if tests requiring backend should be skipped
 * This checks both environment variables and actual backend availability
 */
export function shouldSkipBackendTests(): boolean {
	// If explicitly told backend is not available (set during global setup)
	if (process.env.BACKEND_AVAILABLE === 'false') {
		return true;
	}

	// If explicitly told to skip backend tests
	if (process.env.SKIP_BACKEND_TESTS === 'true') {
		return true;
	}

	// In CI without explicit backend URL, assume no backend
	if (process.env.CI && !process.env.E2E_API_URL) {
		return true;
	}

	return false;
}

/**
 * Returns true if backend is explicitly available
 */
export function isBackendExplicitlyAvailable(): boolean {
	return process.env.BACKEND_AVAILABLE === 'true';
}

/**
 * Gets a descriptive message for why backend tests are being skipped
 */
export function getBackendSkipReason(): string {
	if (process.env.BACKEND_AVAILABLE === 'false') {
		return 'Backend API is not available (detected during global setup)';
	}

	if (process.env.SKIP_BACKEND_TESTS === 'true') {
		return 'Backend tests explicitly disabled via SKIP_BACKEND_TESTS';
	}

	if (process.env.CI && !process.env.E2E_API_URL) {
		return 'Running in CI without backend (no E2E_API_URL configured)';
	}

	return 'Backend tests skipped';
}
