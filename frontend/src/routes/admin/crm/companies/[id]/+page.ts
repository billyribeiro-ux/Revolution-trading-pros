import type { SEOInput } from '#lib/seo/types.js';

export const load = () => ({
	seo: {
		title: 'CRM Company Details | Admin',
		description:
			'Review a saved company record, inspect its business details and website, and open the company editor in the administration dashboard.'
	} satisfies SEOInput
});
