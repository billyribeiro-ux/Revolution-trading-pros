import type { SEOInput } from '#lib/seo/types.js';

export const prerender = false;

export const load = ({ params }: { params: { slug: string } }) => ({
	seo: {
		title: `Subscribe — ${params.slug.replace(/-/g, ' ')}`,
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
