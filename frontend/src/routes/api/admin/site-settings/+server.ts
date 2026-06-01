/**
 * /api/admin/site-settings — Proxy for cms_v2 site settings
 *
 * GET  → GET  /api/admin/cms-v2/settings   (admin only, returns full CmsSiteSettings)
 * PATCH → PUT /api/admin/cms-v2/settings   (admin only, partial update)
 *
 * The frontend hits this SvelteKit endpoint so auth cookies are forwarded
 * server-side rather than exposing the backend directly to the browser.
 */

import type { RequestHandler } from './$types';
import { json } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const API_BASE_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const GET: RequestHandler = async ({ locals }) => {
	const token = locals.accessToken;

	const res = await fetch(`${API_BASE_URL}/api/admin/cms-v2/settings`, {
		headers: {
			Authorization: `Bearer ${token}`,
			Accept: 'application/json'
		}
	});

	const data = await res.json();
	return json(data, { status: res.status });
};

export const PATCH: RequestHandler = async ({ request, locals }) => {
	const token = locals.accessToken;
	const body = await request.json();

	const res = await fetch(`${API_BASE_URL}/api/admin/cms-v2/settings`, {
		method: 'PUT',
		headers: {
			Authorization: `Bearer ${token}`,
			'Content-Type': 'application/json',
			Accept: 'application/json'
		},
		body: JSON.stringify(body)
	});

	const data = await res.json();
	return json(data, { status: res.status });
};
