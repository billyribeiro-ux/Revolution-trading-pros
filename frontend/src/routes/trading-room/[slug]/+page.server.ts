/**
 * Trading Room Server Load - ICT 11+ Principal Engineer Pattern
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Server-side data loading for trading room pages.
 * Ensures proper SSR and Cloudflare Pages compatibility.
 *
 * @version 1.0.0
 */

import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
	// Return the slug to ensure the page is properly rendered server-side
	return {
		slug: params.slug
	};
};

// Prerender this page for known trading rooms
export const prerender = false; // Dynamic route - render on demand
