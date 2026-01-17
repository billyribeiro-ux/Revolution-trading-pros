/**
 * Explosive Swings Trade Tracker - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * SSR pre-fetch for trade plan data
 *
 * @version 1.0.0
 */

import { env } from '$env/dynamic/private';
import type { PageServerLoad } from './$types';

const ROOM_SLUG = 'explosive-swings';

export const load: PageServerLoad = async ({ fetch }) => {
	const baseUrl = env.API_BASE_URL || 'https://revolution-trading-pros-api.fly.dev/api';

	try {
		const [tradePlanRes, statsRes] = await Promise.all([
			fetch(`${baseUrl}/room-content/rooms/${ROOM_SLUG}/trade-plan`).catch(() => null),
			fetch(`${baseUrl}/room-content/rooms/${ROOM_SLUG}/stats`).catch(() => null)
		]);

		const tradePlanData = tradePlanRes?.ok ? await tradePlanRes.json() : { data: [] };
		const statsData = statsRes?.ok ? await statsRes.json() : { data: null };

		return {
			tradePlan: tradePlanData.data || [],
			stats: statsData.data
		};
	} catch (error) {
		console.error('Failed to load trade tracker data:', error);
		return {
			tradePlan: [],
			stats: null
		};
	}
};
