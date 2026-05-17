/**
 * Admin Indicators [id] API Proxy — /api/admin/indicators/:id
 * ══════════════════════════════════════════════════════════════════════
 *
 * Forwards GET / PUT / DELETE for a single indicator and any sub-resources:
 *   /api/admin/indicators/{id}
 *   /api/admin/indicators/{id}/files
 *   /api/admin/indicators/{id}/files/{fileId}
 *   /api/admin/indicators/{id}/videos
 *   /api/admin/indicators/{id}/videos/{videoId}
 *   /api/admin/indicators/{id}/toggle
 *   /api/admin/indicators/{id}/documentation
 *
 * Sub-paths beyond /{id} are handled by the sibling [...rest]/+server.ts.
 * This file handles only the base /{id} route.
 *
 * Built 2026-04-26-audit (P0-8): this proxy was entirely absent.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';

const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

/** Shared helper: forward a request to the backend and return a json() response. */
async function forwardToBackend(
	method: string,
	backendPath: string,
	token: string,
	contentType: string,
	body?: string
) {
	const headers: Record<string, string> = {
		Accept: 'application/json',
		Authorization: `Bearer ${token}`
	};
	if (body !== undefined) {
		headers['Content-Type'] = contentType;
	}

	const response = await fetch(`${BACKEND_URL}${backendPath}`, {
		method,
		headers,
		...(body !== undefined ? { body } : {})
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
		console.error(`[Indicators/${method}] Backend error ${response.status}:`, text);
		return json(
			{
				success: false,
				error: (payload as { error?: string })?.error || `Backend returned ${response.status}`
			},
			{ status: response.status }
		);
	}

	return json(payload ?? { success: true });
}

// ── GET ──────────────────────────────────────────────────────────────────────
export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { params, url } = event;

	const id = params.id;
	const queryString = url.search || '';

	try {
		return await forwardToBackend('GET', `/api/admin/indicators/${id}${queryString}`, token, 'application/json');
	} catch (err) {
		console.error('[Indicators/GET] Fetch failed:', err);
		return json({ success: false, error: err instanceof Error ? err.message : 'Failed to reach backend' }, { status: 502 });
	}
};

// ── PUT ──────────────────────────────────────────────────────────────────────
export const PUT: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { params, request } = event;

	const id = params.id;
	const contentType = request.headers.get('content-type') || 'application/json';

	try {
		const body = await request.text();
		return await forwardToBackend('PUT', `/api/admin/indicators/${id}`, token, contentType, body);
	} catch (err) {
		console.error('[Indicators/PUT] Fetch failed:', err);
		return json({ success: false, error: err instanceof Error ? err.message : 'Failed to reach backend' }, { status: 502 });
	}
};

// ── DELETE ───────────────────────────────────────────────────────────────────
export const DELETE: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { params } = event;

	const id = params.id;

	try {
		return await forwardToBackend('DELETE', `/api/admin/indicators/${id}`, token, 'application/json');
	} catch (err) {
		console.error('[Indicators/DELETE] Fetch failed:', err);
		return json({ success: false, error: err instanceof Error ? err.message : 'Failed to reach backend' }, { status: 502 });
	}
};

// ── PATCH ─────────────────────────────────────────────────────────────────────
export const PATCH: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { params, request } = event;

	const id = params.id;
	const contentType = request.headers.get('content-type') || 'application/json';

	try {
		const body = await request.text();
		return await forwardToBackend('PATCH', `/api/admin/indicators/${id}`, token, contentType, body);
	} catch (err) {
		console.error('[Indicators/PATCH] Fetch failed:', err);
		return json({ success: false, error: err instanceof Error ? err.message : 'Failed to reach backend' }, { status: 502 });
	}
};

// ── POST (for sub-paths like /toggle that accept POST on the base id route) ──
export const POST: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { params, request } = event;

	const id = params.id;
	const contentType = request.headers.get('content-type') || 'application/json';

	try {
		const body = await request.text();
		return await forwardToBackend('POST', `/api/admin/indicators/${id}`, token, contentType, body);
	} catch (err) {
		console.error('[Indicators/POST] Fetch failed:', err);
		return json({ success: false, error: err instanceof Error ? err.message : 'Failed to reach backend' }, { status: 502 });
	}
};
