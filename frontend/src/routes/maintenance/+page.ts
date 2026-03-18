/**
 * Maintenance Page - Route Configuration
 * No SSR needed, no prerender (dynamic page)
 */
export const prerender = false;

export function load() {
	return {
		seo: {
			title: 'Coming Soon | Revolution Trading Pros',
			description:
				'Revolution Trading Pros is upgrading. New institutional scanners, courses, indicators and more surprises coming soon. Sign up to be notified.',
			noindex: true
		}
	};
}
