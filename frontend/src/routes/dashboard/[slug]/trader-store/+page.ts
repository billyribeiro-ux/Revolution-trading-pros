import { error } from '@sveltejs/kit';
import { getTradingRoom, isValidSlug } from '$lib/config/trading-rooms';
import type { Load } from '@sveltejs/kit';

export const load: Load = ({ params }) => {
	const { slug } = params;
	if (!isValidSlug(slug)) throw error(404, { message: 'Trading room not found' });
	const room = getTradingRoom(slug);
	if (!room || !room.features.traderStore) throw error(404, { message: 'Trader store not available' });
	return { room, slug };
};
