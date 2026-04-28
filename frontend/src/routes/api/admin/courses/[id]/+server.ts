/**
 * Admin Course Detail API Proxy
 * ICT 7 Grade - Routes requests to Rust API backend
 */

import { json, error, isHttpError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { requireAdminToken } from '$lib/server/auth';

import { env } from '$env/dynamic/private';
const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

/**
 * FIX-2026-04-26 (P0-2 / CC-2): unify response into the
 * `{ success, data, error? }` envelope every page assumes.
 */
function envelope(
	raw: unknown,
	status: number
): { success: boolean; data?: unknown; error?: string; message?: string } {
	if (raw && typeof raw === 'object') {
		const obj = raw as Record<string, unknown>;
		if (typeof obj.success === 'boolean') {
			return obj as { success: boolean; data?: unknown; error?: string; message?: string };
		}
		if (status < 400) {
			return { success: true, data: obj };
		}
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
	// FIX-2026-04-26 (P1-1): require token; previously cookie-only with fail-open.
	const token = requireAdminToken(event);
	const { id } = event.params;

	try {
		const response = await event.fetch(`${BACKEND_URL}/api/admin/courses/${id}`, {
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
			if (response.status === 401) error(401, wrapped.error || 'Unauthorized');
			return json(wrapped, { status: response.status });
		}
		return json(wrapped);
	} catch (err) {
		if (isHttpError(err)) throw err;
		console.error('[API] Admin course get error:', err);
		return json({ success: false, error: 'Failed to fetch course' }, { status: 500 });
	}
};

export const PUT: RequestHandler = async (event) => {
	const token = requireAdminToken(event);
	const { id } = event.params;
	try {
		const body = await event.request.json();

		const response = await event.fetch(`${BACKEND_URL}/api/admin/courses/${id}`, {
			method: 'PUT',
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
		console.error('[API] Admin course update error:', err);
		return json({ success: false, error: 'Failed to update course' }, { status: 500 });
	}
};

export const DELETE: RequestHandler = async (event) => {
	const token = requireAdminToken(event);
	const { id } = event.params;

	try {
		const response = await event.fetch(`${BACKEND_URL}/api/admin/courses/${id}`, {
			method: 'DELETE',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});

		const raw = await response.json().catch(() => null);
		const wrapped = envelope(raw, response.status);
		return json(wrapped, { status: response.status });
	} catch (err) {
		if (isHttpError(err)) throw err;
		console.error('[API] Admin course delete error:', err);
		return json({ success: false, error: 'Failed to delete course' }, { status: 500 });
	}
};
