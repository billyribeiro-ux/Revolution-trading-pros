import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'New CRM Campaign | Admin',
		description:
			'Write an email campaign, choose a template, configure its sender and save the content before selecting recipients and scheduling delivery.'
	} satisfies SEOInput
});
