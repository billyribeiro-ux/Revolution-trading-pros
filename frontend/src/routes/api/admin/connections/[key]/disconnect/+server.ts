/**
 * Disconnect Service Endpoint — Backend Proxy
 *
 * PRINCIPAL-2026-04-26 (audit 09-system §P0-2 / §P1-4):
 *   - Now requires `super-admin`. Disconnecting privilege-bearing
 *     integrations (e.g. `stripe`, `aws_s3`) is irreversible without the
 *     credentials.
 *   - Validates `:key` against `/^[a-z][a-z0-9_]*$/`.
 *   - 4xx responses forward upstream JSON verbatim.
 */

import { env } from '$env/dynamic/private';
import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { requireSuperadmin } from '$lib/server/auth';

const API_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

const SERVICE_KEY_RE = /^[a-z][a-z0-9_]*$/;

// POST - Disconnect a service
export const POST: RequestHandler = async (event) => {
	const { token } = requireSuperadmin(event);

	const key = event.params.key;
	if (!key || !SERVICE_KEY_RE.test(key)) {
		error(400, 'Invalid service key');
	}

	const upstream = await fetch(`${API_URL}/api/admin/connections/${key}/disconnect`, {
		method: 'POST',
		headers: {
			Authorization: `Bearer ${token}`,
			Accept: 'application/json'
		}
	});

	const text = await upstream.text();
	const headers: Record<string, string> = {};
	const ct = upstream.headers.get('content-type');
	if (ct) headers['Content-Type'] = ct;
	return new Response(text, { status: upstream.status, headers });
};

// -------------------------------------------------------------------
// FIX-2026-04-26: old standalone in-memory implementation removed.
// -------------------------------------------------------------------
