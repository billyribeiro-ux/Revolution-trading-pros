import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Members Management | Admin',
		description:
			'Search and manage member accounts, review subscriptions and apply member segments, tags and saved filters.'
	} satisfies SEOInput
});
