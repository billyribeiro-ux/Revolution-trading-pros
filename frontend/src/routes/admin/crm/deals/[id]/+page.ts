import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'CRM Deal Details | Admin',
		description:
			'Review a sales deal, its value, linked records, stage history, recorded activity and notes in the administration dashboard.'
	} satisfies SEOInput
});
