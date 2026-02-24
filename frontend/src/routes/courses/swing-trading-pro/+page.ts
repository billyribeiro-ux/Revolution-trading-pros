import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Swing Trading Pro | Revolution Trading',
		description: 'Master swing trading with professional strategies to capture multi-day moves and optimal risk-reward setups.',
		canonical: '/courses/swing-trading-pro',
		og: { type: 'website' }
	};
	return { seo };
};
