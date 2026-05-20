/**
 * Built-in Analytics Dashboard API
 *
 * Returns analytics data from the built-in enterprise analytics system.
 * Falls back to aggregated data from internal tracking when external services aren't connected.
 *
 * @version 1.0.0 - December 2025
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { isValidPeriod } from '$lib/server/analytics-proxy';
import { requireAdmin } from '$lib/server/auth';
// R20-A: migrated off local `Promise<any | null>` helper to shared
// `$lib/server/proxy-fetch` (CLAUDE.md URL-fallback pinned once;
// `Promise<unknown>` return; `hasSuccess` + `extractBackendData`
// fix the R18-A Latent Bug §3 short-circuit on `{ data: null }`).
import {
	fetchBackend,
	hasSuccess,
	isObject,
	extractBackendData
} from '$lib/server/proxy-fetch';

// Generate realistic baseline analytics based on actual platform activity
function generateBuiltInAnalytics(_period: string) {
	// These would normally come from the analytics_events table
	// For now, we return structure with null values indicating "collecting data"
	return {
		kpis: {
			sessions: { value: null, change: 0, label: 'Sessions' },
			pageviews: { value: null, change: 0, label: 'Pageviews' },
			unique_visitors: { value: null, change: 0, label: 'Unique Visitors' },
			new_users: { value: null, change: 0, label: 'New Users' },
			bounce_rate: { value: null, change: 0, label: 'Bounce Rate' },
			avg_session_duration: { value: null, change: 0, label: 'Avg. Duration' }
		},
		seo: {
			search_traffic: null,
			impressions: null,
			clicks: null,
			keywords: null,
			avg_position: null,
			indexed_pages: null,
			avg_ctr: null
		},
		top_pages: [],
		device_breakdown: {
			desktop: 0,
			mobile: 0,
			tablet: 0
		},
		time_series: {
			revenue: [],
			users: []
		},
		_builtin: true,
		_message: 'Analytics data is being collected. Stats will appear as traffic is tracked.'
	};
}

export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { url } = event;
	// FIX-2026-04-26 (audit 08-analytics §P1-8 / §P2-9):
	// Validate `period` against the canonical allowlist before string-concat
	// into the upstream URL. Previously `period=30d&secret=true` would pass
	// straight through.
	const rawPeriod = url.searchParams.get('period');
	const period = isValidPeriod(rawPeriod) ? rawPeriod! : '30d';

	// Try to get real analytics from backend first
	const params = new URLSearchParams({ period });
	const backendData = await fetchBackend(
		`/api/admin/analytics/dashboard?${params.toString()}`,
		{
			headers: { Authorization: `Bearer ${token}` }
		},
		'[Analytics dashboard proxy]'
	);

	// R20-A: narrow `unknown` via type-guards before reading nested fields.
	// Accept either of two backend shapes:
	//  (a) envelope `{ success: true, data: { ...kpis... } }`
	//  (b) raw payload with top-level `kpis` field (the built-in shape).
	// R18-A Latent Bug §3 mitigation: `extractBackendData` preserves
	// `{ data: null }` semantics (returns the null) rather than falling
	// through to the envelope and rendering `error` strings as KPI data.
	const isEnvelopeSuccess = hasSuccess(backendData) && backendData.success;
	const hasKpis = isObject(backendData) && 'kpis' in backendData;
	if (isEnvelopeSuccess || hasKpis) {
		return json({
			success: true,
			data: extractBackendData(backendData),
			_source: 'backend'
		});
	}

	// Return built-in analytics structure
	const builtInData = generateBuiltInAnalytics(period);

	return json({
		success: true,
		data: builtInData,
		_source: 'builtin'
	});
};
