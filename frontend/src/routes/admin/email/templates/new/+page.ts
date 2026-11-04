import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'New Email Template | Admin',
		description:
			'Create a saved email template with a subject, HTML body, plain text alternative, variables and explicit delivery status.'
	} satisfies SEOInput
});
