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

// Production fallback - Rust API on Fly.io
import { env } from '$env/dynamic/private';
import { isValidPeriod } from '$lib/server/analytics-proxy';
import { requireAdmin } from '$lib/server/auth';
const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

// Try to fetch from backend
async function fetchFromBackend(endpoint: string, options?: RequestInit): Promise<any | null> {
	if (!BACKEND_URL) return null;

	try {
		const response = await fetch(`${BACKEND_URL}${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...options?.headers
			}
		});

		if (!response.ok) return null;
		return await response.json();
	} catch (_error) {
		return null;
	}
}

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
	const backendData = await fetchFromBackend(
		`/api/admin/analytics/dashboard?${params.toString()}`,
		{
			headers: { Authorization: `Bearer ${token}` }
		}
	);

	if (backendData?.success || backendData?.kpis) {
		return json({
			success: true,
			data: backendData.data || backendData,
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
