/** Preserve the Rust deal contract through the authenticated same-origin proxy. */
import { API_BASE_URL, BACKEND_URL as ENV_BACKEND_URL } from '$app/env/private';
import { requireAdmin } from '#lib/server/auth.js';
import type { RequestHandler } from '@sveltejs/kit';
const backend = API_BASE_URL || ENV_BACKEND_URL || 'http://localhost:8080';

async function forward(
	event: Parameters<RequestHandler>[0],
	method: 'GET' | 'POST'
): Promise<Response> {
	const { token } = requireAdmin(event);
	let body: unknown;
	if (method === 'POST') {
		try {
			body = await event.request.json();
		} catch {
			return Response.json({ error: 'Request must contain valid JSON' }, { status: 400 });
		}
		if (!body || typeof body !== 'object' || Array.isArray(body))
			return Response.json({ error: 'Deal must be a JSON object' }, { status: 400 });
	}
	try {
		const response = await event.fetch(
			`${backend}/api/admin/crm/deals${method === 'GET' ? event.url.search : ''}`,
			{
				method,
				headers: { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' },
				...(method === 'POST' ? { body: JSON.stringify(body) } : {}),
				signal: AbortSignal.timeout(30000)
			}
		);
		const data: unknown = await response.json().catch(() => null);
		if (!response.ok) {
			const message =
				data && typeof data === 'object' && 'error' in data && typeof data.error === 'string'
					? data.error
					: 'Unable to process deal request';
			return Response.json({ error: message }, { status: response.status });
		}
		if (!data || typeof data !== 'object')
			return Response.json({ error: 'Invalid deal response from backend' }, { status: 502 });
		if (method === 'GET') {
			if (
				!('data' in data) ||
				!Array.isArray(data.data) ||
				!('meta' in data) ||
				!('summaries' in data) ||
				!Array.isArray(data.summaries)
			)
				return Response.json({ error: 'Invalid deal list response from backend' }, { status: 502 });
			return Response.json(data);
		}
		return Response.json({ data }, { status: 201 });
	} catch {
		return Response.json({ error: 'Unable to reach deal service' }, { status: 503 });
	}
}
export const GET: RequestHandler = (event) => forward(event, 'GET');
export const POST: RequestHandler = (event) => forward(event, 'POST');
