import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'CRM Deals | Admin',
		description:
			'Search and manage sales deals by pipeline, stage, status and currency, with recorded outcomes and server-side pagination.'
	} satisfies SEOInput
});
