/**
 * Disconnect Service Endpoint — Backend Proxy
 * FIX-2026-04-26: replaced standalone in-memory implementation with real backend proxy.
 */

import { env } from '$env/dynamic/private';
import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// FIX-2026-04-26: canonical env pattern
const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

// POST - Disconnect a service
export const POST: RequestHandler = async ({ params, cookies, fetch }) => {
	const { key } = params;

	const token = cookies.get('rtp_access_token');
	if (!token) error(401, 'Unauthorized');

	const res = await fetch(`${API_URL}/api/admin/connections/${key}/disconnect`, {
		method: 'POST',
		headers: { Authorization: `Bearer ${token}` }
	});
	if (!res.ok) error(res.status as Parameters<typeof error>[0], `Backend returned ${res.status}`);
	return json(await res.json(), { status: res.status });
};

// -------------------------------------------------------------------
// FIX-2026-04-26: old standalone in-memory implementation follows
// -------------------------------------------------------------------
// const connections: Map<string, any> = new Map();
// export const POST: RequestHandler = async ({ params }) => {
//   const { key } = params;
//   const connection = connections.get(key ?? '');
//   if (!connection) { error(404, `No connection found for service '${key}'`); }
//   connections.delete(key ?? '');
//   return json({ success: true, data: { service_key: key, disconnected_at: new Date().toISOString() } });
// };
