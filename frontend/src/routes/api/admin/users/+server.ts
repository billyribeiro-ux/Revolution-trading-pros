/**
 * Users API Endpoint
 *
 * Pure proxy to the Rust admin API for user listing + creation. Surfaces
 * backend errors verbatim — never fabricates data.
 *
 * PRINCIPAL-2026-04-26 (audit 09-system §P0-2 / §P2-1 / §P2-2):
 *   - GET requires `admin` or higher (was: cookie-only).
 *   - POST requires `super-admin` — defense-in-depth on role-mutating create.
 *     The previous code accepted any caller-supplied `roles[]` and forwarded
 *     it. The Rust backend still enforces ACLs but this proxy now mirrors the
 *     check.
 *   - 4xx responses now forward the backend's body verbatim (audit P2-2);
 *     previously the proxy emitted a generic "Failed to create user" string,
 *     hiding field-level validation messages from the UI.
 *   - Auth pattern extracted to `$lib/server/auth.ts`.
 *
 * @version 3.0.0 - April 2026 (audit hardening)
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdmin, requireSuperadmin } from '$lib/server/auth';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

/**
 * Forward the upstream JSON body verbatim (status preserved). Used on both
 * success and 4xx so the client sees the backend's validation messages.
 */
async function forwardJson(upstream: Response): Promise<Response> {
	const text = await upstream.text();
	const headers: Record<string, string> = {};
	const ct = upstream.headers.get('content-type');
	if (ct) headers['Content-Type'] = ct;
	return new Response(text, { status: upstream.status, headers });
}

// GET - List users
export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const queryParams = event.url.searchParams.toString();
	const endpoint = `/api/admin/users${queryParams ? `?${queryParams}` : ''}`;

	const upstream = await fetch(`${API_URL}${endpoint}`, {
		headers: {
			Authorization: `Bearer ${token}`,
			Accept: 'application/json'
		}
	});

	if (!upstream.ok) {
		// Forward the upstream error body so the UI can show field-level
		// validation messages (audit P2-2).
		return forwardJson(upstream);
	}

	return forwardJson(upstream);
};

// POST - Create user (super-admin only — privilege-bearing roles[] in body)
export const POST: RequestHandler = async (event) => {
	const { token } = requireSuperadmin(event);

	let body: unknown;
	try {
		body = await event.request.json();
	} catch (err) {
		console.error('POST /api/admin/users error parsing body:', err);
		error(400, 'Invalid request body');
	}

	const upstream = await fetch(`${API_URL}/api/admin/users`, {
		method: 'POST',
		headers: {
			Authorization: `Bearer ${token}`,
			'Content-Type': 'application/json',
			Accept: 'application/json'
		},
		body: JSON.stringify(body)
	});

	return forwardJson(upstream);
};

// Keep `json` import for parity with sibling proxies (returned by helpers
// when needed). Suppress unused-warning at the type level.
void json;
