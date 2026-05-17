/**
 * Admin Indicators sub-resource catch-all proxy
 * ══════════════════════════════════════════════════════════════════════
 *
 * Handles all sub-paths under /api/admin/indicators/{id}/:
 *   /files                    GET list / POST create
 *   /files/{fileId}           GET / PUT / DELETE
 *   /videos                   GET list / POST create
 *   /videos/{videoId}         GET / PUT / DELETE
 *   /toggle                   POST
 *   /documentation            GET list / POST create
 *
 * Built 2026-04-26-audit (P0-8): proxy layer was entirely missing; every
 * adminFetch('/api/admin/indicators/{id}/files', ...) returned a SvelteKit 404.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';

const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

async function proxy(method: string, backendPath: string, token: string, request: Request) {
	const headers: Record<string, string> = {
		Accept: 'application/json',
		Authorization: `Bearer ${token}`
	};

	let body: string | undefined;
	const hasBody = ['POST', 'PUT', 'PATCH'].includes(method);
	if (hasBody) {
		const contentType = request.headers.get('content-type') || 'application/json';
		headers['Content-Type'] = contentType;
		body = await request.text();
	}

	const response = await fetch(`${BACKEND_URL}${backendPath}`, {
		method,
		headers,
		...(body !== undefined ? { body } : {})
	});

	const text = await response.text();
	let payload: unknown = null;
	if (text) {
		try {
			payload = JSON.parse(text);
		} catch {
			payload = { error: text };
		}
	}

	if (!response.ok) {
		console.error(`[Indicators sub-resource ${method}] Backend ${response.status}:`, text);
		return json(
			{
				success: false,
				error: (payload as { error?: string })?.error || `Backend returned ${response.status}`
			},
			{ status: response.status }
		);
	}

	return json(payload ?? { success: true });
}

// Derive the full backend path from params
function backendPath(id: string, rest: string) {
	return `/api/admin/indicators/${id}/${rest}`;
}

export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { params, request, url } = event;

	const path = backendPath(params.id, params.rest ?? '') + (url.search || '');
	try {
		return await proxy('GET', path, token, request);
	} catch (err) {
		console.error('[Indicators sub-resource GET] Fetch failed:', err);
		return json({ success: false, error: err instanceof Error ? err.message : 'Failed to reach backend' }, { status: 502 });
	}
};

export const POST: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { params, request, url } = event;

	const path = backendPath(params.id, params.rest ?? '') + (url.search || '');
	try {
		return await proxy('POST', path, token, request);
	} catch (err) {
		console.error('[Indicators sub-resource POST] Fetch failed:', err);
		return json({ success: false, error: err instanceof Error ? err.message : 'Failed to reach backend' }, { status: 502 });
	}
};

export const PUT: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { params, request, url } = event;

	const path = backendPath(params.id, params.rest ?? '') + (url.search || '');
	try {
		return await proxy('PUT', path, token, request);
	} catch (err) {
		console.error('[Indicators sub-resource PUT] Fetch failed:', err);
		return json({ success: false, error: err instanceof Error ? err.message : 'Failed to reach backend' }, { status: 502 });
	}
};

export const PATCH: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { params, request, url } = event;

	const path = backendPath(params.id, params.rest ?? '') + (url.search || '');
	try {
		return await proxy('PATCH', path, token, request);
	} catch (err) {
		console.error('[Indicators sub-resource PATCH] Fetch failed:', err);
		return json({ success: false, error: err instanceof Error ? err.message : 'Failed to reach backend' }, { status: 502 });
	}
};

export const DELETE: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { params, request, url } = event;

	const path = backendPath(params.id, params.rest ?? '') + (url.search || '');
	try {
		return await proxy('DELETE', path, token, request);
	} catch (err) {
		console.error('[Indicators sub-resource DELETE] Fetch failed:', err);
		return json({ success: false, error: err instanceof Error ? err.message : 'Failed to reach backend' }, { status: 502 });
	}
};
