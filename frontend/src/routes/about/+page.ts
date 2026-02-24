import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Revolution Trading Pros | The #1 Supportive Live Trading Community',
		description: 'Join a professional trading floor that genuinely cares. Real trades, real-time voice guidance, and institutional data without the hype. Established 2018.',
		canonical: '/about',
		og: { type: 'website' }
	};
	return { seo };
};
