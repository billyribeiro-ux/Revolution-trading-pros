import type { SEOInput } from '#lib/seo/types.js';

export const load = () => ({
	seo: {
		title: 'New CRM Company | Admin',
		description:
			'Create a CRM company record with its website, industry, company size, logo and description in the administration dashboard.'
	} satisfies SEOInput
});
