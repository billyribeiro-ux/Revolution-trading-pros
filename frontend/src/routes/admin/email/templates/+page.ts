import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Email Templates | Admin',
		description:
			'Search and manage saved email templates, review their status and preview the content before sending a test.'
	} satisfies SEOInput
});
