import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Trading Alert Services | Revolution Trading Pros',
		description: 'Get premium trading alerts delivered to your phone. Swing trading and day trading alerts with precise entries, exits, and risk management.',
		canonical: '/alerts',
		og: { type: 'website' }
	};
	return { seo };
};
