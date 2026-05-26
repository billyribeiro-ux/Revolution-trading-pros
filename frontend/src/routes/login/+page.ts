import type { SEOInput } from '$lib/seo/types';

// Disable prerendering for auth page
export const prerender = false;

export const load = () => ({
	seo: {
		title: 'Login to Your Trading Account',
		description:
			'Access your Revolution Trading Pros account. Log in to your live trading rooms, real-time alerts, and expert courses dashboard.'
	} satisfies SEOInput
});
