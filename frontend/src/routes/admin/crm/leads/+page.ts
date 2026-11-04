import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'CRM Leads | Admin',
		description:
			'Manage saved CRM contacts, review lead details, and maintain customer relationships in the administration dashboard.'
	} satisfies SEOInput
});
