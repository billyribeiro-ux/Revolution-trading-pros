/**
 * User Subscriptions API Proxy
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Apple Principal Engineer ICT 7 Grade Implementation
 *
 * DESIGN PRINCIPLES:
 * - Graceful degradation: Always return valid response, never throw
 * - Defensive programming: Validate all inputs, handle all error paths
 * - Structured logging: Contextual logs with request tracing
 * - Security: Token validation, no sensitive data in logs
 *
 * @version 1.0.0
 * @see /api/subscriptions/my (backend endpoint)
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

// ICT 7: Centralized configuration with fallback
const API_URL = process.env.VITE_API_URL || 'https://revolution-trading-pros-api.fly.dev';

// ICT 7: Type-safe response structure
interface SubscriptionsResponse {
	subscriptions: unknown[];
	total: number;
}

/**
 * GET /api/my/subscriptions
 *
 * Proxies subscription data from backend with graceful error handling.
 * Returns empty array on any failure to prevent UI crashes.
 *
 * @returns {SubscriptionsResponse} User's subscriptions or empty array
 */
export const GET: RequestHandler = async ({ cookies, fetch, request }) => {
	// ICT 7: Request tracing for debugging
	const requestId = crypto.randomUUID().slice(0, 8);
	const startTime = performance.now();

	// ICT 7: Defensive - validate token presence
	const token = cookies.get('rtp_access_token');

	if (!token) {
		// ICT 7: Structured logging with context
		console.debug(`[Subscriptions:${requestId}] No auth token - returning empty`);
		return json({
			subscriptions: [],
			total: 0
		} satisfies SubscriptionsResponse);
	}

	try {
		const response = await fetch(`${API_URL}/api/subscriptions/my`, {
			headers: {
				Authorization: `Bearer ${token}`,
				'Content-Type': 'application/json',
				Accept: 'application/json'
			}
		});

		const duration = Math.round(performance.now() - startTime);

		if (!response.ok) {
			// ICT 7: Structured warning with context (no sensitive data)
			console.warn(
				`[Subscriptions:${requestId}] Backend returned ${response.status} (${duration}ms)`
			);
			return json({
				subscriptions: [],
				total: 0
			} satisfies SubscriptionsResponse);
		}

		const data = await response.json();

		// ICT 7: Defensive data extraction with fallbacks
		const subscriptions = data.data || data.subscriptions || [];
		const total = data.total ?? subscriptions.length;

		console.debug(`[Subscriptions:${requestId}] Success: ${total} subscriptions (${duration}ms)`);

		return json({
			subscriptions,
			total
		} satisfies SubscriptionsResponse);
	} catch (error) {
		// ICT 7: Structured error logging (no stack traces in production)
		const duration = Math.round(performance.now() - startTime);
		const errorMessage = error instanceof Error ? error.message : 'Unknown error';

		console.error(`[Subscriptions:${requestId}] Failed: ${errorMessage} (${duration}ms)`);

		// ICT 7: Graceful degradation - never crash the UI
		return json({
			subscriptions: [],
			total: 0
		} satisfies SubscriptionsResponse);
	}
};
