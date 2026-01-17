import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

/**
 * Performance Analytics Proxy - Apple ICT 7 Principal Engineer
 * ══════════════════════════════════════════════════════════════════════════════
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
 * ══════════════════════════════════════════════════════════════════════════════
 */

const PROD_API_ROOT = 'https://revolution-trading-pros-api.fly.dev';
const API_ROOT = env.VITE_API_URL || env.BACKEND_URL || PROD_API_ROOT;
const API_URL = `${API_ROOT}/api`;

export const POST: RequestHandler = async ({ request, cookies }) => {
	// Fire-and-forget: send to backend without awaiting
	// Use global fetch (not SvelteKit's) to avoid any framework error handling
	try {
		const rawBody = await request.text();
		
		if (rawBody) {
			const token = cookies.get('rtp_access_token');
			const headers: Record<string, string> = {
				'Content-Type': 'application/json'
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

	// Always return success immediately
	return json({ ok: true }, { status: 204 });
};
