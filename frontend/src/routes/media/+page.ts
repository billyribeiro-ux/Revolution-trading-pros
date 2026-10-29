import type { SEOInput } from '#lib/seo/types.js';

export const prerender = false;

export const load = () => ({
	seo: {
		title: 'Media Library',
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
