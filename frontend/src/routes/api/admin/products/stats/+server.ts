/**
 * Products Stats API Proxy
 * Routes requests to Rust API backend to avoid CORS issues.
 *
 * R22-A: Deleted the 5xx → mock-data fallback (zeros for every field). On a
 *   transient backend outage the dashboard used to render the "everything is
 *   zero" state silently — admins couldn't tell whether the catalog was
 *   actually empty or the backend was down. Now a 502 is returned so the UI
 *   surfaces a real error and the admin can retry / page on-call.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';

// CLAUDE.md hard rule — API_BASE_URL primary, BACKEND_URL fallback, localhost last.
const BACKEND_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const GET: RequestHandler = async (event) => {
	// PRINCIPAL-2026-05-17 (audit FULL_REPO_AUDIT_2026-05-17 §P2-F): replaced the
	// "no token → return empty mock 200" anti-pattern (which masked auth state
	// from the client) with the canonical RBAC gate. `requireAdmin` throws
	// `error(401|403)` so an unauthenticated/under-privileged caller gets a
	// real status, exactly like every other admin proxy.
	const { token } = requireAdmin(event);
	const { fetch } = event;

	try {
		const response = await fetch(`${BACKEND_URL}/api/admin/products/stats`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});

		// FIX-2026-04-26 (P2-2): forward auth failures (401/403) verbatim so the
		// admin shell can redirect to login instead of swallowing the auth state.
		if (response.status === 401 || response.status === 403) {
			const text = await response.text();
			return new Response(text || JSON.stringify({ message: 'Unauthorized' }), {
				status: response.status,
				headers: { 'Content-Type': 'application/json' }
			});
		}

		if (!response.ok) {
			// R22-A: was: return mock zeros. Now: surface the upstream failure as a
			// 502 so the admin UI can render a real error state and retry.
			console.error(
				`[Products stats proxy] Backend error: ${response.status} ${response.statusText}`
			);
			return json(
				{ success: false, error: 'Unable to load products stats from backend.' },
				{ status: 502 }
			);
		}

		const data = await response.json();
		return json(data);
	} catch (err) {
		// R22-A: was: silent mock-zeros fallback. Now: 500 + log so an outage is visible.
		console.error('[Products stats proxy] Backend fetch failed:', err);
		return json({ success: false, error: 'Products stats backend unreachable.' }, { status: 500 });
	}
};
