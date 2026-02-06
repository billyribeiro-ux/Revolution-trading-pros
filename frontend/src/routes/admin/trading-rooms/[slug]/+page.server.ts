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
 * @version 1.0.0
 */

import { error, redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params, fetch, cookies }) => {
	const { slug } = params;

	if (!slug) {
		throw error(400, 'Room slug is required');
	}

	// Verify admin session exists
	const sessionToken = cookies.get('session_token');
	if (!sessionToken) {
		throw redirect(302, '/login?redirect=/admin/trading-rooms/' + slug);
	}

	// Parallel data fetching for performance
	const [
		tradePlanResult,
		alertsResult,
		weeklyVideoResult,
		roomStatsResult,
		tradesResult,
		videoResourcesResult
	] = await Promise.allSettled([
		fetch(`/api/trade-plans/${slug}?per_page=50`).then((r) => (r.ok ? r.json() : null)),
		fetch(`/api/alerts/${slug}?per_page=50`).then((r) => (r.ok ? r.json() : null)),
		fetch(`/api/weekly-video/${slug}`).then((r) => (r.ok ? r.json() : null)),
		fetch(`/api/room-stats/${slug}`).then((r) => (r.ok ? r.json() : null)),
		fetch(`/api/trades/${slug}?per_page=100`).then((r) => (r.ok ? r.json() : null)),
		fetch(`/api/room-resources?room_slug=${slug}&content_type=video&per_page=50`).then((r) =>
			r.ok ? r.json() : null
		)
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
