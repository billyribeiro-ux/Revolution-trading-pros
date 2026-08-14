/**
 * CRM Stats API - RevolutionCRM Pro
 *
 * Fetches CRM statistics from backend.
 *
 * @version 2.0.0 - January 2026
 */

import { json, error } from '@sveltejs/kit';

import type { RequestHandler } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import { requireAdmin } from '#lib/server/auth.js';
const PROD_BACKEND = API_BASE_URL || BACKEND_URL || 'http://localhost:8080';

export const GET: RequestHandler = async (event) => {
	const backendUrl = PROD_BACKEND;
	const { token } = requireAdmin(event);
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
