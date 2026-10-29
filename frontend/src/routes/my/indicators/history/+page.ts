import type { Load } from '@sveltejs/kit';
import type { SEOInput } from '#lib/seo/types.js';

export const load: Load = () => {
	const seo: SEOInput = {
		title: 'Download History — My Indicators',
		description: 'View your indicator download history.',
		robots: { index: false, follow: false }
	};
	return { seo };
};
