/**
 * Catchall Analytics Proxy — covers every `/api/admin/analytics/*` endpoint
 * EXCEPT `/dashboard` (which keeps its bespoke handler with built-in fallback
 * data at `dashboard/+server.ts`).
 *
 * PRINCIPAL-2026-04-26 (audit 08-analytics §P0-2):
 *   The analytics page tree calls ~25 backend endpoints. Only `dashboard` had
 *   a proxy; everything else returned SvelteKit's default 404 HTML, which
 *   pages with try/catch silently swallowed into empty arrays. This catchall
 *   forwards the request to the same path on the Rust API and surfaces the
 *   real upstream status — a 404 stays a 404 instead of becoming "0 results".
 *
 * Resources covered (per `frontend/src/lib/api/analytics.ts` + page-level
 * `fetch('/api/admin/analytics/...')` calls):
 *   - kpis, kpis/{key}
 *   - funnels (list/create), funnels/{key}, funnels/{key}/dropoff,
 *     funnels/{key}/segment
 *   - cohorts (list/create), cohorts/{key}/matrix, cohorts/{key}/curve,
 *     cohorts/{key}/ltv
 *   - segments (CRUD), segments/{key}
 *   - events, events/timeseries, events/breakdown
 *   - realtime
 *   - attribution/channels, attribution/campaigns, attribution/paths,
 *     attribution/compare
 *   - forecast/{key}, forecast/{key}/accuracy
 *   - reports (CRUD + run)
 *   - recordings
 *   - heatmaps, heatmaps/{url}
 *   - goals (list/create)
 *   - behavior (used by /admin/behavior page; per audit §P2-10 was bypassing
 *     the proxy layer entirely)
 *
 * Auth: cookie-first via `requireAdmin`; query-param allowlist via
 * `proxyAnalytics`.
 */

import type { RequestHandler } from './$types';
import { proxyAnalytics } from '$lib/server/analytics-proxy';

// Generous allowlist covering every query param observed across the analytics
// client. Unknown params (e.g. injection attempts) are dropped.
const ALLOWED_QUERY = [
	'period',
	'granularity',
	'model',
	'periods_ahead',
	'event_name',
	'event_type',
	'dimension',
	'segment_field',
	'limit',
	'page',
	'per_page',
	'filter',
	'search',
	'type'
] as const;

function upstreamPath(rest: string): string {
	// SvelteKit's `[...rest]` parameter is unencoded; pass it through as-is to
	// preserve `/`s in nested paths like `funnels/{key}/dropoff`.
	return `/api/admin/analytics/${rest}`;
}

export const GET: RequestHandler = (event) =>
	proxyAnalytics(event, upstreamPath(event.params.rest), {
		method: 'GET',
		forwardQuery: ALLOWED_QUERY
	});

export const POST: RequestHandler = (event) =>
	proxyAnalytics(event, upstreamPath(event.params.rest), {
		method: 'POST',
		forwardQuery: ALLOWED_QUERY,
		forwardBody: true
	});

export const PUT: RequestHandler = (event) =>
	proxyAnalytics(event, upstreamPath(event.params.rest), {
		method: 'PUT',
		forwardQuery: ALLOWED_QUERY,
		forwardBody: true
	});

export const PATCH: RequestHandler = (event) =>
	proxyAnalytics(event, upstreamPath(event.params.rest), {
		method: 'PATCH',
		forwardQuery: ALLOWED_QUERY,
		forwardBody: true
	});

export const DELETE: RequestHandler = (event) =>
	proxyAnalytics(event, upstreamPath(event.params.rest), {
		method: 'DELETE',
		forwardQuery: ALLOWED_QUERY
	});
