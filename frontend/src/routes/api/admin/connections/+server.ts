/**
 * API Connections Management Proxy
 * FIX-2026-04-26: replaced standalone in-memory implementation with real backend proxy.
 * Old in-memory implementation is preserved below as comments.
 */

import { env } from '$env/dynamic/private';
import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// FIX-2026-04-26: canonical env pattern
const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

// GET - List all connections and available services
export const GET: RequestHandler = async ({ cookies, fetch }) => {
	const token = cookies.get('rtp_access_token');
	if (!token) error(401, 'Unauthorized');

	const res = await fetch(`${API_URL}/api/admin/connections`, {
		headers: { Authorization: `Bearer ${token}` }
	});
	if (!res.ok) error(res.status as Parameters<typeof error>[0], `Backend returned ${res.status}`);
	return json(await res.json());
};

// POST - Initiate a connection (OAuth or setup)
export const POST: RequestHandler = async ({ cookies, fetch, request }) => {
	const token = cookies.get('rtp_access_token');
	if (!token) error(401, 'Unauthorized');

	const body = await request.json();
	const res = await fetch(`${API_URL}/api/admin/connections`, {
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
// interface ServiceField { ... }
// interface ServiceDefinition { ... }
// interface ConnectionData { ... }
// const connections: Map<string, ConnectionData> = new Map();
// const serviceDefinitions: ServiceDefinition[] = [ ... ];
// function getService(key: string) { ... }
// function buildCategories() { ... }
// function buildSummary() { ... }
// export const GET: RequestHandler = async () => { return json({...}); };
// export const POST: RequestHandler = async ({ request }) => { ... };
// (full body ~620 lines — see git history for details)
