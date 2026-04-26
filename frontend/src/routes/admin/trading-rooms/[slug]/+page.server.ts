/**
 * Trading Room Admin - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════════════
 * Apple Principal Engineer ICT 7+ Grade - February 2026
 *
 * SvelteKit best practice: Load data server-side for SSR benefits:
 * - Faster initial page load (no client-side fetch waterfall)
 * - SEO-friendly (content rendered on first paint)
 * - Auth validation at server level
 *
 * @version 1.1.0
 *
 * FIX-2026-04-26-audit (P0-8):
 * - Read the canonical `rtp_access_token` cookie. The previous `session_token` is
 *   never set anywhere in the codebase, so every authenticated admin was being
 *   redirected to /login on each SSR pass.
 * - Forward the bearer token on every SSR fetch so the API actually returns data
 *   instead of silently 401-ing into "no SSR data, fall back to client".
 */

import { redirect } from '@sveltejs/kit';
import { error as kitError } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params, fetch, cookies }) => {
	const { slug } = params;

	if (!slug) {
		kitError(400, 'Room slug is required');
	}

	// FIX-2026-04-26-audit (P0-8): the canonical session cookie is `rtp_access_token`
	// (set by the login proxy — see commit e2356fa46 / CLAUDE.md). The previous code
	// looked for `session_token`, which is never set, so this redirected every admin.
	const sessionToken = cookies.get('rtp_access_token');
	if (!sessionToken) {
		redirect(302, '/login?redirect=/admin/trading-rooms/' + slug);
	}

	// FIX-2026-04-26-audit (P0-8): forward the bearer token to each SSR fetch so the
	// backend doesn't 401 silently. Without this, every SSR fetch returned null and
	// the admin only saw data after the second client-side load.
	const authHeaders: HeadersInit = {
		Authorization: `Bearer ${sessionToken}`,
		Accept: 'application/json'
	};

	// Parallel data fetching for performance
	const [
		tradePlanResult,
		alertsResult,
		weeklyVideoResult,
		roomStatsResult,
		tradesResult,
		videoResourcesResult
	] = await Promise.allSettled([
		fetch(`/api/trade-plans/${slug}?per_page=50`, { headers: authHeaders }).then((r) =>
			r.ok ? r.json() : null
		),
		fetch(`/api/alerts/${slug}?per_page=50`, { headers: authHeaders }).then((r) =>
			r.ok ? r.json() : null
		),
		fetch(`/api/weekly-video/${slug}`, { headers: authHeaders }).then((r) =>
			r.ok ? r.json() : null
		),
		fetch(`/api/room-stats/${slug}`, { headers: authHeaders }).then((r) =>
			r.ok ? r.json() : null
		),
		fetch(`/api/trades/${slug}?per_page=100`, { headers: authHeaders }).then((r) =>
			r.ok ? r.json() : null
		),
		fetch(`/api/room-resources?room_slug=${slug}&content_type=video&per_page=50`, {
			headers: authHeaders
		}).then((r) => (r.ok ? r.json() : null))
	]);

	// Extract successful results with fallbacks
	const tradePlan = tradePlanResult.status === 'fulfilled' ? tradePlanResult.value : null;
	const alerts = alertsResult.status === 'fulfilled' ? alertsResult.value : null;
	const weeklyVideo = weeklyVideoResult.status === 'fulfilled' ? weeklyVideoResult.value : null;
	const roomStats = roomStatsResult.status === 'fulfilled' ? roomStatsResult.value : null;
	const trades = tradesResult.status === 'fulfilled' ? tradesResult.value : null;
	const videoResources =
		videoResourcesResult.status === 'fulfilled' ? videoResourcesResult.value : null;

	return {
		slug,
		initialData: {
			tradePlan: tradePlan?.data ?? tradePlan?.items ?? [],
			tradePlanTotal: tradePlan?.total ?? tradePlan?.data?.length ?? 0,
			alerts: alerts?.data ?? alerts?.items ?? [],
			alertsTotal: alerts?.total ?? alerts?.data?.length ?? 0,
			weeklyVideo: weeklyVideo?.data ?? weeklyVideo ?? null,
			roomStats: roomStats?.data ?? roomStats ?? null,
			trades: trades?.data ?? trades?.items ?? [],
			tradesTotal: trades?.total ?? trades?.data?.length ?? 0,
			videoResources: videoResources?.data ?? videoResources?.items ?? [],
			videoResourcesTotal: videoResources?.total ?? videoResources?.data?.length ?? 0
		}
	};
};
