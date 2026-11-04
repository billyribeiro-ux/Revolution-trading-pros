import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Edit CRM Deal | Admin',
		description:
			'Update a sales deal, its value, pipeline stage, expected close date and linked contact or company in the administration dashboard.'
	} satisfies SEOInput
});
