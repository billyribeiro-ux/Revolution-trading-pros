/**
 * Admin Boards Settings — Cloud Storage Proxy
 * ═══════════════════════════════════════════════════════════════════════════════════
 *
 * FIX-2026-04-26 (P0-3): The boards/settings page previously bound the storage
 * `secret_key` and `access_key` directly into a `<input type="password">` whose
 * `bind:value` two-way-binds back to `storageConfig.secret_key`. `type="password"`
 * only obscures *rendering* — the value is in the DOM, the JS heap, and travels
 * over the wire on every GET. For S3/R2/B2 credentials this is a credential-leak
 * class bug.
 *
 * This proxy enforces the standard "never round-trip secrets" pattern:
 *
 *   - On GET, redact `secret_key` (and `access_key`) to a placeholder if the
 *     backend includes them.
 *   - On PUT, drop redacted/empty `secret_key` / `access_key` fields from the body
 *     so an unchanged password does NOT overwrite the stored credential with the
 *     placeholder string.
 *
 * @version 1.0.0
 */

import { json, error as kitError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

const REDACTED_PLACEHOLDER = '__SECRET_UNCHANGED__';
const SECRET_FIELDS = ['secret_key', 'access_key'] as const;

interface BackendResult {
	data: unknown;
	status: number;
	reachable: boolean;
}

async function callBackend(endpoint: string, options?: RequestInit): Promise<BackendResult> {
	if (!BACKEND_URL) return { data: null, status: 0, reachable: false };

	try {
		const response = await fetch(`${BACKEND_URL}${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...options?.headers
			}
		});
		let parsed: unknown = null;
		const text = await response.text();
		if (text) {
			try {
				parsed = JSON.parse(text);
			} catch {
				parsed = { message: text };
			}
		}
		return { data: parsed, status: response.status, reachable: true };
	} catch (err) {
		console.warn(`Storage settings backend unreachable for ${endpoint}:`, err);
		return { data: null, status: 0, reachable: false };
	}
}

function extractErrorMessage(data: unknown, fallback: string): string {
	if (data && typeof data === 'object') {
		const obj = data as { message?: unknown; error?: unknown };
		if (typeof obj.message === 'string') return obj.message;
		if (typeof obj.error === 'string') return obj.error;
	}
	return fallback;
}

/**
 * Replace any populated secret field with a placeholder so the browser never
 * sees raw credentials. Empty/null fields are left as-is so the UI renders an
 * empty password input.
 */
function redactSecrets(payload: unknown): unknown {
	if (!payload || typeof payload !== 'object') return payload;

	// Some backends wrap as { success, data: {...} }. Redact inside `data` too.
	const root = payload as Record<string, unknown>;
	const next: Record<string, unknown> = { ...root };

	for (const key of SECRET_FIELDS) {
		if (typeof next[key] === 'string' && (next[key] as string).length > 0) {
			next[key] = REDACTED_PLACEHOLDER;
		}
	}

	if (root.data && typeof root.data === 'object') {
		next.data = redactSecrets(root.data);
	}

	return next;
}

/**
 * Strip secret fields whose value is the redacted placeholder OR an empty/null
 * value — those mean "leave the stored credential alone." Real new values pass
 * through verbatim.
 */
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

export const GET: RequestHandler = async ({ request, cookies }) => {
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) kitError(401, 'Unauthorized');

	const result = await callBackend('/api/admin/boards/settings/storage', {
		method: 'GET',
		headers: { Authorization: `Bearer ${token}` }
	});

	if (result.reachable && result.status >= 200 && result.status < 300) {
		return json(redactSecrets(result.data));
	}

	if (result.reachable) {
		kitError(result.status, extractErrorMessage(result.data, 'Failed to load storage config'));
	}

	return json({ data: null, _degraded: true }, { status: 503 });
};

export const PUT: RequestHandler = async ({ request, cookies }) => {
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) kitError(401, 'Unauthorized');

	let body: Record<string, unknown>;
	try {
		body = await request.json();
	} catch {
		return json({ success: false, error: 'Invalid JSON body' }, { status: 400 });
	}

	// FIX-2026-04-26 (P0-3): drop unchanged-secret placeholders so a save with the
	// password input untouched does NOT overwrite the stored credential.
	const sanitized = stripUnchangedSecrets(body);

	const result = await callBackend('/api/admin/boards/settings/storage', {
		method: 'PUT',
		headers: { Authorization: `Bearer ${token}` },
		body: JSON.stringify(sanitized)
	});

	if (result.reachable && result.status >= 200 && result.status < 300) {
		return json(redactSecrets(result.data));
	}

	if (result.reachable) {
		kitError(result.status, extractErrorMessage(result.data, 'Failed to save storage config'));
	}

	return json(
		{
			success: false,
			error: 'Storage settings backend is not reachable. Config was NOT saved.',
			_degraded: true
		},
		{ status: 503 }
	);
};
