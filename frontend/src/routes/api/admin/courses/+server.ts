/**
 * Admin Courses API Proxy
 * ICT 7 Grade - Routes requests to Rust API backend
 */

import { json, error, isHttpError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { requireAdminToken } from '$lib/server/auth';

import { env } from '$env/dynamic/private';
const BACKEND_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

/**
 * FIX-2026-04-26 (P0-2 / CC-2): normalize backend response into the
 * `{ success, data, error? }` envelope every page assumes. Backend mutation
 * endpoints sometimes return a bare entity (`{ id, ... }`); this wrapper
 * makes the proxy contract explicit.
 */
function envelope(
	raw: unknown,
	status: number
): { success: boolean; data?: unknown; error?: string; message?: string } {
	if (raw && typeof raw === 'object') {
		const obj = raw as Record<string, unknown>;
		// Already an envelope
		if (typeof obj.success === 'boolean') {
			return obj as { success: boolean; data?: unknown; error?: string; message?: string };
		}
		// Bare entity from a mutation — wrap it
		if (status < 400) {
			return { success: true, data: obj };
		}
		// Bare error body
		const message =
			(typeof obj.message === 'string' && obj.message) ||
			(typeof obj.error === 'string' && obj.error) ||
			'Request failed';
		return { success: false, error: message, message };
	}
	if (status < 400) return { success: true, data: raw };
	return { success: false, error: 'Request failed' };
}

export const GET: RequestHandler = async (event) => {
	// FIX-2026-04-26 (P1-1): use shared auth helper; surface 401 instead of
	// silently returning empty data on missing token.
	const token = requireAdminToken(event);
	const queryString = event.url.search || '';

	try {
		const response = await event.fetch(`${BACKEND_URL}/api/admin/courses${queryString}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});

		const raw = await response.json().catch(() => null);
		const wrapped = envelope(raw, response.status);

		if (!response.ok) {
			// FIX-2026-04-26 (P1-1): propagate 401 so the page can prompt re-login
			// instead of rendering "0 courses" forever.
			if (response.status === 401) error(401, wrapped.error || 'Unauthorized');
			return json(wrapped, { status: response.status });
		}
		return json(wrapped);
	} catch (err) {
		if (isHttpError(err)) throw err;
		console.error('[API] Admin courses list error:', err);
		return json({ success: false, error: 'Failed to fetch courses', data: null }, { status: 500 });
	}
};

export const POST: RequestHandler = async (event) => {
	// FIX-2026-04-26 (P1-1): require token; previously fail-open with no token.
	const token = requireAdminToken(event);
	try {
		const body = await event.request.json();

		const response = await event.fetch(`${BACKEND_URL}/api/admin/courses`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			},
			body: JSON.stringify(body)
		});

		const raw = await response.json().catch(() => null);
		const wrapped = envelope(raw, response.status);
		return json(wrapped, { status: response.status });
	} catch (err) {
		if (isHttpError(err)) throw err;
		console.error('[API] Admin courses create error:', err);
		return json({ success: false, error: 'Failed to create course' }, { status: 500 });
	}
};
