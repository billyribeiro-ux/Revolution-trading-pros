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
			title: 'Improving Our Services | Revolution Trading Pros',
			description:
				"We're upgrading our infrastructure to serve more students with better speed, reliability, and features. Back soon!",
			og: {
				title: 'Improving Our Services | Revolution Trading Pros',
				description:
					"We're upgrading our infrastructure to serve more students with better speed, reliability, and features."
			}
		}
	};
};

// Prerender this page so it loads instantly even during server downtime
export const prerender = true;
