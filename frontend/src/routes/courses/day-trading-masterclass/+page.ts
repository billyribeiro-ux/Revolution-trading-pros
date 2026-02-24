import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Day Trading Masterclass | Revolution Trading',
		description: 'Master day trading with institutional-grade strategies, real-time execution tactics, and professional risk management.',
		canonical: '/courses/day-trading-masterclass',
		og: { type: 'website' }
	};
	return { seo };
};
