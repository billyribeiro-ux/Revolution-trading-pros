import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'New Contact List | Admin',
		description:
			'Manage a saved CRM contact list, update list details and visibility, and maintain its contact memberships in the administration dashboard.'
	} satisfies SEOInput
});
