import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Our Mission | The Institutional Bridge',
		description: 'Dismantling the retail trader stereotype. We bridge the gap between gambling and institutional risk management through data, discipline, and transparency.',
		canonical: '/our-mission',
		og: { type: 'website' }
	};
	return { seo };
};
