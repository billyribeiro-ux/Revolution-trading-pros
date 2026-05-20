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
 * R21-A: migrated to shared `fetchBackendWithStatus` helper. POST body
 * narrowed with `isObject`. Backend-error-message extraction consolidated.
 *
 * @version 1.1.0 — 2026-05-20
 */

import { json, error, isHttpError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { requireAdmin } from '$lib/server/auth';
import {
	fetchBackendWithStatus,
	isObject,
	extractBackendErrorMessage
} from '$lib/server/proxy-fetch';

export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { url } = event;
	const authHeader = `Bearer ${token}`;
	const queryParams = url.searchParams.toString();
	// Backend exposes forms under /api/forms (admin scope is enforced by
	// the bearer token + role check on the Rust side).
	const endpoint = `/api/forms${queryParams ? `?${queryParams}` : ''}`;

	const { data, status } = await fetchBackendWithStatus(
		endpoint,
		{ headers: { Authorization: authHeader } },
		'[Admin Forms API]'
	);

	if (status === 401 || status === 403) error(status, 'Unauthorized');
	if (status >= 400 || !data) {
		// Return empty list with a flag so consumers can render gracefully
		// instead of throwing. The popup builder uses `data.forms || data || []`.
		return json({ success: false, forms: [], total: 0, error: 'Failed to fetch forms' });
	}

	return json(data);
};

export const POST: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { request } = event;
	const authHeader = `Bearer ${token}`;
	try {
		const body: unknown = await request.json();

		if (!isObject(body)) {
			error(400, 'Invalid request body');
		}

		const { data, status } = await fetchBackendWithStatus(
			'/api/forms',
			{
				method: 'POST',
				headers: { Authorization: authHeader },
				body: JSON.stringify(body)
			},
			'[Admin Forms API]'
		);

		if (status === 401 || status === 403) error(status, 'Unauthorized');
		if (status >= 400) {
			error(status, extractBackendErrorMessage(data, 'Failed to create form'));
		}
		return json(data);
	} catch (err) {
		if (isHttpError(err)) throw err;
		console.error('POST /api/admin/forms error:', err);
		error(400, 'Invalid request body');
	}
};
