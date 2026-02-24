import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Forgot Password - Reset Your Account',
		description: 'Forgot your password? Reset your Revolution Trading Pros account password securely.',
		canonical: '/forgot-password',
		robots: { index: false, follow: true },
		og: { type: 'website' }
	};
	return { seo };
};
