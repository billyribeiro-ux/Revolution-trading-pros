import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Trading Courses & Mentorship | Revolution Trading Pros',
		description: 'Institutional-grade trading education. Learn to read order flow, manage risk, and execute with precision. Join the top 1% of disciplined traders.',
		canonical: '/courses',
		og: { type: 'website' }
	};
	return { seo };
};
