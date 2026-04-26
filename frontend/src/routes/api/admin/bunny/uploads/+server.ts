/**
 * Bunny Uploads List API - Proxy to backend
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/admin/bunny/uploads - List recent video uploads
 *
 * Proxies to backend at /api/admin/bunny/uploads
 *
 * @version 1.1.0 - 2026-04-26 audit fix
 *
 * FIX-2026-04-26-audit (P0-9, P2-9, P3 env-var):
 * - Use the `API_BASE_URL || BACKEND_URL || …` env chain (was just BACKEND_URL).
 * - Forward auth as `Authorization: Bearer <rtp_access_token>` (was a malformed
 *   `Cookie: session=<jwt>` that the backend never reads).
 * - Surface real backend errors instead of returning a fake `{ success: true, data: [] }`
 *   on failure — admins were seeing an empty list whenever the backend hiccuped.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

// GET - List recent uploads
export const GET: RequestHandler = async ({ cookies, request }) => {
	// FIX-2026-04-26-audit: prefer the canonical rtp_access_token cookie set by
	// the login proxy; fall back to a Bearer header for service callers.
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;

	if (!token) {
		return json({ success: false, error: 'Authentication required' }, { status: 401 });
	}

	try {
		const response = await fetch(`${BACKEND_URL}/api/admin/bunny/uploads`, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});

		const text = await response.text();
		let payload: unknown = null;
		if (text) {
			try {
				payload = JSON.parse(text);
			} catch {
				payload = { error: text };
			}
		}

		if (!response.ok) {
			console.error(`[Bunny Uploads] Backend error ${response.status}:`, text);
			return json(
				{
					success: false,
					error: (payload as { error?: string })?.error || `Backend returned ${response.status}`
				},
				{ status: response.status }
			);
		}

		return json(payload ?? { success: true, data: [] });
	} catch (err) {
		console.error('[Bunny Uploads] Fetch failed:', err);
		// FIX-2026-04-26-audit (P2-9): no more lying success — surface the failure.
		return json(
			{
				success: false,
				error: err instanceof Error ? err.message : 'Failed to reach backend'
			},
			{ status: 502 }
		);
	}
};
