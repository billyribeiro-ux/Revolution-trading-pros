import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Position Sizing Guide | Revolution Trading Pros',
		description: 'Master position sizing strategies to protect capital and maximize returns.',
		canonical: '/guides/position-sizing',
		og: { type: 'website' }
	};
	return { seo };
};
