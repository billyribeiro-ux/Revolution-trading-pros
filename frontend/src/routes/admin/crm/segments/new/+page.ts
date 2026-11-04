import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'New Segment | Admin',
		description:
			'Build and maintain dynamic CRM contact segments using validated rules, saved contact fields, and current membership counts.'
	} satisfies SEOInput
});
