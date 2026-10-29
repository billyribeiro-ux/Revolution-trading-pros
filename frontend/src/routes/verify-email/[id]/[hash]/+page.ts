import type { SEOInput } from '#lib/seo/types.js';

// Disable prerendering for dynamic route
export const prerender = false;

export const load = () => ({
	seo: {
		title: 'Verify Email',
		description: 'Verify your email address to complete your Revolution Trading Pros registration.',
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
