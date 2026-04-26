/**
 * Connect Service Endpoint — Backend Proxy
 * FIX-2026-04-26: replaced standalone in-memory implementation with real backend proxy.
 */

import { env } from '$env/dynamic/private';
import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// FIX-2026-04-26: canonical env pattern
const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

// POST - Connect to a service
export const POST: RequestHandler = async ({ params, cookies, fetch, request }) => {
	const { key } = params;

	const token = cookies.get('rtp_access_token');
	if (!token) error(401, 'Unauthorized');

	const body = await request.json();
	const res = await fetch(`${API_URL}/api/admin/connections/${key}/connect`, {
		method: 'POST',
		headers: {
			Authorization: `Bearer ${token}`,
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(body)
	});
	if (!res.ok) error(res.status as Parameters<typeof error>[0], `Backend returned ${res.status}`);
	return json(await res.json(), { status: res.status });
};

// -------------------------------------------------------------------
// FIX-2026-04-26: old standalone in-memory implementation follows
// -------------------------------------------------------------------
// const connections: Map<string, any> = new Map();
// export const POST: RequestHandler = async ({ params, request }) => {
//   const { key } = params;
//   const body = await request.json();
//   const { credentials, environment = 'production' } = body;
//   if (!credentials || Object.keys(credentials).length === 0) { ... }
//   const validationError = await validateConnectionCredentials(key ?? '', credentials);
//   if (validationError) { ... }
//   const connection = { id: `conn_${Date.now()}...`, ... };
//   connections.set(key ?? '', connection);
//   return json({ success: true, data: { ... } });
// };
// async function validateConnectionCredentials(...) { ... }
// function encryptCredentials(...) { ... }
