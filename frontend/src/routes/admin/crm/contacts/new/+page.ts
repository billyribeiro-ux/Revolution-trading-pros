import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'New Contact | Admin',
		description:
			'Update saved CRM contact details, communication fields, company information, and lifecycle status in the administration dashboard.'
	} satisfies SEOInput
});
