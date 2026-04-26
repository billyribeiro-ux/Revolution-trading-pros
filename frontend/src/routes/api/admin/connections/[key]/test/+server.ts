/**
 * Test Service Connection Endpoint — Backend Proxy
 *
 * PRINCIPAL-2026-04-26 (audit 09-system §P0-4):
 *   - Built per CREATE-not-DELETE rule: previously the settings + connections
 *     pages POSTed credential payloads to `/api/admin/connections/{key}/test`
 *     with no `+server.ts` handler — credentials leaked into 404 logs.
 *   - Now a real super-admin-gated proxy that mirrors `connect`'s shape.
 *   - Validates `:key` and caps the body at 64 KiB.
 *
 * Backend route: `POST /api/admin/connections/{key}/test` — if the Rust API
 * does not yet implement this route, callers will receive whatever status
 * the backend returns (404, 405, etc) verbatim, NOT a fake success.
 */

import { env } from '$env/dynamic/private';
import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { requireSuperadmin } from '$lib/server/auth';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

const SERVICE_KEY_RE = /^[a-z][a-z0-9_]*$/;
const MAX_BODY_BYTES = 64 * 1024;

// POST - Test a service connection with caller-supplied credentials
export const POST: RequestHandler = async (event) => {
	const { token } = requireSuperadmin(event);

	const key = event.params.key;
	if (!key || !SERVICE_KEY_RE.test(key)) {
		error(400, 'Invalid service key');
	}

	const lengthHeader = event.request.headers.get('content-length');
	if (lengthHeader && Number(lengthHeader) > MAX_BODY_BYTES) {
		error(413, 'Request body too large');
	}

	const raw = await event.request.text();
	if (raw.length > MAX_BODY_BYTES) {
		error(413, 'Request body too large');
	}

	const upstream = await fetch(`${API_URL}/api/admin/connections/${key}/test`, {
		method: 'POST',
		headers: {
			Authorization: `Bearer ${token}`,
			'Content-Type': 'application/json',
			Accept: 'application/json'
		},
		body: raw
	});

	const text = await upstream.text();
	const headers: Record<string, string> = {};
	const ct = upstream.headers.get('content-type');
	if (ct) headers['Content-Type'] = ct;
	return new Response(text, { status: upstream.status, headers });
};
