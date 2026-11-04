import type { SEOInput } from '#lib/seo/types.js';
export const prerender = false;
export const load = () => ({
	seo: {
		title: 'Email Subscribers',
		description:
			'Manage newsletter subscriber profiles, confirmation requests, tags and email preferences.',
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
