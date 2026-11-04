import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'CRM Email Campaigns | Admin',
		description:
			'Search email campaigns, review recorded delivery counts, and create, edit, duplicate or delete saved campaign content.'
	} satisfies SEOInput
});
