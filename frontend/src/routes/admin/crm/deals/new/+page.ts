import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'New CRM Deal | Admin',
		description:
			'Create a sales deal with an exact amount, pipeline, stage, expected close date and linked CRM contact or company.'
	} satisfies SEOInput
});
