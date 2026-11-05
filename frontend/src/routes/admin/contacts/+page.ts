import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Contacts | Admin',
		description:
			'Manage recorded CRM contacts, review current score evidence, and search saved customer relationships.',
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
