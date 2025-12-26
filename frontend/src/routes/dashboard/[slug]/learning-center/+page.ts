import { error } from '@sveltejs/kit';
import { getTradingRoom, isValidSlug } from '$lib/config/trading-rooms';

export const load = ({ params }: { params: { slug: string } }) => {
	const { slug } = params;
	if (!isValidSlug(slug)) throw error(404, { message: 'Trading room not found' });
	const room = getTradingRoom(slug);
	if (!room || !room.features.learningCenter) throw error(404, { message: 'Learning center not available' });
	return { room, slug };
};
