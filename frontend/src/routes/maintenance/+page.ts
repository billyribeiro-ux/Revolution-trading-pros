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
			title: 'Platform Rebuild in Progress',
			description:
				'Revolution Trading Pros is being rebuilt end to end: faster market data, more rigorous scanners, and an institutional-standard trading curriculum. Progress is posted every trading day. Request first access.',
			og: {
				title: 'Platform Rebuild in Progress',
				description:
					'Faster market data, more rigorous scanners, and an institutional-standard trading curriculum. Progress is posted on this page every trading day.'
			}
		}
	};
};

// Prerender this page so it loads instantly even during server downtime
export const prerender = true;
