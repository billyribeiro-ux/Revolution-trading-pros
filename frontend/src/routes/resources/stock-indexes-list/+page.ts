import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Stock Indexes List | Revolution Trading Pros',
		description: 'Complete stock indexes list for active traders. Track major market indexes and benchmark performance.',
		canonical: '/resources/stock-indexes-list',
		og: { type: 'website' }
	};
	return { seo };
};
