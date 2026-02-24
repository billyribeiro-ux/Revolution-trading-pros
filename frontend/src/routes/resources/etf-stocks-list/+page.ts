import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'ETF & Stocks List | Revolution Trading Pros',
		description: 'Comprehensive ETF and stocks list for active traders. Research and compare top trading instruments.',
		canonical: '/resources/etf-stocks-list',
		og: { type: 'website' }
	};
	return { seo };
};
