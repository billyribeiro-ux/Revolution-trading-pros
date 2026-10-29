import type { SEOInput } from '#lib/seo/types.js';

export const prerender = false;

export const load = () => ({
	seo: {
		title: 'Verify Email',
		description: 'Verify your email address to complete your Revolution Trading Pros registration.',
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
