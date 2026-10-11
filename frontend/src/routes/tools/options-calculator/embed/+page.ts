import type { PageLoad } from './$types';
import type { SEOInput } from '#lib/seo/types.js';

// Client-only: the page reads its state from the URL when its script runs (+page.svelte).
export const prerender = false;
export const ssr = false;

// Head tags come from the root layout's <Seo> (the site's single SEO owner). The widget
// lives inside third-party iframes, so it is kept out of search results.
export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Options Calculator',
		robots: { index: false }
	};
	return { seo };
};
