import type { SEOInput } from '#lib/seo/types.js';

export const prerender = false;

export const load = ({ params }: { params: { slug: string } }) => ({
	seo: {
		title: `${params.slug.replace(/-/g, ' ')} — Learning Center`,
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
