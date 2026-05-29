/**
 * Connect Service Endpoint — Backend Proxy
 *
 * PRINCIPAL-2026-04-26 (audit 09-system §P0-2 / §P1-4):
 *   - Now requires `super-admin` (was: any cookie). Connecting third-party
 *     credentials (Stripe, AWS, Salesforce…) is privilege-bearing.
 *   - Validates `:key` against `/^[a-z][a-z0-9_]*$/` before forwarding to
 *     prevent path-traversal-shaped service keys (P1-4 #2).
 *   - Caps request body at 64 KiB (P1-4 #1) — credential payloads should be
 *     well under this; anything larger is hostile.
 *   - 4xx responses now forward upstream JSON verbatim (P2-5).
 */

import { env } from '$env/dynamic/private';
import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { requireSuperadmin } from '$lib/server/auth';

const API_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

const SERVICE_KEY_RE = /^[a-z][a-z0-9_]*$/;
const MAX_BODY_BYTES = 64 * 1024; // 64 KiB — credential payloads are tiny

// POST - Connect to a service
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

	const upstream = await fetch(`${API_URL}/api/admin/connections/${key}/connect`, {
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

// -------------------------------------------------------------------
// FIX-2026-04-26: old standalone in-memory implementation removed.
// See git history for the deleted block.
// -------------------------------------------------------------------
