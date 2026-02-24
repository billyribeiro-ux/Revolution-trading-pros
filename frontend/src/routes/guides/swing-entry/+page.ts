import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Swing Entry Guide | Revolution Trading Pros',
		description: 'Master swing trading entry techniques for optimal risk-reward setups.',
		canonical: '/guides/swing-entry',
		og: { type: 'website' }
	};
	return { seo };
};
