import type { SEOInput } from '#lib/seo/types.js';
export const prerender = false;
export const load = () => ({
	seo: {
		title: 'CRM Settings',
		description:
			'Review and update saved CRM subscription confirmation and email preference configuration.',
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
