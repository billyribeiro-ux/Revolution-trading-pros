import type { PageLoad } from './$types';

export const prerender = true;
// trailingSlash: 'always' is often preferred for SEO/canonical consistency
export const trailingSlash = 'always'; 

export const load: PageLoad = async () => {
    return {
        // You can pass build-time data here if needed
    };
};