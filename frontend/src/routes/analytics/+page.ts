import type { Load } from '@sveltejs/kit';
import type { SEOInput } from '#lib/seo/types.js';

export const load: Load = () => {
	const seo: SEOInput = {
		title: 'Analytics Dashboard',
		description:
			'Enterprise analytics dashboard with real-time metrics, funnels, cohorts, and AI insights.',
		robots: { index: false, follow: false }
	};
	return { seo };
};
