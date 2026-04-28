/**
 * Admin Boards Settings — Cloud Storage Test Connection Proxy
 * ═══════════════════════════════════════════════════════════════════════════════════
 *
 * FIX-2026-04-26 (P0-3): Test endpoint sibling for the storage proxy. If the
 * caller sends the redacted-placeholder secret, drop it before forwarding so the
 * backend test uses its stored credential instead of the masked placeholder.
 */

import { json, error as kitError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
const PROD_BACKEND =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

const REDACTED_PLACEHOLDER = '__SECRET_UNCHANGED__';
const SECRET_FIELDS = ['secret_key', 'access_key'] as const;

function stripUnchangedSecrets(body: Record<string, unknown>): Record<string, unknown> {
	const next: Record<string, unknown> = { ...body };
	for (const key of SECRET_FIELDS) {
		const value = next[key];
		if (
			value === REDACTED_PLACEHOLDER ||
			value === '' ||
			value === null ||
			value === undefined
		) {
			delete next[key];
		}
	}
	return next;
}

function extractErrorMessage(data: unknown, fallback: string): string {
	if (data && typeof data === 'object') {
		const obj = data as { message?: unknown; error?: unknown };
		if (typeof obj.message === 'string') return obj.message;
		if (typeof obj.error === 'string') return obj.error;
	}
	return fallback;
}

export const POST: RequestHandler = async ({ request, cookies }) => {
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) kitError(401, 'Unauthorized');

	let body: Record<string, unknown>;
	try {
		body = await request.json();
	} catch {
		return json({ success: false, message: 'Invalid JSON body' }, { status: 400 });
	}

	const sanitized = stripUnchangedSecrets(body);

	try {
		const response = await fetch(`${PROD_BACKEND}/api/admin/boards/settings/storage/test`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			},
			body: JSON.stringify(sanitized)
		});

		const text = await response.text();
		let data: unknown = null;
		if (text) {
			try {
				data = JSON.parse(text);
			} catch {
				data = { message: text };
			}
		}

		if (response.status >= 200 && response.status < 300) {
			return json(data);
		}
		kitError(response.status, extractErrorMessage(data, 'Storage connection test failed'));
	} catch (err) {
		console.warn('Storage test backend unreachable:', err);
		return json(
			{ success: false, message: 'Storage backend is not reachable.' },
			{ status: 503 }
		);
	}
};
