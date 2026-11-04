import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'New Email Template | Admin',
		description:
			'Manage reusable CRM email templates, saved HTML content, categories, and design configuration in the administration dashboard.'
	} satisfies SEOInput
});
