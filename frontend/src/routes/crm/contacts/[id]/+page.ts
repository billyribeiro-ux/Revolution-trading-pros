import type { SEOInput } from '#lib/seo/types.js';

export const prerender = false;

export const load = () => ({
	seo: {
		title: 'Contact Profile | CRM',
		description:
			'Review recorded contact details, score evidence, memberships, notes and provider message outcomes.',
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
