/**
 * Cookie Policy Page Server Configuration
 *
 * Disable prerendering since this page uses browser-only APIs
 * for scanning cookies and displaying consent information.
 */

export const prerender = false;
export const ssr = false;
