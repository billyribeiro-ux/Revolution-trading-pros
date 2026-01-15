/**
 * Email Campaign Report - Page Load
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Loads campaign ID from URL parameters for the report page.
 *
 * @version 1.0.0 - January 2026
 */

import type { PageLoad } from './$types';

// Disable prerendering for dynamic route
export const prerender = false;

export const load: PageLoad = ({ params }) => {
	return {
		campaignId: params.id
	};
};
