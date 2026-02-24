import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Options Trading Fundamentals | Revolution Trading',
		description: 'Master options trading with comprehensive training on strategies, Greeks, volatility analysis, and institutional-grade risk management.',
		canonical: '/courses/options-trading',
		og: { type: 'website' }
	};
	return { seo };
};
