import type { Load } from '@sveltejs/kit';
import type { SEOInput } from '#lib/seo/types.js';

export const load: Load = () => {
	const seo: SEOInput = {
		title: 'Behavior Analytics',
		description: 'Enterprise behavioral analytics dashboard.',
		robots: { index: false, follow: false }
	};
	return { seo };
};
