/**
 * Classes Page Configuration - Apple ICT 7
 *
 * SSG Configuration:
 * - prerender: true (static page for SEO)
 * - ssr: true (server-side rendering)
 *
 * @version 1.0.0
 */

import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

// Enable static site generation for SEO
export const prerender = true;

// Enable server-side rendering
export const ssr = true;

export const load = () => ({
	seo: {
		title: 'Trading Classes & Courses',
		description:
			'Browse our trading courses and classes. Learn from professional traders and improve your trading skills with beginner to advanced programs.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Classes', url: `${SITE}/classes` }
			])
		]
	} satisfies SEOInput
});
