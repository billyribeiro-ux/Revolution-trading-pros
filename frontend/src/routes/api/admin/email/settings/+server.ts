/**
 * Email Settings API Endpoint
 *
 * FIX-2026-04-26 (07-marketing audit P0-1, P0-2):
 *   - Added rtp_access_token cookie auth + Bearer fallback. Previously this
 *     endpoint was wide open: anyone could GET / POST it.
 *   - Removed the in-memory `let emailSettings` module-scoped store. SMTP
 *     credentials must NEVER live in a process-global on a multi-tenant
 *     edge runtime. We now proxy through to the Rust backend, which is the
 *     single source of truth for encrypted credential storage.
 *   - Strip masked-password sentinel (`••••••••`) before forwarding so the
 *     literal bullets never reach the SMTP test handler. The client should
 *     omit `password` to keep the stored value; we enforce that here too.
 *   - Never echo plaintext password back to the client. Rust backend should
 *     return `has_password: boolean` instead — we forward whatever it sends
 *     but explicitly delete `password` from the GET response as defense in
 *     depth.
 *
 * @version 3.0.0 — 2026-04-26
 */

import { json, error, isHttpError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
import { requireAdmin, requireSuperadmin } from '$lib/server/auth';
const PROD_BACKEND =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

const PASSWORD_MASK = '••••••••';

async function fetchFromBackend(
	endpoint: string,
	options?: RequestInit
): Promise<{ data: unknown; status: number }> {
	const backendUrl = PROD_BACKEND;

	try {
		const response = await fetch(`${backendUrl}/api${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...(options?.headers || {})
			}
		});

		// Tolerate empty 204 / non-JSON 5xx bodies.
		const text = await response.text();
		let data: unknown = null;
		if (text) {
			try {
				data = JSON.parse(text);
			} catch {
				data = { error: text };
			}
		}
		return { data, status: response.status };
	} catch (err) {
		console.error(`Backend error for ${endpoint}:`, err);
		return { data: null, status: 500 };
	}
}

export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const authHeader = `Bearer ${token}`;

	const { data, status } = await fetchFromBackend('/admin/email/settings', {
		headers: { Authorization: authHeader }
	});

	if (status === 401 || status === 403) error(status, 'Unauthorized');
	if (status >= 400 || !data) {
		// Default empty settings on backend error so the form can still render.
		return json({
			provider: 'smtp',
			host: '',
			port: 587,
			username: '',
			has_password: false,
			encryption: 'tls',
			from_address: '',
			from_name: 'Revolution Trading Pros',
			error: 'Failed to load email settings'
		});
	}

	// Defense in depth: scrub any plaintext password before responding to the
	// browser. The Rust backend SHOULD return `has_password: boolean` only,
	// but if it ever leaks the column we mask it here.
	if (data && typeof data === 'object') {
		const safe = { ...(data as Record<string, unknown>) };
		const hadPwd = typeof safe.password === 'string' && (safe.password as string).length > 0;
		delete safe.password;
		if (!('has_password' in safe)) safe.has_password = hadPwd;
		return json(safe);
	}

	return json(data);
};

export const POST: RequestHandler = async (event) => {
	const { token } = requireSuperadmin(event);
	const { request } = event;
	const authHeader = `Bearer ${token}`;

	try {
		const body = (await request.json()) as Record<string, unknown>;

		// Validate required fields client-side as a fast-fail.
		if (!body.host || !body.port || !body.from_address || !body.from_name) {
			return json({ success: false, error: 'Missing required fields' }, { status: 400 });
		}

		// Strip the masked-password sentinel before forwarding so the backend
		// can treat "missing password" as "keep existing". Two cases:
		//  - body.password === '••••••••' (legacy mask we used to send)
		//  - body.password === ''         (new contract — empty means "no change")
		const forwarded: Record<string, unknown> = { ...body };
		if (
			typeof forwarded.password !== 'string' ||
			forwarded.password === PASSWORD_MASK ||
			forwarded.password === ''
		) {
			delete forwarded.password;
		}

		const { data, status } = await fetchFromBackend('/admin/email/settings', {
			method: 'POST',
			headers: { Authorization: authHeader },
			body: JSON.stringify(forwarded)
		});

		if (status === 401 || status === 403) error(status, 'Unauthorized');
		if (status >= 400) {
			const message =
				(data && typeof data === 'object' && ('message' in data || 'error' in data)
					? (data as { message?: string; error?: string }).message ||
						(data as { error?: string }).error
					: undefined) || 'Failed to save settings';
			error(status, message);
		}

		return json(data ?? { success: true, message: 'Settings saved successfully!' });
	} catch (err) {
		if (isHttpError(err)) throw err;
		console.error('POST /api/admin/email/settings error:', err);
		error(400, 'Invalid request body');
	}
};
