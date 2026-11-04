import type { SEOInput } from '#lib/seo/types.js';
export const prerender = false;
export const load = () => ({
	seo: {
		title: 'Performance Measurements | Admin',
		description:
			'Inspect recorded browser observations, actual host measurements and cache statistics.',
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
