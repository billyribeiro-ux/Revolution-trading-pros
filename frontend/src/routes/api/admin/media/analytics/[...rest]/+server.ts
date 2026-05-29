/**
 * Catchall Media Analytics Proxy
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Forwards `/api/admin/media/analytics/<rest>` to the backend at the same path
 * with Bearer auth. Covers `overview`, `bandwidth`, `formats`, and any future
 * sub-routes the analytics dashboard adds.
 *
 * FIX-2026-04-26-audit (P1-8): `media/analytics/+page.svelte` was calling
 * `/api/media/analytics/{overview,bandwidth,formats}` directly with no proxy
 * in place — every request 404'd in production and the page silently rendered
 * the "Connection error" empty state instead of real data.
 */

import type { RequestHandler } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';

const BACKEND_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

// Generous allowlist for query params observed in the analytics dashboard.
// Unknown params are dropped to keep the forward URL clean.
const ALLOWED_QUERY = ['range', 'period', 'granularity', 'format', 'limit', 'page'] as const;

function buildUrl(rest: string, search: URLSearchParams): string {
	const filtered = new URLSearchParams();
	for (const key of ALLOWED_QUERY) {
		const v = search.get(key);
		if (v !== null) filtered.set(key, v);
	}
	const qs = filtered.toString();
	const path = `/api/admin/media/analytics/${rest}`;
	return `${BACKEND_URL}${path}${qs ? `?${qs}` : ''}`;
}

export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const rest = event.params.rest;
	if (!rest) error(400, 'Missing analytics path');

	try {
		const url = buildUrl(rest, event.url.searchParams);
		const upstream = await fetch(url, {
			headers: {
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});
		const text = await upstream.text();
		const respHeaders: Record<string, string> = {};
		const ct = upstream.headers.get('content-type');
		if (ct) respHeaders['Content-Type'] = ct;
		return new Response(text, { status: upstream.status, headers: respHeaders });
	} catch (err) {
		console.error('[media/analytics proxy] forward failed:', err);
		error(502, err instanceof Error ? err.message : 'Failed to reach backend');
	}
};
