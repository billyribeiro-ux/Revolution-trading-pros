import { error } from '@sveltejs/kit';
import { getTradingRoom, isValidSlug } from '$lib/config/trading-rooms';
import type { PageLoad } from '@sveltejs/kit';

export const load: PageLoad = ({ params }) => {
	const { slug } = params;
	if (!isValidSlug(slug)) throw error(404, { message: 'Trading room not found' });
	const room = getTradingRoom(slug);
	if (!room || !room.features.tradingRoomArchive) throw error(404, { message: 'Archive not available' });
	return { room, slug };
};
