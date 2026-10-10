import type { PageLoad } from './$types';
import type { SEOInput } from '#lib/seo/types.js';
import { generateOGMeta } from '#lib/options-calculator/utils/seo.js';

export const prerender = false;
export const ssr = false;

// Head tags come from the root layout's <Seo> (the site's single SEO owner).
export const load: PageLoad = () => {
	const { 'og:description': description } = generateOGMeta();

	const seo: SEOInput = {
		title: 'Shared Options Analysis',
		description,
		og: { type: 'website' },
		twitter: { card: 'summary_large_image', title: 'Shared Options Analysis' }
	};
	return { seo };
};
