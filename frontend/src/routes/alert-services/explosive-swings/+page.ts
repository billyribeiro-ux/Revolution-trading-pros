import type { PageLoad } from './$types';

// SEO Requirement: Prerender this page as static HTML for instant indexing.
export const prerender = true;
export const trailingSlash = 'always';

export const load: PageLoad = async () => {
    return {};
};