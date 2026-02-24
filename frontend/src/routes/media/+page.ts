import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Media | Revolution Trading Pros',
		description: 'Media coverage, press mentions, and appearances by Revolution Trading Pros.',
		canonical: '/media',
		og: { type: 'website' }
	};
	return { seo };
};
