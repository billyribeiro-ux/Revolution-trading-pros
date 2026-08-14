import type { SEOInput } from '#lib/seo/types.js';

const seo: SEOInput = {
	title: 'Cart',
	description: 'Review your Revolution Trading Pros cart before secure checkout.',
	canonical: '/cart',
	robots: {
		index: false,
		follow: false
	}
};

export const load = () => {
	return { seo };
};
