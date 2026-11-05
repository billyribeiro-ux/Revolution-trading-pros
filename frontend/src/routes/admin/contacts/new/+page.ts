import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'New Contact | Admin',
		description:
			'Create a saved CRM contact with recorded profile fields, status and lifecycle stage.',
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
