import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'News Sitemap Management',
		description:
			'Manage saved publication settings, review actual eligible sources and record sitemap generations.',
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
