import type { SEOInput } from '#lib/seo/types.js';

export const load = () => ({
	seo: {
		title: 'Behavior Tracking | Admin',
		description:
			'Inspect recorded visitor interactions, page engagement, scroll depth, and behavior analytics in the administration dashboard.'
	} satisfies SEOInput
});
