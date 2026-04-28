/**
 * Site Health Run-Tests Endpoint — Backend Proxy
 *
 * PRINCIPAL-2026-04-26 (audit 09-system §P1-5):
 *   - Triggers the backend's full site-health test suite. Running tests can
 *     hammer the database and external services, so this is super-admin
 *     gated rather than admin-gated.
 *   - Built per CREATE-not-DELETE rule.
 */

import { env } from '$env/dynamic/private';
import type { RequestHandler } from './$types';
import { requireSuperadmin } from '$lib/server/auth';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const POST: RequestHandler = async (event) => {
	const { token } = requireSuperadmin(event);

	const upstream = await fetch(`${API_URL}/api/admin/site-health/run-tests`, {
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
