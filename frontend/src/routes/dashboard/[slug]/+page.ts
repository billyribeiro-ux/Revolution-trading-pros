/**
 * Dynamic Trading Room Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Loads trading room configuration based on URL slug
 *
 * @version 1.0.0
 */
import { error } from '@sveltejs/kit';
import { getTradingRoom, isValidSlug } from '$lib/config/trading-rooms';

export const load = ({ params }: { params: { slug: string } }) => {
	const { slug } = params;

	// Check if this is a valid trading room slug
	if (!isValidSlug(slug)) {
		throw error(404, {
			message: 'Trading room not found'
		});
	}

	const room = getTradingRoom(slug);

	if (!room) {
		throw error(404, {
			message: 'Trading room not found'
		});
	}

	return {
		room,
		slug
	};
};
