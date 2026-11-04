import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Edit Email Template | Admin',
		description:
			'Edit saved email template content, preserve its identity, and manage variables, plain text alternatives and active status.'
	} satisfies SEOInput
});
