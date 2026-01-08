import type { PageServerLoad } from './$types';

/**
 * Orders Page Server Load
 * ICT 11+ Protocol: Uses parent layout auth pattern
 * Parent dashboard layout already ensures user is authenticated
 */
export const load: PageServerLoad = async ({ parent }) => {
	// Get user from parent layout (already authenticated by hooks.server.ts)
	const parentData = await parent();

	// Return empty orders for now - the orders will come from API
	// or be populated when the orders API is fully implemented
	return {
		orders: [],
		user: parentData.user
	};
};
