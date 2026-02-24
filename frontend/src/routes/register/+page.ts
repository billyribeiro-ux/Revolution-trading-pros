import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Register - Join Revolution Trading Pros',
		description: 'Create your Revolution Trading Pros account. Get started with live trading rooms, professional alerts, and expert-led courses.',
		canonical: '/register',
		og: { type: 'website' }
	};
	return { seo };
};
