/**
 * OAuth Callback Page Load - ICT Level 7 Principal Engineer Grade
 *
 * Disables SSR for the callback page to ensure client-side handling.
 * This is necessary because the callback needs to:
 * 1. Access URL parameters client-side
 * 2. Store tokens in memory (not accessible server-side)
 * 3. Redirect using client-side navigation
 *
 * @version 1.0.0 - January 2026
 */

export const ssr = false;
export const prerender = false;
