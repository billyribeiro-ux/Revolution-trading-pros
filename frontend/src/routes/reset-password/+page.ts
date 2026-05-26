import type { SEOInput } from '$lib/seo/types';

export const load = () => ({
	seo: {
		title: 'Reset Password — Secure Account Recovery',
		description:
			'Reset your Revolution Trading Pros account password securely. Create a new password for your trading account.'
	} satisfies SEOInput
});
