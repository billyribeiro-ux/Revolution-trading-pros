/**
 * Cart Page Server Load - WordPress EXACT REPLICA
 * ═══════════════════════════════════════════════════════════════════════════
 * Server-side data loading for cart page
 * @version 7.0.0 (WordPress IDENTICAL / December 2025)
 */

import type { ServerLoad } from '@sveltejs/kit';

export const load: ServerLoad = async ({ locals }) => {
	return {
		user: locals.user || null,
		cartNonce: '31d89a8fba',
		wpHttpReferer: '/cart'
	};
};
