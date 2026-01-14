/**
 * Explosive Swings Alerts - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * SSR pre-fetch for alerts data
 * 
 * @version 1.0.0
 */

import { env } from '$env/dynamic/private';
import type { PageServerLoad } from './$types';

const ROOM_SLUG = 'explosive-swings';

export const load: PageServerLoad = async ({ fetch }) => {
	const baseUrl = env.API_BASE_URL || 'https://revolution-trading-pros-api.fly.dev/api';
	
	try {
		const alertsRes = await fetch(`${baseUrl}/room-content/rooms/${ROOM_SLUG}/alerts?per_page=50`).catch(() => null);
		const alertsData = alertsRes?.ok ? await alertsRes.json() : { data: [] };

		return {
			alerts: alertsData.data || []
		};
	} catch (error) {
		console.error('Failed to load alerts:', error);
		return {
			alerts: []
		};
	}
};
