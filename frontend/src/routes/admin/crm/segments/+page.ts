import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Dynamic Contact Segments | Admin',
		description:
			'Review dynamic CRM contact segments, maintain saved rules, refresh current counts, and browse contacts matching each segment.'
	} satisfies SEOInput
});
