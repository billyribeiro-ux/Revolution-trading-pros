import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Edit CRM Campaign | Admin',
		description:
			'Edit a saved email campaign with its full content, sender, template configuration and campaign tracking settings.'
	} satisfies SEOInput
});
