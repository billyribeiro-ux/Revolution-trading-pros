import { error } from '@sveltejs/kit';
import { getTradingRoom, isValidSlug } from '$lib/config/trading-rooms';
import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	const { slug } = params;
	if (!isValidSlug(slug)) throw error(404, { message: 'Trading room not found' });
	const room = getTradingRoom(slug);
	if (!room || !room.features.dailyVideos) throw error(404, { message: 'Daily videos not available' });
	return { room, slug };
};
