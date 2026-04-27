/**
 * CRM Stats API - RevolutionCRM Pro
 *
 * Fetches CRM statistics from backend.
 *
 * @version 2.0.0 - January 2026
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
const PROD_BACKEND =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

export const GET: RequestHandler = async ({ request, cookies }) => {
	const backendUrl = PROD_BACKEND;
	// FIX-2026-04-26: prefer canonical rtp_access_token cookie, fall back to header.
	// Old: const authHeader = request.headers.get('Authorization') || '';
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) error(401, 'Unauthorized');
	const authHeader = `Bearer ${token}`;

	// Audit P2 #13: this proxy used to swallow upstream 5xx errors and
	// return `success: true` with all-zero stats. The dashboard then showed
	// "0 contacts, $0 revenue" — visually indistinguishable from a real
	// empty CRM. We now propagate the upstream status so the frontend can
	// render an explicit "API down" banner instead of silently lying.
	try {
		const response = await fetch(`${backendUrl}/api/admin/crm/stats`, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: authHeader
			}
		});

		if (response.ok) {
			const data = await response.json();
			return json({ success: true, data });
		}

		console.warn(`Backend CRM stats returned ${response.status}`);
		const body = await response.text();
		return new Response(body || JSON.stringify({ success: false, error: 'Upstream error' }), {
			status: response.status,
			headers: { 'Content-Type': 'application/json' }
		});
	} catch (err) {
		console.warn('Backend CRM stats not available:', err);
		// Network/transport failure — surface a 502 so callers can
		// distinguish "real empty CRM" from "backend unreachable".
		error(502, 'Backend CRM stats endpoint unreachable');
	}
};
