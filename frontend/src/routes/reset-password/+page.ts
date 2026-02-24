import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Reset Password - Secure Account Recovery',
		description: 'Reset your Revolution Trading Pros account password securely. Create a new password for your trading account.',
		canonical: '/reset-password',
		robots: { index: false, follow: true },
		og: { type: 'website' }
	};
	return { seo };
};
