import type { SEOInput } from '#lib/seo/types.js';

export const prerender = false;

export const load = () => ({
	seo: {
		title: 'Edit Post',
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
