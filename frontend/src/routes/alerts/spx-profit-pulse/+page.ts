import type { Load } from '@sveltejs/kit';

// Critical: Render this page as static HTML at build time.
export const prerender = true;
export const trailingSlash = 'always';

export const load: Load = async () => {
	return {};
};
