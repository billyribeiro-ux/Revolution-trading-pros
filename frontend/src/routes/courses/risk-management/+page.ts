import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Risk Management Mastery | Revolution Trading',
		description: 'Learn professional risk management techniques used by institutional traders to preserve capital and maximize long-term profitability.',
		canonical: '/courses/risk-management',
		og: { type: 'website' }
	};
	return { seo };
};
