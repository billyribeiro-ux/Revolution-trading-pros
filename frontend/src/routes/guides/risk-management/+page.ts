import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Risk Management Guide | Revolution Trading Pros',
		description: 'Learn professional risk management techniques for consistent trading performance.',
		canonical: '/guides/risk-management',
		og: { type: 'website' }
	};
	return { seo };
};
