/**
 * Bunny Video Create API - Proxy to backend
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * POST /api/admin/bunny/create-video - Create video entry on Bunny.net
 *
 * Proxies to backend at /api/admin/bunny/create-video
 *
 * @version 1.0.0 - ICT 7 Principal Engineer Grade
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';

// FIX-2026-04-26-audit: align with repo-wide proxy env-var chain.
const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

// POST - Create video entry on Bunny.net
export const POST: RequestHandler = async (event) => {
	const { request } = event;

	// FIX-2026-04-26-audit (P1-2): defense-in-depth admin gate. This proxy mints
	// upload URLs against our Bunny library — must be admin-gated locally so a
	// stolen non-admin JWT can never reach the Bunny side, regardless of backend
	// state (the auth check is local; backend can be down and this still works).
	const { token: cookieToken } = requireAdmin(event);

	const body = await request.json();

	if (!body.title) {
		error(400, 'Video title is required');
	}

	// Forward to backend with Bearer token auth
	try {
		const response = await fetch(`${BACKEND_URL}/api/admin/bunny/create-video`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${cookieToken}`
			},
			body: JSON.stringify({
				title: body.title,
				library_id: body.library_id,
				collection_id: body.collection_id
			})
		});

		if (!response.ok) {
			const errorText = await response.text();
			console.error(`[Bunny API] Backend error: ${response.status}`, errorText);
			error(response.status, errorText || 'Failed to create video');
		}

		const backendData = await response.json();

		if (backendData?.error) {
			error(backendData.status || 500, backendData.error);
		}

		if (backendData?.success) {
			return json(backendData);
		}

		error(500, 'Failed to create video on Bunny.net');
	} catch (err) {
		console.error('[Bunny API] Error:', err);
		if (err && typeof err === 'object' && 'status' in err) {
			throw err;
		}
		error(500, 'Failed to create video on Bunny.net');
	}
};
