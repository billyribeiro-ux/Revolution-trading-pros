import type { PageLoad } from '@sveltejs/kit';

// SEO Requirement: Prerender this page as static HTML for instant indexing.
export const prerender = true;
export const trailingSlash = 'always';

export const load: PageLoad = async () => {
	return {};
};
