/**
 * Explosive Swings Video Library - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * SSR pre-fetch for weekly videos archive
 *
 * @version 1.0.0
 */

import { env } from '$env/dynamic/private';
import type { PageServerLoad } from './$types';
import { logger } from '$lib/utils/logger';

const ROOM_SLUG = 'explosive-swings';

export const load: PageServerLoad = async ({ fetch }) => {
	const baseUrl = env.API_BASE_URL || 'https://revolution-trading-pros-api.fly.dev/api';

	try {
		const videosRes = await fetch(
			`${baseUrl}/room-content/rooms/${ROOM_SLUG}/weekly-videos?per_page=50`
		).catch(() => null);
		const videosData = videosRes?.ok ? await videosRes.json() : { data: [] };

		return {
			videos: videosData.data || []
		};
	} catch (error) {
		logger.error('Failed to load video library:', error);
		return {
			videos: []
		};
	}
};
