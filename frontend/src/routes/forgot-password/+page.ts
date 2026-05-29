import type { SEOInput } from '$lib/seo/types';

export const load = () => ({
	seo: {
		title: 'Forgot Password — Reset Your Account',
		description: 'Reset your Revolution Trading Pros account password securely.'
	} satisfies SEOInput
});
