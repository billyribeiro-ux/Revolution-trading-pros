/**
 * Traders API Endpoint
 *
 * Handles trader management for trading rooms.
 * Returns mock data when backend is not connected.
 *
 * @version 1.0.0 - December 2025
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { requireAdmin } from '$lib/server/auth';
// R20-A: migrated off local `Promise<any | null>` helper to shared
// `$lib/server/proxy-fetch` (CLAUDE.md URL-fallback pinned once;
// `Promise<unknown>` return type; `hasSuccess` narrowing replaces the
// unsound `backendData?.success` pattern).
import { fetchBackend, hasSuccess, isObject } from '$lib/server/proxy-fetch';

/** Body shape for POST /api/admin/trading-rooms/traders — every field optional from the wire. */
interface TraderCreateBody {
	name?: string;
	slug?: string;
	title?: string;
	bio?: string;
	photo_url?: string | null;
	email?: string;
	social_links?: Record<string, unknown>;
	specialties?: string[];
	is_active?: boolean;
	sort_order?: number;
}

// Mock traders data — PRESERVED as a local-dev reference only (audit
// 2026-05-16). The real backend (`admin_list_traders`, trading_rooms.rs)
// is now implemented, so this is no longer a live data path. `_`-prefixed
// so the config's varsIgnorePattern (/^_/) silences no-unused-vars
// without deleting the reference.
const _mockTraders = [
	{
		id: 1,
		name: 'Mike Thompson',
		slug: 'mike-thompson',
		title: 'Head Day Trader',
		bio: 'Over 15 years of trading experience specializing in momentum day trading.',
		photo_url: null,
		email: 'mike@revolutiontrading.com',
		social_links: { twitter: '@miketrading' },
		specialties: ['Day Trading', 'Momentum', 'Options'],
		is_active: true,
		sort_order: 1,
		daily_videos_count: 45,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 2,
		name: 'Sarah Chen',
		slug: 'sarah-chen',
		title: 'Senior Swing Trader',
		bio: 'Expert in technical analysis and swing trading strategies.',
		photo_url: null,
		email: 'sarah@revolutiontrading.com',
		social_links: { twitter: '@sarahtrading' },
		specialties: ['Swing Trading', 'Technical Analysis', 'Charts'],
		is_active: true,
		sort_order: 2,
		daily_videos_count: 32,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 3,
		name: 'James Wilson',
		slug: 'james-wilson',
		title: 'Options Specialist',
		bio: 'SPX and options trading expert with focus on risk management.',
		photo_url: null,
		email: 'james@revolutiontrading.com',
		social_links: { twitter: '@jamesoptions' },
		specialties: ['Options', 'SPX', 'Risk Management'],
		is_active: true,
		sort_order: 3,
		daily_videos_count: 28,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 4,
		name: 'Emily Davis',
		slug: 'emily-davis',
		title: 'Small Account Mentor',
		bio: 'Specializes in growing small accounts with disciplined trading.',
		photo_url: null,
		email: 'emily@revolutiontrading.com',
		social_links: { twitter: '@emilytrading' },
		specialties: ['Small Accounts', 'Position Sizing', 'Discipline'],
		is_active: true,
		sort_order: 4,
		daily_videos_count: 22,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	}
];

// GET - List traders
export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { url } = event;
	// `active_only` is forwarded intact to the backend via
	// `url.searchParams.toString()` below and honored there by the now-real
	// `admin_list_traders` (TradersQuery, trading_rooms.rs). Parsing it into
	// a local const here was vestigial — commented out (not deleted) per the
	// audit's "comment, don't delete" rule.
	// const activeOnly = url.searchParams.get('active_only') === 'true';

	// Try backend first
	const backendData = await fetchBackend(
		`/api/admin/trading-rooms/traders?${url.searchParams.toString()}`,
		{
			headers: { Authorization: `Bearer ${token}` }
		},
		'[Trading-rooms traders proxy]'
	);

	// R20-A: `hasSuccess` narrows `unknown` before reading `.success`.
	// The previous `backendData?.success` was unsound on a non-null primitive
	// response (would NPE on `.success` access).
	if (hasSuccess(backendData) && backendData.success) {
		return json(backendData);
	}

	// FIX-2026-04-26-audit (P1-11): surface backend failure instead of returning phantom
	// mock traders (Mike/Sarah/James/Emily). Admins were seeing fake dropdowns on any
	// backend hiccup with no indication that real data was unavailable.
	// TODO(2026-04-26-audit): gate mock on env.ENABLE_MOCK_DATA if needed for local dev.
	console.error('[Trading-rooms traders proxy] Backend unavailable or non-success:', backendData);
	return json(
		{
			success: false,
			error: 'Unable to load traders from backend. Please check the API connection.',
			_mock: false
		},
		{ status: 502 }
	);
};

// POST - Create trader
export const POST: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);

	// R20-A: narrow request.json() to a non-null object BEFORE forwarding,
	// to surface a 400 (vs 500 NPE) for `null` / primitive bodies and to
	// type the downstream JSON.stringify shape explicitly (R18-A Latent
	// Bug §2 mitigation, applied uniformly across admin proxies).
	const rawBody: unknown = await event.request.json();
	if (!isObject(rawBody)) {
		return json(
			{ success: false, error: 'Request body must be a JSON object' },
			{ status: 400 }
		);
	}
	const body = rawBody as TraderCreateBody;

	// Try backend first
	const backendData = await fetchBackend(
		'/api/admin/trading-rooms/traders',
		{
			method: 'POST',
			headers: { Authorization: `Bearer ${token}` },
			body: JSON.stringify(body)
		},
		'[Trading-rooms traders proxy POST]'
	);

	if (hasSuccess(backendData) && backendData.success) {
		return json(backendData);
	}

	// FIX-2026-04-26-audit (P1-11): surface backend failure on POST — a silent mock-create
	// would persist nothing to the DB while appearing to succeed.
	console.error('[Trading-rooms traders proxy POST] Backend unavailable or non-success:', backendData);
	return json(
		{
			success: false,
			error: 'Unable to create trader — backend is unavailable. Please try again.'
		},
		{ status: 502 }
	);
};
