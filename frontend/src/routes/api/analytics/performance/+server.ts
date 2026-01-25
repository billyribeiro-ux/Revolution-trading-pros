/**
 * ═══════════════════════════════════════════════════════════════════════════
 * Performance Analytics Proxy - Apple ICT 7 Principal Engineer Grade
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Pattern: Fire-and-forget with immediate 204 response
 * 
 * Architecture:
 * 1. Return 204 IMMEDIATELY to caller (non-blocking)
 * 2. Fire-and-forget backend call (don't await response)
 * 3. Silent failure - analytics errors must never affect UX
 *
 * Design Principles:
 * - Analytics is best-effort, not guaranteed delivery
 * - User experience > data collection
 * - Graceful degradation when backend unavailable
 * - Correlation ID in header for debugging
 */
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const PROD_API_ROOT = 'https://revolution-trading-pros-api.fly.dev';
const API_ROOT = env.VITE_API_URL || env.BACKEND_URL || PROD_API_ROOT;
const API_URL = `${API_ROOT}/api`;

function generateRequestId(): string {
	return `rtp-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 9)}`;
}

export const POST: RequestHandler = async ({ request, cookies }) => {
	const requestId = generateRequestId();

	// Fire-and-forget: send to backend without awaiting
	try {
		const rawBody = await request.text();

		if (rawBody) {
			const token = cookies.get('rtp_access_token');
			const headers: Record<string, string> = {
				'Content-Type': 'application/json',
				'X-Request-ID': requestId
			};
			if (token) headers.Authorization = `Bearer ${token}`;

			// Fire-and-forget: don't await, don't care about result
			globalThis.fetch(`${API_URL}/analytics/performance`, {
				method: 'POST',
				headers,
				body: rawBody
			}).catch(() => {
				// Silently ignore - analytics failures are acceptable
			});
		}
	} catch {
		// Silently ignore any parsing errors
	}

	// ICT 7: Return 204 immediately with correlation ID header
	return new Response(null, {
		status: 204,
		headers: { 'X-Request-ID': requestId }
	});
};
