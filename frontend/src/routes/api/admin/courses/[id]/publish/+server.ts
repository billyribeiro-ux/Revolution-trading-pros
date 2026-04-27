/**
 * Admin Course Publish API Proxy
 * ICT 7 Grade - Routes requests to Rust API backend
 */

import { json, isHttpError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { requireAdminToken } from '$lib/server/auth';

import { env } from '$env/dynamic/private';
const PROD_BACKEND =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';
const BACKEND_URL = PROD_BACKEND;

function envelope(
	raw: unknown,
	status: number
): { success: boolean; data?: unknown; error?: string; message?: string } {
	if (raw && typeof raw === 'object') {
		const obj = raw as Record<string, unknown>;
		if (typeof obj.success === 'boolean') {
			return obj as { success: boolean; data?: unknown; error?: string; message?: string };
		}
		if (status < 400) return { success: true, data: obj };
		const message =
			(typeof obj.message === 'string' && obj.message) ||
			(typeof obj.error === 'string' && obj.error) ||
			'Request failed';
		return { success: false, error: message, message };
	}
	if (status < 400) return { success: true, data: raw };
	return { success: false, error: 'Request failed' };
}

export const POST: RequestHandler = async (event) => {
	// FIX-2026-04-26 (P1-1): require token via shared helper.
	const token = requireAdminToken(event);
	const { id } = event.params;
	try {
		const response = await event.fetch(`${BACKEND_URL}/api/admin/courses/${id}/publish`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});

		const raw = await response.json().catch(() => null);
		return json(envelope(raw, response.status), { status: response.status });
	} catch (err) {
		if (isHttpError(err)) throw err;
		console.error('[API] Admin course publish error:', err);
		return json({ success: false, error: 'Failed to publish course' }, { status: 500 });
	}
};
