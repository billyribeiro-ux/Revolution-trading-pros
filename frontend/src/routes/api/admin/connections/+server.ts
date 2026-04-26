/**
 * API Connections Management Proxy
 *
 * PRINCIPAL-2026-04-26 (audit 09-system §P2-1 / §P2-4):
 *   - Auth-pattern unified to `requireAdmin` (GET) / `requireSuperadmin`
 *     (POST creates a new connection definition — privilege-bearing).
 *   - Status casts to `error()` are safer-bounded.
 *   - 4xx responses forward upstream body verbatim.
 */

import { env } from '$env/dynamic/private';
import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { requireAdmin, requireSuperadmin } from '$lib/server/auth';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

async function forwardJson(upstream: Response): Promise<Response> {
	const text = await upstream.text();
	const headers: Record<string, string> = {};
	const ct = upstream.headers.get('content-type');
	if (ct) headers['Content-Type'] = ct;
	return new Response(text, { status: upstream.status, headers });
}

// GET - List all connections and available services
export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);

	const upstream = await fetch(`${API_URL}/api/admin/connections`, {
		headers: {
			Authorization: `Bearer ${token}`,
			Accept: 'application/json'
		}
	});
	if (!upstream.ok) {
		// Forward upstream JSON so the UI sees real error details.
		return forwardJson(upstream);
	}
	return forwardJson(upstream);
};

// POST - Initiate a connection (OAuth or setup) — super-admin only
export const POST: RequestHandler = async (event) => {
	const { token } = requireSuperadmin(event);

	let body: unknown;
	try {
		body = await event.request.json();
	} catch {
		error(400, 'Invalid request body');
	}

	const upstream = await fetch(`${API_URL}/api/admin/connections`, {
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

// -------------------------------------------------------------------
// FIX-2026-04-26: old standalone in-memory implementation removed.
// See git history for the deleted block.
// -------------------------------------------------------------------
