import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

// Disable prerendering for auth page
export const prerender = false;

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Login to Your Trading Account | Revolution Trading Pros',
		description: 'Access your Revolution Trading Pros account. Log in to your live trading rooms, real-time alerts, and expert courses dashboard.',
		canonical: '/login',
		og: { type: 'website' }
	};
	return { seo };
};
