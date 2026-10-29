import type { SEOInput } from '#lib/seo/types.js';

export const prerender = false;

export const load = ({ params }: { params: { slug: string } }) => ({
	seo: {
		title: `Day Trading Room — ${params.slug.replace(/-/g, ' ')}`,
		description:
			'Daily trading video updates and institutional market analysis from the Day Trading Room.',
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
