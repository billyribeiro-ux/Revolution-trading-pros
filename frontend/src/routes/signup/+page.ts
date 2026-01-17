/**
 * Signup Page Configuration - Apple ICT 7
 *
 * SSR Configuration:
 * - prerender: false (dynamic page with auth redirect)
 * - ssr: true (server-side rendering for SEO)
 *
 * @version 1.0.0
 */

// Disable prerendering - page has dynamic auth redirect
export const prerender = false;

// Enable server-side rendering for SEO
export const ssr = true;
