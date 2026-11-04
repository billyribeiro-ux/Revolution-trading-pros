import type { SEOInput } from '#lib/seo/types.js';

export const load = () => ({
	seo: {
		title: 'CRM Companies | Admin',
		description:
			'Manage company records, search and filter accounts, and review contact and deal information in the CRM administration dashboard.'
	} satisfies SEOInput
});
