/**
 * Coupons API Proxy
 * Routes requests to Rust API backend to avoid CORS issues
 *
 * @version 2.1.0 - January 2026 - Cloudflare compatible
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

// Production fallback - Rust API on Fly.io
import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';
const PROD_BACKEND = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

// Empty data for graceful degradation
const EMPTY_DATA = {
	coupons: [],
	total: 0
};

/**
 * Get backend URL - always use production backend for reliability
 * Cloudflare platform.env is typed differently, so we use the hardcoded fallback
 */
function getBackendUrl(): string {
	return PROD_BACKEND;
}

export const GET: RequestHandler = async (event) => {
	// PRINCIPAL-2026-05-17 (audit FULL_REPO_AUDIT_2026-05-17 §P2-F): replaced the
	// "no token → return EMPTY_DATA 200" anti-pattern (which masked auth state
	// from the client and let the list page render "No coupons yet" for an
	// unauthenticated/expired session) with the canonical RBAC gate.
	// `requireAdmin` throws `error(401|403)` so the proxy short-circuits with a
	// real status, exactly like every other admin proxy.
	const { token } = requireAdmin(event);
	try {
		const backendUrl = getBackendUrl();
		const authHeader = `Bearer ${token}`;

		const response = await fetch(`${backendUrl}/api/admin/coupons`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: authHeader
			}
		});

		// FIX-2026-04-26 (P2-1): forward real backend status. Previously a 5xx
		// was silently masked as 200 + EMPTY_DATA, which made the list page
		// render "No coupons yet" and let admins create duplicates of coupons
		// that already exist. Forward the status so the page's retry path
		// (admin/coupons/+page.svelte) can fire.
		if (!response.ok) {
			const text = await response.text();
			return new Response(text || JSON.stringify(EMPTY_DATA), {
				status: response.status,
				headers: { 'Content-Type': 'application/json' }
			});
		}

		const text = await response.text();
		if (!text) {
			return json(EMPTY_DATA);
		}

		try {
			const data = JSON.parse(text);
			return json(data);
		} catch {
			return json(EMPTY_DATA);
		}
	} catch (error) {
		// Network-level failure — surface 502 so the page knows the upstream
		// is unreachable instead of treating it as "no coupons exist".
		console.error('[API] List coupons error:', error);
		return json({ message: 'Backend unavailable' }, { status: 502 });
	}
};

export const POST: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { request } = event;
	try {
		const backendUrl = getBackendUrl();
		const authHeader = `Bearer ${token}`;
		const body = await request.json();

		const response = await fetch(`${backendUrl}/api/admin/coupons`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: authHeader
			},
			body: JSON.stringify(body)
		});

		const text = await response.text();
		if (!text) {
			return json({ message: 'Empty response from server' }, { status: response.status });
		}

		try {
			const data = JSON.parse(text);
			return json(data, { status: response.status });
		} catch {
			return json({ message: 'Invalid response from server' }, { status: 500 });
		}
	} catch (error) {
		console.error('[API] Create coupon error:', error);
		return json({ message: 'Failed to create coupon' }, { status: 500 });
	}
};
