import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Exit Strategies Guide | Revolution Trading Pros',
		description: 'Learn professional exit strategies for locking in profits and managing risk.',
		canonical: '/guides/exit-strategies',
		og: { type: 'website' }
	};
	return { seo };
};
