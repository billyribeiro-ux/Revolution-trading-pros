/**
 * Explosive Swings Alerts - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * SSR pre-fetch for alerts data
 *
 * @version 1.1.0 - ICT 7 update: proper error handling and typed return
 */

import { env } from '$env/dynamic/private';
import type { PageServerLoad } from './$types';
import type { RoomAlert } from '$lib/types/trading';
import { logger } from '$lib/utils/logger';

const ROOM_SLUG = 'explosive-swings';

export const load: PageServerLoad = async ({ fetch, cookies }) => {
	const baseUrl = env.API_BASE_URL || 'https://revolution-trading-pros-api.fly.dev/api';

	try {
		const response = await fetch(`${baseUrl}/room-content/rooms/${ROOM_SLUG}/alerts?per_page=50`, {
			headers: {
				cookie: cookies.get('session') ? `session=${cookies.get('session')}` : ''
			}
		});

		if (!response.ok) {
			return {
				alerts: [] as RoomAlert[],
				total: 0,
				error: `Failed to load alerts (${response.status})`
			};
		}

		const data = await response.json();

		if (!data.success && !data.data) {
			return {
				alerts: [] as RoomAlert[],
				total: 0,
				error: data.error || 'Failed to load alerts'
			};
		}

		return {
			alerts: (data.data || []) as RoomAlert[],
			total: data.total ?? data.data?.length ?? 0,
			error: undefined
		};
	} catch (err) {
		logger.error('[alerts/+page.server.ts] Failed to fetch alerts:', err);
		return {
			alerts: [] as RoomAlert[],
			total: 0,
			error: 'Unable to connect to server'
		};
	}
};
