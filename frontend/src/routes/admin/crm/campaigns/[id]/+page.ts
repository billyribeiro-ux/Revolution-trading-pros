import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'CRM Campaign Report | Admin',
		description:
			'Review saved campaign content, delivery status and recorded email activity, or duplicate the campaign for another draft.'
	} satisfies SEOInput
});
