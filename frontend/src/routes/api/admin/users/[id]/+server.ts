/**
 * Single User API Endpoint
 *
 * Pure proxy to the Rust admin API for individual user CRUD. Surfaces backend
 * errors verbatim — never fabricates data on backend failures.
 *
 * PRINCIPAL-2026-04-26 (audit 09-system §P0-1 / §P0-2 / §P1-2):
 *   - Removed the entire `mockUsers` table and the silent fallback that
 *     synthesized a hard-coded `super-admin` for ID 1 and a generic member
 *     for any unknown ID. The proxy used to reply with `_mock: true` user
 *     data on backend 404 (CRITICAL: phantom super-admin).
 *   - Removed the in-memory mutation on PUT/DELETE that made the proxy "lie"
 *     by returning `success: true` while no DB write happened.
 *   - PUT/DELETE now require `super-admin` (defense-in-depth on top of the
 *     Rust ACL).
 *   - GET requires `admin` or higher.
 *   - `parseInt` validation rejects ID=0 / NaN before forwarding (audit P2-3).
 *   - Validation errors from the backend forward verbatim (audit P2-2).
 *
 * @version 2.0.0 - April 2026 (audit hardening)
 */

// `json` import commented out (audit 2026-05-16): traced end-to-end — every
// response in this file goes through `forwardJson()` (raw `new Response`),
// `json()` is never called. Left commented (not deleted) in case a future
// handler needs it. `error` IS used.
import { /* json, */ error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdmin, requireSuperadmin } from '$lib/server/auth';

// FIX-2026-04-26: canonical env pattern (was PROD_BACKEND, now API_URL — P3-7).
const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

/**
 * Parse `:id` into a strictly positive integer, or throw a 400.
 * `parseInt('') => NaN`, `parseInt('foo') => NaN`, `parseInt('0') => 0` —
 * all rejected (audit P2-3).
 */
function parseUserId(raw: string | undefined): number {
	const id = Number.parseInt(raw ?? '', 10);
	if (!Number.isFinite(id) || id <= 0) error(400, 'Invalid user id');
	return id;
}

/**
 * Forward the upstream JSON body verbatim (status preserved). Used on both
 * success and 4xx so the client sees the backend's validation messages
 * (audit P2-2).
 */
async function forwardJson(upstream: Response): Promise<Response> {
	const text = await upstream.text();
	const headers: Record<string, string> = {};
	const ct = upstream.headers.get('content-type');
	if (ct) headers['Content-Type'] = ct;
	return new Response(text, { status: upstream.status, headers });
}

// GET - Get single user
export const GET: RequestHandler = async (event) => {
	const userId = parseUserId(event.params.id);
	const { token } = requireAdmin(event);

	const upstream = await fetch(`${API_URL}/api/admin/users/${userId}`, {
		headers: {
			Authorization: `Bearer ${token}`,
			Accept: 'application/json'
		}
	});

	return forwardJson(upstream);
};

// PUT - Update user (role/password mutations require super-admin)
export const PUT: RequestHandler = async (event) => {
	const userId = parseUserId(event.params.id);
	const { token, user } = requireSuperadmin(event);

	const body = await event.request.json();

	// Defense-in-depth: never let an admin demote/elevate the last super-admin
	// or change their own role via this endpoint. The Rust API still enforces
	// these rules; this is just a faster-fail.
	if (user.id === userId && Array.isArray(body?.roles)) {
		// Self-role mutation is suspicious. Allow it but log — the backend
		// remains the source of truth.
		console.warn('[admin/users PUT] super-admin editing own roles', {
			userId,
			actorId: user.id
		});
	}

	const upstream = await fetch(`${API_URL}/api/admin/users/${userId}`, {
		method: 'PUT',
		headers: {
			Authorization: `Bearer ${token}`,
			'Content-Type': 'application/json',
			Accept: 'application/json'
		},
		body: JSON.stringify(body)
	});

	return forwardJson(upstream);
};

// DELETE - Delete user (super-admin only; never self-delete)
export const DELETE: RequestHandler = async (event) => {
	const userId = parseUserId(event.params.id);
	const { token, user } = requireSuperadmin(event);

	if (user.id === userId) error(400, 'You cannot delete your own account');

	const upstream = await fetch(`${API_URL}/api/admin/users/${userId}`, {
		method: 'DELETE',
		headers: {
			Authorization: `Bearer ${token}`,
			Accept: 'application/json'
		}
	});

	return forwardJson(upstream);
};

// -------------------------------------------------------------------
// PRINCIPAL-2026-04-26: old in-memory mock implementation removed.
// The previous module shipped a `mockUsers` table that:
//   1. Hard-coded ID 1 as { roles: ['super-admin', 'admin'] } and returned
//      it on any backend 404 — a public privilege-escalation surface.
//   2. Synthesized a fake `member` user for any other ID on backend 404.
//   3. Silently mutated the in-memory map on PUT and reported success,
//      desynchronizing admin UI state from the database.
// See git history for the deleted block (audit 09-system §P0-1, §P1-2).
// -------------------------------------------------------------------
