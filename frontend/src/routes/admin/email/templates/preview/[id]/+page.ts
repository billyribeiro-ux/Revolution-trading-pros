import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Email Template Preview | Admin',
		description:
			'Preview saved email content with explicit sample data and inspect the rendered subject, HTML and plain text alternative.'
	} satisfies SEOInput
});
