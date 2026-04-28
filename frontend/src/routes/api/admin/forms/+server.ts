/**
 * Admin Forms Proxy
 *
 * FIX-2026-04-26 (07-marketing audit P0-6):
 *   admin/popups/new called `/api/admin/forms` to populate its "Form
 *   Integration" select. The proxy did not exist — request fell through
 *   the catch-all to a backend route that may not match — so the dropdown
 *   was always empty and admins could "save" popups with `has_form: true`
 *   and `form_id: undefined`. We now expose an explicit proxy.
 *
 * Auth: rtp_access_token cookie (preferred) or Bearer header fallback.
 *
 * @version 1.0.0 — 2026-04-26
 */

import { json, error, isHttpError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
const PROD_BACKEND =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

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

function readAuth(request: Request, cookies: Parameters<RequestHandler>[0]['cookies']): string {
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) error(401, 'Unauthorized');
	return `Bearer ${token}`;
}

export const GET: RequestHandler = async ({ url, request, cookies }) => {
	const authHeader = readAuth(request, cookies);
	const queryParams = url.searchParams.toString();
	// Backend exposes forms under /api/forms (admin scope is enforced by
	// the bearer token + role check on the Rust side).
	const endpoint = `/forms${queryParams ? `?${queryParams}` : ''}`;

	const { data, status } = await fetchFromBackend(endpoint, {
		headers: { Authorization: authHeader }
	});

	if (status === 401 || status === 403) error(status, 'Unauthorized');
	if (status >= 400 || !data) {
		// Return empty list with a flag so consumers can render gracefully
		// instead of throwing. The popup builder uses `data.forms || data || []`.
		return json({ success: false, forms: [], total: 0, error: 'Failed to fetch forms' });
	}

	return json(data);
};

export const POST: RequestHandler = async ({ request, cookies }) => {
	const authHeader = readAuth(request, cookies);
	try {
		const body = await request.json();
		const { data, status } = await fetchFromBackend('/forms', {
			method: 'POST',
			headers: { Authorization: authHeader },
			body: JSON.stringify(body)
		});

		if (status === 401 || status === 403) error(status, 'Unauthorized');
		if (status >= 400) {
			const message =
				(data && typeof data === 'object' && ('message' in data || 'error' in data)
					? (data as { message?: string; error?: string }).message ||
						(data as { error?: string }).error
					: undefined) || 'Failed to create form';
			error(status, message);
		}
		return json(data);
	} catch (err) {
		if (isHttpError(err)) throw err;
		console.error('POST /api/admin/forms error:', err);
		error(400, 'Invalid request body');
	}
};
