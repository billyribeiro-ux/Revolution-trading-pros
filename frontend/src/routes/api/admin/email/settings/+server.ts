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
 * R21-A: migrated to shared `fetchBackendWithStatus` helper. POST body now
 * narrowed with `isObject` (was: `as Record<string, unknown>` cast hid the
 * possibility of `null` / primitive body and the downstream `.password`
 * reads would silently no-op). Latent Bug §2 mitigation extended to this
 * file.
 *
 * @version 3.1.0 — 2026-05-20
 */

import { json, error, isHttpError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { requireAdmin, requireSuperadmin } from '$lib/server/auth';
import {
	fetchBackendWithStatus,
	isObject,
	extractBackendErrorMessage
} from '$lib/server/proxy-fetch';

const PASSWORD_MASK = '••••••••';

export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const authHeader = `Bearer ${token}`;

	const { data, status } = await fetchBackendWithStatus(
		'/api/admin/email/settings',
		{ headers: { Authorization: authHeader } },
		'[Email Settings API]'
	);

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
	if (isObject(data)) {
		const safe = { ...data };
		const hadPwd = typeof safe.password === 'string' && safe.password.length > 0;
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
		const body: unknown = await request.json();

		if (!isObject(body)) {
			return json({ success: false, error: 'Invalid request body' }, { status: 400 });
		}

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

		const { data, status } = await fetchBackendWithStatus(
			'/api/admin/email/settings',
			{
				method: 'POST',
				headers: { Authorization: authHeader },
				body: JSON.stringify(forwarded)
			},
			'[Email Settings API]'
		);

		if (status === 401 || status === 403) error(status, 'Unauthorized');
		if (status >= 400) {
			error(status, extractBackendErrorMessage(data, 'Failed to save settings'));
		}

		return json(data ?? { success: true, message: 'Settings saved successfully!' });
	} catch (err) {
		if (isHttpError(err)) throw err;
		console.error('POST /api/admin/email/settings error:', err);
		error(400, 'Invalid request body');
	}
};
