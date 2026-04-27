/**
 * Shared helper for `/api/admin/analytics/**` proxy `+server.ts` files.
 *
 * PRINCIPAL-2026-04-26 (audit 08-analytics §P0-2):
 *   25 of 26 client-side analytics endpoints called proxies that did not
 *   exist. Pages with explicit try/catch swallowed the resulting 404 HTML
 *   into empty arrays; pages without one surfaced "Failed to load X" toasts.
 *   This module unifies the pattern so every new proxy is one short call.
 *
 * Usage:
 *   import { proxyAnalytics } from '$lib/server/analytics-proxy';
 *
 *   export const GET: RequestHandler = (event) =>
 *     proxyAnalytics(event, '/api/admin/analytics/recordings', {
 *       method: 'GET',
 *       forwardQuery: ['period', 'filter', 'page']
 *     });
 *
 * The helper:
 *   - validates auth via `requireAdmin` (cookie or Bearer fallback)
 *   - validates `period` against the known allowlist (audit §P1-8)
 *   - rebuilds the query string from the allowlist via `URLSearchParams`
 *     (no string concat — fixes audit §P2-9 injection vector)
 *   - forwards the upstream status verbatim (no fake success masking 404)
 */

import type { RequestEvent } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdmin } from './auth';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

/**
 * Allowlist for the `period` query param. Anything outside this set is
 * dropped from the forwarded URL — preventing injection like
 * `period=30d&secret_admin_param=true` from the audit's example.
 */
const PERIOD_ALLOWLIST = new Set(['1d', '24h', '7d', '14d', '30d', '60d', '90d', '180d', '1y']);

export function isValidPeriod(value: string | null): boolean {
	return value !== null && PERIOD_ALLOWLIST.has(value);
}

export interface ProxyOptions {
	method?: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH';
	/**
	 * Query params to read off `event.url.searchParams` and forward to the
	 * upstream URL. Other params are dropped. `period` is additionally
	 * validated against {@link PERIOD_ALLOWLIST}.
	 */
	forwardQuery?: readonly string[];
	/**
	 * If true, read the request body as JSON and forward it. Default false.
	 */
	forwardBody?: boolean;
}

export async function proxyAnalytics(
	event: RequestEvent,
	upstreamPath: string,
	options: ProxyOptions = {}
): Promise<Response> {
	const { token } = requireAdmin(event);
	const method = options.method ?? 'GET';

	// Build forwarded query string from the allowlist only.
	const forwarded = new URLSearchParams();
	if (options.forwardQuery) {
		for (const key of options.forwardQuery) {
			const value = event.url.searchParams.get(key);
			if (value === null) continue;
			if (key === 'period' && !isValidPeriod(value)) continue;
			forwarded.append(key, value);
		}
	}

	const queryString = forwarded.toString();
	const url = queryString ? `${API_URL}${upstreamPath}?${queryString}` : `${API_URL}${upstreamPath}`;

	const headers: Record<string, string> = {
		Authorization: `Bearer ${token}`,
		Accept: 'application/json'
	};

	let body: string | null = null;
	if (options.forwardBody && method !== 'GET' && method !== 'DELETE') {
		const text = await event.request.text();
		if (text) {
			body = text;
			headers['Content-Type'] = event.request.headers.get('content-type') || 'application/json';
		}
	}

	const upstream = await fetch(url, { method, headers, body });
	const responseText = await upstream.text();
	const responseHeaders: Record<string, string> = {};
	const ct = upstream.headers.get('content-type');
	if (ct) responseHeaders['Content-Type'] = ct;
	return new Response(responseText, { status: upstream.status, headers: responseHeaders });
}
