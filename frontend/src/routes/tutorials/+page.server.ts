import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import type { SEOInput } from '#lib/seo/types.js';
import { readPublicVideos } from '#lib/server/seo-publications.js';
export const prerender = false;
export const load: PageServerLoad = async ({ url }) => {
	const raw = url.searchParams.get('page') ?? '1';
	if (!/^[1-9]\d{0,6}$/.test(raw)) error(400, 'Choose a positive video page');
	const videos = await readPublicVideos(Number(raw));
	if (videos.meta.current_page > videos.meta.last_page) error(404, 'Video page not found');
	return {
		videos,
		seo: {
			title: 'Trading Tutorials — Public Video Library',
			description:
				'Browse published educational trading videos with actual authors, publication dates, descriptions and duration. Trading involves a risk of loss.',
			canonical: '/tutorials',
			robots: { index: raw === '1', follow: true }
		} satisfies SEOInput
	};
};
