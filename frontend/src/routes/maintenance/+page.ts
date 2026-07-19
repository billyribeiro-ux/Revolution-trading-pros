/**
 * Maintenance Page - Route Configuration
 * ═══════════════════════════════════════════════════════════════════════════
 * This page is displayed during infrastructure upgrades.
 * It uses prerender to ensure fast loading even when API is unavailable.
 * ═══════════════════════════════════════════════════════════════════════════
 */

import type { PageLoad } from './$types';

export const load: PageLoad = () => {
	return {
		seo: {
			title: 'The Revolution Is Loading',
			description:
				"We're rebuilding the entire trading platform: faster data, institutional-grade scanners, and a pro-desk trading university. A new desk note and build-log entry land every trading day — reserve first access.",
			og: {
				title: 'The Revolution Is Loading',
				description:
					"We didn't go quiet. We went to work. Faster data, institutional-grade scanners, and a pro-desk trading university — with fresh build-log entries every trading day."
			}
		}
	};
};

// Prerender this page so it loads instantly even during server downtime
export const prerender = true;
