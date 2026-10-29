import type { SEOInput } from '#lib/seo/types.js';

export const ssr = false;
export const prerender = false;

export const load = () => ({
	seo: {
		title: 'Authenticating...',
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
