import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Edit Lead | Admin',
		description:
			'Update saved CRM contact details, communication fields, company information, and lifecycle status in the administration dashboard.'
	} satisfies SEOInput
});
