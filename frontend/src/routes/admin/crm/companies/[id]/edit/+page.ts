import type { SEOInput } from '#lib/seo/types.js';

export const load = () => ({
	seo: {
		title: 'Edit CRM Company | Admin',
		description:
			'Update a saved CRM company record and persist changes to its company details, website, industry, logo and description.'
	} satisfies SEOInput
});
