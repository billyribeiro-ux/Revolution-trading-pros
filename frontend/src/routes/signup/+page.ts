/**
 * Signup Page Configuration - Apple ICT 7
 *
 * SSR Configuration:
 * - prerender: false (dynamic page with auth redirect)
 * - ssr: true (server-side rendering for SEO)
 *
 * @version 1.0.0
 */

import type { SEOInput } from '$lib/seo/types';

// Disable prerendering - page has dynamic auth redirect
export const prerender = false;

// Enable server-side rendering for SEO
export const ssr = true;

export const load = () => ({
	seo: {
		title: 'Sign Up — Create Your Trading Account',
		description:
			'Join Revolution Trading Pros. Create your free account to access live trading rooms, professional alerts, courses, and our trading community.'
	} satisfies SEOInput
});
