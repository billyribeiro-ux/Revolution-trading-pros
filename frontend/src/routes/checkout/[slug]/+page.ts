import type { SEOInput } from '#lib/seo/types.js';

export const prerender = false;

export const load = () => ({
	seo: {
		title: 'Checkout',
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
